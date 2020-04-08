pub mod models;

use crate::models::{AzureIndexChangedResults, AzureSearchResults, IndexEntry};
use async_trait::async_trait;
use core::fmt::Debug;
use serde::ser::Serialize;
use std::collections::HashMap;

#[derive(Clone)]
struct AzureConfig {
    search_service: String,
    search_index: String,
    api_key: String,
    api_version: String,
}

pub struct AzureSearchClient {
    client: reqwest::Client,
    config: AzureConfig,
}

pub fn get_env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("Set env variable {} first!", key))
}

pub fn factory() -> AzureSearchClient {
    let api_key = get_env("AZURE_API_ADMIN_KEY");
    let search_index = get_env("AZURE_SEARCH_INDEX");
    let search_service = get_env("SEARCH_SERVICE");
    let api_version = get_env("AZURE_SEARCH_API_VERSION");

    AzureSearchClient {
        client: reqwest::Client::new(),
        config: AzureConfig {
            api_key,
            search_index,
            search_service,
            api_version,
        },
    }
}

#[async_trait]
pub trait Searchable {
    async fn search(&self, &mut search_term: String) -> Result<AzureSearchResults, reqwest::Error>;
}

#[async_trait]
impl Searchable for AzureSearchClient {
    async fn search(&self, search_term: String) -> Result<AzureSearchResults, reqwest::Error> {
        search(search_term, &self.client, self.config.clone()).await
    }
}

impl AzureSearchClient {
    pub async fn delete(
        &self,
        key_name: &str,
        value: &str,
    ) -> Result<AzureIndexChangedResults, anyhow::Error> {
        let mut key_values = HashMap::new();
        key_values.insert(key_name.to_string(), value.to_string());
        key_values.insert("@search.action".to_string(), "delete".to_string());

        update_index(key_values, &self.client, &self.config).await
    }

    pub async fn create(
        &self,
        key_values: IndexEntry,
    ) -> Result<AzureIndexChangedResults, anyhow::Error> {
        update_index(key_values, &self.client, &self.config).await
    }
}

async fn search(
    search_term: String,
    client: &reqwest::Client,
    config: AzureConfig,
) -> Result<AzureSearchResults, reqwest::Error> {
    let req = build_search(search_term, &client, config)?;
    tracing::debug!("Requesting from URL: {}", &req.url());
    client
        .execute(req)
        .await?
        .error_for_status()?
        .json::<AzureSearchResults>()
        .await
}

fn build_search(
    search_term: String,
    client: &reqwest::Client,
    config: AzureConfig,
) -> Result<reqwest::Request, reqwest::Error> {
    let base_url = format!(
        "https://{search_service}.search.windows.net/indexes/{search_index}/docs",
        search_service = config.search_service,
        search_index = config.search_index
    );

    let req = client
        .get(&base_url)
        .query(&[
            ("api-version", config.api_version),
            ("highlight", "content".to_string()),
            ("queryType", "full".to_string()),
            ("@count", "true".to_string()),
            ("@top", "10".to_string()),
            ("@skip", "0".to_string()),
            ("search", search_term),
            ("scoringProfile", "preferKeywords".to_string()),
        ])
        .header("api-key", &config.api_key)
        .build()?;

    Ok(req)
}

async fn update_index<T>(
    key_values: T,
    client: &reqwest::Client,
    config: &AzureConfig,
) -> Result<AzureIndexChangedResults, anyhow::Error>
where
    T: Serialize + Sized + Debug,
{
    let base_url = format!(
        "https://{search_service}.search.windows.net/indexes/{search_index}/docs/index",
        search_service = config.search_service,
        search_index = config.search_index
    );

    let mut body = HashMap::new();
    body.insert("value", [key_values]);

    let req = client
        .post(&base_url)
        .query(&[("api-version", &config.api_version)])
        .header("api-key", &config.api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;

    tracing::debug!("\nBody: {:?}", &body);
    tracing::debug!("\nRequest: {:?}", &req);
    tracing::debug!("\nRequesting from URL: {}", &req.url());

    let h = client.execute(req).await?;

    if h.status() == reqwest::StatusCode::OK {
        h.json::<AzureIndexChangedResults>()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    } else {
        let error_message = h.text().await?;
        Err(anyhow::anyhow!(error_message))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn given_we_have_a_search_client() -> reqwest::Client {
        reqwest::Client::new()
    }

    fn given_we_have_a_search_term() -> String {
        "cool beans".to_string()
    }

    fn given_we_have_a_config() -> AzureConfig {
        AzureConfig {
            api_key: "api_key".to_string(),
            search_index: "search_index".to_string(),
            search_service: "search_service".to_string(),
            api_version: "api_version".to_string(),
        }
    }

    fn when_we_build_a_search_request(
        client: reqwest::Client,
        search_term: String,
        config: AzureConfig,
    ) -> Result<reqwest::Request, reqwest::Error> {
        build_search(search_term, &client, config)
    }

    fn then_search_url_is_as_expected(actual_result: Result<reqwest::Request, reqwest::Error>) {
        if let Ok(actual) = actual_result {
            let actual = actual.url().to_string();
            let expected = "https://search_service.search.windows.net/indexes/search_index/docs?api-version=api_version&highlight=content&queryType=full&%40count=true&%40top=10&%40skip=0&search=cool+beans&scoringProfile=preferKeywords"
                .to_string();

            assert_eq!(actual, expected);
        } else {
            assert!(false, "Provided search request is an error");
        }
    }

    #[test]
    fn test_build_search() {
        let client = given_we_have_a_search_client();
        let search_term = given_we_have_a_search_term();
        let config = given_we_have_a_config();
        let actual = when_we_build_a_search_request(client, search_term, config);
        then_search_url_is_as_expected(actual);
    }
}
