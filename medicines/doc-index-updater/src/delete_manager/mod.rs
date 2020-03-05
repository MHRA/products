use crate::service_bus_client::delete_factory;
use azure_sdk_core::errors::AzureError;
use std::time::Duration;
use tokio::time::delay_for;

#[tracing::instrument]
pub async fn delete_service_worker() -> Result<String, AzureError> {
    let mut delete_client = delete_factory().await?;

    loop {
        let message = delete_client
            .peek_lock(time::Duration::days(1), Some(time::Duration::seconds(1)))
            .await
            .map_err(|e| tracing::error!("{:?}", e))?;
        tracing::info!("{:?} message receive!", message);
        // TODO: delete file in blob storage
        // TODO: Update index
        // TODO: Notify state manager
        delay_for(Duration::from_secs(10)).await;
    }
}
