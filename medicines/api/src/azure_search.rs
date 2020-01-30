use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AzureHighlight {
    #[serde(rename="content")]
    content: Vec<String>
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
    created: String,
    facets: Vec<String>,
    keywords: Option<String>,
    metadata_storage_size: i32,
    release_state: String,
    rev_label: Option<String>,
    suggestions: Vec<String>,
    #[serde(rename="@search.score")]
    score: f32,
    #[serde(rename="@search.highlights")]
    highlights: AzureHighlight,
}

#[derive(Debug, Deserialize)]
pub struct AzureSearchResults {
    pub value: Vec<AzureResult>,
    #[serde(rename="@odata.context")]
    context: String,
    #[serde(rename="@odata.count")]
    count: Option<i32>,
}

fn get_env(key: &str) -> String {
    match std::env::var(key) {
        Ok(b) => b,
        Err(_) => "".to_owned()
    }
}

pub async fn azure_search(search_term: String) -> Result<AzureSearchResults, reqwest::Error> {
    let search_service = get_env("AZURE_SEARCH_SERVICE");
    let search_index = get_env("AZURE_SEARCH_INDEX");
    let api_key = get_env("AZURE_SEARCH_KEY");

    let base_url = format!(
        "https://{search_service}.search.windows.net/indexes/{search_index}/docs",
        search_service = search_service,
        search_index = search_index
    );

    let client = reqwest::Client::new();
    let req = client
        .get(&base_url)
        .query(&[
            ("api-version","2017-11-11"),
            ("api-key", &api_key),
            ("highlight","content"),
            ("queryType","full"),
            ("@count","true"),
            ("@top","10"),
            ("@skip","0"),
            ("search", &search_term),
            ("scoringProfile","preferKeywords")])
        .build()
        .unwrap();

    let r = client.execute(req)
        .await;

    let s = match r {
        Ok(t) => t,
        Err(e) => return Err(e)
    };

    s.json::<AzureSearchResults>()
        .await
}