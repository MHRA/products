use crate::{
    models::DeleteMessage,
    search,
    service_bus_client::{delete_factory, RetrieveFromQueueError},
    storage_client,
};
use azure_sdk_core::{errors::AzureError, prelude::*, DeleteSnapshotsMethod};
use azure_sdk_service_bus::prelude::Client;
use azure_sdk_storage_blob::prelude::*;
use std::time::Duration;
use tokio::time::delay_for;

pub enum DeleteManagerError {
    AzureError(AzureError),
    ReqwestError(reqwest::Error),
    DeleteManagerError(String),
}

impl From<AzureError> for DeleteManagerError {
    fn from(e: AzureError) -> Self {
        Self::AzureError(e)
    }
}

#[tracing::instrument]
pub async fn delete_service_worker(
    storage_container_name: String,
) -> Result<String, DeleteManagerError> {
    let mut delete_client = delete_factory().await?;

    loop {
        let message_result: Result<DeleteMessage, RetrieveFromQueueError> =
            delete_client.receive().await;
        match message_result {
            Ok(message) => {
                tracing::info!("{:?} message receive!", message);

                let blob_name = get_blob_name_from_message(message).await?;
                delete_blob(&storage_container_name, &blob_name).await?;
                // TODO: Update index
                // TODO: Notify state manager
            }
            Err(azure_error) => tracing::warn!("Azure error! {:?}", azure_error),
        };

        delay_for(Duration::from_secs(10)).await;
    }
}

pub async fn get_blob_name_from_message(
    message: DeleteMessage,
) -> Result<String, DeleteManagerError> {
    let search_client = search::factory();
    let search_results = search_client
        .azure_search(&message.document_content_id)
        .await;
    match search_results {
        Ok(results) => {
            for result in results.search_results {
                if result.file_name == message.document_content_id {
                    return Ok(message.document_content_id);
                }
            }
            let error_message = format!(
                "Cannot find document with content ID {}",
                message.document_content_id
            );
            Err(DeleteManagerError::DeleteManagerError(error_message))
        }
        Err(reqwest_error) => Err(DeleteManagerError::ReqwestError(reqwest_error)),
    }
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
