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
    pub search_results: Vec<AzureResult>,
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

pub fn get_env(key: &str) -> String {
    std::env::var(key).expect(&format!("Set env variable {} first!", key))
}

pub fn factory() -> AzureSearchClient {
    let api_key = get_env("SEARCH_API_KEY");
    let search_index = get_env("SEARCH_INDEX");
    let search_service = get_env("SEARCH_SERVICE");

    AzureSearchClient {
        client: reqwest::Client::new(),
        config: AzureConfig {
            api_key,
            search_index,
            search_service,
        },
    }
}

impl AzureSearchClient {
    pub async fn azure_search(
        &self,
        search_term: &String,
    ) -> Result<AzureSearchResults, reqwest::Error> {
        azure_search(&search_term, &self.client, &self.config).await
    }
}

async fn azure_search(
    search_term: &String,
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
