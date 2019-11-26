use std::fs;

pub fn create_index() {
    let index_definition = get_index_definition();
}

fn get_index_definition() -> String {
    let index_definition_path = "../definitions/indexes/azureblob-index.json";

    fs::read_to_string(index_definition_path)
        .expect("Could not read index definition.")
}