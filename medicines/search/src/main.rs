#[macro_use]
extern crate clap;

use clap::App;
use search::{datasource, index, indexer};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {

    }
}
