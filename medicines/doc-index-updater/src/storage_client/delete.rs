use super::client::BlobClient;
use async_trait::async_trait;
use azure_sdk_core::{
    errors::AzureError, BlobNameSupport, ContainerNameSupport, DeleteSnapshotsMethod,
    DeleteSnapshotsMethodSupport,
};
use azure_sdk_storage_blob::Blob;

#[async_trait]
pub trait DeleteBlob {
    async fn delete_blob(
        &mut self,
        container_name: &str,
        blob_name: &str,
    ) -> Result<(), AzureError>;
}

#[async_trait]
impl DeleteBlob for BlobClient {
    async fn delete_blob(
        &mut self,
        container_name: &str,
        blob_name: &str,
    ) -> Result<(), AzureError> {
        self.azure_client
            .delete_blob()
            .with_container_name(&container_name)
            .with_blob_name(&blob_name)
            .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
            .finalize()
            .await?;
        Ok(())
    }
}
