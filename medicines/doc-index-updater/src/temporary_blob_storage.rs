use crate::{
    create_manager::hash,
    pars_upload::SubmissionError,
    storage_client::{self, BlobClient},
};
use async_trait::async_trait;
use azure_sdk_core::{
    BlobNameSupport, BodySupport, ContainerNameSupport, ContentMD5Support, ContentTypeSupport,
};
use azure_sdk_storage_blob::Blob;

#[async_trait]
pub trait StorageClient {
    async fn add_file(self, file_data: &[u8]) -> Result<StorageFile, SubmissionError>;
    fn get_file(self, storage_file_identifier: StorageFile) -> Vec<u8>;
}

pub struct TemporaryBlobStorage {}

pub struct StorageFile {
    pub name: String,
    pub path: String,
}

fn storage_client_factory() -> Result<BlobClient, SubmissionError> {
    let client = storage_client::factory().map_err(|e| {
        tracing::error!("Error creating storage client: {:?}", e);
        SubmissionError::UploadError {
            message: format!("Couldn't create storage client: {:?}", e),
        }
    })?;

    Ok(client)
}

#[async_trait]
impl StorageClient for TemporaryBlobStorage {
    async fn add_file(self, file_data: &[u8]) -> Result<StorageFile, SubmissionError> {
        let storage_client = storage_client_factory()?.azure_client;

        let container_name =
            std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");
        let file_digest = md5::compute(&file_data[..]);
        let name = format!("temp/{}", hash::sha1(&file_data));

        storage_client
            .put_block_blob()
            .with_container_name(&container_name)
            .with_blob_name(&name)
            .with_content_type("application/pdf")
            .with_body(&file_data[..])
            .with_content_md5(&file_digest[..])
            .finalize()
            .await
            .map_err(|e| {
                tracing::error!("Error uploading file to blob storage: {:?}", e);
                SubmissionError::UploadError {
                    message: format!("Couldn't create blob: {:?}", e),
                }
            })?;

        let storage_account =
            std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
        let path = format!(
            "https://{}.blob.core.windows.net/{}/{}",
            &storage_account, &container_name, &name
        );

        Ok(StorageFile {
            name: name.to_owned(),
            path,
        })
    }
    fn get_file(self, _storage_file_identifier: StorageFile) -> Vec<u8> {
        todo!()
    }
}
