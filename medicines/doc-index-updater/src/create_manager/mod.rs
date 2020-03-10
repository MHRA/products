use crate::{
    models::{CreateMessage, JobStatus},
    service_bus_client::{create_factory, DocIndexUpdaterQueue},
    state_manager::StateManager,
};
use anyhow::anyhow;
use azure_sdk_core::errors::AzureError;
use std::time::Duration;
use tokio::time::delay_for;

mod sftp_client;

#[tracing::instrument]
pub async fn create_service_worker(state_manager: StateManager) -> Result<String, anyhow::Error> {
    tracing::info!("Starting create service worker");
    let mut create_client = create_factory().await.map_err(|e| {
        tracing::error!("{:?}", e);
        anyhow!("Couldn't create the Create Queue")
    })?;

    loop {
        match try_process_from_queue(&mut create_client).await {
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
        delay_for(Duration::from_secs(5)).await;
    }
}

async fn create_file_in_blob(file: Vec<u8>) -> String {
    format!("create file in blob: {:?}", file)
}

async fn update_index(blob: String) {
    tracing::debug!("update the index for {}", blob);
}

async fn try_process_from_queue(
    create_client: &mut DocIndexUpdaterQueue,
) -> Result<FileProcessStatus, anyhow::Error> {
    tracing::info!("Checking for create messages");
    let message_result: Result<CreateMessage, AzureError> = create_client.receive().await;
    if let Ok(message) = message_result {
        tracing::info!("{:?} message receive!", message);
        let file = sftp_client::retrieve(message.document.file_path).await?;
        let blob = create_file_in_blob(file).await;
        update_index(blob).await;

        Ok(FileProcessStatus::Success(message.job_id))
    } else {
        Ok(FileProcessStatus::NothingToProcess)
    }
}

enum FileProcessStatus {
    Success(uuid::Uuid),
    NothingToProcess,
}
