use azure_sdk_core::{errors::AzureError, prelude::*};
use azure_sdk_storage_blob::{
    container::{PublicAccess, PublicAccessSupport},
    prelude::*,
};
use azure_sdk_storage_core::prelude::*;
use sha1;
use std::collections::HashMap;
use tokio_core::reactor::Core;

#[allow(clippy::implicit_hasher)]
pub fn upload(
    blob_name: &str,
    client: &Client,
    core: &mut Core,
    data: &[u8],
    metadata: &HashMap<&str, &str>,
) -> Result<(), AzureError> {
    println!("Saving {:?} to blob storage...", blob_name);
    let container_name = "docs";

    if core
        .run(client.list_containers().finalize())?
        .incomplete_vector
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        core.run(
            client
                .create_container()
                .with_container_name(container_name)
                .with_public_access(PublicAccess::Blob)
                .finalize(),
        )?;
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]);
    let future = client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_content_type("application/pdf")
        .with_metadata(metadata)
        .with_body(&data[..])
        .with_content_md5(&digest[..])
        .finalize();

    core.run(future)?;

    trace!("created {:?}", blob_name);
    Ok(())
}
