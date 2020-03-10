use crate::{
    models::DeleteMessage,
    service_bus_client::{delete_factory, RetrieveFromQueueError},
};
use azure_sdk_core::errors::AzureError;
use std::time::Duration;
use tokio::time::delay_for;

#[tracing::instrument]
pub async fn delete_service_worker() -> Result<String, AzureError> {
    let mut delete_client = delete_factory().await?;

    loop {
        let message_result: Result<DeleteMessage, RetrieveFromQueueError> =
            delete_client.receive().await;
        if let Ok(message) = message_result {
            tracing::info!("{:?} message receive!", message);
        }
        delay_for(Duration::from_secs(10)).await;
    }
    // TODO: delete file in blob storage
    // TODO: Update index
    // TODO: Notify state manager
}
