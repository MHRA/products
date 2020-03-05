use crate::service_bus_client::delete_factory;
use std::time::Duration;
use tokio::time::delay_for;

pub async fn delete_service_worker() -> Result<String, AzureError> {
    let mut delete_client = delete_factory().await?;

    loop {
        let message = delete_client
            .peek_lock(time::Duration::days(1), time::Duration::from_secs(1))
            .await?;
        println!("{:?} message receive!", message);
        // TODO: delete file in blob storage
        // TODO: Update index
        // TODO: Notify state manager
        delay_for(Duration::from_secs(10)).await;
    }
}
