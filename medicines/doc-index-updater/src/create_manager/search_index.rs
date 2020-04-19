use crate::create_manager::Blob;
use search_client::{models::IndexEntry, CreateIndexEntry};

pub async fn add_blob_to_search_index(
    search_client: impl CreateIndexEntry,
    blob: Blob,
) -> Result<(), anyhow::Error> {
    let entry: IndexEntry = blob.into();

    tracing::info!("Creating index entry ({:?})", entry);
    search_client.create_index_entry(entry).await.map_err(|e| {
        tracing::error!("Error creating index entry ({:?})", e);
        e
    })?;
    Ok(())
}
