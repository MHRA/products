use crate::{csv, metadata, model, pdf, storage};
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use std::{collections::HashMap, fs, path::Path, str};
use tokio_core::reactor::Core;
use crate::{hash::hash, report::Report};

pub fn import(dir: &Path, client: Client, mut core: Core) -> Result<(), AzureError> {
    if let Ok(records) = csv::load_csv(dir) {
        let mut report = Report::new();
        for path in pdf::get_pdfs(dir)? {
            let key = path
                .file_stem()
                .expect("file has no stem")
                .to_str()
                .unwrap();
            println!("Processing {}...", path.to_str().unwrap());
            if let Some(record) = records.get(&key.to_lowercase()) {
                let mut metadata: HashMap<&str, &str> = HashMap::new();
                let file_name = metadata::sanitize(&record.filename);
                metadata.insert("file_name", &file_name);
                let release_state = metadata::sanitize(&record.release_state);
                metadata.insert("release_state", &release_state);
                let doc_type = format!("{:?}", model::DocType::Par);

                if release_state != "Y" {
                    println!("Skipping {} because it is not released.", file_name);
                    report.add_skipped_unreleased(&file_name, &release_state);
                    continue;
                }

                metadata.insert("doc_type", &doc_type);
                let title = metadata::sanitize(&record.title);
                metadata.insert("title", &title);
                let keywords = metadata::tokenize(&record.keywords);
                metadata.insert("keywords", &keywords);
                let suggestions = metadata::to_json(metadata::to_array(&record.keywords));
                metadata.insert("suggestions", &suggestions);
                let created = record.created.to_rfc3339();
                metadata.insert("created", &created);
                let author = metadata::sanitize(&record.author);
                metadata.insert("author", &author);

                let file_data = fs::read(path)?;
                let hash = hash(&file_data);

                if report.already_uploaded_file_with_hash(&hash) {
                    println!("Skipping {} because it is a duplicate.", file_name);
                    report.add_skipped_duplicate(&file_name, &hash);
                    continue;
                }

                storage::upload(&hash, &client, &mut core, &file_data, &metadata)?;
                report.add_uploaded(&file_name, &hash);
            } else {
                println!("Skipping {} because it does not have metadata.", path.to_str().unwrap());
                report.add_skipped_incomplete(path.to_str().unwrap());
            }
        }
        report.print_report();
    }
    Ok(())
}
