use azure_sdk_core::{errors::AzureError, prelude::*};
use azure_sdk_storage_blob::{
    container::{PublicAccess, PublicAccessSupport},
    prelude::*,
};
use azure_sdk_storage_core::prelude::*;
use std::collections::HashMap;
use tokio_core::reactor::Core;

#[allow(clippy::implicit_hasher)]
pub fn upload_with_retry(
    blob_name: &str,
    client: &Client,
    core: &mut Core,
    data: &[u8],
    metadata: &HashMap<&str, &str>,
    verbosity: i8,
) -> Result<(), AzureError> {
    let mut attempt = 1;
    let mut result = Ok(());
    while attempt < 4 {
        result = upload(&blob_name, &client, core, &data, &metadata, verbosity);
        match &result {
            Ok(_v) => return result,
            Err(e) => println!("Error uploading file: {:?}, attempt: {:?}", e, attempt),
        }
        attempt += 1;
    }
    return result;
}

#[allow(clippy::implicit_hasher)]
pub fn create_container(client: &Client, core: &mut Core) -> Result<(), AzureError> {
    let container_name =
        std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");
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
                .with_container_name(&container_name)
                .with_public_access(PublicAccess::Blob)
                .finalize(),
        )?;
    }
    Ok(())
}

#[allow(clippy::implicit_hasher)]
pub fn upload(
    blob_name: &str,
    client: &Client,
    core: &mut Core,
    data: &[u8],
    metadata: &HashMap<&str, &str>,
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
