use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::Client;

pub use client::BlobClient;
pub use client::StorageClient;
pub use delete::DeleteBlob;
pub use get::GetBlob;

mod client;
mod delete;
mod get;

pub struct BlobResponse {
    pub blob_name: String,
    pub data: Vec<u8>,
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
