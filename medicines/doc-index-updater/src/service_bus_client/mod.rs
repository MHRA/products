use azure_sdk_service_bus::prelude::*;
use std::error::Error;

pub async fn create_queue_client() -> Result<(Client), Box<dyn Error>> {
    let service_bus_namespace = std::env::var("SERVICE_BUS_NAMESPACE")
        .expect("Set env variable SERVICE_BUS_NAMESPACE first!");

    let queue_name =
        std::env::var("DELETE_QUEUE_NAME").expect("Set env variable DELETE_QUEUE_NAME first!");

    let policy_name = std::env::var("DELETE_QUEUE_POLICY_NAME")
        .expect("Set env variable DELETE_QUEUE_POLICY_NAME first!");

    let policy_key = std::env::var("DELETE_QUEUE_POLICY_KEY")
        .expect("Set env variable DELETE_QUEUE_POLICY_KEY first!");

    Ok(Client::new(
        service_bus_namespace.to_owned(),
        event_hub_name = queue_name,
        policy_name.to_owned(),
        policy_key.to_owned(),
    ))
}

pub async fn create_queue_client() -> Result<(Client), Box<dyn Error>> {
    let service_bus_namespace = std::env::var("SERVICE_BUS_NAMESPACE")
        .expect("Set env variable SERVICE_BUS_NAMESPACE first!");

    let queue_name =
        std::env::var("create_QUEUE_NAME").expect("Set env variable create_QUEUE_NAME first!");

    let policy_name = std::env::var("create_QUEUE_POLICY_NAME")
        .expect("Set env variable create_QUEUE_POLICY_NAME first!");

    let policy_key = std::env::var("create_QUEUE_POLICY_KEY")
        .expect("Set env variable create_QUEUE_POLICY_KEY first!");

    Ok(Client::new(
        service_bus_namespace.to_owned(),
        event_hub_name = queue_name,
        policy_name.to_owned(),
        policy_key.to_owned(),
    ))
}
