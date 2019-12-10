#[macro_use]
extern crate clap;

use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use clap::App;
use import::{par, spc_pil};
use std::path::Path;
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

    match matches.subcommand() {
        ("spcpil", Some(m)) => {
            let path = m
                .value_of("directory")
                .expect("yaml is incorrect: directory should be a required arg");
            let (client, core) = initialize()?;
            let dir = Path::new(&path);
            spc_pil::import(dir, client, core, verbosity)?
        }
        ("par", Some(m)) => {
            let path = m
                .value_of("directory")
                .expect("yaml is incorrect: directory should be a required arg");
            let (client, core) = initialize()?;
            let dir = Path::new(&path);
            par::import(dir, client, core, verbosity)?
        }
        _ => println!("yaml is incorrect: pdf is currently the only subcommand"),
    }
    Ok(())
}

fn initialize() -> Result<(Client, Core), AzureError> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let core = Core::new()?;

    Ok((Client::new(&account, &master_key)?, core))
}
