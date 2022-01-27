use azure_core::prelude::Metadata;
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use chrono::{DateTime, Utc};
use futures::stream::StreamExt;
use std::error::Error;

fn derive_territory(pl_number: &str) -> Option<&str> {
    let ni = "NI";
    let gb = "GB";

    if pl_number.contains(ni) {
        Some(ni)
    } else if pl_number.contains(gb) {
        Some(gb)
    } else {
        None
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let account = std::env::var("PRODUCTS_STORAGE_ACCOUNT")
        .expect("Set env variable PRODUCTS_STORAGE_ACCOUNT first!");
    let master_key = std::env::var("PRODUCTS_STORAGE_MASTER_KEY")
        .expect("Set env variable PRODUCTS_STORAGE_MASTER_KEY first!");
    let container_name = std::env::var("PRODUCTS_STORAGE_CONTAINER_NAME")
        .expect("Set env variable PRODUCTS_STORAGE_CONTAINER_NAME first!");

    let http_client = azure_core::new_http_client();
    let storage_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_storage_client();

    let container_client = storage_client.as_container_client(&container_name);

    let mut stream = Box::pin(
        container_client
            .list_blobs()
            .include_metadata(true)
            .stream(),
    );

    let mut blob_list: Vec<Blob> = vec![];
    let pl_key = "pl_number";
    let territory_key = "territory";
    let doc_type_key = "doc_type";
    let par = String::from("Par");

    let date = chrono::DateTime::<Utc>::from(
        DateTime::parse_from_rfc3339("2021-01-01T00:00:00+00:00").unwrap(),
    );

    let mut i = 0;
    let mut total_count = 0;
    while let Some(value) = stream.next().await {
        for blob in value.unwrap().blobs.blobs.into_iter() {
            if total_count % 1000 == 0 {
                println!("Searched {} records", total_count);
            }
            total_count = total_count + 1;

            let metadata = match blob.metadata {
                Some(ref data) => data,
                None => continue,
            };

            match metadata.get(doc_type_key) {
                Some(blob_doc_type) => {
                    if blob_doc_type.ne(&par) {
                        continue;
                    }
                }
                None => {
                    println!("No doc_type for blob {}", blob.name);
                    continue;
                }
            }

            if date > blob.properties.creation_time {
                continue;
            }

            if metadata.get(territory_key).is_some() {
                continue;
            }

            let pl = match metadata.get(pl_key) {
                Some(blob_pl_number) => blob_pl_number,
                None => {
                    println!("No pl_number for blob {}", blob.name);
                    continue;
                }
            };

            if pl.contains("NI") || pl.contains("GB") {
                blob_list.push(blob);
                i = i + 1;
            }
        }
    }

    println!("Matched blobs: {}, Total count: {}", i, total_count);

    let territory_header = "territory".to_string();
    for blob in blob_list {
        let metadata = blob.metadata.unwrap();

        let mut new_metadata = Metadata::new();
        for (key, value) in metadata.iter() {
            new_metadata.insert(key, value.as_bytes().to_owned());
        }

        let territory = derive_territory(metadata.get(pl_key).unwrap()).unwrap();

        new_metadata.insert(&territory_header, territory.as_bytes().to_owned());

        println!("{:?}", new_metadata);

        let blob_name = blob.name.clone();
        let blob_client = container_client.as_blob_client(blob.name);

        match blob_client
            .set_metadata()
            .metadata(&new_metadata)
            .execute()
            .await
        {
            Ok(_) => println!("Set territory for {} to {}", blob_name, territory),
            Err(_) => println!("Error setting territory for {} to {}", blob_name, territory),
        }
    }

    Ok(())
}
