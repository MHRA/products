use azure_sdk_core::errors::AzureError;
use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug)]
pub enum DocType {
    Par,
    Pil,
    Spc,
}

#[derive(Error, Debug)]
pub enum ImportError {
    #[error(transparent)]
    AzureError(#[from] AzureError),
    #[error("Could not open worksheet: {0}")]
    WorkbookOpenError(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    #[serde(rename = "dDocName")]
    pub filename: String,

    #[serde(rename = "dDocType")]
    pub doc_type: String,

    #[serde(rename = "dDocTitle")]
    pub title: String,

    #[serde(rename = "dSecurityGroup")]
    pub security_group: String,

    #[serde(rename = "dDocAuthor", default)]
    pub author: String,

    #[serde(rename = "dRevLabel")]
    pub rev_label: String,

    #[serde(rename = "dCreateDate", with = "crate::date_de")]
    pub created: DateTime<Utc>,

    #[serde(rename = "dReleaseState")]
    pub release_state: String,

    #[serde(rename = "xKeywords", default)]
    pub keywords: String,

    #[serde(rename = "xProductName", default)]
    pub product_name: String,

    #[serde(rename = "xSubstanceName", default)]
    pub substance_name: String,

    #[serde(rename = "xSecondLevel", default)]
    pub second_level: String,
}
