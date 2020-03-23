use serde_derive::Deserialize;

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
    pub value: Vec<AzureResult>,
    #[serde(rename = "@odata.context")]
    context: String,
    #[serde(rename = "@odata.count")]
    count: Option<i32>,
}

struct AzureConfig {
    search_service: String,
    search_index: String,
    api_key: String,
}

pub struct AzureSearchClient {
    client: reqwest::Client,
    config: AzureConfig,
}

pub struct AzureContext {
    pub client: AzureSearchClient,
}

impl juniper::Context for AzureContext {}

fn get_env(key: &str) -> String {
    match std::env::var(key) {
        Ok(b) => {
            if b.is_empty() {
                panic!(
                    "Key '{:#}' found in environment variables but was '{:#}",
                    key, b
                )
            }
            b
        }
        Err(e) => panic!(
            "Key '{:#}' not found in environment variables: '{:#}",
            key, e
        ),
    }
}

pub fn create_context() -> AzureContext {
    let api_key = get_env("AZURE_SEARCH_KEY");
    let search_index = get_env("AZURE_SEARCH_INDEX");
    let search_service = get_env("AZURE_SEARCH_SERVICE");

    AzureContext {
        client: AzureSearchClient {
            client: reqwest::Client::new(),
            config: AzureConfig {
                api_key,
                search_index,
                search_service,
            },
        },
    }
}

impl AzureSearchClient {
    #[allow(dead_code)]
    pub async fn azure_search(
        &self,
        search_term: String,
    ) -> Result<AzureSearchResults, reqwest::Error> {
        azure_search(search_term, &self.client, &self.config).await
    }

    pub async fn filter_by_collection(
        &self,
        field_name: String,
        value: String,
        operator: String,
    ) -> Result<AzureSearchResults, reqwest::Error> {
        filter_by_collection(field_name, value, operator, &self.client, &self.config).await
    }
}

#[allow(dead_code)]
async fn azure_search(
    search_term: String,
    client: &reqwest::Client,
    config: &AzureConfig,
) -> Result<AzureSearchResults, reqwest::Error> {
    let base_url = format!(
        "https://{search_service}.search.windows.net/indexes/{search_index}/docs",
        search_service = config.search_service,
        search_index = config.search_index
    );

    let req = client
        .get(&base_url)
        .query(&[
            ("api-version", "2017-11-11"),
            ("api-key", &config.api_key),
            ("highlight", "content"),
            ("queryType", "full"),
            ("@count", "true"),
            ("@top", "10"),
            ("@skip", "0"),
            ("search", &search_term),
            ("scoringProfile", "preferKeywords"),
        ])
        .build()?;

    println!("Requesting from URL: {}", &req.url());

    client
        .execute(req)
        .await?
        .json::<AzureSearchResults>()
        .await
}

fn build_filter_by_collection_request(
    field_name: &String,
    value: &String,
    operator: &String,
    client: &reqwest::Client,
    config: &AzureConfig,
) -> Result<reqwest::Request, reqwest::Error> {
    let base_url = format!(
        "https://{search_service}.search.windows.net/indexes/{search_index}/docs",
        search_service = config.search_service,
        search_index = config.search_index
    );

    let filter = format!(
        "{field_name}/any(value: value {operator} '{value}')",
        field_name = field_name,
        value = value,
        operator = operator,
    );

    client
        .get(&base_url)
        .query(&[
            ("api-version", "2017-11-11"),
            ("api-key", &config.api_key),
            ("$filter", &filter),
        ])
        .build()
}

async fn filter_by_collection(
    field_name: String,
    value: String,
    operator: String,
    client: &reqwest::Client,
    config: &AzureConfig,
) -> Result<AzureSearchResults, reqwest::Error> {
    let req = build_filter_by_collection_request(&field_name, &value, &operator, &client, &config)?;
    client
        .execute(req)
        .await?
        .json::<AzureSearchResults>()
        .await
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_filter_by_collection_request() {
        let client = reqwest::Client::new();
        let config = AzureConfig {
            search_service: "my_cool_service".to_string(),
            search_index: "my_cool_search_index".to_string(),
            api_key: "my_cool_api_key".to_string(),
        };

        let req = build_filter_by_collection_request(
            &"my_cool_field".to_string(),
            &"my cool value".to_string(),
            &"cooler_than".to_string(),
            &client,
            &config,
        )
        .unwrap();

        let url = req.url();
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.host_str(), Some("my_cool_service.search.windows.net"));
        assert_eq!(url.path(), "/indexes/my_cool_search_index/docs");

        let mut query = url.query_pairs();
        assert_eq!(
            query
                .find(|query_pair| query_pair.0 == "api-version")
                .unwrap()
                .1,
            "2017-11-11"
        );
        assert_eq!(
            query
                .find(|query_pair| query_pair.0 == "api-key")
                .unwrap()
                .1,
            "my_cool_api_key"
        );
        assert_eq!(
            query
                .find(|query_pair| query_pair.0 == "$filter")
                .unwrap()
                .1,
            "my_cool_field/any(value: value cooler_than 'my cool value')"
        );
    }
}
