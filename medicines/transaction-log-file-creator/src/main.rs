extern crate lazy_static;
use anyhow::anyhow;
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::Client;
use chrono::{DateTime, Duration, Utc};

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
    let blob_name = get_log_file_name_for_next_month(Utc::now());
    let log_container_name = get_log_container_name();
    create_append_blob(&client, &blob_name, &log_container_name).await
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

fn get_log_file_name_for_next_month(date: DateTime<Utc>) -> String {
    (date + Duration::days(31))
        .format("file-change-log-%Y-%m")
        .to_string()
}

fn get_log_container_name() -> String {
    std::env::var("LOG_STORAGE_CONTAINER").expect("Set env variable LOG_STORAGE_CONTAINER first!")
}

async fn create_append_blob(
    client: &Client,
    blob_name: &str,
    container_name: &str,
) -> Result<(), anyhow::Error> {
    let match_condition_preventing_blob_overwrite = IfMatchCondition::NotMatch("*");

    client
        .put_append_blob()
        .with_container_name(container_name)
        .with_blob_name(blob_name)
        .with_content_type("text/csv")
        .with_if_match_condition(match_condition_preventing_blob_overwrite)
        .finalize()
        .await
        .map_err(|e| {
            eprintln!("Error creating append blob: {:?}", e);
            anyhow!("Couldn't create append blob")
        })?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("1996-12-19T16:39:57-08:00".to_string(), "file-change-log-1997-01".to_string())]
    #[test_case("2020-01-01T00:00:00-00:00".to_string(), "file-change-log-2020-02".to_string())]
    #[test_case("2022-02-01T00:00:00-00:00".to_string(), "file-change-log-2022-03".to_string())]
    fn test_get_log_file_name_adds_1_month(input: String, expected: String) {
        let date = chrono::DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&input).unwrap());
        let log_file_name = get_log_file_name_for_next_month(date);
        assert_eq!(log_file_name, expected);
    }
}
