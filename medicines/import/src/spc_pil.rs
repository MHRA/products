use crate::{csv, hash::hash, hashfile::read_hashfile, metadata, model, model::Record, pdf, report::Report, storage};
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use indicatif::{HumanDuration, ProgressBar};
use std::{collections::HashMap, fs, fs::File, path::Path, str, time::Instant};
use tokio_core::reactor::Core;

enum Action {
    Upload,
    Replace,
    Skip,
}

pub fn import(
    dir: &Path,
    client: Client,
    mut core: Core,
    verbosity: i8,
    dryrun: bool,
    csv: File,
    old: Option<File>,
    hashfile: Option<File>,
) -> Result<(), AzureError> {
    if let Ok(records) = csv::load_csv(csv) {
        let old_records: HashMap<String, Record> = match old {
            Some(old_csv) => csv::load_csv(old_csv).expect("Couldn't load old CSV."),
            None => HashMap::new(),
        };
        let mut old_hashes: HashMap<String, Vec<String>> = match hashfile {
            Some(json) => read_hashfile(json).expect("Couldn't parse old hashfile"),
            None => HashMap::new()
        };
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
                let mut action: Action = Action::Upload;
                let metadata = generate_metadata(record);

                if let Some(old_record) = old_records.get(&key.to_lowercase()) {
                    let old_metadata = generate_metadata(old_record);

                    if metadata == old_metadata {
                        report.add_skipped_unchanged(metadata.get("file_name").unwrap());
                        action = Action::Skip;
                    } else {
                        action = Action::Replace;
                    }
                }

                match metadata.get("release_state").map(String::as_str) {
                    Some("Y") => (),
                    None => panic!("Release state should never be empty"),
                    _ => {
                        report.add_skipped_unreleased(
                            metadata.get("file_name").unwrap(),
                            metadata.get("release_state").unwrap(),
                        );
                        action = Action::Skip;
                    }
                }

                let file_data = fs::read(path.clone())?;
                let hash = hash(&file_data);

                if (report).already_uploaded_file_with_hash(&hash) {
                    report.add_skipped_duplicate(metadata.get("file_name").unwrap(), &hash);
                    action = Action::Skip;
                }

                match action {
                    Action::Upload => {
                        if !dryrun {
                            storage::upload(
                                &hash, &client, &mut core, &file_data, &metadata, verbosity,
                            )?;
                        }

                        report.add_uploaded(
                            metadata.get("file_name").unwrap(),
                            &hash,
                            metadata.get("pl_number").unwrap(),
                        );
                    }
                    Action::Replace => {
                        let hashes_to_delete = old_hashes.entry(key.to_string()).or_insert(Vec::new());

                        if !dryrun {
                            hashes_to_delete.iter().for_each(|old_hash| {
                                match storage::delete(old_hash, &client, &mut core, verbosity) {
                                    Ok(_) => (),
                                    Err(_) => {
                                        if verbosity >= 1 {
                                            println!("Encountered an error when deleting {}.", old_hash);
                                        }
                                        ()
                                    },
                                }
                            });

                            storage::upload(
                                &hash, &client, &mut core, &file_data, &metadata, verbosity,
                            )?;
                        }

                        report.add_replaced(metadata.get("file_name").unwrap(), hashes_to_delete.to_vec());
                    }
                    _ => (),
                }
            } else if let Some(old_record) = old_records.get(&key.to_lowercase()) {
                let hashes_to_delete = old_hashes.entry(key.to_string()).or_insert(Vec::new());

                hashes_to_delete.iter().for_each(|old_hash| {
                    if !dryrun {
                        match storage::delete(old_hash, &client, &mut core, verbosity) {
                            Ok(_) => (),
                            Err(_) => {
                                if verbosity >= 1 {
                                    println!("Encountered an error when deleting {}.", old_hash);
                                }
                                ()
                            },
                        }
                    }

                    report.add_deleted(&old_record.filename, hashes_to_delete.to_vec());
                });
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

fn generate_metadata(record: &Record) -> HashMap<&str, String> {
    let mut metadata: HashMap<&str, String> = HashMap::new();

    metadata.insert("file_name", metadata::sanitize(&record.filename));
    metadata.insert("release_state", metadata::sanitize(&record.release_state));
    metadata.insert(
        "doc_type",
        format!(
            "{:?}",
            match record.second_level.as_ref() {
                "PIL" => model::DocType::Pil,
                "SPC" => model::DocType::Spc,
                _ => panic!("unexpected doc type"),
            }
        ),
    );
    metadata.insert("title", metadata::sanitize(&record.title));
    metadata.insert(
        "pl_number",
        metadata::extract_product_licences(&metadata::sanitize(&record.title)),
    );
    metadata.insert("rev_label", metadata::sanitize(&record.rev_label));
    metadata.insert("created", record.created.to_rfc3339());
    metadata.insert("product_name", metadata::sanitize(&record.product_name));
    metadata.insert(
        "substance_name",
        metadata::to_json(metadata::to_array(&record.substance_name)),
    );
    metadata.insert(
        "facets",
        metadata::to_json(metadata::create_facets_by_active_substance(
            &metadata::sanitize(&record.product_name),
            metadata::to_array(&record.substance_name),
        )),
    );
    metadata
}
