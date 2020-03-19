use crate::{create_manager::metadata::BlobMetadata, models::IndexEntry, search_client};

//use std::time::{Duration, Instant};

pub async fn add_to_search_index(
    search_client: &search_client::AzureSearchClient,
    blob_name: &str,
    _blob: BlobMetadata,
    _file_size: usize,
) -> Result<(), anyhow::Error> {
    let _storage_account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let _container_name =
        std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");
    // let storage_path = format!(
    //     "https://{}.blob.core.windows.net/{}/{}",
    //     &storage_account, &container_name, &blob_name
    // );

    let entry = IndexEntry::for_blob(blob_name.to_owned());

    tracing::info!("Creating index entry ({:?})", entry);
    search_client.create(entry).await.map_err(|e| {
        tracing::error!("Error creating index entry ({:?})", e);
        e
    })?;
    Ok(())
}
