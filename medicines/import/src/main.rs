#[macro_use]
extern crate clap;

use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use clap::App;
use import::DocType;
use std::{fs, fs::DirEntry, io, path::Path};
use tokio_core::reactor::Core;

fn main() -> Result<(), AzureError> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    match matches.subcommand_matches("pdf") {
        Some(pdf_matches) => {
            let path = pdf_matches
                .value_of("directory")
                .expect("yaml is incorrect: directory should be a required arg");
            let (client, mut core) = initialize()?;
            let dir = Path::new(&path);
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
                                import::upload(&client, &mut core, &fs::read(path)?, doc_type)?
                            }
                        }
                    }
                }
            }
        }
        None => println!("yaml is incorrect: pdf is currently the only subcommand"),
    };
    Ok(())
}

fn get_doc_type(file_stem: &str) -> Option<DocType> {
    if file_stem.starts_with("label-and-leaflet") {
        Some(DocType::PilLabelAndLeaflet)
    } else if file_stem.starts_with("label") {
        Some(DocType::PilLabel)
    } else if file_stem.starts_with("leaflet") {
        Some(DocType::PilLeaflet)
    } else if file_stem.starts_with("spc") {
        Some(DocType::Spc)
    } else {
        None
    }
}

fn initialize() -> Result<(Client, Core), AzureError> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let core = Core::new()?;

    Ok((Client::new(&account, &master_key)?, core))
}
