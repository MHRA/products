use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::Client;

pub fn factory() -> Result<Client, AzureError> {
    let storage_account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");

    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    Client::new(&storage_account, &master_key)
}
