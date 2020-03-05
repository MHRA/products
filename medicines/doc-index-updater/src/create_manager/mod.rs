use crate::service_bus_client::create_factory;
use azure_sdk_core::errors::AzureError;
use azure_sdk_service_bus::prelude::Client;
use std::time::Duration;
use tokio::time::delay_for;

#[tracing::instrument]
pub async fn create_service_worker() -> Result<String, AzureError> {
    let mut create_client = create_factory().await?;

    loop {
        if let Ok(message) = get_message(&mut create_client).await {
            tracing::info!("{:?} message receive!", message);
        }
        delay_for(Duration::from_secs(10)).await;
    }
    // TODO: create file in blob storage
    // TODO: Update index
    // TODO: Notify state manager
}

pub async fn get_message(create_client: &mut Client) -> Result<String, AzureError> {
    let message = create_client
        .peek_lock(time::Duration::days(1), Some(time::Duration::seconds(1)))
        .await
        .map_err(|e| tracing::error!("{:?}", e))?;

    return Ok(message);
}
