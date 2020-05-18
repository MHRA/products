pub use azure_blob_client::AzureBlobStorage;
pub use client::StorageClient;
pub use delete::DeleteBlob;
pub use get::GetBlob;
pub use sftp_client::SftpClient;

mod azure_blob_client;
mod client;
mod delete;
mod get;
pub mod models;
mod sftp_client;

#[cfg(test)]
pub mod test {
    use super::models::StorageClientError;
    use crate::storage_client::DeleteBlob;
    use async_trait::async_trait;

    pub struct TestAzureStorageClient {
        pub can_delete_blob: bool,
    }

    #[async_trait]
    impl DeleteBlob for TestAzureStorageClient {
        async fn delete_blob(&mut self, _blob_name: &str) -> Result<(), StorageClientError> {
            if self.can_delete_blob {
                Ok(())
            } else {
                Err(StorageClientError::Generic(anyhow::anyhow!(
                    "blob could not be deleted".to_string()
                )))
            }
        }
    }
}
