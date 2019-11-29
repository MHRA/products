use crate::azure_rest;
use actix_web::client;

pub fn create_index() -> Result<(), client::SendRequestError> {
    let search_service = std::env::var("SEARCH_SERVICE")
        .expect("Set env variable SEARCH_SERVICE first!");
    let index_definition = get_index_definition();
    let url = "https://SEARCH_SERVICE_PLACEHOLDER.search.windows.net/indexes?api-version=2019-05-06"
        .replace("SEARCH_SERVICE_PLACEHOLDER", &search_service);
    let api_key = std::env::var("API_ADMIN_KEY")
        .expect("Set env variable API_ADMIN_KEY first!");

    azure_rest::send_json_post_request(index_definition, &url, &api_key)
}

fn get_index_definition() -> String {
    include_str!("../definitions/indexes/azureblob-index.json").to_string()
}
