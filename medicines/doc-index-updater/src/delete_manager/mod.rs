use crate::{
    models::{DeleteMessage, JobStatus},
    search,
    service_bus_client::{
        delete_factory, DocIndexUpdaterQueue, RetrieveFromQueueError, RetrievedMessage,
    },
    state_manager::StateManager,
    storage_client,
};
use anyhow::anyhow;
use azure_sdk_core::{errors::AzureError, prelude::*, DeleteSnapshotsMethod};
use azure_sdk_storage_blob::prelude::*;
use std::time::Duration;
use tokio::time::delay_for;

#[tracing::instrument]
pub async fn delete_service_worker(
    time_to_wait: Duration,
    state_manager: StateManager,
) -> Result<String, anyhow::Error> {
    tracing::info!("Starting delete service worker");
    let mut delete_client = delete_factory().await.map_err(|e| {
        tracing::error!("{:?}", e);
        anyhow!("Couldn't delete the Delete Queue")
    })?;

    loop {
        match try_process_from_queue(&mut delete_client).await {
            Ok(status) => match status {
                FileProcessStatus::Success(job_id) => {
                    let _ = state_manager.set_status(job_id, JobStatus::Done).await?;
                }
                FileProcessStatus::NothingToProcess => {}
            },
            Err(e) => {
                tracing::error!("{:?}", e);
            }
        }
        delay_for(time_to_wait).await;
    }
}

async fn try_process_from_queue(
    delete_client: &mut DocIndexUpdaterQueue,
) -> Result<FileProcessStatus, anyhow::Error> {
    tracing::info!("Checking for delete messages");
    let retrieved_result: Result<RetrievedMessage<DeleteMessage>, RetrieveFromQueueError> =
        delete_client.receive().await;

    if let Ok(retrieval) = retrieved_result {
        let message = retrieval.message.clone();
        tracing::info!("{:?} message receive!", message);

        let storage_container_name = std::env::var("STORAGE_CONTAINER")?;
        let blob_name = get_blob_name_from_content_id(&message.document_content_id).await?;
        delete_blob(&storage_container_name, &blob_name)
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e);
                anyhow!("Couldn't delete blob {}", &blob_name)
            })?;
        retrieval.remove().await?;
        Ok(FileProcessStatus::Success(message.job_id))
    } else {
        Ok(FileProcessStatus::NothingToProcess)
    }
}

enum FileProcessStatus {
    Success(uuid::Uuid),
    NothingToProcess,
}

pub async fn get_blob_name_from_content_id(content_id: &String) -> Result<String, anyhow::Error> {
    let search_client = search::factory();
    let search_results = search_client.azure_search(&content_id).await?;
    for result in search_results.search_results {
        if &result.file_name == content_id {
            return Ok(result.metadata_storage_name);
        }
    }

    let error_message = format!("Cannot find document with content ID {}", content_id);
    Err(anyhow!(error_message))
}

pub async fn delete_blob(container_name: &str, blob_name: &str) -> Result<(), AzureError> {
    let storage_client = storage_client::factory()?;
    storage_client
        .delete_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .finalize()
        .await?;
    Ok(())
}
