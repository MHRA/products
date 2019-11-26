use std::fs;

pub fn create_indexer() {
    let indexer_definition = get_indexer_definition();
}

fn get_indexer_definition() -> String {
    let indexer_definition_path = "../definitions/indexers/azureblob-indexer.json";

    fs::read_to_string(indexer_definition_path)
        .expect("Could not read indexer definition.")
}