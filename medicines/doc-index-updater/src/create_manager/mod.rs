use crate::{
    hash::compute_sha_hash,
    models::{CreateMessage, JobStatus},
    service_bus_client::{create_factory, DocIndexUpdaterQueue, RetrieveFromQueueError},
    state_manager::StateManager,
    storage_client
};
use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use anyhow::anyhow;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::delay_for;

mod sftp_client;

#[tracing::instrument]
pub async fn create_service_worker(
    storage_container_name: String,
    time_to_wait: Duration,
    state_manager: StateManager,
) -> Result<String, anyhow::Error> {
    tracing::info!("Starting create service worker");
    let mut create_client = create_factory().await.map_err(|e| {
        tracing::error!("{:?}", e);
        anyhow!("Couldn't create the Create Queue")
    })?;

    loop {
        match try_process_from_queue(&mut create_client, &storage_container_name).await {
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

async fn add_to_search_index(blob: String) {
    tracing::debug!("update the index for {}", blob);
}

async fn try_process_from_queue(
    create_client: &mut DocIndexUpdaterQueue,
    storage_container_name: &str
) -> Result<FileProcessStatus, anyhow::Error> {
    tracing::info!("Checking for create messages");
    let message_result: Result<CreateMessage, RetrieveFromQueueError> =
        create_client.receive().await;
    if let Ok(message) = message_result {
        tracing::info!("{:?} message receive!", message);
        let file =
            sftp_client::retrieve(message.document.file_source, message.document.file_path).await?;
        let metadata = HashMap::new();
        let blob_response = create_blob(storage_container_name, &file, &metadata).await?;
        add_to_search_index("blob data".to_string()).await;
        Ok(FileProcessStatus::Success(message.job_id))
    } else {
        Ok(FileProcessStatus::NothingToProcess)
    }
}

enum FileProcessStatus {
    Success(uuid::Uuid),
    NothingToProcess,
}

pub async fn create_blob(
    container_name: &str, 
    file_data: &[u8],
    metadata: &HashMap<&str, &str>
) -> Result<(), anyhow::Error> {
    let storage_client = storage_client::factory()?;
    let blob_name = compute_sha_hash(&file_data);
    let file_digest = md5::compute(&file_data[..]);
    storage_client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_content_type("application/pdf")
        .with_metadata(metadata)
        .with_body(&file_data[..])
        .with_content_md5(&file_digest[..])
        .finalize()
        .await
        .map_err(|e| {
            tracing::error!("{:?}", e);
            anyhow!("Couldn't create blob")
        })?;
    Ok(())
}
