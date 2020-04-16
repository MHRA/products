use async_trait::async_trait;
use azure_sdk_core::{errors::AzureError, prelude::*, DeleteSnapshotsMethod};
use azure_sdk_storage_blob::Blob;
use azure_sdk_storage_core::prelude::*;

pub struct BlobClient {
    pub azure_client: Client,
}

impl BlobClient {
    pub fn new(azure_client: Client) -> BlobClient {
        BlobClient {
            azure_client: azure_client,
        }
    }
}

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

pub fn factory() -> Result<BlobClient, AzureError> {
    let storage_account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");

    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    match base64::decode(&master_key) {
        Ok(_) => Ok(BlobClient::new(Client::new(&storage_account, &master_key)?)),
        Err(e) => Err(AzureError::Base64DecodeError(e)),
    }
}
