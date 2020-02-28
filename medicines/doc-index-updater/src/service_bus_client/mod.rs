use azure_sdk_core::errors::AzureError;
use azure_sdk_service_bus::prelude::*;
use tokio::time::delay_for;

pub async fn delete_queue_client() -> Result<Client, AzureError> {
    let service_bus_namespace = std::env::var("SERVICE_BUS_NAMESPACE")
        .expect("Set env variable SERVICE_BUS_NAMESPACE first!");

    let queue_name =
        std::env::var("DELETE_QUEUE_NAME").expect("Set env variable DELETE_QUEUE_NAME first!");

    let policy_name = std::env::var("DELETE_QUEUE_POLICY_NAME")
        .expect("Set env variable DELETE_QUEUE_POLICY_NAME first!");

    let policy_key = std::env::var("DELETE_QUEUE_POLICY_KEY")
        .expect("Set env variable DELETE_QUEUE_POLICY_KEY first!");

    Client::new(service_bus_namespace, queue_name, policy_name, policy_key)
}

pub async fn create_queue_client() -> Result<Client, AzureError> {
    let service_bus_namespace = std::env::var("SERVICE_BUS_NAMESPACE")
        .expect("Set env variable SERVICE_BUS_NAMESPACE first!");

    let queue_name =
        std::env::var("CREATE_QUEUE_NAME").expect("Set env variable CREATE_QUEUE_NAME first!");

    let policy_name = std::env::var("CREATE_QUEUE_POLICY_NAME")
        .expect("Set env variable CREATE_QUEUE_POLICY_NAME first!");

    let policy_key = std::env::var("CREATE_QUEUE_POLICY_KEY")
        .expect("Set env variable CREATE_QUEUE_POLICY_KEY first!");

    Client::new(service_bus_namespace, queue_name, policy_name, policy_key)
}

pub async fn delete_service_worker() -> Result<String, AzureError> {
    let mut delete_client = delete_queue_client().await?;

    loop {
        let message = delete_client.peek_lock(time::Duration::days(1)).await?;
        println!("{:?} message receive!", message);
        // TODO: delete file in blob storage
        // TODO: Update index
        // TODO: Notify state manager
        delay_for(Duration::from_secs(1)).await;
    }
}
