use super::{
    models::{StorageClientError, StorageFile},
    storage_client_factory_with_a_slightly_different_error_type, GetBlob, StorageClient,
};
use crate::create_manager::hash;
use async_trait::async_trait;
use azure_sdk_core::{
    BlobNameSupport, BodySupport, ContainerNameSupport, ContentMD5Support, ContentTypeSupport,
    MetadataSupport,
};
use azure_sdk_storage_blob::Blob;
use std::collections::HashMap;

pub struct TemporaryBlobStorage {
    container_name: String,
    prefix: String,
    storage_account: String,
}

impl Default for TemporaryBlobStorage {
    fn default() -> Self {
        Self::temporary()
    }
}

impl TemporaryBlobStorage {
    pub fn temporary() -> Self {
        let container_name =
            std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");
        let storage_account =
            std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");

        Self {
            container_name,
            prefix: "temp/".to_owned(),
            storage_account,
        }
    }
    pub fn permanent() -> Self {
        let container_name =
            std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");
        let storage_account =
            std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");

        Self {
            container_name,
            prefix: "".to_owned(),
            storage_account,
        }
    }
}

#[async_trait]
impl StorageClient for TemporaryBlobStorage {
    async fn add_file(
        self,
        file_data: &[u8],
        metadata_ref: HashMap<&str, &str>,
    ) -> Result<StorageFile, StorageClientError> {
        let storage_client =
            storage_client_factory_with_a_slightly_different_error_type()?.azure_client;

        let file_digest = md5::compute(&file_data[..]);
        let name = format!("{}{}", &self.prefix, hash::sha1(&file_data));

        storage_client
            .put_block_blob()
            .with_container_name(&self.container_name)
            .with_blob_name(&name)
            .with_content_type("application/pdf")
            .with_metadata(&metadata_ref)
            .with_body(&file_data[..])
            .with_content_md5(&file_digest[..])
            .finalize()
            .await
            .map_err(|e| {
                tracing::error!("Error uploading file to blob storage: {:?}", e);
                StorageClientError::UploadError {
                    message: format!("Couldn't create blob: {:?}", e),
                }
            })?;

        let path = format!(
            "https://{}.blob.core.windows.net/{}/{}",
            &self.storage_account, &self.container_name, &name
        );

        Ok(StorageFile { name, path })
    }
    async fn get_file(self, storage_file: StorageFile) -> Result<Vec<u8>, StorageClientError> {
        let mut storage_client = storage_client_factory_with_a_slightly_different_error_type()?;

        let file_data = storage_client
            .get_blob(&self.container_name, &storage_file.name)
            .await
            .map_err(|e| {
                tracing::error!("Error retrieving file from blob storage: {:?}", e);
                StorageClientError::RetrievalError {
                    message: format!("Couldn't retrieve blob: {:?}", e),
                }
            })?
            .data;

        Ok(file_data)
    }
}
