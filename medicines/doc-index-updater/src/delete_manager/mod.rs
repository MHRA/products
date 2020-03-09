use crate::{
    models::DeleteMessage,
    service_bus_client::{delete_factory, RetrieveFromQueueError},
    storage_client,
};
use azure_sdk_core::{errors::AzureError, prelude::*, DeleteSnapshotsMethod};
use azure_sdk_service_bus::prelude::Client;
use azure_sdk_storage_blob::prelude::*;
use std::time::Duration;
use tokio::time::delay_for;

#[tracing::instrument]
pub async fn delete_service_worker(storage_container_name: String) -> Result<String, AzureError> {
    let mut delete_client = delete_factory().await?;

    loop {
        let message_result: Result<DeleteMessage, RetrieveFromQueueError> =
            delete_client.receive().await;
        if let Ok(message) = message_result {
            tracing::info!("{:?} message receive!", message);

            let blob_name = get_blob_name_from_message(message);
            delete_blob(&storage_container_name, &blob_name);
            // TODO: Update index
            // TODO: Notify state manager
        }
        delay_for(Duration::from_secs(10)).await;
    }
}

pub fn get_blob_name_from_message(message: DeleteMessage) -> String {
    // TODO: This will need to be replaced with some mechanism to find a blob name
    // for the given CON number, which might be a round trip to the search index.
    message.document_content_id
}

pub async fn delete_blob(container_name: &str, blob_name: &str) -> Result<(), AzureError> {
    let storage_client = storage_client::factory()?;
    storage_client
        .delete_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_delete_snapshots_method(DeleteSnapshotsMethod::Include) // could also be Only; not 100% sure what this means
        .finalize()
        .await?;
    Ok(())
}
