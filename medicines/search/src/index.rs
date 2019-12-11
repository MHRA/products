use crate::{azure_rest, env::get_from_env};
use actix_web::client;
use crate::env::{SEARCH_SERVICE, INDEX_NAME, API_ADMIN_KEY};

pub fn create_index() -> Result<(), client::SendRequestError> {
    let search_service = get_from_env(SEARCH_SERVICE);
    let index_name = get_from_env(INDEX_NAME);
    let api_key = get_from_env(API_ADMIN_KEY);
    let index_definition = get_index_definition(get_raw_index_definition(), &index_name);
    let url = get_base_url(&search_service);

    azure_rest::make_post_request_with_body(index_definition, &url, &api_key)
}

pub fn delete_index() -> Result<(), client::SendRequestError> {
    let api_key = get_from_env(API_ADMIN_KEY);
    let index_name = get_from_env(INDEX_NAME);
    let search_service = get_from_env(SEARCH_SERVICE);
    let url = get_resource_url(&search_service, &index_name);

    azure_rest::make_delete_request(&url, &api_key)
}

fn get_base_url(search_service: &str) -> String {
    "https://SEARCH_SERVICE_PLACEHOLDER.search.windows.net/indexes?api-version=2019-05-06"
        .replace("SEARCH_SERVICE_PLACEHOLDER", search_service)
}

fn get_resource_url(search_service: &str, index_name: &str) -> String {
    "https://SEARCH_SERVICE_PLACEHOLDER.search.windows.net/indexes/INDEX_NAME_PLACEHOLDER?api-version=2019-05-06"
        .replace("SEARCH_SERVICE_PLACEHOLDER", search_service)
        .replace("INDEX_NAME_PLACEHOLDER", index_name)
}

fn get_raw_index_definition() -> String {
    include_str!("../definitions/indexes/default.json").to_string()
}

fn get_index_definition(raw_index_definition: String, index_name: &str) -> String {
    raw_index_definition
        .replace("INDEX_NAME_PLACEHOLDER", index_name)
}

#[test]
fn test_get_base_url() {
    assert_eq!(
        get_base_url("service_name"),
        "https://service_name.search.windows.net/indexes?api-version=2019-05-06".to_string()
    );
}

#[test]
fn test_get_resource_url() {
    assert_eq!(
        get_resource_url("service_name", "index_name"),
        "https://service_name.search.windows.net/indexes/index_name?api-version=2019-05-06".to_string()
    );
}

#[test]
fn test_get_index_definition() {
    let index_raw_definition = r###"{
      "name": "INDEX_NAME_PLACEHOLDER",
      "fields": [
        {
          "name": "content",
          "type": "Edm.String",
          "facetable": false,
          "filterable": false,
          "key": false,
          "retrievable": false,
          "searchable": true,
          "sortable": false,
          "analyzer": "standard.lucene",
          "indexAnalyzer": null,
          "searchAnalyzer": null,
          "synonymMaps": [],
          "fields": []
        },
        ...
      ],
      "suggesters": [
        {
          "name": "azure-suggester",
          "searchMode": "analyzingInfixMatching",
          "sourceFields": ["suggestions", "title", "substance_name", "product_name"]
        }
      ],
      "scoringProfiles": [
        {
          "name": "preferKeywords",
          "text": {
            "weights": {
              "keywords": 3,
              "product_name": 3
            }
          }
        }
      ],
      "defaultScoringProfile": "",
      "corsOptions": {
        "allowedOrigins": ["*"],
        "maxAgeInSeconds": 300
      },
      "analyzers": [],
      "charFilters": [],
      "tokenFilters": [],
      "tokenizers": [],
      "@odata.etag": "\"0x8D77267697666D4\""
    }"###;

    let index_replaced = get_index_definition(index_raw_definition.to_string(), "index_name");

    let index_expected_replaced = r###"{
      "name": "index_name",
      "fields": [
        {
          "name": "content",
          "type": "Edm.String",
          "facetable": false,
          "filterable": false,
          "key": false,
          "retrievable": false,
          "searchable": true,
          "sortable": false,
          "analyzer": "standard.lucene",
          "indexAnalyzer": null,
          "searchAnalyzer": null,
          "synonymMaps": [],
          "fields": []
        },
        ...
      ],
      "suggesters": [
        {
          "name": "azure-suggester",
          "searchMode": "analyzingInfixMatching",
          "sourceFields": ["suggestions", "title", "substance_name", "product_name"]
        }
      ],
      "scoringProfiles": [
        {
          "name": "preferKeywords",
          "text": {
            "weights": {
              "keywords": 3,
              "product_name": 3
            }
          }
        }
      ],
      "defaultScoringProfile": "",
      "corsOptions": {
        "allowedOrigins": ["*"],
        "maxAgeInSeconds": 300
      },
      "analyzers": [],
      "charFilters": [],
      "tokenFilters": [],
      "tokenizers": [],
      "@odata.etag": "\"0x8D77267697666D4\""
    }"###.to_string();

    assert_eq!(index_replaced, index_expected_replaced);
}