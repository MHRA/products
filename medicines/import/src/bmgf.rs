use crate::model::ImportError;
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_blob::Blob;
use azure_sdk_storage_core::prelude::*;
use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use indicatif::{HumanDuration, ProgressBar};
use std::{collections::HashMap, fs, path::Path, str, time::Instant};
use tokio_core::reactor::Core;

pub fn import(
    path: &Path,
    client: Box<dyn Client>,
    verbosity: i8,
    dryrun: bool,
) -> Result<(), ImportError> {
    // TODO: Add facets based on active substance - I, Ibuprofen
    // Map to model when reading from

    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    let mut range: Range<DataType>;

    match workbook.worksheet_range("Sheet 1") {
        Some(Ok(retrieved_range)) => range = retrieved_range,
        Some(Err(e)) => return Err(ImportError::WorkbookOpenError(format!("{:?}", e))),
        None => {
            return Err(ImportError::WorkbookOpenError(String::from(
                "Couldn't open workbook",
            )))
        }
    }

    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            continue;
        }
        let report_name = row.get(0).unwrap().to_string();
        let active_substances = row.get(1).unwrap().to_string().to_uppercase();
        let pl_number = row.get(2).unwrap().to_string();
        let summary = row.get(3).unwrap().to_string();
        let mut facets = active_substances
            .split(',')
            .map(|active_substance| {
                let active_substance = active_substance.trim();
                let substance_first_letter = active_substance.chars().next().unwrap();
                let substance_facet = format!("{}, {}", substance_first_letter, active_substance);
                return vec![substance_first_letter.to_string(), substance_facet];
            })
            .flatten()
            .collect::<Vec<String>>();
        facets.sort();
        facets.dedup();
    }

    // for x in 0..range.get_size().0 {
    //     match range.get_value((0, x.into())) {
    //         Some(value) => print!("{}", value),
    //         None => print!("No value!"),
    //     }
    // }

    // let total_cells = range.get_size().0 * range.get_size().1;
    // let non_empty_cells: usize = range.used_cells().count();
    // println!(
    //     "Found {} cells in 'Sheet 1', including {} non empty cells",
    //     total_cells, non_empty_cells
    // );
    // // alternatively, we can manually filter rows
    // assert_eq!(
    //     non_empty_cells,
    //     range
    //         .rows()
    //         .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty))
    //         .count()
    // );

    // Open file and read in metadata from excel file
    // Upload PDF file, attaching metadata
    // Upload all other files unless this is just to upload the PDFs

    // if let Ok(records) = csv::load_csv(dir) {
    //     if dryrun {
    //         println!("This is a dry run, nothing will be uploaded!");
    //     }
    //     let started = Instant::now();
    //     let mut report = Report::new(verbosity);
    //     let pdfs = pdf::get_pdfs(dir).expect("Could not load any PDFs.");
    //     let progress_bar = ProgressBar::new(pdfs.len() as u64);
    //     for path in pdfs {
    //         let key = path
    //             .file_stem()
    //             .expect("file has no stem")
    //             .to_str()
    //             .unwrap();
    //         if let Some(record) = records.get(&key.to_lowercase()) {
    //             let mut metadata: HashMap<&str, &str> = HashMap::new();

    //             let file_name = metadata::sanitize(&record.filename);
    //             metadata.insert("file_name", &file_name);

    //             let release_state = metadata::sanitize(&record.release_state);
    //             metadata.insert("release_state", &release_state);

    //             if release_state != "Y" {
    //                 report.add_skipped_unreleased(&file_name, &release_state);
    //                 continue;
    //             }

    //             let doc_type = format!(
    //                 "{:?}",
    //                 match record.second_level.as_ref() {
    //                     "PIL" => model::DocType::Pil,
    //                     "SPC" => model::DocType::Spc,
    //                     _ => panic!("unexpected doc type"),
    //                 }
    //             );
    //             metadata.insert("doc_type", &doc_type);

    //             let title = metadata::sanitize(&record.title);
    //             metadata.insert("title", &title);

    //             let pl_numbers = metadata::extract_product_licences(&title);
    //             metadata.insert("pl_number", &pl_numbers);

    //             let rev_label = metadata::sanitize(&record.rev_label);
    //             metadata.insert("rev_label", &rev_label);

    //             let created = record.created.to_rfc3339();
    //             metadata.insert("created", &created);

    //             let product_name = metadata::sanitize(&record.product_name);
    //             metadata.insert("product_name", &product_name);

    //             let active_substances = metadata::to_array(&record.substance_name);
    //             let substance_name = metadata::to_json(active_substances.clone());
    //             metadata.insert("substance_name", &substance_name);

    //             let facets = metadata::to_json(metadata::create_facets_by_active_substance(
    //                 &product_name,
    //                 active_substances,
    //             ));
    //             metadata.insert("facets", &facets);

    //             let file_data = fs::read(path)?;
    //             let hash = hash(&file_data);

    //             if (report).already_uploaded_file_with_hash(&hash) {
    //                 report.add_skipped_duplicate(&file_name, &hash);
    //                 continue;
    //             }

    //             if !dryrun {
    //                 storage::upload(&hash, &client, &mut core, &file_data, &metadata, verbosity)?;
    //             }
    //             report.add_uploaded(&file_name, &hash, &pl_numbers);
    //         } else {
    //             report.add_skipped_incomplete(key);
    //         }
    //         if verbosity == 0 {
    //             progress_bar.inc(1);
    //         }
    //     }
    //     progress_bar.finish();
    //     println!(
    //         "Importing SPCs & PILs finished in {}",
    //         HumanDuration(started.elapsed())
    //     );
    //     report.print_report();
    // }
    Ok(())
}
