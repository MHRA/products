use crate::models::{Document, DocumentType};

use lazy_static;
use regex::Regex;
use std::{collections::HashMap, str};

#[derive(Clone)]
pub struct BlobMetadata {
    file_name: String,
    doc_type: DocumentType,
    title: String,
    pl_number: String,
    product_names: Vec<String>,
    active_substances: Vec<String>,
    author: String,
    keywords: Option<Vec<String>>,
}

impl BlobMetadata {
    fn facets(&self) -> String {
        to_json(create_facets_by_active_substance(
            self.product_names.join(", "),
            self.active_substances.clone(),
        ))
    }

    pub fn product_names_json(&self) -> String {
        to_json(self.product_names.clone())
    }
}

pub fn derive_metadata_from_message(document: Document) -> BlobMetadata {
    let title = sanitize(&document.name);
    let pl_number = extract_product_licences(&title);

    BlobMetadata {
        file_name: sanitize(&document.id),
        doc_type: document.document_type,
        title,
        pl_number,
        product_names: document.products,
        active_substances: document.active_substances,
        author: sanitize(&document.author),
        keywords: document.keywords,
    }
}

impl Into<HashMap<String, String>> for BlobMetadata {
    fn into(self) -> HashMap<String, String> {
        let mut metadata: HashMap<String, String> = HashMap::new();

        metadata.insert("file_name".to_string(), self.file_name.clone());
        metadata.insert("doc_type".to_string(), self.doc_type.clone().to_string());
        metadata.insert("title".to_string(), self.title.clone());
        metadata.insert("pl_number".to_string(), self.pl_number.clone());
        metadata.insert(
            "product_name".to_string(),
            to_json(self.product_names.clone()),
        );
        metadata.insert(
            "substance_name".to_string(),
            to_json(self.active_substances.clone()),
        );
        metadata.insert("author".to_string(), self.author.clone());
        metadata.insert("facets".to_string(), self.facets());
        if let Some(keywords) = self.keywords.clone() {
            metadata.insert("keywords".to_string(), keywords.join(" "));
        }

        metadata
    }
}

pub fn sanitize(s: &str) -> String {
    s.replace(|c: char| !c.is_ascii(), "")
        .replace("\n", " ")
        .trim()
        .to_string()
}

pub fn to_json(words: Vec<String>) -> String {
    serde_json::to_string(&words).expect("Couldn't create JSON array.")
}

pub fn create_facets_by_active_substance(
    product: String,
    active_substances: Vec<String>,
) -> Vec<String> {
    let mut facets: Vec<String> = active_substances
        .iter()
        .map(|a| {
            let first = a.chars().next().unwrap();
            vec![
                first.to_string(),
                [first.to_string(), a.to_string()].join(", "),
                [first.to_string(), a.to_string(), product.to_string()].join(", "),
            ]
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
        let expected_product_name = "[\"Effective product 1\",\"Effective product 2\"]".to_string();
        let expected_substance_name = "[\"Paracetamol\",\"Caffeine\"]".to_string();
        let expected_keywords = "Very good for you Cures headaches PL 12345/6789".to_string();
        let expected_pl_number = "[\"PL123456789\"]".to_string();

        let output_metadata: HashMap<String, String> = derive_metadata_from_message(doc).into();

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
    fn sanitize_remove_newline() {
        assert_eq!(sanitize("newline\ntest"), "newline test");
    }
    #[test]
    fn sanitize_remove_non_ascii() {
        assert_eq!(sanitize("emoji🙂 ∫test"), "emoji test");
    }
    #[test]
    fn sanitize_trim() {
        assert_eq!(sanitize(" test "), "test");
    }
    #[test]
    fn test_create_facets_by_active_substance() {
        let active_substances = vec![
            "LOSARTAN POTASSIUM".to_string(),
            "HYDROCHLOROTHIAZIDE".to_string(),
            "L-TEST".to_string(),
        ];
        let product =
            "LOSARTAN POTASSIUM / HYDROCHLOROTHIAZIDE 100 MG /25 MG FILM-COATED TABLETS".to_owned();
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
            create_facets_by_active_substance(product, active_substances),
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
}
