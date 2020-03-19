use crate::{
    create_manager::models::{BlobMetadata, IndexEntry},
    search_client,
};

//use std::time::{Duration, Instant};

pub async fn add_to_search_index(
    search_client: &search_client::AzureSearchClient,
    blob_name: &str,
    blob: BlobMetadata,
    file_size: usize,
) -> Result<(), anyhow::Error> {
    let storage_account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let container_name =
        std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");
    let storage_path = format!(
        "https://{}.blob.core.windows.net/{}/{}",
        &storage_account, &container_name, &blob_name
    );

    let entry = IndexEntry::for_blob(blob_name.to_owned(), blob, file_size, storage_path);

    tracing::info!("Creating index entry ({:?})", entry);
    search_client.create(entry).await.map_err(|e| {
        tracing::error!("Error creating index entry ({:?})", e);
        e
    })?;
    Ok(())
}
