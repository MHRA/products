use crate::{csv, hash::hash, metadata, model, pdf, report::Report, storage};
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use indicatif::{HumanDuration, ProgressBar};
use std::{collections::HashMap, fs, path::Path, str, time::Instant};
use tokio_core::reactor::Core;

pub fn import(
    dir: &Path,
    client: Client,
    mut core: Core,
    verbosity: i8,
    dryrun: bool,
) -> Result<(), AzureError> {
    if let Ok(records) = csv::load_csv(dir) {
        if dryrun {
            println!("This is a dry run, nothing will be uploaded!");
        }
        let started = Instant::now();
        let mut report = Report::new(verbosity);
        let pdfs = pdf::get_pdfs(dir).expect("Could not load any PDFs.");
        let progress_bar = ProgressBar::new(pdfs.len() as u64);
        for path in pdfs {
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

                let pl_number = metadata::extract_product_license(&title);
                metadata.insert("pl_number", &pl_number);

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
                    report.add_skipped_duplicate(&file_name, &hash);
                    continue;
                }

                if !dryrun {
                    storage::upload(&hash, &client, &mut core, &file_data, &metadata, verbosity)?;
                }
                report.add_uploaded(&file_name, &hash);
            } else {
                report.add_skipped_incomplete(key);
            }
            if verbosity == 0 {
                progress_bar.inc(1);
            }
        }
        progress_bar.finish();
        println!(
            "Importing SPCs & PILs finished in {}",
            HumanDuration(started.elapsed())
        );
        report.print_report();
    }
    Ok(())
}
