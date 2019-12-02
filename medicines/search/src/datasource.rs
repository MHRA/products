use actix_web::client;
use crate::azure_rest;

pub fn create_datasource() -> Result<(), client::SendRequestError> {
    let search_service = std::env::var("SEARCH_SERVICE")
        .expect("Set env variable SEARCH_SERVICE first!");
    let datasource_definition = get_datasource_definition();
    let url = "https://SEARCH_SERVICE_PLACEHOLDER.search.windows.net/datasources?api-version=2019-05-06"
        .replace("SEARCH_SERVICE_PLACEHOLDER", &search_service);
    let api_key = std::env::var("API_ADMIN_KEY")
        .expect("Set env variable API_ADMIN_KEY first!");

    azure_rest::send_json_post_request(datasource_definition, &url, &api_key)
}

fn get_datasource_definition() -> String {
    let storage_account = std::env::var("STORAGE_ACCOUNT")
        .expect("Set env variable STORAGE_ACCOUNT first!");
    let storage_master_key = std::env::var("STORAGE_MASTER_KEY")
        .expect("Set env variable STORAGE_MASTER_KEY first!");

    include_str!("../definitions/datasources/docs.json")
        .to_string()
        .replace("ACCOUNT_NAME_PLACEHOLDER", &storage_account)
        .replace("ACCOUNT_KEY_PLACEHOLDER", &storage_master_key)
}
