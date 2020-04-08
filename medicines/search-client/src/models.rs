use core::fmt::Debug;
use serde_derive::{Deserialize, Serialize};

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
    pub created: Option<String>,
    pub facets: Vec<String>,
    pub keywords: Option<String>,
    pub metadata_storage_size: i32,
    pub release_state: Option<String>,
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

#[derive(Debug, Serialize)]
pub struct IndexEntry {
    content: String,
    rev_label: String,
    metadata_storage_path: String,
    metadata_content_type: String,
    product_name: String,
    metadata_language: String,
    created: String,
    release_state: String,
    keywords: String,
    title: String,
    pl_number: Vec<String>,
    file_name: String,
    metadata_storage_content_type: String,
    metadata_storage_size: usize,
    metadata_storage_last_modified: String,
    metadata_storage_content_md5: String,
    metadata_storage_name: String,
    doc_type: String,
    suggestions: Vec<String>,
    substance_name: Vec<String>,
    facets: Vec<String>,
}
