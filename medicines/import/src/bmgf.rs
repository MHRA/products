use crate::{metadata, model::ImportError, storage};
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_blob::Blob;
use azure_sdk_storage_core::prelude::*;
use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use indicatif::{HumanDuration, ProgressBar};
use std::{collections::HashMap, fs, path::Path, str, time::Instant};
use tokio_core::reactor::Core;

pub fn get_worksheet_range(path: &Path, sheet_name: &str) -> Result<Range<DataType>, ImportError> {
    let mut workbook: Xlsx<_> =
        open_workbook(path).expect(&format!("Cannot open file: {:?}", path));

    match workbook.worksheet_range(sheet_name) {
        Some(Ok(retrieved_range)) => Ok(retrieved_range),
        Some(Err(e)) => Err(ImportError::WorkbookOpenError(format!("{:?}", e))),
        None => Err(ImportError::WorkbookOpenError(String::from(
            "Couldn't open workbook",
        ))),
    }
}

pub fn extract_file_data(row: &[DataType]) -> HashMap<String, String> {
    let mut metadata: HashMap<String, String> = HashMap::new();

    let report_name = row
        .get(0)
        .expect("Report name should be in first column")
        .to_string();
    metadata.insert("report_name".to_string(), report_name);

    let file_name = row
        .get(1)
        .expect("File name should be in second column")
        .to_string();
    metadata.insert("file_name".to_string(), file_name);
    // let pdf_file_path = path.parent();

    let summary = row
        .get(2)
        .expect("Summary should be in third column")
        .to_string();
    metadata.insert("summary".to_string(), summary);

    let active_substances = row
        .get(3)
        .expect("Active substances should be in fourth column")
        .to_string()
        .to_uppercase();
    let active_substances = metadata::to_array(&active_substances);
    metadata.insert(
        "active_substances".to_string(),
        metadata::to_json(active_substances.clone()),
    );

    let facets = metadata::create_facets_by_active_substance(active_substances);
    metadata.insert("facets".to_string(), metadata::to_json(facets));

    let products = row
        .get(4)
        .expect("Products should be in fifth column")
        .to_string()
        .to_uppercase();
    let products = metadata::to_array(&products);
    metadata.insert("products".to_string(), metadata::to_json(products));

    let pl_numbers = row
        .get(5)
        .expect("PL numbers should be in sixth column")
        .to_string();
    let pl_numbers = metadata::extract_product_licences(&pl_numbers);
    metadata.insert("pl_numbers".to_string(), pl_numbers);

    let pbpk_models = row
        .get(6)
        .expect("PBPK models should be in seventh column")
        .to_string();
    let pbpk_models = metadata::to_array(&pbpk_models);
    metadata.insert("pbpk_models".to_string(), metadata::to_json(pbpk_models));

    let matrices = row
        .get(7)
        .expect("Matrices should be in eight column")
        .to_string();
    let matrices = metadata::to_array(&matrices);
    metadata.insert("matrices".to_string(), metadata::to_json(matrices));

    metadata
}

pub fn import(
    path: &Path,
    client: Box<dyn Client>,
    verbosity: i8,
    dryrun: bool,
) -> Result<(), ImportError> {
    // TODO: Add facets based on active substance - I, Ibuprofen
    // Map to model when reading from
    if dryrun {
        println!("This is a dry run, nothing will be uploaded!");
    }
    let started = Instant::now();

    let range = get_worksheet_range(path, "Sheet 1")?;

    let progress_bar = ProgressBar::new((range.rows().count() as u64) - 1);
    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            continue;
        }
        let metadata = extract_file_data(row);
        let _ = storage::upload(&client, path, &metadata, verbosity);
        progress_bar.inc(1);
    }
    progress_bar.finish();

    println!(
        "Uploading BMGF reports finished in {}",
        HumanDuration(started.elapsed())
    );

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_data_from_row() {
        // let range: [DataType] = [];
        let report_name = DataType::String(String::from("Example report"));
        let file_name = DataType::String(String::from("Example file name"));
        let summary = DataType::String(String::from("An example summary"));
        let active_substances = DataType::String(String::from("Substance 1, Substance 2"));
        let products = DataType::String(String::from("Product 1, Product 2"));
        let pl_numbers = DataType::String(String::from("PL 12345/1234, PL 23456/2345"));
        let pbpk_models = DataType::String(String::from("Model 1, Model 2"));
        let matrices = DataType::String(String::from("Matrix 1, Matrix 2"));

        let row = [
            report_name,
            file_name,
            summary,
            active_substances,
            products,
            pl_numbers,
            pbpk_models,
            matrices,
        ];
        let data = extract_file_data(&row);
        assert_eq!(data.get("report_name").unwrap(), "Example report");
        assert_eq!(data.get("file_name").unwrap(), "Example file name");
        assert_eq!(data.get("summary").unwrap(), "An example summary");
        assert_eq!(
            data.get("active_substances").unwrap(),
            "[\"SUBSTANCE 1\",\"SUBSTANCE 2\"]"
        );
        assert_eq!(
            data.get("products").unwrap(),
            "[\"PRODUCT 1\",\"PRODUCT 2\"]"
        );
        assert_eq!(
            data.get("pl_numbers").unwrap(),
            "[\"PL123451234\",\"PL234562345\"]"
        );
        assert_eq!(
            data.get("pbpk_models").unwrap(),
            "[\"Model 1\",\"Model 2\"]"
        );
        assert_eq!(data.get("matrices").unwrap(), "[\"Matrix 1\",\"Matrix 2\"]");
        assert_eq!(
            data.get("facets").unwrap(),
            "[\"S\",\"S, SUBSTANCE 1\",\"S, SUBSTANCE 2\"]"
        );
    }
}
