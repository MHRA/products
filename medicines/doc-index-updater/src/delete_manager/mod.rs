use crate::{
    models::DeleteMessage,
    search,
    service_bus_client::{delete_factory, RetrieveFromQueueError, RetrievedMessage},
    storage_client,
};
use anyhow::anyhow;
use azure_sdk_core::{errors::AzureError, prelude::*, DeleteSnapshotsMethod};
use azure_sdk_storage_blob::prelude::*;
use std::time::Duration;
use tokio::time::delay_for;

#[tracing::instrument]
pub async fn delete_service_worker(
    storage_container_name: String,
) -> Result<String, anyhow::Error> {
    let mut delete_client = delete_factory().await.map_err(|e| {
        tracing::error!("{:?}", e);
        anyhow!("Couldn't create the delete queue client")
    })?;

    loop {
        let retrieved_result: Result<RetrievedMessage<DeleteMessage>, RetrieveFromQueueError> =
            delete_client.receive().await;

        match retrieved_result {
            Ok(retrieval) => {
                let message = retrieval.message;
                tracing::info!("{:?} message receive!", message);

                let blob_name = get_blob_name_from_content_id(&message.document_content_id).await?;
                delete_blob(&storage_container_name, &blob_name)
                    .await
                    .map_err(|e| {
                        tracing::error!("{:?}", e);
                        anyhow!("Couldn't delete blob {}", &blob_name)
                    })?;
                // TODO: Update index
                let queue_removal_result =
                    retrieval.peek_lock.delete_message().await.map_err(|e| {
                        tracing::error!("{:?}", e);
                        anyhow!("Queue Removal Error")
                    });
                tracing::info!("Removed job from ServiceBus ({:?})", queue_removal_result);

                // TODO: Notify state manager
            }
            Err(azure_error) => tracing::warn!("Azure error! {:?}", azure_error),
        };

        delay_for(Duration::from_secs(10)).await;
    }
}

pub async fn get_blob_name_from_content_id(content_id: &String) -> Result<String, anyhow::Error> {
    let search_client = search::factory();
    let search_results = search_client.azure_search(&content_id).await?;
    for result in search_results.search_results {
        if &result.file_name == content_id {
            return Ok(result.metadata_storage_name);
        }
    }

    let error_message = format!("Cannot find document with content ID {}", content_id);
    Err(anyhow!(error_message))
}

pub async fn delete_blob(container_name: &str, blob_name: &str) -> Result<(), AzureError> {
    let storage_client = storage_client::factory()?;
    storage_client
        .delete_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .finalize()
        .await?;
    Ok(())
}
