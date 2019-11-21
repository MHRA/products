use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use std::path::Path;
use tokio_core::reactor::Core;

pub fn import(_dir: &Path, _client: Client, mut _core: Core) -> Result<(), AzureError> {
    Ok(())
}
