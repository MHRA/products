use super::{models::StorageClientError, AzureBlobStorage};
use async_trait::async_trait;
use azure_sdk_core::{
    BlobNameSupport, ContainerNameSupport, DeleteSnapshotsMethod, DeleteSnapshotsMethodSupport,
};
use azure_sdk_storage_blob::Blob;

#[async_trait]
pub trait DeleteBlob {
    async fn delete_blob(&mut self, blob_name: &str) -> Result<(), StorageClientError>;
}

#[async_trait]
impl DeleteBlob for AzureBlobStorage {
    async fn delete_blob(&mut self, blob_name: &str) -> Result<(), StorageClientError> {
        self.get_azure_client()?
            .delete_blob()
            .with_container_name(&self.container_name)
            .with_blob_name(&blob_name)
            .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
            .finalize()
            .await?;
        Ok(())
    }
}
