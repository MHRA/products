#[macro_use]
extern crate clap;

use clap::App;
use search::{datasource, index, indexer};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("define_datasource", Some(_m)) =>
            datasource::create_datasource().expect("Failed to create datasource"),
        ("define_index", Some(_m)) =>
            index::create_index().expect("Failed to create index"),
        ("define_indexer", Some(_m)) =>
            indexer::create_indexer().expect("Failed to create indexer"),
        _ => panic!("Subcommand not recognized."),
    }
}
