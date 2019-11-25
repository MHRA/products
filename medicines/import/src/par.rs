use crate::{model, storage};
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use chrono::{DateTime, Utc};
use csv;
use std::{
    collections::HashMap,
    fs,
    fs::{DirEntry, File},
    io::BufReader,
    path::Path,
    str,
};
use tantivy::tokenizer::*;
use tokio_core::reactor::Core;

#[derive(Debug, Deserialize, Clone)]
struct Record {
    #[serde(rename = "dDocName")]
    filename: String,
    #[serde(rename = "dDocTitle")]
    title: String,
    #[serde(rename = "dDocAuthor")]
    author: String,
    #[serde(rename = "dCreateDate", with = "crate::date_de")]
    created: DateTime<Utc>,
    #[serde(rename = "dReleaseState")]
    release_state: String,
    #[serde(rename = "xKeywords")]
    keywords: String,
}

pub fn import(dir: &Path, client: Client, mut core: Core) -> Result<(), AzureError> {
    if dir.is_dir() {
        if let Some(Ok(f)) = fs::read_dir(dir)?.find(|f| is_csv(f.as_ref().unwrap())) {
            println!("Found CSV file: {:?}", f);
            let file = File::open(&f.path())?;
            let mut rdr = csv::Reader::from_reader(BufReader::new(file));
            let records = rdr
                .deserialize()
                .map(|r: Result<Record, csv::Error>| {
                    let r = r.expect("Failed to deserialize");
                    (r.filename.clone().to_lowercase(), r)
                })
                .collect::<HashMap<String, Record>>();

            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "pdf" && fs::metadata(&path)?.len() > 0 {
                        let key = &path.file_stem().unwrap().to_str().unwrap();
                        if let Some(record) = records.get(&key.to_lowercase()) {
                            let mut metadata: HashMap<&str, &str> = HashMap::new();
                            let file_name = sanitize(&record.filename);
                            metadata.insert("file_name", &file_name);
                            let doc_type = format!("{:?}", model::DocType::Par);
                            metadata.insert("doc_type", &doc_type);
                            let title = sanitize(&record.title);
                            metadata.insert("title", &title);
                            let keywords = tokenize(&record.keywords);
                            metadata.insert("keywords", &keywords);
                            let created = record.created.to_rfc3339();
                            metadata.insert("created", &created);
                            let author = sanitize(&record.author);
                            metadata.insert("author", &author);
                            let release_state = sanitize(&record.release_state);
                            metadata.insert("release_state", &release_state);
                            storage::upload(&client, &mut core, &fs::read(path)?, &metadata)?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn is_csv(f: &DirEntry) -> bool {
    "csv" == f.path().extension().unwrap_or_default()
}

fn sanitize(s: &str) -> String {
    s.replace(|c: char| !c.is_ascii(), "")
        .replace("\n", " ")
        .trim()
        .to_string()
}

fn tokenize(s: &str) -> String {
    let s1 = s.replace(|c: char| !c.is_ascii(), "");
    let en_stem = SimpleTokenizer
        .filter(RemoveLongFilter::limit(40))
        .filter(LowerCaser)
        .filter(StopWordFilter::default());
    let mut tokens: Vec<Token> = vec![];
    {
        let mut add_token = |token: &Token| {
            tokens.push(token.clone());
        };
        en_stem.token_stream(&s1).process(&mut add_token);
    }
    tokens
        .iter()
        .map(|t| t.text.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn tokenize_remove_newline() {
        assert_eq!(tokenize("newline\ntest"), "newline test");
    }
    #[test]
    fn tokenize_remove_unicode() {
        assert_eq!(tokenize("emoji🙂 ∫test"), "emoji test");
    }
    #[test]
    fn tokenize_sample_keywords1() {
        let s1 = "ukpar, public assessment report, par, national procedure,Ibuprofen, Phenylephrine Hydrochloride, Ibuprofen and Phenylephrine Hydrochloride 200 mg/6.1 mg Tablets, 200 mg, 6.1 mg, cold, flu, congestion, aches, pains, headache, fever, sore throat, blocked nose, sinuses";
        let s2 = "ukpar public assessment report par national procedure ibuprofen phenylephrine hydrochloride ibuprofen phenylephrine hydrochloride 200 mg 6 1 mg tablets 200 mg 6 1 mg cold flu congestion aches pains headache fever sore throat blocked nose sinuses";
        assert_eq!(tokenize(s1), s2);
    }
}
