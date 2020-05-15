use super::client::BlobClient;
use super::BlobResponse;
use async_trait::async_trait;
use azure_sdk_core::{errors::AzureError, prelude::*};
use azure_sdk_storage_blob::Blob;

#[async_trait]
pub trait GetBlob {
    async fn get_blob(
        &mut self,
        container_name: &str,
        blob_name: &str,
    ) -> Result<BlobResponse, AzureError>;
}

#[async_trait]
impl GetBlob for BlobClient {
    async fn get_blob(
        &mut self,
        container_name: &str,
        blob_name: &str,
    ) -> Result<BlobResponse, AzureError> {
        let blob = self
            .azure_client
            .get_blob()
            .with_container_name(&container_name)
            .with_blob_name(&blob_name)
            .finalize()
            .await?;

        Ok(BlobResponse {
            blob_name: blob.blob.name,
            data: blob.data,
        })
    }
}
