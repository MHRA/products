use crate::{model, storage};
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use chrono::{DateTime, Utc};
use csv;
use std::{
    collections::HashMap,
    fs,
    fs::{DirEntry, File},
    io::BufReader,
    path::Path,
    str,
};
use tokio_core::reactor::Core;
use urlencoding;

#[derive(Debug, Deserialize, Clone)]
struct Record {
    #[serde(rename = "dDocName")]
    filename: String,
    #[serde(rename = "dDocTitle")]
    title: String,
    #[serde(rename = "dDocAuthor")]
    author: String,
    #[serde(rename = "dCreateDate", with = "crate::date_de")]
    created: DateTime<Utc>,
    #[serde(rename = "dReleaseState")]
    release_state: String,
    #[serde(rename = "xKeywords")]
    keywords: String,
}

pub fn import(dir: &Path, client: Client, mut core: Core) -> Result<(), AzureError> {
    if dir.is_dir() {
        if let Some(Ok(f)) = fs::read_dir(dir)?.find(|f| is_csv(f.as_ref().unwrap())) {
            println!("Found CSV file: {:?}", f);
            let file = File::open(&f.path())?;
            let mut rdr = csv::Reader::from_reader(BufReader::new(file));
            let records = rdr
                .deserialize()
                .map(|r: Result<Record, csv::Error>| {
                    let r = r.expect("Failed to deserialize");
                    (r.filename.clone().to_lowercase(), r)
                })
                .collect::<HashMap<String, Record>>();

            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "pdf" && fs::metadata(&path)?.len() > 0 {
                        let key = &path.file_stem().unwrap().to_str().unwrap();
                        println!("{:?}", key);
                        if let Some(record) = records.get(&key.to_lowercase()) {
                            let mut metadata = HashMap::new();
                            let d = format!("{:?}", model::DocType::Par);
                            let created = record.created.to_string();
                            let title = urlencoding::encode(&record.title);
                            let author = urlencoding::encode(&record.author);
                            let keywords = urlencoding::encode(&record.keywords);
                            metadata.insert("doc_type", d.as_str());
                            metadata.insert("file_name", &record.filename);
                            metadata.insert("title", &title);
                            metadata.insert("author", &author);
                            metadata.insert("created", &created);
                            metadata.insert("release_state", &record.release_state);
                            metadata.insert("keywords", &keywords);
                            storage::upload(&client, &mut core, &fs::read(path)?, &metadata)?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn is_csv(f: &DirEntry) -> bool {
    "csv" == f.path().extension().unwrap_or_default()
}
