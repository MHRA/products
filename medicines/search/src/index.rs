use crate::{
    azure_rest,
    env::{get_from_env, INDEX_NAME, SEARCH_API_ADMIN_KEY, SEARCH_SERVICE},
};
use reqwest::Url;

pub async fn create_or_update_index(index_definition: &str) -> Result<(), reqwest::Error> {
    let search_service = get_from_env(SEARCH_SERVICE);
    let index_name = get_from_env(INDEX_NAME);
    let api_key = get_from_env(SEARCH_API_ADMIN_KEY);
    let raw_index_definition;
    match index_definition {
        "bmgf" => raw_index_definition = get_bmgf_raw_index_definition(),
        _ => raw_index_definition = get_default_raw_index_definition(),
    }
    let index_definition = get_index_definition(raw_index_definition, &index_name);
    let mut url = Url::parse(&get_base_url(&search_service)).unwrap();
    url.set_path(&format!("{}/{}", url.path(), index_name));

    azure_rest::make_put_request_with_body(index_definition, url, &api_key).await
}

pub async fn delete_index() -> Result<(), reqwest::Error> {
    let api_key = get_from_env(SEARCH_API_ADMIN_KEY);
    let index_name = get_from_env(INDEX_NAME);
    let search_service = get_from_env(SEARCH_SERVICE);
    let url = get_resource_url(&search_service, &index_name);

    azure_rest::make_delete_request(&url, &api_key).await
}

fn get_base_url(search_service: &str) -> String {
    "https://SEARCH_SERVICE_PLACEHOLDER.search.windows.net/indexes/?api-version=2019-05-06"
        .replace("SEARCH_SERVICE_PLACEHOLDER", search_service)
}

fn get_resource_url(search_service: &str, index_name: &str) -> String {
    "https://SEARCH_SERVICE_PLACEHOLDER.search.windows.net/indexes/INDEX_NAME_PLACEHOLDER?api-version=2019-05-06"
        .replace("SEARCH_SERVICE_PLACEHOLDER", search_service)
        .replace("INDEX_NAME_PLACEHOLDER", index_name)
}

fn get_default_raw_index_definition() -> String {
    include_str!("../definitions/indexes/default.json").to_string()
}

fn get_bmgf_raw_index_definition() -> String {
    include_str!("../definitions/indexes/bmgf.json").to_string()
}

fn get_index_definition(raw_index_definition: String, index_name: &str) -> String {
    raw_index_definition.replace("INDEX_NAME_PLACEHOLDER", index_name)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_base_url() {
        assert_eq!(
            get_base_url("service_name"),
            "https://service_name.search.windows.net/indexes/?api-version=2019-05-06".to_string()
        );
    }

    #[test]
    fn test_get_resource_url() {
        assert_eq!(
            get_resource_url("service_name", "index_name"),
            "https://service_name.search.windows.net/indexes/index_name?api-version=2019-05-06"
                .to_string()
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
    }"###
            .to_string();

        assert_eq!(index_replaced, index_expected_replaced);
    }
}
