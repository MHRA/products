use crate::{
    models::{CreateMessage, JobStatus},
    service_bus_client::create_factory,
    sftp_client,
    state_manager::StateManager,
};
use anyhow::anyhow;
use azure_sdk_core::errors::AzureError;
use ssh2::{Error, File};
use std::{io::Read, time::Duration};
use tokio::time::delay_for;

#[tracing::instrument]
pub async fn create_service_worker(state_manager: StateManager) -> Result<String, anyhow::Error> {
    tracing::info!("Starting create service worker");
    let mut create_client = create_factory().await.map_err(|e| {
        tracing::error!("{:?}", e);
        anyhow!("Couldn't create the Create Queue")
    })?;
    let mut sftp_client = sftp_client::sftp_factory().await.map_err(|e| {
        tracing::error!("{:?}", e);
        anyhow!("Couldn't create the SFTP Queue")
    })?;

    loop {
        tracing::info!("Checking for create messages");
        let message_result: Result<CreateMessage, AzureError> = create_client.receive().await;
        if let Ok(message) = message_result {
            tracing::info!("{:?} message receive!", message);
            let mut file =
                retrieve_file_from_sftp(&mut sftp_client, message.document.file_path).await?;
            let blob = create_file_in_blob(&mut file).await;
            update_index(blob).await;
            let _ = state_manager
                .set_status(message.job_id, JobStatus::Done)
                .await?;
        }
        delay_for(Duration::from_secs(10)).await;
    }
}

async fn retrieve_file_from_sftp(sftp: &mut ssh2::Sftp, filepath: String) -> Result<File, Error> {
    let path = std::path::Path::new(&filepath);
    sftp.open(path).map_err(|e| {
        tracing::error!("{:?}", e);
        e
    })
}

async fn create_file_in_blob(file: &mut File) -> String {
    let mut some_string = "".to_owned();
    let _ = file.read_to_string(&mut some_string);
    tracing::info!("{:?}", some_string);
    todo!("create file in blob")
}

async fn update_index(blob: String) {
    todo!("update the index for {}", blob)
}
