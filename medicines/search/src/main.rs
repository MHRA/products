extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::fs;

fn main() {
    let storage_account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let storage_master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let datasource_definition = get_datasource_definition(&storage_account, &storage_master_key);
    let index_definition = get_index_definition();
    let indexer_definition = get_indexer_definition();

    // To-Do: Make the HTTP calls.
}

fn get_datasource_definition(storage_account: &String, storage_master_key: &String) -> String {
    let datasource_definition_path = "../definitions/datasources/docs.json";

    fs::read_to_string(datasource_definition_path)
        .expect("Could not read datasource definition.")
        .replace("ACCOUNT_NAME_PLACEHOLDER", &storage_account)
        .replace("ACCOUNT_KEY_PLACEHOLDER", &storage_master_key)
}

fn get_index_definition() -> String {
    let index_definition_path = "../definitions/indexes/azureblob-index.json";

    fs::read_to_string(index_definition_path)
        .expect("Could not read index definition.")
}

fn get_indexer_definition() -> String {
    let indexer_definition_path = "../definitions/indexers/azureblob-indexer.json";

    fs::read_to_string(indexer_definition_path)
        .expect("Could not read indexer definition.")
}
