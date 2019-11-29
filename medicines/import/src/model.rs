use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum DocType {
    Par,
    PilLabel,
    PilLabelAndLeaflet,
    PilLeaflet,
    Spc,
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
    #[serde(rename = "dDocAuthor")]
    pub author: String,
    #[serde(rename = "dRevLabel")]
    pub rev_label: String,
    #[serde(rename = "dCreateDate", with = "crate::date_de")]
    pub created: DateTime<Utc>,
    #[serde(rename = "dReleaseState")]
    pub release_state: String,
    #[serde(rename = "xKeywords")]
    pub keywords: String,
    #[serde(rename = "xProductName", default)]
    pub product_name: String,
    #[serde(rename = "xSubstanceName", default)]
    pub substance_name: String,
    #[serde(rename = "xSecondLevel", default)]
    pub second_level: String,
}
