use super::{
    models::{BlobResponse, StorageClientError},
    AzureBlobStorage,
};
use async_trait::async_trait;
use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::Blob;

#[async_trait]
pub trait GetBlob {
    async fn get_blob(&self, blob_name: &str) -> Result<BlobResponse, StorageClientError>;
}

#[async_trait]
impl GetBlob for AzureBlobStorage {
    async fn get_blob(&self, blob_name: &str) -> Result<BlobResponse, StorageClientError> {
        let blob = self
            .get_azure_client()?
            .get_blob()
            .with_container_name(&self.container_name)
            .with_blob_name(&blob_name)
            .finalize()
            .await?;

        Ok(BlobResponse {
            blob_name: blob.blob.name,
            data: blob.data,
        })
    }
}
