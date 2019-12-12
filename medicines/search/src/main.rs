#[macro_use]
extern crate clap;

use clap::App;
use search::{datasource, index, indexer};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("create_datasource", Some(_m)) =>
            datasource::create_datasource().expect("Failed to create datasource"),
        ("delete_datasource", Some(_m)) =>
            datasource::delete_datasource().expect("Failed to delete datasource"),
        ("create_index", Some(_m)) =>
            index::create_index().expect("Failed to create index"),
        ("delete_index", Some(_m)) =>
            index::delete_index().expect("Failed to delete index."),
        ("create_indexer", Some(_m)) =>
            indexer::create_indexer().expect("Failed to create indexer"),
        ("delete_indexer", Some(_m)) =>
            indexer::delete_indexer().expect("Failed to delete indexer."),
        ("reset_indexer", Some(_m)) =>
            indexer::reset_indexer().expect("Failed to reset indexer."),
        ("run_indexer", Some(_m)) =>
            indexer::run_indexer().expect("Failed to run indexer."),
        _ => panic!("Subcommand not recognized."),
    }
}
