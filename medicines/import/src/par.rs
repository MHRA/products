use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use csv;
use std::{
    collections::HashMap,
    fs,
    fs::{DirEntry, File},
    io::BufReader,
    path::Path,
};
use tokio_core::reactor::Core;

#[derive(Debug, Deserialize, Clone)]
struct Record {
    #[serde(rename = "dDocAuthor")]
    author: String,
    #[serde(rename = "dDocName")]
    filename: String,
}

pub fn import(dir: &Path, _client: Client, mut _core: Core) -> Result<(), AzureError> {
    if dir.is_dir() {
        if let Some(Ok(f)) = fs::read_dir(dir)?.find(|f| is_csv(f.as_ref().unwrap())) {
            println!("Found CSV file: {:?}", f);
            let file = File::open(&f.path())?;
            let mut rdr = csv::Reader::from_reader(BufReader::new(file));
            let records = rdr
                .deserialize()
                .map(|r: Result<Record, csv::Error>| {
                    let r = r.unwrap();
                    (r.filename.clone().to_lowercase(), r)
                })
                .collect::<HashMap<String, Record>>();
            println!("{:?}", records);
        }
    }
    Ok(())
}

fn is_csv(f: &DirEntry) -> bool {
    "csv" == f.path().extension().unwrap_or_default()
}
