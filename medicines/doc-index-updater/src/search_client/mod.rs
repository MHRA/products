use crate::models::IndexEntry;
use core::fmt::Debug;
use serde::ser::Serialize;
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct AzureHighlight {
    #[serde(rename = "content")]
    content: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AzureResult {
    pub doc_type: String,
    pub file_name: String,
    pub metadata_storage_name: String,
    pub metadata_storage_path: String,
    pub product_name: Option<String>,
    pub substance_name: Vec<String>,
    pub title: String,
    pub created: String,
    pub facets: Vec<String>,
    pub keywords: Option<String>,
    pub metadata_storage_size: i32,
    pub release_state: String,
    pub rev_label: Option<String>,
    pub suggestions: Vec<String>,
    #[serde(rename = "@search.score")]
    pub score: f32,
    #[serde(rename = "@search.highlights")]
    pub highlights: Option<AzureHighlight>,
}

#[derive(Debug, Deserialize)]
pub struct AzureSearchResults {
    #[serde(rename = "value")]
    pub search_results: Vec<AzureResult>,
    #[serde(rename = "@odata.context")]
    context: String,
    #[serde(rename = "@odata.count")]
    count: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AzureIndexChangedResults {
    pub value: Vec<AzureIndexChangedResult>,
    #[serde(rename = "@odata.context")]
    context: String,
}

#[derive(Debug, Deserialize)]
pub struct AzureIndexChangedResult {
    pub key: String,
    pub status: bool,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "statusCode")]
    pub status_code: u16,
}

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

impl AzureSearchClient {
    pub async fn search(&self, search_term: String) -> Result<AzureSearchResults, reqwest::Error> {
        search(search_term, &self.client, self.config.clone()).await
    }

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

    println!("Requesting from URL: {}", &req.url());

    client
        .execute(req)
        .await?
        .json::<AzureSearchResults>()
        .await
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
