use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::Blob;
use azure_sdk_storage_core::prelude::Client;
use futures::stream::StreamExt;
use std::error::Error;

#[tokio::main]
async fn main() {
    let _ = get_blob_contents().await;
}

async fn get_blob_contents() -> Result<(), Box<dyn Error>> {
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let container_name = std::env::var("STORAGE_CONTAINER_NAME").expect("Set env variable STORAGE_MASTER_KEY first!");
    let contents_log_container_name = std::env::var("STORAGE_CONTAINER_BACKUP_NAME").expect("Set env variable STORAGE_MASTER_KEY first!");

    let client = Client::new(&account, &master_key)?;

    let mut stream = Box::pin(
        client
            .list_blobs()
            .with_container_name(&container_name)
            .with_include_metadata()
            .stream(),
    );

    let mut count: i32 = 0;
    let mut blob_list: Vec<String> = vec![String::from("Blob name, CON, PLs, created, modified")];

    while let Some(value) = stream.next().await {
        // let len = value?.incomplete_vector.len();
        for cont in value?.incomplete_vector.iter() {
            if count > 3 {
                break;
            }

            let con = if let Some(file_name) = cont.metadata.get("file_name") {
                file_name.to_owned()
            } else {
                String::from("")
            };

            let pls = if let Some(pls) = cont.metadata.get("pl_number") {
                pls.to_owned()
            } else {
                String::from("")
            };

            let created = cont.creation_time.to_string();

            let modified = if let Some(date) = cont.last_modified {
                date.to_string()
            } else {
                String::from("")
            };

            blob_list.push(format!(
                "{}, {}, {}, {}, {}",
                cont.name, con, pls, created, modified
            ));
            count += 1;
        }
    }

    // construct blob name to include date
    // save file first before uploading as blob or find way to convert to file data
    // also include content md5
    // extract out into methods
    // add tests
    // find how to run in git workflow

    let blob_name = client
        .put_block_blob()
        .with_container_name(&contents_log_container_name)
        .with_blob_name(&name)
        .with_content_type("application/pdf")
        .with_metadata(&metadata_ref)
        .with_body(&file_data[..])
        .with_content_md5(&file_digest[..])
        .finalize()
        .await
        .map_err(|e| anyhow!("Couldn't upload to blob storage: {:?}", e))?;

    println!("{:?}", blob_list);

    Ok(())
}
