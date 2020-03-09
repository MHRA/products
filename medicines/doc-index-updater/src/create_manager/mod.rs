use crate::{models::CreateMessage, service_bus_client::create_factory};
use azure_sdk_core::errors::AzureError;
use std::time::Duration;
use tokio::time::delay_for;

#[tracing::instrument]
pub async fn create_service_worker() -> Result<String, AzureError> {
    let mut create_client = create_factory().await?;

    loop {
        let message_result: Result<CreateMessage, AzureError> = create_client.receive().await;
        if let Ok(message) = message_result {
            tracing::info!("{:?} message receive!", message);
            retrieve_file_from_sftp(message.document.file_path).await;
            // TODO: create file in blob storage
            // TODO: Update index
            // TODO: Notify state manager
        }
        delay_for(Duration::from_secs(10)).await;
    }
}

async fn retrieve_file_from_sftp(filepath: String) -> String {
    filepath
}
