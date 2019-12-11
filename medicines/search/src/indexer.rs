use crate::{
    azure_rest,
    env::{get_from_env, API_ADMIN_KEY, DATASOURCE_NAME, INDEXER_NAME, INDEX_NAME, SEARCH_SERVICE},
};
use actix_web::client;

pub fn create_indexer() -> Result<(), client::SendRequestError> {
    let api_key = get_from_env(API_ADMIN_KEY);
    let datasource_name = get_from_env(DATASOURCE_NAME);
    let index_name = get_from_env(INDEX_NAME);
    let indexer_name = get_from_env(INDEXER_NAME);
    let search_service = get_from_env(SEARCH_SERVICE);
    let indexer_definition = get_indexer_definition(
        get_raw_indexer_definition(),
        &datasource_name,
        &index_name,
        &indexer_name,
    );
    let url = get_base_url(&search_service);

    azure_rest::make_post_request_with_body(indexer_definition, &url, &api_key)
}

pub fn delete_indexer() -> Result<(), client::SendRequestError> {
    let api_key = get_from_env(API_ADMIN_KEY);
    let indexer_name = get_from_env(INDEXER_NAME);
    let search_service = get_from_env(SEARCH_SERVICE);
    let url = get_resource_url(&search_service, &indexer_name);

    azure_rest::make_delete_request(&url, &api_key)
}

pub fn run_indexer() -> Result<(), client::SendRequestError> {
    let api_key = get_from_env(API_ADMIN_KEY);
    let indexer_name = get_from_env(INDEXER_NAME);
    let search_service = get_from_env(SEARCH_SERVICE);
    let url = get_run_url(&search_service, &indexer_name);

    azure_rest::make_post_request(&url, &api_key)
}

pub fn reset_indexer() -> Result<(), client::SendRequestError> {
    let api_key = get_from_env(API_ADMIN_KEY);
    let indexer_name = get_from_env(INDEXER_NAME);
    let search_service = get_from_env(SEARCH_SERVICE);
    let url = get_reset_url(&search_service, &indexer_name);

    azure_rest::make_post_request(&url, &api_key)
}

fn get_raw_indexer_definition() -> String {
    include_str!("../definitions/indexers/default.json").to_string()
}

fn get_indexer_definition(
    raw_indexer_definition: String,
    datasource_name: &str,
    index_name: &str,
    indexer_name: &str,
) -> String {
    raw_indexer_definition
        .replace("DATASOURCE_NAME_PLACEHOLDER", datasource_name)
        .replace("INDEX_NAME_PLACEHOLDER", index_name)
        .replace("INDEXER_NAME_PLACEHOLDER", indexer_name)
}

fn get_base_url(search_service: &str) -> String {
    "https://SEARCH_SERVICE_PLACEHOLDER.search.windows.net/indexers?api-version=2019-05-06"
        .replace("SEARCH_SERVICE_PLACEHOLDER", &search_service)
}

fn get_resource_url(search_service: &str, indexer_name: &str) -> String {
    "https://SEARCH_SERVICE_PLACEHOLDER.search.windows.net/indexers/INDEXER_NAME_PLACEHOLDER?api-version=2019-05-06"
        .replace("SEARCH_SERVICE_PLACEHOLDER", &search_service)
        .replace("INDEXER_NAME_PLACEHOLDER", &indexer_name)
}

fn get_run_url(search_service: &str, indexer_name: &str) -> String {
    "https://SEARCH_SERVICE_PLACEHOLDER.search.windows.net/indexers/INDEXER_NAME_PLACEHOLDER/run?api-version=2019-05-06"
        .replace("SEARCH_SERVICE_PLACEHOLDER", &search_service)
        .replace("INDEXER_NAME_PLACEHOLDER", &indexer_name)
}

fn get_reset_url(search_service: &str, indexer_name: &str) -> String {
    "https://SEARCH_SERVICE_PLACEHOLDER.search.windows.net/indexers/INDEXER_NAME_PLACEHOLDER/reset?api-version=2019-05-06"
        .replace("SEARCH_SERVICE_PLACEHOLDER", &search_service)
        .replace("INDEXER_NAME_PLACEHOLDER", &indexer_name)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_base_url() {
        assert_eq!(
            get_base_url("service_name"),
            "https://service_name.search.windows.net/indexers?api-version=2019-05-06".to_string()
        );
    }

    #[test]
    fn test_get_resource_url() {
        assert_eq!(
            get_resource_url("service_name", "indexer_name"),
            "https://service_name.search.windows.net/indexers/indexer_name?api-version=2019-05-06"
                .to_string()
        );
    }

    #[test]
    fn test_get_run_url() {
        assert_eq!(
        get_run_url("service_name", "indexer_name"),
        "https://service_name.search.windows.net/indexers/indexer_name/run?api-version=2019-05-06".to_string()
    );
    }

    #[test]
    fn test_get_reset_url() {
        assert_eq!(
        get_reset_url("service_name", "indexer_name"),
        "https://service_name.search.windows.net/indexers/indexer_name/reset?api-version=2019-05-06".to_string()
    );
    }

    #[test]
    fn test_get_index_definition() {
        let indexer_raw_definition = r###"{
      "name": "INDEXER_NAME_PLACEHOLDER",
      "dataSourceName": "DATASOURCE_NAME_PLACEHOLDER",
      "targetIndexName": "INDEX_NAME_PLACEHOLDER",
      "parameters": {
        "configuration": { "indexStorageMetadataOnlyForOversizedDocuments": true }
      },
      "fieldMappings": [
        {
          "sourceFieldName": "suggestions",
          "mappingFunction": { "name": "jsonArrayToStringCollection" }
        },
        {
          "sourceFieldName": "substance_name",
          "mappingFunction": { "name": "jsonArrayToStringCollection" }
        },
        {
          "sourceFieldName": "facets",
          "mappingFunction": { "name": "jsonArrayToStringCollection" }
        }
      ]
    }"###;

        let indexer_replaced = get_indexer_definition(
            indexer_raw_definition.to_string(),
            "datasource_name",
            "index_name",
            "indexer_name",
        );

        let indexer_expected = r###"{
      "name": "indexer_name",
      "dataSourceName": "datasource_name",
      "targetIndexName": "index_name",
      "parameters": {
        "configuration": { "indexStorageMetadataOnlyForOversizedDocuments": true }
      },
      "fieldMappings": [
        {
          "sourceFieldName": "suggestions",
          "mappingFunction": { "name": "jsonArrayToStringCollection" }
        },
        {
          "sourceFieldName": "substance_name",
          "mappingFunction": { "name": "jsonArrayToStringCollection" }
        },
        {
          "sourceFieldName": "facets",
          "mappingFunction": { "name": "jsonArrayToStringCollection" }
        }
      ]
    }"###
            .to_string();

        assert_eq!(indexer_replaced, indexer_expected);
    }
}
