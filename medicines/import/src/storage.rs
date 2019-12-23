use azure_sdk_core::{errors::AzureError, prelude::*};
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use std::collections::HashMap;
use tokio_core::reactor::Core;

#[allow(clippy::implicit_hasher)]
pub fn upload(
    blob_name: &str,
    client: &Client,
    core: &mut Core,
    data: &[u8],
    metadata: &HashMap<&str, String>,
    verbosity: i8,
) -> Result<(), AzureError> {
    let container_name =
        std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");

    if verbosity >= 2 {
        println!("---------------");
        println!("Blob storage name for file is:");
        println!("{}", blob_name);
        println!("Metadata for file is:");
        println!("{:?}", metadata);
    }

    let digest = md5::compute(&data[..]);
    let metadata = metadata.iter().map(|(k, v)| (*k, v.as_str())).collect();

    let future = client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_content_type("application/pdf")
        .with_metadata(&metadata)
        .with_body(&data[..])
        .with_content_md5(&digest[..])
        .finalize();

    core.run(future)?;

    trace!("created {:?}", blob_name);
    Ok(())
}

pub fn delete(
    blob_name: &str,
    client: &Client,
    core: &mut Core,
    verbosity: i8,
) -> Result<(), AzureError> {
    let container_name =
        std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");

    if verbosity >= 2 {
        println!("---------------");
        println!("DELETE: Blob storage name for file is:");
        println!("{}", blob_name);
    }

    let future = client
        .delete_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .finalize();

    core.run(future)?;

    trace!("deleted {:?}", blob_name);
    Ok(())
}
