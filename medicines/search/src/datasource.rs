use std::fs;

pub fn create_datasource() {
    let storage_account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let storage_master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let datasource_definition = get_datasource_definition(&storage_account, &storage_master_key);
}

fn get_datasource_definition(storage_account: &String, storage_master_key: &String) -> String {
    let datasource_definition_path = "../definitions/datasources/docs.json";

    fs::read_to_string(datasource_definition_path)
        .expect("Could not read datasource definition.")
        .replace("ACCOUNT_NAME_PLACEHOLDER", &storage_account)
        .replace("ACCOUNT_KEY_PLACEHOLDER", &storage_master_key)
}