extern crate lazy_static;
use azure_sdk_core::{errors::AzureError, prelude::*};
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use futures::StreamExt;
use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Arc::new(get_client()?);
    let blobs_list = get_blobs_list(&client).await?;
    copy_blobs(&client, blobs_list).await?;
    println!("Completed successfully");
    Ok(())
}

fn get_client() -> Result<Box<dyn Client>, AzureError> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    Ok(Box::new(client::with_access_key(&account, &master_key)))
}

fn get_copy_source_base_url() -> String {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let container_name = std::env::var("SOURCE_CONTAINER_NAME")
        .expect("Set env variable SOURCE_CONTAINER_NAME first!");
    format!(
        "https://{}.blob.core.windows.net/{}",
        account, container_name
    )
}

async fn get_blobs_list(client: &Arc<Box<dyn Client>>) -> Result<Vec<String>, AzureError> {
    let container_name = std::env::var("SOURCE_CONTAINER_NAME")
        .expect("Set env variable SOURCE_CONTAINER_NAME first!");

    let mut blob_stream = Box::pin(
        client
            .list_blobs()
            .with_container_name(&container_name)
            .stream(),
    );

    let mut blob_list: Vec<String> = vec![];

    while let Some(value) = blob_stream.next().await {
        for blob in value?.incomplete_vector.iter() {
            blob_list.push(blob.name.to_string());
        }
    }

    Ok(blob_list)
}

async fn copy_blobs(
    client: &Arc<Box<dyn Client>>,
    blob_list: Vec<String>,
) -> Result<(), AzureError> {
    let destination = "$web";
    let copy_source_base_url = get_copy_source_base_url();

    for blob in blob_list {
        let source_url = &format!("{}/{}", copy_source_base_url, &blob);
        let blob_name = &format!("content/{}", &blob);
        client
            .copy_blob_from_url()
            .with_container_name(&destination)
            .with_blob_name(blob_name)
            .with_source_url(source_url)
            .with_is_synchronous(false)
            .finalize()
            .await?;
    }

    Ok(())
}
