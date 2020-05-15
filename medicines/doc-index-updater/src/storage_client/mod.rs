pub use azure_blob_client::AzureBlobStorage;
pub use client::StorageClient;
pub use delete::DeleteBlob;
pub use get::GetBlob;

mod azure_blob_client;
mod client;
mod delete;
mod get;
pub mod models;

pub struct BlobResponse {
    pub blob_name: String,
    pub data: Vec<u8>,
}

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
                Err(StorageClientError::ClientError {
                    message: "blob could not be deleted".to_string(),
                })
            }
        }
    }
}
