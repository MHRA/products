use crate::{csv, metadata, model, pdf, storage};
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use std::{collections::HashMap, fs, path::Path, str};
use tokio_core::reactor::Core;

pub fn import(dir: &Path, client: Client, mut core: Core) -> Result<(), AzureError> {
    if let Ok(records) = csv::load_csv(dir) {
        for path in pdf::get_pdfs(dir)? {
            let key = path
                .file_stem()
                .expect("file has no stem")
                .to_str()
                .unwrap();
            if let Some(record) = records.get(&key.to_lowercase()) {
                let mut metadata: HashMap<&str, &str> = HashMap::new();

                let file_name = metadata::sanitize(&record.filename);
                metadata.insert("file_name", &file_name);

                let doc_type = format!(
                    "{:?}",
                    match record.second_level.as_ref() {
                        "PIL" => model::DocType::Pil,
                        "SPC" => model::DocType::Spc,
                        _ => panic!("unexpected doc type"),
                    }
                );
                metadata.insert("doc_type", &doc_type);

                let title = metadata::sanitize(&record.title);
                metadata.insert("title", &title);

                let rev_label = metadata::sanitize(&record.rev_label);
                metadata.insert("rev_label", &rev_label);

                let created = record.created.to_rfc3339();
                metadata.insert("created", &created);

                let release_state = metadata::sanitize(&record.release_state);
                metadata.insert("release_state", &release_state);

                let product_name = metadata::sanitize(&record.product_name);
                metadata.insert("product_name", &product_name);

                let substance_name = metadata::sanitize(&record.substance_name);
                metadata.insert("substance_name", &substance_name);

                storage::upload(&client, &mut core, &fs::read(path)?, &metadata)?;
            }
        }
    }
    Ok(())
}
