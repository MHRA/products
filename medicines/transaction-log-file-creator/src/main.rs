#[macro_use]
extern crate lazy_static;
use anyhow::anyhow;
use azure_sdk_core::{errors::AzureError, prelude::*};
use azure_sdk_storage_blob::{blob::Blob, prelude::*};
use azure_sdk_storage_core::prelude::Client;
use chrono::{DateTime, Utc};
use futures::stream::StreamExt;
use regex::Regex;
use std::error::Error;

#[tokio::main]
async fn main() {
    match create_log_file().await {
        Ok(()) => println!("Log blob created successfully"),
        Err(e) => {
            eprintln!("Error creating blob: {:?}", e);
            panic!("Couldn't create blob");
        }
    }
}

async fn create_log_file() -> Result<(), anyhow::Error> {
    let client = get_client()?;
    let blob_name = get_log_file_name(Utc::now());
    create_append_blob(&client, blob_name).await
}

fn get_client() -> Result<Client, anyhow::Error> {
    let account =
        std::env::var("LOG_STORAGE_ACCOUNT").expect("Set env variable LOG_STORAGE_ACCOUNT first!");
    let master_key = std::env::var("LOG_STORAGE_MASTER_KEY")
        .expect("Set env variable LOG_STORAGE_MASTER_KEY first!");
    Client::new(&account, &master_key).map_err(|e| {
        eprint!("Error creating storage client: {:?}", e);
        anyhow!("Error creating storage client")
    })
}

fn get_log_file_name(date: DateTime<Utc>) -> String {
    date.format("file-change-log-%Y-%m").to_string()
}

async fn create_append_blob(client: &Client, blob_name: String) -> Result<(), anyhow::Error> {
    let log_container_name = std::env::var("LOG_STORAGE_CONTAINER_NAME")
        .expect("Set env variable LOG_STORAGE_CONTAINER_NAME first!");

    client
        .put_append_blob()
        .with_container_name(&log_container_name)
        .with_blob_name(&blob_name)
        .with_content_type("text/csv")
        .finalize()
        .await
        .map_err(|e| {
            eprint!("Error creating storage client: {:?}", e);
            anyhow!("Error creating storage client")
        })?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use azure_sdk_storage_blob::blob::BlobType;
    use test_case::test_case;

    #[test]
    fn test_get_log_file_name() {
        let date = chrono::DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap(),
        );
        let log_file_name = get_log_file_name(date);
        assert_eq!(log_file_name, "file-change-log-1996-12".to_string());
    }
}
