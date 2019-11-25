use crate::{model, storage};
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use std::{collections::HashMap, fs, path::Path};
use tokio_core::reactor::Core;

pub fn import(dir: &Path, client: Client, mut core: Core) -> Result<(), AzureError> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "pdf" && fs::metadata(&path)?.len() > 0 {
                    if let Some(doc_type) = get_doc_type(
                        Path::file_stem(&path)
                            .expect("no file stem")
                            .to_str()
                            .expect("cannot convert OSStr to str"),
                    ) {
                        println!("{:?} {:?}", path, doc_type);
                        let mut metadata: HashMap<&str, &str> = HashMap::new();
                        let doc_type = format!("{:?}", &doc_type);
                        metadata.insert("doc_type", &doc_type);
                        storage::upload(&client, &mut core, &fs::read(path)?, &metadata)?
                    }
                }
            }
        }
    }
    Ok(())
}

fn get_doc_type(file_stem: &str) -> Option<model::DocType> {
    if file_stem.starts_with("label-and-leaflet") {
        Some(model::DocType::PilLabelAndLeaflet)
    } else if file_stem.starts_with("label") {
        Some(model::DocType::PilLabel)
    } else if file_stem.starts_with("leaflet") {
        Some(model::DocType::PilLeaflet)
    } else if file_stem.starts_with("spc") {
        Some(model::DocType::Spc)
    } else {
        None
    }
}
