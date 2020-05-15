use crate::temporary_blob_storage::{StorageClientError, StorageFile};
use async_trait::async_trait;
use azure_sdk_storage_core::prelude::Client;
use std::collections::HashMap;

pub struct BlobClient {
    pub azure_client: Client,
}

impl BlobClient {
    pub fn new(azure_client: Client) -> BlobClient {
        BlobClient { azure_client }
    }
}

#[async_trait]
pub trait StorageClient {
    async fn add_file(
        self,
        file_data: &[u8],
        metadata_ref: HashMap<&str, &str>,
    ) -> Result<StorageFile, StorageClientError>;
    async fn get_file(
        self,
        storage_file_identifier: StorageFile,
    ) -> Result<Vec<u8>, StorageClientError>;
}
