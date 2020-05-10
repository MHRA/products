#[macro_use]
extern crate lazy_static;
use azure_sdk_core::{errors::AzureError, prelude::*};
use azure_sdk_storage_blob::{blob::Blob, prelude::*};
use azure_sdk_storage_core::prelude::Client;
use chrono::{DateTime, Utc};
use futures::stream::StreamExt;
use regex::Regex;
use std::error::Error;

#[tokio::main]
async fn main() {
    create_blob_log().await;
}

async fn create_blob_log() -> Result<(), Box<dyn Error>> {
    let client = get_client()?;
    let blobs_list = get_blobs_list(&client).await?;
    write_to_log_store(&client, blobs_list).await?;
    Ok(())
}

fn get_client() -> Result<Client, AzureError> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    Client::new(&account, &master_key)
}

async fn get_blobs_list(client: &Client) -> Result<Vec<String>, AzureError> {
    let container_name = std::env::var("STORAGE_CONTAINER_NAME")
        .expect("Set env variable STORAGE_MASTER_KEY first!");

    let mut blob_stream = Box::pin(
        client
            .list_blobs()
            .with_container_name(&container_name)
            .with_include_metadata()
            .stream(),
    );

    let mut blob_list: Vec<String> = vec![String::from("Blob name, CON, PLs, created, modified")];

    while let Some(value) = blob_stream.next().await {
        for blob in value?.incomplete_vector.iter() {
            let blob_strings = extract_blob_strings(blob);
            blob_list.extend_from_slice(&blob_strings);
        }
    }

    Ok(blob_list)
}

fn extract_blob_strings(blob: &Blob) -> Vec<String> {
    let con = match blob.metadata.get("file_name") {
        Some(file_name) => file_name.to_owned(),
        None => String::from(""),
    };

    let created = blob.creation_time.to_string();

    let modified = match blob.last_modified {
        Some(date) => date.to_string(),
        None => String::from(""),
    };

    let pls = match blob.metadata.get("pl_number") {
        Some(pls_string) => {
            let pls_vec = get_pls_vec_from_string(pls_string);
            match pls_vec.is_empty() {
                true => vec![String::from("")],
                false => pls_vec,
            }
        }
        None => vec![String::from("")],
    };

    let mut blob_strings = vec![];
    for pl in pls {
        blob_strings.push(format!(
            "{}, {}, {}, {}, {}",
            blob.name, con, pl, created, modified
        ));
    }

    blob_strings
}

async fn write_to_log_store(
    client: &Client,
    blob_list: Vec<String>,
) -> Result<(), AzureError> {
    let contents_log_container_name = std::env::var("STORAGE_CONTAINER_BACKUP_NAME")
        .expect("Set env variable STORAGE_MASTER_KEY first!");

    let blobs_as_string = blob_list.join("\n");
    let file_data = blobs_as_string.as_bytes();
    let file_digest = md5::compute(&file_data[..]);

    let now: DateTime<Utc> = Utc::now();
    let blob_name = now.format("docs-content-log-%Y-%m-%d.csv").to_string();

    client
        .put_block_blob()
        .with_container_name(&contents_log_container_name)
        .with_blob_name(&blob_name)
        .with_content_type("text/csv")
        .with_body(&file_data[..])
        .with_content_md5(&file_digest[..])
        .finalize()
        .await?;

    Ok(())
}

fn get_pls_vec_from_string(pl_string: &str) -> Vec<String> {
    lazy_static! {
        static ref RE_PL: Regex = Regex::new(r"(PL|PLPI|THR)[0-9]+").unwrap();
    }
    RE_PL
        .captures_iter(pl_string)
        .map(|cap| cap[0].to_string())
        .collect()
}
