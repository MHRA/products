extern crate lazy_static;
use anyhow::anyhow;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::Client;
use chrono::{DateTime, Utc};
use hyper::StatusCode;

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
    let log_container_name = std::env::var("LOG_STORAGE_CONTAINER")
        .expect("Set env variable LOG_STORAGE_CONTAINER first!");

    if !should_create_blob(client, &blob_name, &log_container_name).await {
        return Err(anyhow!("Didn't create blob - it may already exist"));
    }

    client
        .put_append_blob()
        .with_container_name(&log_container_name)
        .with_blob_name(&blob_name)
        .with_content_type("text/csv")
        .finalize()
        .await
        .map_err(|e| {
            eprintln!("Error creating append blob: {:?}", e);
            anyhow!("Couldn't create append blob")
        })?;

    Ok(())
}

async fn should_create_blob(client: &Client, blob_name: &str, container_name: &str) -> bool {
    let get_blob_exists_result = get_if_file_exists(client, blob_name, container_name).await;
    result_is_file_does_not_exist_error(get_blob_exists_result)
}

async fn get_if_file_exists(
    client: &Client,
    blob_name: &str,
    container_name: &str,
) -> Result<bool, AzureError> {
    client
        .get_blob()
        .with_container_name(container_name)
        .with_blob_name(blob_name)
        .finalize()
        .await
        .map(|_response| true)
}

fn result_is_file_does_not_exist_error(result: Result<bool, AzureError>) -> bool {
    match result {
        Err(AzureError::UnexpectedHTTPResult(e)) => e.status_code() == StatusCode::NOT_FOUND,
        Err(e) => {
            eprintln!("Error getting file: {:?}", e);
            false
        }
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use azure_sdk_core::errors::UnexpectedHTTPResult;
    use hyper::StatusCode;
    use test_case::test_case;

    #[test]
    fn test_get_log_file_name() {
        let date = chrono::DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap(),
        );
        let log_file_name = get_log_file_name(date);
        assert_eq!(log_file_name, "file-change-log-1996-12".to_string());
    }

    fn get_file_not_found_error() -> AzureError {
        AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
            StatusCode::from_u16(200).unwrap(),
            StatusCode::from_u16(404).unwrap(),
            &"",
        ))
    }

    fn get_other_error() -> AzureError {
        AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
            StatusCode::from_u16(200).unwrap(),
            StatusCode::from_u16(400).unwrap(),
            &"",
        ))
    }

    #[test_case(Err(get_file_not_found_error()), true)]
    #[test_case(Err(get_other_error()), false)]
    #[test_case(Ok(true), false)]
    fn check_result_returns_true_when_file_does_not_exist(
        input: Result<bool, AzureError>,
        expected_output: bool,
    ) {
        let result = result_is_file_does_not_exist_error(input);
        assert_eq!(result, expected_output);
    }
}
