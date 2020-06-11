extern crate lazy_static;
use azure_sdk_core::{errors::AzureError, prelude::*};
use azure_sdk_storage_blob::{blob::Blob, prelude::*};
use azure_sdk_storage_core::prelude::Client;
use chrono::{DateTime, Utc};
use futures::stream::StreamExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let blobs_list = get_blobs_list(&get_read_client()?).await?;
    write_to_log_store(&get_write_client()?, blobs_list).await?;
    println!("Completed successfully");
    Ok(())
}

fn get_read_client() -> Result<Client, AzureError> {
    let account = std::env::var("PRODUCTS_STORAGE_ACCOUNT")
        .expect("Set env variable PRODUCTS_STORAGE_ACCOUNT first!");
    let master_key = std::env::var("PRODUCTS_STORAGE_MASTER_KEY")
        .expect("Set env variable PRODUCTS_STORAGE_MASTER_KEY first!");
    Client::new(&account, &master_key)
}

fn get_write_client() -> Result<Client, AzureError> {
    let account =
        std::env::var("LOG_STORAGE_ACCOUNT").expect("Set env variable LOG_STORAGE_ACCOUNT first!");
    let master_key = std::env::var("LOG_STORAGE_MASTER_KEY")
        .expect("Set env variable LOG_STORAGE_MASTER_KEY first!");
    Client::new(&account, &master_key)
}

async fn get_blobs_list(client: &Client) -> Result<Vec<String>, AzureError> {
    let container_name = std::env::var("PRODUCTS_STORAGE_CONTAINER_NAME")
        .expect("Set env variable PRODUCTS_STORAGE_CONTAINER_NAME first!");

    let mut blob_stream = Box::pin(
        client
            .list_blobs()
            .with_container_name(&container_name)
            .with_include_metadata()
            .stream(),
    );

    let mut blob_list: Vec<String> = vec![];

    while let Some(value) = blob_stream.next().await {
        for blob in value?.incomplete_vector.iter() {
            blob_list.push(get_blob_string(blob));
        }
    }

    Ok(blob_list)
}

fn get_blob_string(blob: &Blob) -> String {
    format!("{:?}", blob)
}

async fn write_to_log_store(client: &Client, blob_list: Vec<String>) -> Result<(), AzureError> {
    let contents_log_container_name = std::env::var("LOG_STORAGE_CONTAINER_NAME")
        .expect("Set env variable LOG_STORAGE_CONTAINER_NAME first!");

    let blobs_as_string = blob_list.join("\n");
    let file_data = blobs_as_string.as_bytes();
    let file_digest = md5::compute(&file_data[..]);

    let now: DateTime<Utc> = Utc::now();
    let blob_name = now.format("docs-content-log-%Y-%m-%d.txt").to_string();

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

#[cfg(test)]
mod test {
    use super::*;
    use azure_sdk_core::lease::LeaseState;
    use azure_sdk_storage_blob::blob::BlobType;
    use std::collections::HashMap;

    fn get_test_blob(
        name: String,
        container_name: String,
        date: DateTime<Utc>,
        pl: String,
        file_name: String,
        doc_type: String,
    ) -> Blob {
        let mut metadata: HashMap<String, String> = HashMap::new();
        metadata.insert(
            String::from("pl_number"),
            String::from(format!("[\"{}\"]", pl)),
        );
        metadata.insert(String::from("file_name"), file_name);
        metadata.insert(String::from("doc_type"), doc_type);
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
            metadata,
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
        let doc_type = String::from("Spc");
        let blob = get_test_blob(name, container_name, date, pl, file_name, doc_type);

        // Have to match metadata components separately as metadata hashmap randomly ordered
        let expected_body = String::from(
            "Blob { name: \"test_blog\", container_name: \"test_container\", snapshot_time: None, creation_time: 1996-12-20T00:39:57Z, last_modified: Some(1996-12-20T00:39:57Z), etag: None, content_length: 5, content_type: None, content_encoding: None, content_language: None, content_md5: None, cache_control: None, content_disposition: None, x_ms_blob_sequence_number: None, blob_type: BlockBlob, access_tier: None, lease_status: None, lease_state: Available, lease_duration: None, copy_id: None, copy_status: None, copy_source: None, copy_progress: None, copy_completion_time: None, copy_status_description: None, incremental_copy: None, server_encrypted: false, access_tier_inferred: None, access_tier_change_time: None, deleted_time: None, remaining_retention_days: None, metadata: {",
        );
        let expected_file_name = String::from(r#""file_name": "CON1234567"#);
        let expected_pl_number = String::from(r#""pl_number": "[\"PL1234567\"]"#);
        let expected_doc_type = String::from(r#""doc_type": "Spc""#);

        let actual = get_blob_string(&blob);

        assert!(actual.contains(&expected_body));
        assert!(actual.contains(&expected_file_name));
        assert!(actual.contains(&expected_pl_number));
        assert!(actual.contains(&expected_doc_type));
    }
}
