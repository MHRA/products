use crate::{
    models::{CreateMessage, JobStatus},
    service_bus_client::{
        create_factory, DocIndexUpdaterQueue, RetrieveFromQueueError, RetrievedMessage,
    },
    state_manager::StateManager,
};
use anyhow::anyhow;
use std::time::Duration;
use tokio::time::delay_for;

mod sftp_client;

#[tracing::instrument]
pub async fn create_service_worker(
    time_to_wait: Duration,
    state_manager: StateManager,
) -> Result<String, anyhow::Error> {
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
        delay_for(time_to_wait).await;
    }
}

async fn create_file_in_blob(file: Vec<u8>) -> String {
    format!("create file in blob: {:?}", file)
}

async fn add_to_search_index(blob: String) {
    tracing::debug!("update the index for {}", blob);
}

async fn try_process_from_queue(
    create_client: &mut DocIndexUpdaterQueue,
) -> Result<FileProcessStatus, anyhow::Error> {
    tracing::info!("Checking for create messages");
    let retrieved_result: Result<RetrievedMessage<CreateMessage>, RetrieveFromQueueError> =
        create_client.receive().await;

    if let Ok(retrieval) = retrieved_result {
        let message = retrieval.message.clone();
        tracing::info!("{:?} message receive!", message);
        let file =
            sftp_client::retrieve(message.document.file_source, message.document.file_path).await?;
        let blob = create_file_in_blob(file).await;
        add_to_search_index(blob).await;
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
