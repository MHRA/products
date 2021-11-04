use super::sanitiser::{SanitisedString, VecSanitisedString};
use crate::{create_manager::Blob, models::Document};
use chrono::{SecondsFormat, Utc};
use regex::Regex;
use search_client::models::{DocumentType, IndexEntry, TerritoryType};
use std::{collections::HashMap, str};

#[derive(Clone, Debug, PartialEq)]
pub struct BlobMetadata {
    pub file_name: SanitisedString,
    pub doc_type: DocumentType,
    pub title: SanitisedString,
    pub pl_number: String,
    pub territory: Option<TerritoryType>,
    pub product_names: VecSanitisedString,
    pub active_substances: VecSanitisedString,
    pub author: SanitisedString,
    pub keywords: Option<VecSanitisedString>,
}

impl BlobMetadata {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        file_name: String,
        doc_type: DocumentType,
        title: String,
        pl_number: String,
        territory: Option<TerritoryType>,
        product_names: Vec<String>,
        active_substances: Vec<String>,
        author: String,
        keywords: Option<Vec<String>>,
    ) -> Self {
        BlobMetadata {
            file_name: file_name.into(),
            doc_type,
            title: title.into(),
            pl_number,
            territory,
            product_names: product_names.into(),
            active_substances: active_substances.into(),
            author: author.into(),
            keywords: keywords.map(|keywords| keywords.into()),
        }
    }
    fn facets(&self) -> Vec<String> {
        create_facets_by_active_substance(
            self.product_names.clone(),
            self.active_substances.clone(),
        )
    }
}

impl Into<BlobMetadata> for Document {
    fn into(self) -> BlobMetadata {
        let title = SanitisedString::from(&self.name);
        let pl_number = format_product_licence(&self.pl_number);

        BlobMetadata {
            file_name: SanitisedString::from(&self.id),
            doc_type: self.document_type,
            title,
            pl_number,
            territory: self.territory,
            product_names: VecSanitisedString::from(
                self.products
                    .iter()
                    .map(|a| a.to_uppercase())
                    .collect::<Vec<String>>(),
            ),
            active_substances: VecSanitisedString::from(
                self.active_substances
                    .iter()
                    .map(|a| a.to_uppercase())
                    .collect::<Vec<String>>(),
            ),
            author: SanitisedString::from(&self.author),
            keywords: match self.keywords {
                Some(a) => Some(VecSanitisedString::from(a)),
                None => None,
            },
        }
    }
}

impl Into<HashMap<String, String>> for BlobMetadata {
    fn into(self) -> HashMap<String, String> {
        let mut metadata: HashMap<String, String> = HashMap::new();

        metadata.insert("file_name".to_string(), self.file_name.to_string());
        metadata.insert("doc_type".to_string(), self.doc_type.to_string());
        metadata.insert("title".to_string(), self.title.to_string());
        metadata.insert("product_name".to_string(), self.product_names.join(", "));
        metadata.insert(
            "substance_name".to_string(),
            self.active_substances.to_json(),
        );
        metadata.insert("facets".to_string(), to_json(self.facets()));
        if let Some(keywords) = self.keywords.clone() {
            metadata.insert("keywords".to_string(), keywords.join(" "));
        }
        metadata.insert("pl_number".to_string(), self.pl_number.clone());
        if let Some(territory) = self.territory {
            metadata.insert("territory".to_string(), territory.to_string());
        }
        metadata.insert("author".to_string(), self.author.to_string());

        metadata
    }
}

impl From<Blob> for IndexEntry {
    fn from(blob: Blob) -> Self {
        Self {
            content: "Content not yet available".to_owned(),
            rev_label: "1".to_owned(),
            product_name: blob.metadata.product_names.join(", "),
            created: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            release_state: "Y".to_owned(),
            keywords: blob
                .metadata
                .keywords
                .clone()
                .unwrap_or_default()
                .join(", "),
            title: blob.metadata.title.to_string(),
            pl_number: vec![blob.metadata.pl_number.to_string()],
            territory: blob.metadata.territory,
            file_name: blob.metadata.file_name.to_string(),
            doc_type: blob.metadata.doc_type,
            suggestions: vec![],
            substance_name: blob.metadata.active_substances.to_vec_string(),
            facets: blob.metadata.facets(),
            metadata_storage_content_type: String::default(),
            metadata_storage_size: blob.size,
            metadata_storage_last_modified: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            metadata_storage_content_md5: String::default(),
            metadata_storage_name: blob.name.to_owned(),
            metadata_storage_path: blob.path,
            metadata_content_type: String::default(),
            metadata_language: String::default(),
        }
    }
}

pub fn to_json(words: Vec<String>) -> String {
    serde_json::to_string(&words).expect("Couldn't create JSON array.")
}

pub fn create_facets_by_active_substance(
    products: VecSanitisedString,
    active_substances: VecSanitisedString,
) -> Vec<String> {
    let mut facets: Vec<String> = active_substances
        .to_vec_string()
        .iter()
        .map(|a| {
            if let Some(first) = a.to_string().chars().next() {
                vec![
                    first.to_string(),
                    [first.to_string(), a.to_string()].join(", "),
                    [first.to_string(), a.to_string(), products.join(", ")].join(", "),
                ]
            } else {
                vec![]
            }
        })
        .flatten()
        .collect();
    facets.sort();
    facets.dedup();
    facets
}

pub fn format_product_licence(input: &str) -> String {
    lazy_static! {
        static ref RE_WHITESPACE: Regex = Regex::new(r"(\s+|/|_|-)").expect("cannot compile regex");
        static ref RE_PL: Regex = Regex::new(r"(?i:[A-Z]+)(\s+|/|_|-)*\d{5}(\s+|/|_|-)*\d{4}")
            .expect("cannot compile regex");
    }
    let product_licences: Vec<String> = RE_PL
        .find_iter(input)
        .map(|m| {
            RE_WHITESPACE
                .replace_all(m.as_str(), "")
                .to_ascii_uppercase()
        })
        .collect();

    to_json(product_licences)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::FileSource;
    use search_client::models::DocumentType;
    use test_case::test_case;

    #[test]
    fn derive_metadata() {
        let doc = Document {
            id: "CON123456".to_string(),
            name: "Paracetamol Plus PL 12345/6789".to_string(),
            document_type: DocumentType::Spc,
            author: "JRR Tolkien".to_string(),
            products: vec![
                "Effective product 1".to_string(),
                "Effective product 2".to_string(),
            ],
            keywords: Some(vec![
                "Very good for you".to_string(),
                "Cures headaches".to_string(),
                "PL 12345/6789".to_string(),
            ]),
            pl_number: "PL 12345/6789".to_string(),
            territory: Some(TerritoryType::UK),
            active_substances: vec!["Paracetamol".to_string(), "Caffeine".to_string()],
            file_path: "location/on/disk".to_string(),
            file_source: FileSource::Sentinel,
        };

        let expected_file_name = "CON123456".to_string();
        let expected_doc_type = "Spc".to_string();
        let expected_title = "Paracetamol Plus PL 12345/6789".to_string();
        let expected_author = "JRR Tolkien".to_string();
        let expected_product_name = "EFFECTIVE PRODUCT 1, EFFECTIVE PRODUCT 2".to_string();
        let expected_substance_name = "[\"PARACETAMOL\",\"CAFFEINE\"]".to_string();
        let expected_keywords = "Very good for you Cures headaches PL 12345/6789".to_string();
        let expected_pl_number = "[\"PL123456789\"]".to_string();
        let expected_territory = "UK".to_string();

        let output_metadata: HashMap<String, String> = Into::<BlobMetadata>::into(doc).into();

        assert_eq!(output_metadata["file_name"], expected_file_name);
        assert_eq!(output_metadata["doc_type"], expected_doc_type);
        assert_eq!(output_metadata["title"], expected_title);
        assert_eq!(output_metadata["author"], expected_author);
        assert_eq!(output_metadata["product_name"], expected_product_name);
        assert_eq!(output_metadata["substance_name"], expected_substance_name);
        assert_eq!(output_metadata["keywords"], expected_keywords);
        assert_eq!(output_metadata["pl_number"], expected_pl_number);
        assert_eq!(output_metadata["territory"], expected_territory);
    }

    #[test]
    fn test_create_facets_by_active_substance() {
        let active_substances = vec![
            "LOSARTAN POTASSIUM".to_string(),
            "HYDROCHLOROTHIAZIDE".to_string(),
            "L-TEST".to_string(),
        ];
        let products = vec![
            "LOSARTAN POTASSIUM / HYDROCHLOROTHIAZIDE 100 MG /25 MG FILM-COATED TABLETS".to_owned(),
        ];
        let expected = vec![
            "H",
            "H, HYDROCHLOROTHIAZIDE",
            "H, HYDROCHLOROTHIAZIDE, LOSARTAN POTASSIUM / HYDROCHLOROTHIAZIDE 100 MG /25 MG FILM-COATED TABLETS",
            "L",
            "L, L-TEST",
            "L, L-TEST, LOSARTAN POTASSIUM / HYDROCHLOROTHIAZIDE 100 MG /25 MG FILM-COATED TABLETS",
            "L, LOSARTAN POTASSIUM",
            "L, LOSARTAN POTASSIUM, LOSARTAN POTASSIUM / HYDROCHLOROTHIAZIDE 100 MG /25 MG FILM-COATED TABLETS",
        ];
        assert_eq!(
            create_facets_by_active_substance(
                VecSanitisedString::from(products),
                VecSanitisedString::from(active_substances)
            ),
            expected
        );
    }

    #[test]
    fn test_create_facets_by_active_substance_sanitises() {
        let active_substances = vec!["CAFÉ".to_string(), "FÊTE".to_string(), "NAÏVE".to_string()];
        let products = vec!["MOTÖRHEAD".to_owned()];
        let expected = vec![
            "C",
            "C, CAF",
            "C, CAF, MOTRHEAD",
            "F",
            "F, FTE",
            "F, FTE, MOTRHEAD",
            "N",
            "N, NAVE",
            "N, NAVE, MOTRHEAD",
        ];
        assert_eq!(
            create_facets_by_active_substance(
                VecSanitisedString::from(products),
                VecSanitisedString::from(active_substances)
            ),
            expected
        );
    }

    #[test_case("PL 12345/1234", "[\"PL123451234\"]")]
    #[test_case("PL12345/1234", "[\"PL123451234\"]")]
    #[test_case("PLGB 12345/1234", "[\"PLGB123451234\"]")]
    #[test_case("PLNI 12345/1234", "[\"PLNI123451234\"]")]
    #[test_case("THR 12345/1234", "[\"THR123451234\"]")]
    #[test_case("NR 12345/1234", "[\"NR123451234\"]")]
    #[test_case("NEW 12345/1234", "[\"NEW123451234\"]")]
    #[test_case("12345/1234", "[]")]
    #[test_case("NO PL", "[]")]
    fn format_product_licence_test(input: &str, output: &str) {
        assert_eq!(format_product_licence(input), output);
    }

    #[test]
    fn parses_blob_metadata_from_document() {
        let document = Document {
            id: "con12345".to_string(),
            name: "Some SPC".to_string(),
            document_type: DocumentType::Spc,
            author: "test".to_string(),
            products: vec![
                "Generic Paracetamol".to_string(),
                "Special Paracetamol".to_string(),
            ],
            keywords: None,
            pl_number: "PL 12345/0010-0001".to_string(),
            territory: Some(TerritoryType::UK),
            active_substances: vec!["paracetamol".to_string()],
            file_source: FileSource::Sentinel,
            file_path: "/home/sentinel/something.pdf".to_string(),
        };

        let result: BlobMetadata = document.into();

        assert_eq!(
            result,
            BlobMetadata {
                file_name: SanitisedString::from("con12345".to_string()),
                doc_type: DocumentType::Spc,
                title: SanitisedString::from("Some SPC".to_string()),
                pl_number: "[\"PL123450010\"]".to_string(),
                territory: Some(TerritoryType::UK),
                product_names: VecSanitisedString::from(vec![
                    "GENERIC PARACETAMOL".to_string(),
                    "SPECIAL PARACETAMOL".to_string()
                ]),
                active_substances: VecSanitisedString::from(vec!["PARACETAMOL".to_string()]),
                author: SanitisedString::from("test".to_string()),
                keywords: None,
            }
        )
    }
}
