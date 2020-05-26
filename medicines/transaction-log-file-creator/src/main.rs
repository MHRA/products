extern crate lazy_static;
use anyhow::anyhow;
use async_trait::async_trait;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::blob::responses::GetBlobResponse;
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
    async fn create_append_blob(
        &self,
        blob_name: &String,
        container_name: &String,
    ) -> Result<(), AzureError>;
    async fn get_append_blob(
        &self,
        blob_name: &String,
        container_name: &String,
    ) -> Result<GetBlobResponse, AzureError>;
}

#[async_trait]
impl AppendBlob for Client {
    async fn create_append_blob(
        &self,
        blob_name: &String,
        container_name: &String,
    ) -> Result<(), AzureError> {
        &self
            .put_append_blob()
            .with_container_name(&container_name)
            .with_blob_name(&blob_name)
            .with_content_type("text/csv")
            .finalize()
            .await?;
        Ok(())
    }

    async fn get_append_blob(
        &self,
        blob_name: &String,
        container_name: &String,
    ) -> Result<GetBlobResponse, AzureError> {
        self.get_blob()
            .with_container_name(container_name)
            .with_blob_name(blob_name)
            .finalize()
            .await
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

async fn create_append_blob(
    client: &dyn AppendBlob,
    blob_name: String,
) -> Result<(), anyhow::Error> {
    let log_container_name = std::env::var("LOG_STORAGE_CONTAINER")
        .expect("Set env variable LOG_STORAGE_CONTAINER first!");

    if blob_already_exists(client, &blob_name, &log_container_name).await {
        return Err(anyhow!("Couldn't create blob - already exists"));
    }

    client
        .create_append_blob(&blob_name, &log_container_name)
        .await
        .map_err(|e| {
            eprintln!("Error creating append blob: {:?}", e);
            anyhow!("Couldn't create append blob")
        })?;

    Ok(())
}

async fn blob_already_exists(
    client: &dyn AppendBlob,
    blob_name: &String,
    container_name: &String,
) -> bool {
    let result = client.get_append_blob(blob_name, container_name).await;
    match result {
        Ok(_) => true,
        Err(e) => match e {
            AzureError::UnexpectedHTTPResult(e) => {
                if e.to_string().contains("The specified blob does not exist") {
                    return false;
                }
                true
            }
            _ => true,
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use azure_sdk_storage_blob::blob::BlobType;
    use test_case::test_case;

    struct TestClient {
        pub response: Result<GetBlobResponse, AzureError>,
    }

    impl TestClient {
        pub fn new(response: Result<GetBlobResponse, AzureError>) -> self {
            TestClient { response }
        }
    }

    #[async_trait]
    impl AppendBlob for TestClient {
        async fn create_append_blob(
            &self,
            blob_name: &String,
            container_name: &String,
        ) -> Result<(), AzureError> {
            unimplemented!();
        }

        async fn get_append_blob(
            &self,
            blob_name: &String,
            container_name: &String,
        ) -> Result<GetBlobResponse, AzureError> {
            self.response
        }
    }

    #[test]
    fn test_get_log_file_name() {
        let date = chrono::DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap(),
        );
        let log_file_name = get_log_file_name(date);
        assert_eq!(log_file_name, "file-change-log-1996-12".to_string());
    }

    #[test]
    fn test_blob_already_exists_when_exists() {
        let date = chrono::DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap(),
        );
        let log_file_name = get_log_file_name(date);
        assert_eq!(log_file_name, "file-change-log-1996-12".to_string());
    }
}
