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
    let _ = create_blob_log().await;
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

async fn write_to_log_store(client: &Client, blob_list: Vec<String>) -> Result<(), AzureError> {
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

#[cfg(test)]
mod test {
    use super::*;
    use azure_sdk_core::lease::LeaseState;
    use azure_sdk_storage_blob::blob::BlobType;
    use std::collections::HashMap;
    use test_case::test_case;

    #[test_case(
        &"[\"PL1234567\",\"PLPI987654321\", \"THR54321678\"]",
        vec![String::from("PL1234567"), String::from("PLPI987654321"), String::from("THR54321678")]
    )]
    #[test_case(
        &"[\"POL1234567\",\"12345678\", \"THR 54321678\"]",
        vec![]
    )]
    #[test_case(
        &"",
        vec![]
    )]
    fn test_get_pls_vec_from_string(input: &str, expected: Vec<String>) {
        let actual = get_pls_vec_from_string(input);
        assert_eq!(actual.len(), expected.len());
        println!("{:?}", &actual);
        for i in 0..actual.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    fn get_test_blob(name: String, container_name: String, date: DateTime<Utc>, pl: String, file_name: String) -> Blob {
        let mut metadata: HashMap<String,String> = HashMap::new();
        metadata.insert(String::from("pl_number"), String::from(format!("[\"{}\"]", pl)));
        metadata.insert(String::from("file_name"), file_name);
        Blob {
            name,
            container_name,
            snapshot_time: None,
            creation_time: date,
            last_modified: Some(date),
            etag: None,
            content_language: None,
            content_length: 5,
            content_type: None,
            content_encoding: None,
            content_md5: None,
            cache_control: None,
            content_disposition: None,
            x_ms_blob_sequence_number: None,
            blob_type: BlobType::BlockBlob,
            access_tier: None,
            lease_status: None,
            lease_state: LeaseState::Available,
            lease_duration: None,
            copy_id: None,
            copy_status: None,
            copy_source: None,
            copy_progress: None,
            copy_completion_time: None,
            copy_status_description: None,
            incremental_copy: None,
            server_encrypted: false,
            access_tier_inferred: None,
            access_tier_change_time: None,
            deleted_time: None,
            remaining_retention_days: None,
            metadata: metadata,
        }
    }

    #[test]
    fn test_get_blob_string() {
        let date = chrono::DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap(),
        );
        let name = String::from("test_blog");
        let container_name = String::from("test_container");
        let pl = String::from("PL1234567");
        let file_name = String::from("CON1234567");
        let blob = get_test_blob(name, container_name, date, pl, file_name);
        let expected = String::from("test_blog, CON1234567, PL1234567, 1996-12-20 00:39:57 UTC, 1996-12-20 00:39:57 UTC");
        
        let actual = extract_blob_strings(&blob);

        assert_eq!(actual[0], expected);
    }
}
