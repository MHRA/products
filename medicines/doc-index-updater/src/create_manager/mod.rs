use crate::{
    create_manager::models::BlobMetadata,
    models::{CreateMessage, JobStatus, Message},
    search_client,
    service_bus_client::{
        create_factory, DocIndexUpdaterQueue, RetrieveFromQueueError, RetrievedMessage,
    },
    state_manager::StateManager,
    storage_client,
};
use uuid::Uuid;

use anyhow::anyhow;
use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use search_index::add_blob_to_search_index;
use std::{collections::HashMap, time::Duration};
use tokio::time::delay_for;

mod hash;
pub mod models;
mod search_index;
mod sftp_client;

#[tracing::instrument]
pub async fn create_service_worker(
    time_to_wait: Duration,
    state_manager: StateManager,
) -> Result<(), anyhow::Error> {
    tracing::info!("Starting create service worker");
    let mut create_client = create_factory()
        .await
        .map_err(|e| anyhow!("Couldn't create service bus client: {:?}", e))?;

    loop {
        match try_process_from_queue(&mut create_client, &state_manager).await {
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
    tracing::info!("Checking for create messages");
    let retrieved_result: Result<RetrievedMessage<CreateMessage>, RetrieveFromQueueError> =
        service_bus_client.receive().await;

    if let Ok(retrieval) = retrieved_result {
        let processing_result = retrieval.message.clone().process().await;
        match processing_result {
            Ok(job_id) => {
                state_manager.set_status(job_id, JobStatus::Done).await?;
                retrieval.remove().await?;
            }
            Err(e) => {
                tracing::error!(
                    "Error {:?} while processing message {}",
                    e,
                    retrieval.message.job_id
                );
                handle_processing_error(e, &state_manager, retrieval).await?;
            }
        };
    }
    Ok(())
}

async fn handle_processing_error(
    e: anyhow::Error,
    state_manager: &StateManager,
    retrieval: RetrievedMessage<CreateMessage>,
) -> anyhow::Result<()> {
    if e.to_string() == "Couldn't retrieve file: [-31] Failed opening remote file".to_string() {
        tracing::info!("Updating state to errored and removing message");
        let _ = state_manager
            .set_status(
                retrieval.message.job_id,
                JobStatus::Error {
                    message: "Couldn't find file".to_string(),
                    code: "404".to_string(),
                },
            )
            .await?;
        let _ = retrieval.remove().await?;
    }
    Ok(())
}

pub async fn process_message(message: CreateMessage) -> Result<Uuid, anyhow::Error> {
    tracing::info!("Message received: {:?} ", message);

    let search_client = search_client::factory();
    let storage_client = storage_client::factory()
        .map_err(|e| anyhow!("Couldn't create storage client: {:?}", e))?;

    let file = sftp_client::retrieve(
        message.document.file_source.clone(),
        message.document.file_path.clone(),
    )
    .await
    .map_err(|e| anyhow!("Couldn't retrieve file: {:?}", e))?;

    let metadata: BlobMetadata = message.document.into();
    let blob = create_blob(&storage_client, &file, metadata.clone().into()).await?;
    let name = blob.name.clone();
    tracing::info!("Uploaded blob {} for job {}", &name, &message.job_id);

    add_blob_to_search_index(&search_client, blob).await?;
    tracing::info!("Added to index {} for job {}", &name, &message.job_id);

    Ok(message.job_id)
}

async fn create_blob(
    storage_client: &azure_sdk_storage_core::prelude::Client,
    file_data: &[u8],
    metadata: BlobMetadata,
) -> Result<Blob, anyhow::Error> {
    let name = hash::sha1(&file_data);
    let file_digest = md5::compute(&file_data[..]);
    let container_name =
        std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");
    let mut metadata_ref: HashMap<&str, &str> = HashMap::new();
    let hashmap: HashMap<String, String> = metadata.clone().into();
    for (key, val) in &hashmap {
        metadata_ref.insert(&key, &val);
    }

    storage_client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&name)
        .with_content_type("application/pdf")
        .with_metadata(&metadata_ref)
        .with_body(&file_data[..])
        .with_content_md5(&file_digest[..])
        .finalize()
        .await
        .map_err(|e| anyhow!("Couldn't upload to blob storage: {:?}", e))?;

    let storage_account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let path = format!(
        "https://{}.blob.core.windows.net/{}/{}",
        &storage_account, &container_name, &name
    );

    Ok(Blob {
        metadata,
        name,
        size: file_data.len(),
        path,
    })
}

pub struct Blob {
    metadata: BlobMetadata,
    name: String,
    size: usize,
    path: String,
}
