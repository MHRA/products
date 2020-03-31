use crate::{
    create_manager::{models::IndexEntry, Blob},
    search_client,
};

pub async fn add_blob_to_search_index(
    search_client: &search_client::AzureSearchClient,
    blob: Blob,
) -> Result<(), anyhow::Error> {
    let entry: IndexEntry = blob.into();

    tracing::info!("Creating index entry ({:?})", entry);
    search_client.create(entry).await.map_err(|e| {
        tracing::error!("Error creating index entry ({:?})", e);
        e
    })?;
    Ok(())
}
