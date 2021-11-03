pub use crate::document_type::{
    DocTypeParseError, DocumentType, TerritoryType, TerritoryTypeParseError,
};
use chrono::{SecondsFormat, Utc};
use core::fmt::Debug;
use serde_derive::{Deserialize, Serialize};
use std::clone::Clone;

#[derive(Clone, Debug, Deserialize)]
pub struct AzureHighlight {
    pub content: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IndexResult {
    pub doc_type: DocumentType,
    pub territory: Option<TerritoryType>,
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
pub struct IndexResults {
    #[serde(rename = "value")]
    pub search_results: Vec<IndexResult>,
    #[serde(rename = "@odata.context")]
    pub context: String,
    #[serde(rename = "@odata.count")]
    pub count: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReportResult {
    pub active_substances: Option<Vec<String>>,
    #[serde(rename = "@search.highlights")]
    pub highlights: Option<AzureHighlight>,
    #[serde(rename = "@search.score")]
    pub score: f32,
    pub file_name: String,
    pub metadata_storage_path: String,
    pub products: Option<Vec<String>>,
    pub summary: String,
    pub pbpk_models: Option<Vec<String>>,
    pub matrices: Option<Vec<String>>,
    pub metadata_storage_name: String,
    pub report_name: String,
    pub metadata_storage_size: i32,
    pub pregnancy_trimesters: Option<Vec<String>>,
    pub pl_numbers: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ReportResults {
    #[serde(rename = "value")]
    pub search_results: Vec<ReportResult>,
    #[serde(rename = "@odata.context")]
    pub context: String,
    #[serde(rename = "@odata.count")]
    pub count: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Facet {
    pub count: i32,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FacetResult {
    pub facets: Vec<Facet>,
}

#[derive(Debug, Deserialize)]
pub struct FacetResults {
    #[serde(rename = "value")]
    pub search_results: Vec<IndexResult>,
    #[serde(rename = "@search.facets")]
    pub facet_results: FacetResult,
    #[serde(rename = "@odata.context")]
    pub context: String,
}

#[derive(Debug, Deserialize)]
pub struct AzureIndexChangedResults {
    pub value: Vec<AzureIndexChangedResult>,
    #[serde(rename = "@odata.context")]
    context: String,
}

impl AzureIndexChangedResults {
    pub fn new(index_changed_result: AzureIndexChangedResult) -> AzureIndexChangedResults {
        AzureIndexChangedResults {
            context: "context".to_string(),
            value: vec![index_changed_result],
        }
    }
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
    pub territory: TerritoryType,
    pub file_name: String,
    pub metadata_storage_content_type: String,
    pub metadata_storage_size: usize,
    pub metadata_storage_last_modified: String,
    pub metadata_storage_content_md5: String,
    pub metadata_storage_name: String,
    pub doc_type: DocumentType,
    pub suggestions: Vec<String>,
    pub substance_name: Vec<String>,
    pub facets: Vec<String>,
}

// The IndexResult model does not contain all of the information we want in the index,
// however, the automatic index rebuild will populate the missing information.
impl From<IndexResult> for IndexEntry {
    fn from(res: IndexResult) -> Self {
        Self {
            content: "Content not yet available".to_owned(),
            rev_label: match res.rev_label {
                Some(rl) => rl,
                None => "1".to_owned(),
            },
            product_name: match res.product_name {
                Some(pn) => pn,
                None => "".to_owned(),
            },
            created: match res.created {
                Some(cr) => cr,
                None => Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            },
            release_state: match res.release_state {
                Some(rs) => rs,
                None => "Y".to_owned(),
            },
            keywords: match res.keywords {
                Some(k) => k,
                None => "".to_owned(),
            },
            title: res.title,
            pl_number: vec![],
            file_name: res.file_name,
            doc_type: res.doc_type,
            territory: match res.territory {
                Some(t) => t,
                None => TerritoryType::UK,
            },
            suggestions: res.suggestions,
            substance_name: res.substance_name,
            facets: res.facets,
            metadata_storage_content_type: String::default(),
            metadata_storage_size: res.metadata_storage_size as usize,
            metadata_storage_last_modified: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            metadata_storage_content_md5: String::default(),
            metadata_storage_name: res.metadata_storage_name,
            metadata_storage_path: res.metadata_storage_path,
            metadata_content_type: String::default(),
            metadata_language: String::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn deserializes_correctly() {
        let json = "{\"@odata.context\":\"https://mhraproductsproduction.search.windows.net/indexes('products-index')/$metadata#docs(*)\",\"@odata.count\":4,\"value\":[
            {\"@search.score\":1.0,\"rev_label\":\"1\",\"metadata_storage_path\":\"https://mhraproductsproduction.blob.core.windows.net/docs/2f81f987a17ab865c4b6302a2711709794f6b3de\",\"product_name\":\"LARIAM 250MG TABLETS\",\"created\":\"2019-12-20T05:07:00+00:00\",\"release_state\":\"Y\",\"keywords\":null,\"title\":\"leaflet MAH BRAND_PLPI 20774-1329.pdf\",\"file_name\":\"CON1576818432234\",\"metadata_storage_size\":181194,\"metadata_storage_name\":\"2f81f987a17ab865c4b6302a2711709794f6b3de\",\"doc_type\":\"Pil\",\"suggestions\":[],\"substance_name\":[\"MEFLOQUINE\"],\"facets\":[\"M\",\"M, MEFLOQUINE\",\"M, MEFLOQUINE, LARIAM 250MG TABLETS\"]},
            {\"@search.score\":1.0,\"rev_label\":\"1\",\"metadata_storage_path\":\"https://mhraproductsproduction.blob.core.windows.net/docs/bda052785cf870d1249269797b36a856cd2ed8ed\",\"product_name\":\"LARIAM 250MG TABLETS\",\"created\":\"2019-08-09T05:23:00+00:00\",\"release_state\":\"Y\",\"keywords\":null,\"title\":\"leaflet MAH BRAND_PL 27041-0012.pdf\",\"file_name\":\"CON1565324634716\",\"metadata_storage_size\":394446,\"metadata_storage_name\":\"bda052785cf870d1249269797b36a856cd2ed8ed\",\"doc_type\":\"Pil\",\"suggestions\":[],\"substance_name\":[\"MEFLOQUINE HYDROCHLORIDE\"],\"facets\":[\"M\",\"M, MEFLOQUINE HYDROCHLORIDE\",\"M, MEFLOQUINE HYDROCHLORIDE, LARIAM 250MG TABLETS\"]},
            {\"@search.score\":1.0,\"rev_label\":\"1\",\"metadata_storage_path\":\"https://mhraproductsproduction.blob.core.windows.net/docs/4183369448f303b3d15b8edf002db03434183773\",\"product_name\":\"LARIAM 250MG TABLETS\",\"created\":\"2020-01-10T05:06:00+00:00\",\"release_state\":\"Y\",\"keywords\":null,\"title\":\"leaflet MAH BRAND_PLPI 20636-1861.pdf\",\"file_name\":\"CON1578632786776\",\"metadata_storage_size\":184883,\"metadata_storage_name\":\"4183369448f303b3d15b8edf002db03434183773\",\"doc_type\":\"Pil\",\"suggestions\":[],\"substance_name\":[\"MEFLOQUINE HYDROCHLORIDE\"],\"facets\":[\"M\",\"M, MEFLOQUINE HYDROCHLORIDE\",\"M, MEFLOQUINE HYDROCHLORIDE, LARIAM 250MG TABLETS\"]},
            {\"@search.score\":1.0,\"rev_label\":\"1\",\"metadata_storage_path\":\"https://mhraproductsproduction.blob.core.windows.net/docs/947128e7c11cb9f45aada193748f24ec6cdc5b52\",\"product_name\":\"LARIAM 250MG TABLETS\",\"created\":\"2019-08-09T05:23:00+00:00\",\"release_state\":\"Y\",\"keywords\":null,\"title\":\"spc-doc_PL 27041-0012.pdf\",\"file_name\":\"CON1565324634426\",\"metadata_storage_size\":71288,\"metadata_storage_name\":\"947128e7c11cb9f45aada193748f24ec6cdc5b52\",\"doc_type\":\"Spc\",\"suggestions\":[],\"substance_name\":[\"MEFLOQUINE HYDROCHLORIDE\"],\"facets\":[\"M\",\"M, MEFLOQUINE HYDROCHLORIDE\",\"M, MEFLOQUINE HYDROCHLORIDE, LARIAM 250MG TABLETS\"]}
        ]}";

        let results: IndexResults = serde_json::from_str(json).unwrap();

        assert_eq!(results.search_results.len(), 4);
        assert_eq!(
            results.search_results[0].product_name,
            Some("LARIAM 250MG TABLETS".into())
        );
        assert_eq!(results.search_results[0].doc_type, DocumentType::Pil);
        assert_eq!(results.search_results[3].doc_type, DocumentType::Spc);
    }

    #[test]
    fn index_results_deserializes_correctly() {
        let json = "{\"@odata.context\":\"https://mhraproductsproduction.search.windows.net/indexes('products-index')/$metadata#docs(*)\",\"value\":[],\"@search.facets\":{\"facets\":[
            {\"value\":\"A, ACETOMENAPHTHONE, KETOVITE TABLETS\",\"count\":3},
            {\"value\":\"A, ACETYLCYSTEINE\",\"count\":10},
            {\"value\":\"A, ALANINE, NUTRIFLEX LIPID PERI EMULSION FOR INFUSION\",\"count\":6}
        ]}}";

        let results: FacetResults = serde_json::from_str(json).unwrap();

        assert_eq!(results.facet_results.facets.len(), 3);
        assert_eq!(
            results.facet_results.facets[0].value,
            "A, ACETOMENAPHTHONE, KETOVITE TABLETS".to_string()
        );
        assert_eq!(results.facet_results.facets[2].count, 6);
    }
}
