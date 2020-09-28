use crate::{metadata, model::ImportError, storage};
use azure_sdk_storage_core::prelude::*;
use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use indicatif::{HumanDuration, ProgressBar};
use std::{collections::HashMap, path::Path, time::Instant};

pub async fn import(
    path: &Path,
    client: Box<dyn Client>,
    verbosity: i8,
    dry_run: bool,
) -> Result<(), ImportError> {
    if dry_run {
        println!("This is a dry run, nothing will be uploaded!");
    }
    let started = Instant::now();

    let range = get_worksheet_range(path, "Sheet 1")?;

    let progress_bar = ProgressBar::new((range.rows().count() as u64) - 1);

    let mut uploaded_html_files: Vec<Vec<u8>> = vec![];

    for row in range.rows().skip(1) {
        let metadata = extract_file_data(row);
        match storage::upload_report(&client, path, &metadata, verbosity, dry_run).await {
            Ok(()) => {
                let azure_html_path = format!(
                    "{}/{}\n",
                    metadata.get("report_name").unwrap(),
                    metadata.get("file_name").unwrap()
                );
                uploaded_html_files.push(azure_html_path.as_bytes().to_owned());
            }
            Err(e) => eprint!("Error uploading report: {}", e.to_string()),
        }
        progress_bar.inc(1);
    }

    let uploaded_html_files_index: Vec<u8> = uploaded_html_files.into_iter().flatten().collect();
    let _ = storage::upload_index_file(&uploaded_html_files_index, &client, dry_run).await?;
    progress_bar.finish();

    if dry_run {
        println!("Dry run completed successfully. No files were uploaded.");
    } else {
        println!(
            "Uploading BMGF reports finished in {}",
            HumanDuration(started.elapsed())
        );
    }

    Ok(())
}

pub fn get_worksheet_range(path: &Path, sheet_name: &str) -> Result<Range<DataType>, ImportError> {
    let mut workbook: Xlsx<_> =
        open_workbook(path).expect("Cannot find Excel worksheet - please check -d argument");

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
    let report_name = metadata::sanitize(&report_name);
    metadata.insert("report_name".to_string(), metadata::sanitize(&report_name));
    metadata.insert("id".to_string(), metadata::to_id(&report_name));

    let file_name = row
        .get(1)
        .expect("File name should be in second column")
        .to_string();
    metadata.insert("file_name".to_string(), metadata::sanitize(&file_name));
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
    let active_substances = metadata::to_array(&metadata::sanitize(&active_substances));
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
    let products = metadata::to_array(&metadata::sanitize(&products));
    metadata.insert("products".to_string(), metadata::to_json(products));

    let pl_numbers = row
        .get(5)
        .expect("PL numbers should be in sixth column")
        .to_string();
    let pl_numbers = metadata::extract_product_licences(&metadata::sanitize(&pl_numbers));
    metadata.insert("pl_numbers".to_string(), pl_numbers);

    let pbpk_models = row
        .get(6)
        .expect("PBPK models should be in seventh column")
        .to_string();
    let pbpk_models = metadata::to_array(&metadata::sanitize(&pbpk_models));
    metadata.insert("pbpk_models".to_string(), metadata::to_json(pbpk_models));

    let matrices = row
        .get(7)
        .expect("Pregnancy trimesters should be in eight column")
        .to_string();
    let matrices = metadata::to_array(&metadata::sanitize(&matrices));
    metadata.insert(
        "pregnancy_trimesters".to_string(),
        metadata::to_json(matrices),
    );

    let matrices = row
        .get(8)
        .expect("Matrices should be in ninth column")
        .to_string();
    let matrices = metadata::to_array(&metadata::sanitize(&matrices));
    metadata.insert("matrices".to_string(), metadata::to_json(matrices));

    metadata
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_data_from_row() {
        let report_name = DataType::String(String::from("Example report"));
        let file_name = DataType::String(String::from("Example file name"));
        let summary = DataType::String(String::from("An example summary"));
        let active_substances = DataType::String(String::from("Substance 1, Substance 2"));
        let products = DataType::String(String::from("Product 1, Product 2"));
        let pl_numbers = DataType::String(String::from("PL 12345/1234, PL 23456/2345"));
        let pbpk_models = DataType::String(String::from("Model 1, Model 2"));
        let pregnancy_trimesters = DataType::String(String::from("First, Second"));
        let matrices = DataType::String(String::from("Matrix 1, Matrix 2"));

        let row = [
            report_name,
            file_name,
            summary,
            active_substances,
            products,
            pl_numbers,
            pbpk_models,
            pregnancy_trimesters,
            matrices,
        ];
        let data = extract_file_data(&row);
        assert_eq!(data.get("report_name").unwrap(), "Example report");
        assert_eq!(data.get("id").unwrap(), "Example-report");
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
        assert_eq!(
            data.get("pregnancy_trimesters").unwrap(),
            "[\"First\",\"Second\"]"
        );
        assert_eq!(data.get("matrices").unwrap(), "[\"Matrix 1\",\"Matrix 2\"]");
        assert_eq!(
            data.get("facets").unwrap(),
            "[\"S\",\"S, SUBSTANCE 1\",\"S, SUBSTANCE 2\"]"
        );
    }
}
