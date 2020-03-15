use crate::{
    models::{CreateMessage, FileProcessStatus, JobStatus},
    search_client,
    service_bus_client::{
        create_factory, DocIndexUpdaterQueue, RetrieveFromQueueError, RetrievedMessage,
    },
    state_manager::StateManager,
    storage_client,
};

use anyhow::anyhow;
use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio::time::delay_for;

mod hash;
mod metadata;
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
    let search_client = search_client::factory();
    let storage_client = storage_client::factory().map_err(|e| {
        tracing::error!("{:?}", e);
        anyhow!("Couldn't create storage client")
    })?;
    loop {
        match try_process_from_queue(
            &mut create_client,
            &state_manager,
            &search_client,
            &storage_client,
        )
        .await
        {
            Ok(()) => {}
            Err(e) => tracing::error!("{:?}", e),
        }
        delay_for(time_to_wait).await;
    }
}

async fn try_process_from_queue(
    service_bus_client: &mut DocIndexUpdaterQueue,
    state_manager: &StateManager,
    search_client: &search_client::AzureSearchClient,
    storage_client: &azure_sdk_storage_core::prelude::Client,
) -> Result<(), anyhow::Error> {
    tracing::info!("Checking for create messages");
    let retrieved_result: Result<RetrievedMessage<CreateMessage>, RetrieveFromQueueError> =
        service_bus_client.receive().await;

    if let Ok(retrieval) = retrieved_result {
        let processing_result =
            process_message(retrieval.message.clone(), &search_client, &storage_client).await;
        match processing_result {
            Ok(FileProcessStatus::Success(job_id)) => {
                let _ = state_manager.set_status(job_id, JobStatus::Done).await?;
            }
            Ok(FileProcessStatus::NothingToProcess) => {}
            Err(e) => {
                if e.to_string()
                    == "Couldn't retrieve file: [-31] Failed opening remote file".to_string()
                {
                    tracing::info!("Updating state to errored and removing message");
                    let _ = state_manager.set_status(
                        retrieval.message.job_id,
                        JobStatus::Error {
                            message: "Couldn't find file".to_string(),
                            code: "404".to_string(),
                        },
                    );
                    // TODO: Uncomment before releasing
                    // or when we have a centralised SFTP server for dev
                    // let _ = retrieval.remove().await?;
                }
                return Err(e);
            }
        };
    }
    Ok(())
}

async fn process_message(
    message: CreateMessage,
    search_client: &search_client::AzureSearchClient,
    storage_client: &azure_sdk_storage_core::prelude::Client,
) -> Result<FileProcessStatus, anyhow::Error> {
    tracing::info!("Message received: {:?} ", message);

    let file = sftp_client::retrieve(
        message.document.file_source.clone(),
        message.document.file_path.clone(),
    )
    .await
    .map_err(|e| anyhow!("Couldn't retrieve file: {:?}", e))?;

    let metadata = metadata::derive_metadata_from_message(&message.document);
    let file_digest = md5::compute(&file[..]);
    let file_data_hash = hash::sha1(&file);
    create_blob(
        &storage_client,
        &file_data_hash,
        &file,
        &metadata,
        &file_digest,
    )
    .await?;
    tracing::info!(
        "Uploaded blob {} for job {}",
        &file_data_hash,
        &message.job_id
    );

    add_to_search_index(&search_client, &file_data_hash, metadata).await?;
    tracing::info!(
        "Added to index {} for job {}",
        &file_data_hash,
        &message.job_id
    );

    Ok(FileProcessStatus::Success(message.job_id))
}

async fn create_blob(
    storage_client: &azure_sdk_storage_core::prelude::Client,
    blob_name: &str,
    file_data: &[u8],
    metadata: &HashMap<String, String>,
    file_digest: &md5::Digest,
) -> Result<(), anyhow::Error> {
    let container_name =
        std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");
    let mut metadata_ref: HashMap<&str, &str> = HashMap::new();
    for (key, val) in metadata {
        metadata_ref.insert(&key, &val);
    }

    storage_client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(blob_name)
        .with_content_type("application/pdf")
        .with_metadata(&metadata_ref)
        .with_body(&file_data[..])
        .with_content_md5(&file_digest[..])
        .finalize()
        .await
        .map_err(|e| anyhow!("Couldn't upload to blob storage: {:?}", e))?;
    Ok(())
}

async fn add_to_search_index(
    search_client: &search_client::AzureSearchClient,
    blob_name: &str,
    mut file_metadata: HashMap<String, String>,
) -> Result<(), anyhow::Error> {
    file_metadata.insert("rev_label".to_string(), "1".to_string());
    let storage_account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let container_name =
        std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");
    let storage_path = format!(
        "https://{}.blob.core.windows.net/{}/{}",
        &storage_account, &container_name, &blob_name
    );

    let mut file_data = HashMap::new();
    file_data.insert("rev_label".to_string(), "1".to_string());
    file_data.insert("@odata.context".to_string(), "https://mhraproductsdev.search.windows.net/indexes('products-index')/$metadata#docs(*)/$entity".to_string());
    file_data.insert("metadata_storage_path".to_string(), storage_path);
    file_data.insert(
        "product_name".to_string(),
        file_metadata["product_name"].to_string(),
    );
    file_data.insert("created".to_string(), format!("{:?}", Instant::now()));
    file_data.insert("release_state".to_string(), "Y".to_string());
    file_data.insert("keywords".to_string(), "".to_string());
    file_data.insert(
        "title".to_string(),
        "A fun title of an example document".to_string(),
    );
    file_data.insert("file_name".to_string(), "CON971821".to_string());
    file_data.insert("metadata_storage_size".to_string(), "3131211".to_string());
    file_data.insert("metadata_storage_name".to_string(), blob_name.to_string());
    file_data.insert("doc_type".to_string(), "Par".to_string());
    file_data.insert("suggestions".to_string(), "[]".to_string());
    file_data.insert("substance_name".to_string(), "substance".to_string());
    file_data.insert("facets".to_string(), "[]".to_string());

    search_client.create(file_data).await?;
    Ok(())
}
