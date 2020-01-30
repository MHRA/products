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
    count: i32,
}

pub async fn azure_search() -> Result<AzureSearchResults, reqwest::Error> {
    let r = reqwest::get("https://mhraproductsprod-1.search.windows.net/indexes/products-index-20200120/docs?api-key=45E5B47F3127148F3CE04EC272D0E458&api-version=2017-11-11&highlight=content&queryType=full&%24count=true&%24top=10&%24skip=0&search=ibuprofen&scoringProfile=preferKeywords")
        .await;

    let s = match r {
        Ok(t) => t,
        Err(e) => return Err(e)
    };

    s.json::<AzureSearchResults>()
        .await
}