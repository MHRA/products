extern crate lazy_static;
use anyhow::anyhow;
use async_trait::async_trait;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::Client;
use chrono::{DateTime, Utc};

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

#[async_trait]
trait AppendBlob {
    async fn get_append_blob_exists(
        &self,
        blob_name: &str,
        container_name: &str,
    ) -> Result<bool, AzureError>;
}

#[async_trait]
impl AppendBlob for Client {
    async fn get_append_blob_exists(
        &self,
        blob_name: &str,
        container_name: &str,
    ) -> Result<bool, AzureError> {
        let _ = self
            .get_blob()
            .with_container_name(container_name)
            .with_blob_name(blob_name)
            .finalize()
            .await?;
        Ok(true)
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

async fn should_create_blob(
    client: &dyn AppendBlob,
    blob_name: &str,
    container_name: &str,
) -> bool {
    let blob_exists_result = client
        .get_append_blob_exists(blob_name, container_name)
        .await;
    match blob_exists_result {
        Err(AzureError::UnexpectedHTTPResult(e)) => {
            let blob_does_not_exist =
                e.to_string() == "Unexpected HTTP result (expected: [200], received: 404)";
            return blob_does_not_exist;
        }
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use azure_sdk_core::errors::UnexpectedHTTPResult;
    use hyper::StatusCode;
    use tokio_test::block_on;

    #[test]
    fn test_get_log_file_name() {
        let date = chrono::DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap(),
        );
        let log_file_name = get_log_file_name(date);
        assert_eq!(log_file_name, "file-change-log-1996-12".to_string());
    }

    #[test]
    fn should_create_blob_returns_true_when_it_does_not_already_exist() {
        let client = TestClient {};
        let should_create_blob_result = block_on(should_create_blob(
            &client,
            &"blob_not_found",
            &"container_name",
        ));
        assert_eq!(should_create_blob_result, true);
    }

    #[test]
    fn should_create_blob_returns_false_when_it_does_already_exist() {
        let client = TestClient {};
        let should_create_blob_result = block_on(should_create_blob(
            &client,
            &"blob_exists",
            &"container_name",
        ));
        assert_eq!(should_create_blob_result, false);
    }

    #[test]
    fn should_create_blob_returns_false_when_encountering_another_error() {
        let client = TestClient {};
        let should_create_blob_result = block_on(should_create_blob(
            &client,
            &"other_error",
            &"container_name",
        ));
        assert_eq!(should_create_blob_result, false);
    }

    struct TestClient {}

    #[async_trait]
    impl AppendBlob for TestClient {
        async fn get_append_blob_exists(
            &self,
            blob_name: &str,
            _container_name: &str,
        ) -> Result<bool, AzureError> {
            match blob_name {
                "blob_exists" => Ok(true),
                "blob_not_found" => {
                    Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
                        StatusCode::from_u16(200).unwrap(),
                        StatusCode::from_u16(404).unwrap(),
                        &"",
                    )))
                }
                "other_error" => Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
                    StatusCode::from_u16(200).unwrap(),
                    StatusCode::from_u16(400).unwrap(),
                    &"",
                ))),
                _ => panic!("Test blob name does not match any arms"),
            }
        }
    }
}
