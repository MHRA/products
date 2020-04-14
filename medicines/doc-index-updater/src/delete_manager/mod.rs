use crate::{
    models::{DeleteMessage, JobStatus},
    service_bus_client::{
        delete_factory, ProcessMessageError, ProcessRetrievalError, RemoveableMessage,
        RetrievedMessage,
    },
    state_manager::{JobStatusClient, StateManager},
    storage_client,
};
use anyhow::anyhow;
use async_trait::async_trait;
use azure_sdk_core::{errors::AzureError, prelude::*, DeleteSnapshotsMethod};
use azure_sdk_storage_blob::prelude::*;
use search_client::{Deletable, Searchable};
use std::time::Duration;
use tokio::time::delay_for;
use uuid::Uuid;

pub async fn delete_service_worker(
    time_to_wait: Duration,
    state_manager: StateManager,
) -> Result<String, anyhow::Error> {
    tracing::info!("Starting delete service worker");
    let mut delete_client = delete_factory()
        .await
        .map_err(|e| anyhow!("Couldn't create service bus client: {:?}", e))?;

    loop {
        match delete_client
            .try_process_from_queue::<DeleteMessage>(&state_manager)
            .await
        {
            Ok(()) => {}
            Err(e) => tracing::error!("{:?}", e),
        }
        delay_for(time_to_wait).await;
    }
}

#[async_trait]
impl ProcessRetrievalError for RetrievedMessage<DeleteMessage> {
    async fn handle_processing_error(
        &mut self,
        error: ProcessMessageError,
        state_manager: &impl JobStatusClient,
    ) -> anyhow::Result<()> {
        handle_processing_error_for_delete_message(self, error, state_manager).await
    }
}

async fn handle_processing_error_for_delete_message<T>(
    removeable_message: &mut T,
    error: ProcessMessageError,
    state_manager: &impl JobStatusClient,
) -> anyhow::Result<()>
where
    T: RemoveableMessage<DeleteMessage>,
{
    tracing::info!("Handling processing error. Setting error state in state manager");
    state_manager
        .set_status(
            removeable_message.get_message().job_id,
            JobStatus::Error {
                message: error.to_string(),
                code: "".to_string(),
            },
        )
        .await?;

    if let ProcessMessageError::DocumentNotFoundInIndex(id) = error {
        tracing::info!(
            "Document {} wasn't found during delete, removing message",
            id
        );
        let _remove = removeable_message.remove().await?;
    }
    Ok(())
}

pub async fn process_message(message: DeleteMessage) -> Result<Uuid, ProcessMessageError> {
    tracing::info!("Message received: {:?} ", message);

    let search_client = search_client::factory();
    let storage_client = storage_client::factory()
        .map_err(|e| anyhow!("Couldn't create storage client: {:?}", e))?;

    let storage_container_name = std::env::var("STORAGE_CONTAINER").map_err(anyhow::Error::from)?;
    let blob_name =
        get_blob_name_from_content_id(message.document_content_id.clone(), &search_client).await?;

    tracing::info!(
        "Found blob name {} for document content ID {} from index",
        &blob_name,
        &message.document_content_id
    );
    delete_from_index(search_client, &blob_name).await?;
    tracing::info!("Deleted blob {} from index", &blob_name);
    delete_blob(&storage_client, &storage_container_name, &blob_name)
        .await
        .map_err(|e| {
            tracing::error!("Error deleting blob: {:?}", e);
            anyhow!("Couldn't delete blob {}", &blob_name)
        })?;
    tracing::info!(
        "Deleted blob {} from storage container {}",
        &blob_name,
        &storage_container_name
    );

    Ok(message.job_id)
}

pub async fn get_blob_name_from_content_id(
    content_id: String,
    search_client: &impl Searchable,
) -> Result<String, ProcessMessageError> {
    let search_results = search_client
        .search(content_id.to_owned())
        .await
        .map_err(anyhow::Error::from)?;
    for result in search_results.search_results {
        if result.file_name == content_id {
            return Ok(result.metadata_storage_name);
        }
    }
    Err(ProcessMessageError::DocumentNotFoundInIndex(content_id))
}

async fn delete_blob(
    storage_client: &azure_sdk_storage_core::prelude::Client,
    container_name: &str,
    blob_name: &str,
) -> Result<(), AzureError> {
    storage_client
        .delete_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .finalize()
        .await?;
    Ok(())
}

pub async fn delete_from_index(
    search_client: impl Deletable,
    blob_name: &str,
) -> Result<(), anyhow::Error> {
    search_client
        .delete(&"metadata_storage_name".to_string(), &blob_name)
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    use crate::{
        models::DeleteMessage, service_bus_client::test::TestRemoveableMessage,
        state_manager::TestJobStatusClient,
    };
    use search_client::{models::AzureSearchResults, Searchable};
    use tokio_test::block_on;

    #[test]
    fn not_found_error_during_delete_removes_message_since_no_need_to_retry() {
        let state_manager = given_a_state_manager();
        let mut removeable_message = given_we_have_a_delete_message();
        let error = given_document_not_found_in_index();
        let result = when_we_handle_the_error(&mut removeable_message, error, state_manager);
        then_message_is_removed(result, removeable_message);
    }

    #[test]
    fn recoverable_error_during_delete_does_not_remove_message_from_servicebus() {
        let state_manager = given_a_state_manager();
        let mut removeable_message = given_we_have_a_delete_message();
        let error = given_an_unknown_error();
        let result = when_we_handle_the_error(&mut removeable_message, error, state_manager);
        then_message_is_not_removed(result, removeable_message);
    }

    fn given_document_not_found_in_index() -> ProcessMessageError {
        ProcessMessageError::DocumentNotFoundInIndex("any id".to_owned())
    }

    fn given_an_unknown_error() -> ProcessMessageError {
        anyhow!("Any other error").into()
    }

    fn given_a_state_manager() -> impl JobStatusClient {
        TestJobStatusClient {}
    }

    fn given_we_have_a_delete_message() -> TestRemoveableMessage<DeleteMessage> {
        let delete_message = DeleteMessage {
            document_content_id: "our_id".to_owned(),
            job_id: Uuid::new_v4(),
        };

        TestRemoveableMessage::<DeleteMessage> {
            remove_was_called: false,
            message: delete_message,
        }
    }

    fn when_we_handle_the_error(
        removeable_message: &mut TestRemoveableMessage<DeleteMessage>,
        error: ProcessMessageError,
        state_manager: impl JobStatusClient,
    ) -> Result<(), anyhow::Error> {
        block_on(handle_processing_error_for_delete_message(
            removeable_message,
            error,
            &state_manager,
        ))
    }

    fn then_message_is_removed(
        result: Result<(), anyhow::Error>,
        removeable_message: TestRemoveableMessage<DeleteMessage>,
    ) {
        assert!(result.is_ok());
        assert_eq!(
            removeable_message.remove_was_called, true,
            "Didn't remove message, but should"
        );
    }

    fn then_message_is_not_removed(
        result: Result<(), anyhow::Error>,
        removeable_message: TestRemoveableMessage<DeleteMessage>,
    ) {
        assert!(result.is_ok());
        assert_eq!(
            removeable_message.remove_was_called, false,
            "Removed message, but shouldn't"
        );
    }

    #[test]
    fn get_blob_name_from_content_id_raises_document_not_found_in_index_error_when_not_there() {
        let search_client = given_a_search_client_that_returns_no_results();
        let result = when_getting_blob_name_from_content_id(search_client);
        then_document_not_found_in_index_error_is_raised(result);
    }

    fn given_a_search_client_that_returns_no_results() -> impl Searchable {
        TestAzureSearchClientWithNoResults {}
    }

    fn when_getting_blob_name_from_content_id(
        search_client: impl Searchable,
    ) -> Result<String, ProcessMessageError> {
        block_on(get_blob_name_from_content_id(
            String::from("non existent content id"),
            &search_client,
        ))
    }

    fn then_document_not_found_in_index_error_is_raised(
        result: Result<String, ProcessMessageError>,
    ) {
        assert_eq!(result.is_err(), true);

        assert!(
            if let Err(ProcessMessageError::DocumentNotFoundInIndex(_)) = result {
                true
            } else {
                false
            },
            format!(
                "Should have been an error with type: DocumentNotFoundInIndex, but was {:?}",
                result
            )
        );
    }

    struct TestAzureSearchClientWithNoResults {}

    #[async_trait]
    impl Searchable for TestAzureSearchClientWithNoResults {
        async fn search(&self, _search_term: String) -> Result<AzureSearchResults, reqwest::Error> {
            Ok(AzureSearchResults {
                search_results: vec![],
                context: String::from(""),
                count: None,
            })
        }
    }
}
