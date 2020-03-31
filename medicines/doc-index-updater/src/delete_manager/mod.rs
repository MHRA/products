use crate::{
    models::{DeleteMessage, JobStatus},
    search_client,
    service_bus_client::{delete_factory, ProcessRetrievalError, RetrievedMessage},
    state_manager::StateManager,
    storage_client,
};
use anyhow::anyhow;
use async_trait::async_trait;
use azure_sdk_core::{errors::AzureError, prelude::*, DeleteSnapshotsMethod};
use azure_sdk_storage_blob::prelude::*;
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
        self,
        e: anyhow::Error,
        state_manager: &StateManager,
    ) -> anyhow::Result<()> {
        tracing::info!("Setting error state in state manager");
        state_manager
            .set_status(
                self.message.job_id,
                JobStatus::Error {
                    message: e.to_string(),
                    code: "".to_string(),
                },
            )
            .await?;
        let _ = self.remove().await?;
        Ok(())
    }
}

pub async fn process_message(message: DeleteMessage) -> Result<Uuid, anyhow::Error> {
    tracing::info!("Message received: {:?} ", message);

    let search_client = search_client::factory();
    let storage_client = storage_client::factory()
        .map_err(|e| anyhow!("Couldn't create storage client: {:?}", e))?;

    let storage_container_name = std::env::var("STORAGE_CONTAINER")?;
    let blob_name =
        get_blob_name_from_content_id(message.document_content_id.clone(), &search_client).await?;

    tracing::info!(
        "Found blob name {} for document content ID {} from index",
        &blob_name,
        &message.document_content_id
    );
    delete_from_index(&search_client, &blob_name).await?;
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
    search_client: &search_client::AzureSearchClient,
) -> Result<String, anyhow::Error> {
    let search_results = search_client.search(content_id.to_owned()).await?;
    for result in search_results.search_results {
        if result.file_name == content_id {
            return Ok(result.metadata_storage_name);
        }
    }
    Err(anyhow!(format!(
        "Cannot find document with content ID {}",
        content_id
    )))
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
    search_client: &search_client::AzureSearchClient,
    blob_name: &str,
) -> Result<(), anyhow::Error> {
    search_client
        .delete(&"metadata_storage_name".to_string(), &blob_name)
        .await?;
    Ok(())
}
