use chrono::{SecondsFormat, Utc};
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
    pub context: String,
    #[serde(rename = "@odata.count")]
    pub count: Option<i32>,
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
    pub content: String,
    pub rev_label: String,
    pub metadata_storage_path: String,
    pub metadata_content_type: String,
    pub product_name: String,
    pub metadata_language: String,
    pub created: String,
    pub release_state: String,
    pub keywords: String,
    pub title: String,
    pub pl_number: Vec<String>,
    pub file_name: String,
    pub metadata_storage_content_type: String,
    pub metadata_storage_size: usize,
    pub metadata_storage_last_modified: String,
    pub metadata_storage_content_md5: String,
    pub metadata_storage_name: String,
    pub doc_type: String,
    pub suggestions: Vec<String>,
    pub substance_name: Vec<String>,
    pub facets: Vec<String>,
}

// The AzureResult model does not contain all of the information we want in the index,
// however, the automatic index rebuild will populate the missing information.
impl From<AzureResult> for IndexEntry {
    fn from(res: AzureResult) -> Self {
        Self {
            content: "Content not yet available".to_owned(),
            rev_label: match res.rev_label {
                Some(rl) => rl.clone(),
                None => "1".to_owned(),
            },
            product_name: match res.product_name {
                Some(pn) => pn.clone(),
                None => "".to_owned(),
            },
            created: match res.created {
                Some(cr) => cr.clone(),
                None => Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            },
            release_state: match res.release_state {
                Some(rs) => rs.clone(),
                None => "Y".to_owned(),
            },
            keywords: match res.keywords {
                Some(k) => k.clone(),
                None => "".to_owned(),
            },
            title: res.title.clone(),
            pl_number: vec![],
            file_name: res.file_name.clone(),
            doc_type: res.doc_type.clone(),
            suggestions: res.suggestions.clone(),
            substance_name: res.substance_name.clone(),
            facets: res.facets.clone(),
            metadata_storage_content_type: String::default(),
            metadata_storage_size: res.metadata_storage_size as usize,
            metadata_storage_last_modified: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            metadata_storage_content_md5: String::default(),
            metadata_storage_name: res.metadata_storage_name.clone(),
            metadata_storage_path: res.metadata_storage_path.clone(),
            metadata_content_type: String::default(),
            metadata_language: String::default(),
        }
    }
}
