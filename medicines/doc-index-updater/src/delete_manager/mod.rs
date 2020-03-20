use crate::{
    models::{DeleteMessage, JobStatus},
    search_client,
    service_bus_client::{
        delete_factory, DocIndexUpdaterQueue, RetrieveFromQueueError, RetrievedMessage,
    },
    state_manager::StateManager,
    storage_client,
};
use anyhow::anyhow;
use azure_sdk_core::{errors::AzureError, prelude::*, DeleteSnapshotsMethod};
use azure_sdk_storage_blob::prelude::*;
use std::time::Duration;
use tokio::time::delay_for;
use uuid::Uuid;

#[tracing::instrument]
pub async fn delete_service_worker(
    time_to_wait: Duration,
    state_manager: StateManager,
) -> Result<String, anyhow::Error> {
    tracing::info!("Starting delete service worker");
    let mut delete_client = delete_factory()
        .await
        .map_err(|e| anyhow!("Couldn't create service bus client: {:?}", e))?;

    loop {
        match try_process_from_queue(&mut delete_client, &state_manager).await {
            Ok(()) => {}
            Err(e) => tracing::error!("{:?}", e),
        }
        delay_for(time_to_wait).await;
    }
}

async fn try_process_from_queue(
    service_bus_client: &mut DocIndexUpdaterQueue,
    state_manager: &StateManager,
) -> Result<(), anyhow::Error> {
    tracing::info!("Checking for delete messages");
    let retrieved_result: Result<RetrievedMessage<DeleteMessage>, RetrieveFromQueueError> =
        service_bus_client.receive().await;

    if let Ok(retrieval) = retrieved_result {
        let processing_result = process_message(retrieval.message.clone()).await;
        match processing_result {
            Ok(job_id) => {
                let _ = state_manager.set_status(job_id, JobStatus::Done).await?;
                let _ = retrieval.remove().await?;
            }
            Err(e) => {
                tracing::error!(
                    "Error {:?} while processing message {}",
                    e,
                    retrieval.message.job_id
                );
                tracing::info!(
                    "Setting error state in state manager for job {}",
                    retrieval.message.job_id
                );
                state_manager
                    .set_status(
                        retrieval.message.job_id,
                        JobStatus::Error {
                            message: e.to_string(),
                            code: "".to_string(),
                        },
                    )
                    .await?;
                let _ = retrieval.remove().await?;
            }
        };
    }
    Ok(())
}

async fn process_message(message: DeleteMessage) -> Result<Uuid, anyhow::Error> {
    tracing::info!("Message received: {:?} ", message);

    let search_client = search_client::factory();
    let storage_client = storage_client::factory()
        .map_err(|e| anyhow!("Couldn't create storage client: {:?}", e))?;

    let storage_container_name = std::env::var("STORAGE_CONTAINER")?;
    let blob_name =
        get_blob_name_from_content_id(message.document_content_id.clone(), &search_client).await?;
    tracing::info!(
        "Found blob name {} for document content ID {} from index for job {}",
        &blob_name,
        &message.document_content_id,
        &message.job_id,
    );
    delete_from_index(&search_client, &blob_name).await?;
    tracing::info!(
        "Deleted blob {} from index for job {}",
        &blob_name,
        &message.job_id
    );
    delete_blob(&storage_client, &storage_container_name, &blob_name)
        .await
        .map_err(|e| {
            tracing::error!("Error deleting blob: {:?}", e);
            anyhow!("Couldn't delete blob {}", &blob_name)
        })?;
    tracing::info!(
        "Deleted blob {} from storage container {} for job {}",
        &blob_name,
        &storage_container_name,
        &message.job_id
    );

    Ok(message.job_id)
}

pub async fn get_blob_name_from_content_id(
    content_id: String,
    search_client: &search_client::AzureSearchClient,
) -> Result<String, anyhow::Error> {
    let search_results = search_client.search(content_id.to_owned()).await?;
    for result in search_results.search_results {
        if result.file_name == content_id {
            return Ok(result.metadata_storage_name);
        }
    }
    let error_message = format!("Cannot find document with content ID {}", content_id);
    Err(anyhow!(error_message))
}

async fn delete_blob(
    storage_client: &azure_sdk_storage_core::prelude::Client,
    container_name: &str,
    blob_name: &str,
) -> Result<(), AzureError> {
    storage_client
        .delete_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .finalize()
        .await?;
    Ok(())
}

pub async fn delete_from_index(
    search_client: &search_client::AzureSearchClient,
    blob_name: &str,
) -> Result<(), anyhow::Error> {
    search_client
        .delete(&"metadata_storage_name".to_string(), &blob_name)
        .await?;
    Ok(())
}
