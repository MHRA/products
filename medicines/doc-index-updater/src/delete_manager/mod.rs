use crate::{models::DeleteMessage, service_bus_client::delete_factory};
use azure_sdk_core::errors::AzureError;
use azure_sdk_service_bus::prelude::Client;
use std::time::Duration;
use tokio::time::delay_for;

#[tracing::instrument]
pub async fn delete_service_worker() -> Result<String, AzureError> {
    let mut delete_client = delete_factory().await?;

    loop {
        let message_result: Result<DeleteMessage, AzureError> = delete_client.receive().await;
        if let Ok(message) = message_result {
            tracing::info!("{:?} message receive!", message);
        }
        delay_for(Duration::from_secs(10)).await;
    }
    // TODO: delete file in blob storage
    // TODO: Update index
    // TODO: Notify state manager
}

pub async fn get_message(delete_client: &mut Client) -> Result<String, AzureError> {
    let message = delete_client
        .peek_lock(time::Duration::days(1), Some(time::Duration::seconds(1)))
        .await
        .map_err(|e| tracing::error!("{:?}", e))?;

    Ok(message)
}
