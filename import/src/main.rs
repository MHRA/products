use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use tokio_core::reactor::Core;

fn main() -> Result<(), AzureError> {
    let (client, mut core) = initialize()?;
    spc_pil_import::upload(
        client,
        &mut core,
        include_bytes!("../tests/fixtures/MHRA MIP HLD v0.1.pdf"),
    )
}

fn initialize() -> Result<(Client, Core), AzureError> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let core = Core::new()?;

    Ok((Client::new(&account, &master_key)?, core))
}
