#[macro_use]
extern crate clap;

use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use clap::App;
use std::fs;
use tokio_core::reactor::Core;

fn main() -> Result<(), AzureError> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    Ok(match matches.subcommand_matches("pdf") {
        Some(pdf_matches) => {
            let path = pdf_matches
                .value_of("file")
                .expect("yaml is incorrect: file should be a required arg");
            println!("importing {}", path);
            let (client, mut core) = initialize()?;
            import::upload(client, &mut core, &fs::read(path)?)?
        }
        None => println!("yaml is incorrect: pdf is currently the only subcommand"),
    })
}

fn initialize() -> Result<(Client, Core), AzureError> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let core = Core::new()?;

    Ok((Client::new(&account, &master_key)?, core))
}
