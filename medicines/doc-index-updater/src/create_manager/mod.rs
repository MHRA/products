use crate::{
    create_manager::models::BlobMetadata,
    models::{CreateMessage, JobStatus},
    search_client,
    service_bus_client::{
        create_factory, ProcessMessageError, ProcessRetrievalError, RemoveableMessage,
        RetrievedMessage,
    },
    state_manager::{JobStatusClient, StateManager},
    storage_client,
};
use anyhow::anyhow;
use async_trait::async_trait;
use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use search_index::add_blob_to_search_index;
pub use sftp_client::SftpError;
use std::{collections::HashMap, time::Duration};
use tokio::time::delay_for;
use uuid::Uuid;

mod hash;
pub mod models;
mod sanitiser;
mod search_index;
mod sftp_client;

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
    removeable_message: &mut T,
    error: ProcessMessageError,
    state_manager: &impl JobStatusClient,
) -> anyhow::Result<()>
where
    T: RemoveableMessage<CreateMessage>,
{
    if let ProcessMessageError::SftpError(SftpError::CouldNotRetrieveFile) = error {
        tracing::warn!("Couldn't find file. Updating state to errored and removing message.");
        let _ = state_manager
            .set_status(
                removeable_message.get_message().job_id,
                JobStatus::Error {
                    message: "Couldn't find file".to_string(),
                    code: "404".to_string(),
                },
            )
            .await?;
        let _ = removeable_message.remove().await?;
    }
    Ok(())
}

pub async fn process_message(message: CreateMessage) -> Result<Uuid, ProcessMessageError> {
    tracing::debug!("Message received: {:?} ", message);

    let search_client = search_client::factory();
    let storage_client = storage_client::factory()
        .map_err(|e| anyhow!("Couldn't create storage client: {:?}", e))?;

    let file = sftp_client::retrieve(
        message.document.file_source.clone(),
        message.document.file_path.clone(),
    )
    .await?;

    let metadata: BlobMetadata = message.document.into();
    let blob = create_blob(&storage_client, &file, metadata.clone()).await?;
    let name = blob.name.clone();

    tracing::debug!("Uploaded blob {}.", &name);

    add_blob_to_search_index(search_client, blob).await?;

    tracing::info!("Successfully added {} to index.", &name);

    Ok(message.job_id)
}

async fn create_blob(
    storage_client: &azure_sdk_storage_core::prelude::Client,
    file_data: &[u8],
    metadata: BlobMetadata,
) -> Result<Blob, anyhow::Error> {
    let name = hash::sha1(&file_data);
    let file_digest = md5::compute(&file_data[..]);
    let container_name =
        std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");
    let mut metadata_ref: HashMap<&str, &str> = HashMap::new();
    let hashmap: HashMap<String, String> = metadata.clone().into();
    for (key, val) in &hashmap {
        metadata_ref.insert(&key, &val);
    }

    storage_client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&name)
        .with_content_type("application/pdf")
        .with_metadata(&metadata_ref)
        .with_body(&file_data[..])
        .with_content_md5(&file_digest[..])
        .finalize()
        .await
        .map_err(|e| anyhow!("Couldn't upload to blob storage: {:?}", e))?;

    let storage_account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let path = format!(
        "https://{}.blob.core.windows.net/{}/{}",
        &storage_account, &container_name, &name
    );

    Ok(Blob {
        metadata,
        name,
        size: file_data.len(),
        path,
    })
}

pub struct Blob {
    metadata: BlobMetadata,
    name: String,
    size: usize,
    path: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        models::{test::get_test_create_message, CreateMessage},
        service_bus_client::test::TestRemoveableMessage,
        state_manager::TestJobStatusClient,
    };
    use tokio_test::block_on;

    fn given_an_error_has_occurred() -> ProcessMessageError {
        anyhow!("literally any error").into()
    }

    fn given_file_not_found() -> ProcessMessageError {
        ProcessMessageError::SftpError(SftpError::CouldNotRetrieveFile)
    }

    fn given_we_have_a_create_message() -> TestRemoveableMessage<CreateMessage> {
        TestRemoveableMessage::<CreateMessage> {
            message: get_test_create_message(Uuid::new_v4()),
            remove_was_called: false,
        }
    }

    fn when_we_handle_the_error(
        removeable_message: &mut TestRemoveableMessage<CreateMessage>,
        error: ProcessMessageError,
        state_manager: TestJobStatusClient,
    ) -> Result<(), anyhow::Error> {
        block_on(handle_processing_error_for_create_message(
            removeable_message,
            error,
            &state_manager,
        ))
    }

    #[test]
    fn test_an_unknown_error_does_not_remove_create_message() {
        let mut removeable_message = given_we_have_a_create_message();
        let error = given_an_error_has_occurred();

        let result =
            when_we_handle_the_error(&mut removeable_message, error, TestJobStatusClient {});

        assert!(result.is_ok());
        assert_eq!(removeable_message.remove_was_called, false);
    }

    #[test]
    fn test_file_not_found_removes_create_message() {
        let mut removeable_message = given_we_have_a_create_message();
        let error = given_file_not_found();

        let result =
            when_we_handle_the_error(&mut removeable_message, error, TestJobStatusClient {});

        assert!(result.is_ok());
        assert!(
            removeable_message.remove_was_called,
            "Message should be removed"
        );
    }
}
