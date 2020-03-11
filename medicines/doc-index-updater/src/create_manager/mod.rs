use crate::{
    hash::compute_sha_hash,
    models::{CreateMessage, JobStatus, Document},
    service_bus_client::{create_factory, DocIndexUpdaterQueue, RetrieveFromQueueError},
    state_manager::StateManager,
    storage_client
};
use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use anyhow::anyhow;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::delay_for;

mod sftp_client;
mod metadata;

#[tracing::instrument]
pub async fn create_service_worker(
    storage_container_name: String,
    time_to_wait: Duration,
    state_manager: StateManager,
) -> Result<String, anyhow::Error> {
    tracing::info!("Starting create service worker");
    let mut create_client = create_factory().await.map_err(|e| {
        tracing::error!("{:?}", e);
        anyhow!("Couldn't create the Create Queue")
    })?;

    loop {
        match try_process_from_queue(&mut create_client, &storage_container_name).await {
            Ok(status) => match status {
                FileProcessStatus::Success(job_id) => {
                    let _ = state_manager.set_status(job_id, JobStatus::Done).await?;
                }
                FileProcessStatus::NothingToProcess => {}
            },
            Err(e) => {
                tracing::error!("{:?}", e);
            }
        }
        delay_for(time_to_wait).await;
    }
}

async fn add_to_search_index(blob: String) {
    tracing::debug!("update the index for {}", blob);
}

async fn try_process_from_queue(
    create_client: &mut DocIndexUpdaterQueue,
    storage_container_name: &str
) -> Result<FileProcessStatus, anyhow::Error> {
    tracing::info!("Checking for create messages");
    let message_result: Result<CreateMessage, RetrieveFromQueueError> =
        create_client.receive().await;
    if let Ok(message) = message_result {
        tracing::info!("{:?} message receive!", message);
        let file =
            sftp_client::retrieve(message.document.file_source.clone(), message.document.file_path.to_string()).await?;
        let metadata = derive_metadata_from_message(&message.document);
        let blob_response = create_blob(storage_container_name, &file, &metadata).await?;
        add_to_search_index("blob data".to_string()).await;
        Ok(FileProcessStatus::Success(message.job_id))
    } else {
        Ok(FileProcessStatus::NothingToProcess)
    }
}

pub fn derive_metadata_from_message(document: &Document) -> HashMap<String, String> {
    let mut metadata: HashMap<String, String> = HashMap::new();
    
    let file_name = metadata::sanitize(&document.id);
    metadata.insert("file_name".to_string(), file_name);
    metadata.insert("doc_type".to_string(), document.document_type.to_string());
    
    let title = metadata::sanitize(&document.name);
    let pl_numbers = metadata::extract_product_licences(&title);

    metadata.insert("title".to_string(), title);
    metadata.insert("pl_number".to_string(), pl_numbers);

    let product_names = metadata::to_json(&document.products);
    let first_product_name = if document.products.len() > 0 {
        document.products[0].to_string()
    } else {
        "".to_string()
    };
    metadata.insert("product_name".to_string(), product_names);

    let active_substances = metadata::to_json(&document.active_substances);
    metadata.insert("substance_name".to_string(), active_substances);
    // What should facets be when there are possibly multiple products and multiple active substances?

    let facets = metadata::to_json(&metadata::create_facets_by_active_substance(
        &first_product_name,
        document.active_substances.clone(),
    ));
    metadata.insert("facets".to_string(), facets);

    let author = metadata::sanitize(&document.author);
    metadata.insert("author".to_string(), author);

    if let Some(keywords) = &document.keywords {
        let keywords_joined = metadata::vec_to_spaced_string(&keywords);
        metadata.insert("keywords".to_string(), keywords_joined);
    }

    return metadata;
}

enum FileProcessStatus {
    Success(uuid::Uuid),
    NothingToProcess,
}

pub async fn create_blob(
    container_name: &str, 
    file_data: &[u8],
    metadata: &HashMap<String, String>
) -> Result<(), anyhow::Error> {
    let storage_client = storage_client::factory()?;
    let blob_name = compute_sha_hash(&file_data);
    let file_digest = md5::compute(&file_data[..]);
    let mut metadata_ref : HashMap<&str, &str> = HashMap::new();
    for (key, val) in metadata {
        metadata_ref.insert(&key, &val);
    }
    
    storage_client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_content_type("application/pdf")
        .with_metadata(&metadata_ref)
        .with_body(&file_data[..])
        .with_content_md5(&file_digest[..])
        .finalize()
        .await
        .map_err(|e| {
            tracing::error!("{:?}", e);
            anyhow!("Couldn't create blob")
        })?;
    Ok(())
}
