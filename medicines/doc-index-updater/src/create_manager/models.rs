use super::sanitiser::{SanitisedString, VecSanitisedString};
use crate::{
    create_manager::Blob,
    models::{Document, DocumentType},
};
use chrono::{SecondsFormat, Utc};
use mhra_products_search_client_temp::models::IndexEntry;
use regex::Regex;
use std::{collections::HashMap, str};

#[derive(Clone, Debug, PartialEq)]
pub struct BlobMetadata {
    file_name: SanitisedString,
    doc_type: DocumentType,
    title: SanitisedString,
    pl_number: String,
    product_names: VecSanitisedString,
    active_substances: VecSanitisedString,
    author: SanitisedString,
    keywords: Option<VecSanitisedString>,
}

impl BlobMetadata {
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
        let pl_number = extract_product_licences(&self.pl_number);

        BlobMetadata {
            file_name: SanitisedString::from(&self.id),
            doc_type: self.document_type,
            title,
            pl_number,
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
            file_name: blob.metadata.file_name.to_string(),
            doc_type: blob.metadata.doc_type.to_string(),
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

pub fn extract_product_licences(input: &str) -> String {
    lazy_static! {
        static ref RE_WHITESPACE: Regex = Regex::new(r"(\s+|/|_|-)").expect("cannot compile regex");
        static ref RE_PL: Regex = Regex::new(r"(?i:\b|PL)(\s+|/|_|-)*\d{5}(\s+|/|_|-)*\d{4}")
            .expect("cannot compile regex");
    }
    let product_licences: Vec<String> = RE_PL
        .find_iter(input)
        .map(|m| {
            RE_WHITESPACE
                .replace_all(m.as_str(), "")
                .to_ascii_uppercase()
        })
        .map(|s| {
            if s.starts_with("PL") {
                s
            } else {
                String::from("PL") + s.as_str()
            }
        })
        .collect();

    to_json(product_licences)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::{DocumentType, FileSource};

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

        let output_metadata: HashMap<String, String> = Into::<BlobMetadata>::into(doc).into();

        assert_eq!(output_metadata["file_name"], expected_file_name);
        assert_eq!(output_metadata["doc_type"], expected_doc_type);
        assert_eq!(output_metadata["title"], expected_title);
        assert_eq!(output_metadata["author"], expected_author);
        assert_eq!(output_metadata["product_name"], expected_product_name);
        assert_eq!(output_metadata["substance_name"], expected_substance_name);
        assert_eq!(output_metadata["keywords"], expected_keywords);
        assert_eq!(output_metadata["pl_number"], expected_pl_number);
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

    #[test]
    fn extract_product_license_test() {
        let input = vec![
            "00 PL123451234",
            "01 pl123451234",
            "02 123451234",
            "03 PL 12345 1234",
            "04 PL  12345 1234",
            "05 test pl 12345   1234",
            "06 pl  12345   1234 test",
            "07 12345 1234",
            "08 PL 12345/1234",
            "09 PL/12345/1234",
            "10 pl 12345/1234",
            "11 pl/12345/1234",
            "12 12345/1234",
            "13 PL 12345_1234",
            "14 PL_12345_1234",
            "15 pl 12345_1234",
            "16 pl_12345_1234",
            "17 12345_1234",
            "18 PL 12345-1234",
            "19 PL-12345-1234",
            "20 pl 12345-1234",
            "21 pl-12345-1234",
            "22 12345-1234",
            "23 12345-1234GG",
            "PL 12345/1234-0001",
            "leaflet MAH GENERIC_PL 12345-1234R.pdf",
        ];
        let output = "[\"PL123451234\"]";
        input
            .iter()
            .for_each(|i| assert_eq!(extract_product_licences(i), output));
    }
    #[test]
    fn extract_multiple_product_licences() {
        let input = "00 PL123451234 01 pl123451235__ 02 123451236-03 PL 12345 1237";
        let output = "[\"PL123451234\",\"PL123451235\",\"PL123451236\",\"PL123451237\"]";

        assert_eq!(extract_product_licences(input), output);
    }
    #[test]
    fn extract_product_license_test_not_found() {
        assert_eq!(extract_product_licences("no pl number here"), "[]");
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
