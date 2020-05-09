#[macro_use]
extern crate lazy_static;
use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::Blob;
use azure_sdk_storage_core::prelude::Client;
use chrono::{DateTime, Utc};
use futures::stream::StreamExt;
use regex::Regex;
use std::error::Error;

#[tokio::main]
async fn main() {
    let _ = get_blob_contents().await;
}

async fn get_blob_contents() -> Result<(), Box<dyn Error>> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let container_name = std::env::var("STORAGE_CONTAINER_NAME")
        .expect("Set env variable STORAGE_MASTER_KEY first!");
    let contents_log_container_name = std::env::var("STORAGE_CONTAINER_BACKUP_NAME")
        .expect("Set env variable STORAGE_MASTER_KEY first!");

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
            if count > 2 {
                break;
            }

            let con = match cont.metadata.get("file_name") {
                Some(file_name) => file_name.to_owned(),
                None => String::from(""),
            };

            let created = cont.creation_time.to_string();

            let modified = match cont.last_modified {
                Some(date) => date.to_string(),
                None => String::from(""),
            };

            let pls = match cont.metadata.get("pl_number") {
                Some(pls_string) => get_pls_vec_from_string(pls_string),
                None => vec![String::from("")],
            };

            for pl in pls {
                blob_list.push(format!(
                    "{}, {}, {}, {}, {}",
                    cont.name, con, pl, created, modified
                ));
            }

            count += 1;
        }
    }

    println!("{:?}", blob_list);
    // extract out into methods
    // add tests
    // find how to run in git workflow
    // let blobs_as_string = blob_list.join("\n");
    // let file_data = blobs_as_string.as_bytes();
    // let file_digest = md5::compute(&file_data[..]);

    // let now: DateTime<Utc> = Utc::now();
    // let blob_name = now.format("docs-content-log-%Y-%m-%d.csv").to_string();

    // client
    //     .put_block_blob()
    //     .with_container_name(&contents_log_container_name)
    //     .with_blob_name(&blob_name)
    //     .with_content_type("text/csv")
    //     .with_body(&file_data[..])
    //     .with_content_md5(&file_digest[..])
    //     .finalize()
    //     .await?;

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
