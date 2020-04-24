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
use search_client::{
    models::{IndexEntry, IndexResult},
    CreateIndexEntry, DeleteIndexEntry, Search,
};
use std::time::Duration;
use storage_client::DeleteBlob;
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

    let error_message = error.to_string();

    match error {
        ProcessMessageError::DocumentNotFoundInIndex(id) => {
            tracing::warn!(
                "Document {} wasn't found during delete, removing message",
                id
            );

            state_manager
                .set_status(
                    removeable_message.get_message().job_id,
                    JobStatus::Error {
                        message: error_message,
                        code: "".to_string(),
                    },
                )
                .await?;
            let _remove = removeable_message.remove().await?;
        }
        ProcessMessageError::FailedRestoringIndex(_, _) => {
            tracing::error!("{}", error_message);
            state_manager
                .set_status(
                    removeable_message.get_message().job_id,
                    JobStatus::Error {
                        message: error_message,
                        code: "".to_string(),
                    },
                )
                .await?;
            let _remove = removeable_message.remove().await?;
        }
        ProcessMessageError::FailedDeletingBlob(_, _) => {
            tracing::error!("{}", error_message);
        }
        _ => {}
    }

    Ok(())
}

pub async fn process_message(message: DeleteMessage) -> Result<Uuid, ProcessMessageError> {
    tracing::info!("Message received: {:?} ", message);

    let search_client = search_client::factory();
    let storage_client = storage_client::factory()
        .map_err(|e| anyhow!("Couldn't create storage client: {:?}", e))?;

    process_delete_message(message, storage_client, search_client).await
}

async fn process_delete_message(
    message: DeleteMessage,
    mut storage_client: impl DeleteBlob,
    search_client: impl Search + DeleteIndexEntry + CreateIndexEntry,
) -> Result<Uuid, ProcessMessageError> {
    let storage_container_name = std::env::var("STORAGE_CONTAINER").map_err(anyhow::Error::from)?;

    let index_record: IndexResult =
        get_index_record_from_content_id(message.document_content_id.clone(), &search_client)
            .await?;
    let blob_name = index_record.metadata_storage_name.clone();

    tracing::debug!(
        "Found blob name {} for document content ID {} from index",
        &blob_name,
        &message.document_content_id
    );

    search_client
        .delete_index_entry(&"metadata_storage_name".to_string(), &blob_name)
        .await?;
    tracing::debug!("Deleted blob {} from index", &blob_name);

    if let Err(e) = storage_client
        .delete_blob(&storage_container_name, &blob_name)
        .await
    {
        tracing::debug!(
            "Error deleting blob: {:?}, re-creating index: {:?}",
            e,
            &index_record
        );

        search_client
            .create_index_entry(IndexEntry::from(index_record.clone()))
            .await
            .map_err(|err| {
                ProcessMessageError::FailedRestoringIndex(blob_name.clone(), err.to_string())
            })?;
        return Err(ProcessMessageError::FailedDeletingBlob(
            blob_name.clone(),
            e.to_string(),
        ));
    }

    tracing::info!(
        "Successfully deleted blob {} from storage container {}",
        &blob_name,
        &storage_container_name
    );

    Ok(message.job_id)
}

pub async fn get_index_record_from_content_id(
    content_id: String,
    search_client: &impl Search,
) -> Result<IndexResult, ProcessMessageError> {
    let search_results = search_client
        .search(&content_id)
        .await
        .map_err(anyhow::Error::from)?;
    for result in search_results.search_results {
        if result.file_name == content_id {
            return Ok(result);
        }
    }
    Err(ProcessMessageError::DocumentNotFoundInIndex(content_id))
}

#[cfg(test)]
mod test {
    use super::*;
    use azure_sdk_core::errors::AzureError;
    use pretty_assertions::assert_eq;

    use crate::{
        models::DeleteMessage, service_bus_client::test::TestRemoveableMessage,
        state_manager::test::TestJobStatusClient,
    };
    use search_client::{
        models::{AzureIndexChangedResult, AzureIndexChangedResults, IndexResult, IndexResults},
        Search,
    };
    use std::env;
    use tokio_test::block_on;

    #[test]
    fn not_found_error_during_delete_removes_message_since_no_need_to_retry() {
        let state_manager = given_a_state_manager();
        let mut removeable_message = given_we_have_a_delete_message();
        let error = given_document_not_found_in_index();

        block_on(handle_processing_error_for_delete_message(
            &mut removeable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        assert_eq!(
            removeable_message.remove_was_called, true,
            "Didn't remove message, but should"
        );
    }

    #[test]
    fn not_found_error_during_delete_sets_job_status_as_error() {
        let state_manager = given_a_state_manager();
        let mut removeable_message = given_we_have_a_delete_message();
        let error = given_document_not_found_in_index();

        block_on(handle_processing_error_for_delete_message(
            &mut removeable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        let result =
            block_on(state_manager.get_status(removeable_message.get_message().job_id)).unwrap();
        assert_eq!(
            result.status,
            JobStatus::Error {
                message: String::from("Cannot find document with ID any id"),
                code: String::from(""),
            },
        );
    }

    #[test]
    fn recoverable_error_during_delete_does_not_remove_message_from_servicebus() {
        let state_manager = given_a_state_manager();
        let mut removeable_message = given_we_have_a_delete_message();
        let error = given_an_unknown_error();

        block_on(handle_processing_error_for_delete_message(
            &mut removeable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        assert_eq!(
            removeable_message.remove_was_called, false,
            "Removed message, but shouldn't"
        );
    }

    #[test]
    fn recoverable_error_during_delete_leaves_job_status_as_accepted() {
        let state_manager = given_a_state_manager();
        let mut removeable_message = given_we_have_a_delete_message();
        given_the_delete_job_is_accepted(removeable_message.get_message().job_id, &state_manager);
        let error = given_an_unknown_error();

        block_on(handle_processing_error_for_delete_message(
            &mut removeable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        let result =
            block_on(state_manager.get_status(removeable_message.get_message().job_id)).unwrap();
        assert_eq!(result.status, JobStatus::Accepted);
    }

    #[test]
    fn failure_to_delete_blob_leaves_job_status_as_accepted() {
        let state_manager = given_a_state_manager();
        let mut removeable_message = given_we_have_a_delete_message();
        given_the_delete_job_is_accepted(removeable_message.get_message().job_id, &state_manager);
        let error = given_a_delete_blob_error();

        block_on(handle_processing_error_for_delete_message(
            &mut removeable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        let result =
            block_on(state_manager.get_status(removeable_message.get_message().job_id)).unwrap();
        assert_eq!(result.status, JobStatus::Accepted);
    }

    #[test]
    fn failure_to_delete_blob_does_not_remove_message_from_service_bus() {
        let state_manager = given_a_state_manager();
        let mut removeable_message = given_we_have_a_delete_message();
        let error = given_a_delete_blob_error();

        block_on(handle_processing_error_for_delete_message(
            &mut removeable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        assert_eq!(
            removeable_message.remove_was_called, false,
            "Removed message, but shouldn't"
        );
    }

    #[test]
    fn failure_to_restore_index_removes_message_since_cannot_be_retried() {
        let state_manager = given_a_state_manager();
        let mut removeable_message = given_we_have_a_delete_message();
        let blob_id = "Blob Id".to_string();
        let error = given_failure_to_restore_index(blob_id);

        block_on(handle_processing_error_for_delete_message(
            &mut removeable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        assert_eq!(
            removeable_message.remove_was_called, true,
            "Didn't remove message, but should"
        );
    }

    #[test]
    fn failure_to_restore_index_leaves_job_status_as_error() {
        let state_manager = given_a_state_manager();
        let mut removeable_message = given_we_have_a_delete_message();
        let blob_id = "Blob Id".to_string();
        let error = given_failure_to_restore_index(blob_id);

        block_on(handle_processing_error_for_delete_message(
            &mut removeable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        let result =
            block_on(state_manager.get_status(removeable_message.get_message().job_id)).unwrap();

        let expected = JobStatus::Error {
            message: String::from("Cannot restore index for blob with ID Blob Id: Error message"),
            code: String::from(""),
        };

        assert_eq!(result.status, expected);
    }

    #[test]
    fn index_with_doc_returns_success() {
        let removeable_message = given_we_have_a_delete_message().message;
        let search_client = given_a_search_client_that_returns_results();
        let storage_client = given_a_storage_client();
        given_the_necessary_env_vars_are_initialised();

        let result = block_on(process_delete_message(
            removeable_message,
            storage_client,
            search_client,
        ));

        assert_eq!(result.is_err(), false);
    }

    #[test]
    fn failure_to_delete_blob_returns_expected_error() {
        let removeable_message = given_we_have_a_delete_message().message;
        let search_client = given_a_search_client_that_returns_results();
        let storage_client = given_a_storage_client_that_cannot_delete_blob();
        given_the_necessary_env_vars_are_initialised();

        let result = block_on(process_delete_message(
            removeable_message,
            storage_client,
            search_client,
        ));

        match result {
            Ok(_) => panic!("Error expected"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    ProcessMessageError::FailedDeletingBlob(
                        "storage_name".to_string(),
                        "Generic error: blob could not be deleted".to_string()
                    )
                    .to_string()
                );
            }
        }
    }

    #[test]
    fn failure_to_restore_index_returns_expected_error() {
        let removeable_message = given_we_have_a_delete_message().message;
        let search_client = given_a_search_client_that_cannot_restore_index();
        let storage_client = given_a_storage_client_that_cannot_delete_blob();
        given_the_necessary_env_vars_are_initialised();

        let result = block_on(process_delete_message(
            removeable_message,
            storage_client,
            search_client,
        ));

        match result {
            Ok(_) => panic!("Error expected"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    ProcessMessageError::FailedRestoringIndex(
                        "storage_name".to_string(),
                        "Index could not be created".to_string()
                    )
                    .to_string()
                );
            }
        }
    }

    #[test]
    fn failure_to_delete_index_returns_expected_error() {
        let removeable_message = given_we_have_a_delete_message().message;
        let search_client = given_a_search_client_that_cannot_delete_index();
        let storage_client = given_a_storage_client();
        given_the_necessary_env_vars_are_initialised();

        let result = block_on(process_delete_message(
            removeable_message,
            storage_client,
            search_client,
        ));

        match result {
            Ok(_) => panic!("Error expected"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    ProcessMessageError::Generic(anyhow!("Index could not be deleted")).to_string()
                );
            }
        }
    }

    fn given_the_necessary_env_vars_are_initialised() {
        env::set_var("STORAGE_CONTAINER", "storage_container");
    }

    fn given_document_not_found_in_index() -> ProcessMessageError {
        ProcessMessageError::DocumentNotFoundInIndex("any id".to_owned())
    }

    fn given_an_unknown_error() -> ProcessMessageError {
        anyhow!("Any other error").into()
    }

    fn given_a_delete_blob_error() -> ProcessMessageError {
        ProcessMessageError::FailedDeletingBlob("Blob Id".to_string(), "Error message".to_string())
    }

    fn given_failure_to_restore_index(blob_id: String) -> ProcessMessageError {
        ProcessMessageError::FailedRestoringIndex(blob_id, "Error message".to_string())
    }

    fn given_an_index_search_result() -> IndexResult {
        IndexResult {
            doc_type: "Spc".to_string(),
            file_name: "our_id".to_string(),
            metadata_storage_name: "storage_name".to_string(),
            metadata_storage_path: "test/path".to_string(),
            product_name: Some("product".to_string()),
            substance_name: vec!["substance".to_string()],
            title: "title".to_string(),
            created: None,
            facets: vec!["facet".to_string()],
            keywords: None,
            metadata_storage_size: 300,
            release_state: None,
            rev_label: None,
            suggestions: vec!["suggestion".to_string()],
            score: 1.0,
            highlights: None,
        }
    }

    fn given_a_state_manager() -> TestJobStatusClient {
        TestJobStatusClient::accepted()
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

    fn given_the_delete_job_is_accepted(id: Uuid, state_manager: &TestJobStatusClient) {
        let _ = block_on(state_manager.set_status(id, JobStatus::Accepted));
    }

    #[test]
    fn get_blob_name_from_content_id_raises_document_not_found_in_index_error_when_not_there() {
        let search_client = given_a_search_client_that_returns_no_results();
        let result = when_getting_blob_name_from_content_id(search_client);
        then_document_not_found_in_index_error_is_raised(result);
    }

    fn given_a_search_client_that_returns_no_results() -> impl Search {
        TestAzureSearchClient {
            can_insert_index: true,
            can_delete_index: true,
            search_results: vec![],
        }
    }

    fn given_a_search_client_that_returns_results(
    ) -> impl Search + CreateIndexEntry + DeleteIndexEntry {
        TestAzureSearchClient {
            can_insert_index: true,
            can_delete_index: true,
            search_results: vec![given_an_index_search_result()],
        }
    }

    fn given_a_search_client_that_cannot_restore_index(
    ) -> impl Search + CreateIndexEntry + DeleteIndexEntry {
        TestAzureSearchClient {
            can_insert_index: false,
            can_delete_index: true,
            search_results: vec![given_an_index_search_result()],
        }
    }

    fn given_a_search_client_that_cannot_delete_index(
    ) -> impl Search + CreateIndexEntry + DeleteIndexEntry {
        TestAzureSearchClient {
            can_insert_index: true,
            can_delete_index: false,
            search_results: vec![given_an_index_search_result()],
        }
    }

    fn given_a_storage_client() -> impl DeleteBlob {
        TestAzureStorageClient {
            can_delete_blob: true,
        }
    }

    fn given_a_storage_client_that_cannot_delete_blob() -> impl DeleteBlob {
        TestAzureStorageClient {
            can_delete_blob: false,
        }
    }

    fn when_getting_blob_name_from_content_id(
        search_client: impl Search,
    ) -> Result<String, ProcessMessageError> {
        Ok(when_getting_index_record_from_content_id(search_client)?.metadata_storage_name)
    }

    fn when_getting_index_record_from_content_id(
        search_client: impl Search,
    ) -> Result<IndexResult, ProcessMessageError> {
        block_on(get_index_record_from_content_id(
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

    struct TestAzureSearchClient {
        pub search_results: Vec<IndexResult>,
        pub can_insert_index: bool,
        pub can_delete_index: bool,
    }

    #[async_trait]
    impl Search for TestAzureSearchClient {
        async fn search(&self, _search_term: &str) -> Result<IndexResults, reqwest::Error> {
            Ok(IndexResults {
                search_results: self.search_results.clone(),
                context: String::from(""),
                count: None,
            })
        }
        async fn filter_by_collection_field(
            &self,
            _field_name: &str,
            _field_value: &str,
        ) -> Result<IndexResults, reqwest::Error> {
            unimplemented!()
        }
    }

    #[async_trait]
    impl DeleteIndexEntry for TestAzureSearchClient {
        async fn delete_index_entry(
            &self,
            key_name: &str,
            _value: &str,
        ) -> Result<AzureIndexChangedResults, anyhow::Error> {
            if !&self.can_delete_index {
                return Err(anyhow!("Index could not be deleted"));
            }

            let index_changed_result = AzureIndexChangedResult {
                key: key_name.to_string(),
                status: true,
                error_message: None,
                status_code: 200,
            };

            Ok(AzureIndexChangedResults::new(index_changed_result))
        }
    }

    #[async_trait]
    impl CreateIndexEntry for TestAzureSearchClient {
        async fn create_index_entry(
            &self,
            _key_values: IndexEntry,
        ) -> Result<AzureIndexChangedResults, anyhow::Error> {
            if !&self.can_insert_index {
                return Err(anyhow!("Index could not be created"));
            }

            let index_changed_result = AzureIndexChangedResult {
                key: "key".to_string(),
                status: true,
                error_message: None,
                status_code: 200,
            };

            Ok(AzureIndexChangedResults::new(index_changed_result))
        }
    }

    struct TestAzureStorageClient {
        pub can_delete_blob: bool,
    }

    #[async_trait]
    impl DeleteBlob for TestAzureStorageClient {
        async fn delete_blob(
            &mut self,
            _container_name: &str,
            _blob_name: &str,
        ) -> Result<(), AzureError> {
            if self.can_delete_blob {
                Ok(())
            } else {
                Err(AzureError::GenericErrorWithText(
                    "blob could not be deleted".to_string(),
                ))
            }
        }
    }
}
