use anyhow::anyhow;
use azure_sdk_storage_core::prelude::Client;

pub fn factory() -> Result<Client, anyhow::Error> {
    let storage_account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");

    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    Client::new(&storage_account, &master_key)
        .map_err(|e| {
            tracing::error!("{:?}", e);
            anyhow!("Couldn't create storage client")
        })
}
