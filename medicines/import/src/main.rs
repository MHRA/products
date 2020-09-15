#[macro_use]
extern crate clap;

use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_blob::Blob;
use azure_sdk_storage_core::prelude::*;
use clap::App;
use import::bmgf;
use std::path::Path;
use tokio_core::reactor::Core;

fn main() -> Result<(), AzureError> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let verbosity: i8;
    match matches.occurrences_of("verbose") {
        0 => verbosity = 0,
        1 => verbosity = 1,
        _ => verbosity = 2,
    };
    let dryrun = matches.is_present("dryrun");
    match matches.subcommand() {
        ("bmgf", Some(m)) => {
            let path = m
                .value_of("directory")
                .expect("yaml is incorrect: directory should be a required arg");
            let client = initialize()?;
            let dir = Path::new(&path);
            bmgf::import(dir, client, verbosity, dryrun)?
        }
        _ => println!("command did not match available commands."),
    }
    Ok(())
}

fn initialize() -> Result<Box<dyn Client>, AzureError> {
    let storage_account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let client = client::with_access_key(&storage_account, &master_key);
    Ok(Box::new(client))
}
