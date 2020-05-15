use crate::{
    create_manager::hash,
    storage_client::{self, BlobClient},
};
use async_trait::async_trait;
use azure_sdk_core::{
    BlobNameSupport, BodySupport, ContainerNameSupport, ContentMD5Support, ContentTypeSupport,
    MetadataSupport,
};
use azure_sdk_storage_blob::Blob;
use storage_client::GetBlob;

#[async_trait]
pub trait StorageClient {
    async fn add_file(self, file_data: &[u8]) -> Result<StorageFile, StorageClientError>;
    async fn get_file(
        self,
        storage_file_identifier: StorageFile,
    ) -> Result<Vec<u8>, StorageClientError>;
}

pub struct StorageFile {
    pub name: String,
}

#[derive(Debug)]
pub enum StorageClientError {
    RetrievalError { message: String },
    UploadError { message: String },
    ClientError { message: String },
}

pub struct TemporaryBlobStorage {
    container_name: String,
    prefix: String,
}

fn storage_client_factory() -> Result<BlobClient, StorageClientError> {
    let client = storage_client::factory().map_err(|e| {
        tracing::error!("Error creating storage client: {:?}", e);
        StorageClientError::ClientError {
            message: format!("Couldn't create storage client: {:?}", e),
        }
    })?;

    Ok(client)
}

impl Default for TemporaryBlobStorage {
    fn default() -> Self {
        let container_name =
            std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");
        Self {
            container_name,
            prefix: "temp/".to_owned(),
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
        let storage_client = storage_client_factory()?.azure_client;

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

        Ok(StorageFile {
            name: name.to_owned(),
        })
    }
    async fn get_file(self, storage_file: StorageFile) -> Result<Vec<u8>, StorageClientError> {
        let mut storage_client = storage_client_factory()?;

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
