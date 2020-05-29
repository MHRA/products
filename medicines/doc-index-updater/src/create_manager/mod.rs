use crate::{
    audit_logger::{AuditLogger, LogTransaction},
    create_manager::models::BlobMetadata,
    models::{CreateMessage, JobStatus},
    service_bus_client::{
        create_factory, ProcessMessageError, ProcessRetrievalError, RemovableMessage,
        RetrievedMessage,
    },
    state_manager::{JobStatusClient, StateManager},
    storage_client::{
        models::{SftpError, StorageClientError},
        AzureBlobStorage, StorageClient,
    },
};
use anyhow::anyhow;
use async_trait::async_trait;
use search_index::add_blob_to_search_index;
use std::{collections::HashMap, time::Duration};
use tokio::time::delay_for;
use uuid::Uuid;

pub mod hash;
pub mod models;
mod retrieve;
mod sanitiser;
mod search_index;

pub async fn create_service_worker(
    time_to_wait: Duration,
    state_manager: StateManager,
) -> Result<(), anyhow::Error> {
    tracing::info!("Starting create service worker");
    let mut create_client = create_factory()
        .await
        .map_err(|e| anyhow!("Couldn't create service bus client: {:?}", e))?;

    loop {
        match create_client
            .try_process_from_queue::<CreateMessage>(&state_manager)
            .await
        {
            Ok(()) => {}
            Err(e) => tracing::error!("{:?}", e),
        }
        delay_for(time_to_wait).await;
    }
}

#[async_trait]
impl ProcessRetrievalError for RetrievedMessage<CreateMessage> {
    async fn handle_processing_error(
        &mut self,
        error: ProcessMessageError,
        state_manager: &impl JobStatusClient,
    ) -> anyhow::Result<()> {
        handle_processing_error_for_create_message(self, error, state_manager).await
    }
}

async fn handle_processing_error_for_create_message<T>(
    removable_message: &mut T,
    error: ProcessMessageError,
    state_manager: &impl JobStatusClient,
) -> anyhow::Result<()>
where
    T: RemovableMessage<CreateMessage>,
{
    if let ProcessMessageError::StorageClientError(StorageClientError::SftpError(
        SftpError::CouldNotRetrieveFile,
    )) = error
    {
        tracing::warn!("Couldn't find file. Updating state to Error and removing message.");
        let _ = state_manager
            .set_status(
                removable_message.get_message().job_id,
                JobStatus::Error {
                    message: "Couldn't find file".to_string(),
                    code: "404".to_string(),
                },
            )
            .await?;
        let _ = removable_message.remove().await?;
    }
    Ok(())
}

pub async fn process_message(message: CreateMessage) -> Result<Uuid, ProcessMessageError> {
    tracing::debug!("Message received: {:?} ", &message);

    let search_client = search_client::factory();

    let message_for_log = message.clone();

    let file = retrieve::retrieve(
        message.document.file_source.clone(),
        message.document.file_path.clone(),
    )
    .await?;

    let metadata: BlobMetadata = message.document.into();
    let blob = create_blob(AzureBlobStorage::permanent(), &file, metadata).await?;
    let name = blob.name.clone();

    tracing::debug!("Uploaded blob {}.", &name);

    add_blob_to_search_index(search_client, blob).await?;

    tracing::info!("Successfully added {} to index.", &name);

    let transaction_logger = AuditLogger {};
    transaction_logger
        .log_create_transaction(&name, message_for_log)
        .await?;

    Ok(message.job_id)
}

async fn create_blob(
    storage_client: impl StorageClient,
    file_data: &[u8],
    metadata: BlobMetadata,
) -> Result<Blob, anyhow::Error> {
    let mut metadata_ref: HashMap<&str, &str> = HashMap::new();
    let hashmap: HashMap<String, String> = metadata.clone().into();
    for (key, val) in &hashmap {
        metadata_ref.insert(&key, &val);
    }

    let storage_file = storage_client
        .add_file(file_data, &metadata.pl_number, metadata_ref)
        .await
        .map_err(|e| anyhow!("Couldn't upload to blob storage: {:?}", e))?;

    Ok(Blob {
        metadata,
        name: storage_file.name,
        size: file_data.len(),
        path: storage_file.path,
    })
}

#[derive(Debug)]
pub struct Blob {
    pub metadata: BlobMetadata,
    pub name: String,
    pub size: usize,
    pub path: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        models::{test::get_test_create_message, CreateMessage},
        service_bus_client::test::TestRemovableMessage,
        state_manager::test::TestJobStatusClient,
    };
    use tokio_test::block_on;

    fn given_an_error_has_occurred() -> ProcessMessageError {
        anyhow!("literally any error").into()
    }

    fn given_file_not_found() -> ProcessMessageError {
        ProcessMessageError::StorageClientError(StorageClientError::SftpError(
            SftpError::CouldNotRetrieveFile,
        ))
    }

    fn given_we_have_a_create_message() -> TestRemovableMessage<CreateMessage> {
        TestRemovableMessage::<CreateMessage> {
            message: get_test_create_message(Uuid::new_v4()),
            remove_was_called: false,
        }
    }

    fn when_we_handle_the_error(
        removable_message: &mut TestRemovableMessage<CreateMessage>,
        error: ProcessMessageError,
        state_manager: TestJobStatusClient,
    ) -> Result<(), anyhow::Error> {
        block_on(handle_processing_error_for_create_message(
            removable_message,
            error,
            &state_manager,
        ))
    }

    #[test]
    fn test_an_unknown_error_does_not_remove_create_message() {
        let mut removable_message = given_we_have_a_create_message();
        let error = given_an_error_has_occurred();

        let result = when_we_handle_the_error(
            &mut removable_message,
            error,
            TestJobStatusClient::accepted(),
        );

        assert!(result.is_ok());
        assert_eq!(removable_message.remove_was_called, false);
    }

    #[test]
    fn test_file_not_found_removes_create_message() {
        let mut removable_message = given_we_have_a_create_message();
        let error = given_file_not_found();

        let result = when_we_handle_the_error(
            &mut removable_message,
            error,
            TestJobStatusClient::accepted(),
        );

        assert!(result.is_ok());
        assert!(
            removable_message.remove_was_called,
            "Message should be removed"
        );
    }
}
