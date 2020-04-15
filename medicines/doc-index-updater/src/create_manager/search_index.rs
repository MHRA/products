use crate::create_manager::Blob;
use fehler::throws;
use search_client::{models::IndexEntry, CreateIndexEntry};

#[throws(anyhow::Error)]
pub async fn add_blob_to_search_index(search_client: impl CreateIndexEntry, blob: Blob) {
    let entry: IndexEntry = blob.into();

    tracing::info!("Creating index entry ({:?})", entry);
    search_client.create_index_entry(entry).await.map_err(|e| {
        tracing::error!("Error creating index entry ({:?})", e);
        e
    })?;
}
