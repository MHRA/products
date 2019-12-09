use crate::{csv, metadata, model, pdf, storage};
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use std::{collections::HashMap, fs, path::Path, str};
use tokio_core::reactor::Core;
use crate::{hash::hash, report::Report};

pub fn import<'a>(dir: &Path, client: Client, mut core: Core) -> Result<(), AzureError> {
    if let Ok(records) = csv::load_csv(dir) {
        let mut report = Report::new();
        for path in pdf::get_pdfs(dir)? {
            println!("Processing {}...", path.to_str().unwrap());
            let key = path
                .file_stem()
                .expect("file has no stem")
                .to_str()
                .unwrap();
            if let Some(record) = records.get(&key.to_lowercase()) {
                let mut metadata: HashMap<&str, &str> = HashMap::new();

                let file_name = metadata::sanitize(&record.filename);
                metadata.insert("file_name", &file_name);

                let release_state = metadata::sanitize(&record.release_state);
                metadata.insert("release_state", &release_state);

                if release_state != "Y" {
                    println!("Skipping {} because it is not released.", file_name);
                    report.add_skipped_unreleased(&file_name, &release_state);
                    continue;
                }

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

                let product_name = metadata::sanitize(&record.product_name);
                metadata.insert("product_name", &product_name);

                let active_substances = metadata::to_array(&record.substance_name);
                let substance_name = metadata::to_json(active_substances.clone());
                metadata.insert("substance_name", &substance_name);

                let facets = metadata::to_json(metadata::create_facets_by_active_substance(
                    &product_name,
                    active_substances,
                ));
                metadata.insert("facets", &facets);

                let file_data = fs::read(path)?;
                let hash = hash(&file_data);

                if (report).already_uploaded_file_with_hash(&hash) {
                    println!("Skipping {} because it is a duplicate.", file_name);
                    report.add_skipped_duplicate(&file_name, &hash);
                    continue;
                }

                storage::upload(&hash, &client, &mut core, &file_data, &metadata)?;
                report.add_uploaded(&file_name, &hash);
            } else {
                println!("Skipping {} because it does not have metadata.", path.to_str().unwrap());
                report.add_skipped_incomplete(key);
            }
        }
        report.print_report();
    }
    Ok(())
}
