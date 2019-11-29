use actix_web::client;
use crate::azure_rest;

pub fn create_indexer() -> Result<(), client::SendRequestError> {
    let search_service = std::env::var("SEARCH_SERVICE")
        .expect("Set env variable SEARCH_SERVICE first!");
    let indexer_definition = get_indexer_definition();
    let url = "https://SEARCH_SERVICE_PLACEHOLDER.search.windows.net/indexers?api-version=2019-05-06"
        .replace("SEARCH_SERVICE_PLACEHOLDER", &search_service);
    let api_key = std::env::var("API_ADMIN_KEY")
        .expect("Set env variable API_ADMIN_KEY first!");

    azure_rest::send_json_post_request(indexer_definition, &url, &api_key)
}

fn get_indexer_definition() -> String {
    include_str!("../definitions/indexers/azureblob-indexer.json").to_string()
}
