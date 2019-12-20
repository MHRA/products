#[macro_use]
extern crate clap;

use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use clap::App;
use import::{par, spc_pil};
use std::{fs::File, path::Path};
use tokio_core::reactor::Core;

fn main() -> Result<(), AzureError> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let verbosity: i8;
    match matches.occurrences_of("verbose") {
        0 => verbosity = 0,
        1 => verbosity = 1,
        2 | _ => verbosity = 2,
    };
    let dryrun = matches.is_present("dryrun");
    match matches.subcommand() {
        ("spcpil", Some(m)) => {
            let path = m
                .value_of("directory")
                .expect("yaml is incorrect: directory should be a required arg");
            let (client, core) = initialize()?;
            let dir = Path::new(&path);
            let csv = open_file(m.value_of("csv"), dir);
            let old = open_file_optional(m.value_of("old"));
            spc_pil::import(dir, client, core, verbosity, dryrun, csv, old)?
        }
        ("par", Some(m)) => {
            let path = m
                .value_of("directory")
                .expect("yaml is incorrect: directory should be a required arg");
            let (client, core) = initialize()?;
            let dir = Path::new(&path);
            let csv = open_file(m.value_of("csv"), dir);
            let old = open_file_optional(m.value_of("old"));
            par::import(dir, client, core, verbosity, dryrun, csv, old)?
        }
        _ => println!("yaml is incorrect: pdf is currently the only subcommand"),
    }
    Ok(())
}

fn open_file(path: Option<&str>, dir: &Path) -> File {
    match path {
        Some(csv) => File::open(csv).expect("CSV could not be opened."),
        None => find_csv(dir).expect("CSV could not be opened.")
    }
}

fn open_file_optional(path: Option<&str>) -> Option<File> {
    match path {
        Some(csv) => Some(File::open(csv).expect("CSV could not be opened.")),
        None => None
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
