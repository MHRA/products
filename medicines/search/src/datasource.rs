use crate::{
    azure_rest,
    env::{
        get_from_env, API_ADMIN_KEY, DATASOURCE_NAME, SEARCH_SERVICE, STORAGE_ACCOUNT,
        STORAGE_CONTAINER, STORAGE_MASTER_KEY,
    },
};

pub async fn create_datasource() -> Result<(), reqwest::Error> {
    let search_service = get_from_env(SEARCH_SERVICE);
    let api_key = get_from_env(API_ADMIN_KEY);
    let datasource_name = get_from_env(DATASOURCE_NAME);
    let storage_account = get_from_env(STORAGE_ACCOUNT);
    let storage_container = get_from_env(STORAGE_CONTAINER);
    let storage_master_key = get_from_env(STORAGE_MASTER_KEY);
    let url = get_base_url(&search_service);

    let datasource_definition = get_datasource_definition(
        get_raw_datasource_definition(),
        &datasource_name,
        &storage_account,
        &storage_container,
        &storage_master_key,
    );

    azure_rest::make_post_request_with_body(datasource_definition, &url, &api_key).await
}

pub async fn delete_datasource() -> Result<(), reqwest::Error> {
    let api_key = get_from_env(API_ADMIN_KEY);
    let datasource_name = get_from_env(DATASOURCE_NAME);
    let search_service = get_from_env(SEARCH_SERVICE);
    let url = get_resource_url(&search_service, &datasource_name);

    azure_rest::make_delete_request(&url, &api_key).await
}

fn get_base_url(search_service: &str) -> String {
    "https://SEARCH_SERVICE_PLACEHOLDER.search.windows.net/datasources?api-version=2019-05-06"
        .replace("SEARCH_SERVICE_PLACEHOLDER", &search_service)
}

fn get_resource_url(search_service: &str, datasource_name: &str) -> String {
    "https://SEARCH_SERVICE_PLACEHOLDER.search.windows.net/datasources/DATASOURCE_NAME_PLACEHOLDER?api-version=2019-05-06"
        .replace("SEARCH_SERVICE_PLACEHOLDER", search_service)
        .replace("DATASOURCE_NAME_PLACEHOLDER", datasource_name)
}

fn get_raw_datasource_definition() -> String {
    include_str!("../definitions/datasources/default.json").to_string()
}

fn get_datasource_definition(
    datasource_definition: String,
    datasource_name: &str,
    storage_account: &str,
    storage_container: &str,
    storage_master_key: &str,
) -> String {
    datasource_definition
        .replace("DATASOURCE_NAME_PLACEHOLDER", &datasource_name)
        .replace("STORAGE_CONTAINER_PLACEHOLDER", &storage_container)
        .replace("ACCOUNT_NAME_PLACEHOLDER", &storage_account)
        .replace("ACCOUNT_KEY_PLACEHOLDER", &storage_master_key)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_base_url() {
        assert_eq!(
            get_base_url("service_name"),
            "https://service_name.search.windows.net/datasources?api-version=2019-05-06"
                .to_string()
        );
    }

    #[test]
    fn test_get_resource_url() {
        assert_eq!(
        get_resource_url("service_name", "datasource_name"),
        "https://service_name.search.windows.net/datasources/datasource_name?api-version=2019-05-06".to_string()
    );
    }

    #[test]
    fn test_get_datasource_definition() {
        let datasource_raw_json = r###"{
      "name": "DATASOURCE_NAME_PLACEHOLDER",
      "type": "azureblob",
      "credentials": {
        "connectionString": "DefaultEndpointsProtocol=https;AccountName=ACCOUNT_NAME_PLACEHOLDER;AccountKey=ACCOUNT_KEY_PLACEHOLDER;"
      },
      "container": { "name": "STORAGE_CONTAINER_PLACEHOLDER" }
    }"###;

        let datasource_replaced = get_datasource_definition(
            datasource_raw_json.to_string(),
            "datasource_name",
            "storage_account",
            "storage_container",
            "storage_master_key",
        );

        let expected_replaced = r###"{
      "name": "datasource_name",
      "type": "azureblob",
      "credentials": {
        "connectionString": "DefaultEndpointsProtocol=https;AccountName=storage_account;AccountKey=storage_master_key;"
      },
      "container": { "name": "storage_container" }
    }"###.to_string();

        assert_eq!(datasource_replaced, expected_replaced);
    }
}
