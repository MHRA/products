use core::fmt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum JobStatus {
    Accepted,
    Done,
    NotFound,
    Error { message: String, code: String },
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let a = match self {
            JobStatus::Accepted => "Accepted".to_string(),
            JobStatus::Done => "Done".to_string(),
            JobStatus::NotFound => "NotFound".to_string(),
            JobStatus::Error { message, code } => format!("Error({}: {})", code, message),
        };
        write!(f, "{}", a)
    }
}

impl FromStr for JobStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<JobStatus, Self::Err> {
        match s {
            "Accepted" => Ok(JobStatus::Accepted),
            "Done" => Ok(JobStatus::Done),
            "Error" => Ok(JobStatus::Error {
                message: "Error status".to_owned(),
                code: "0x0".to_owned(),
            }),
            e => Err(format!("Status unknown: {}", e)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobStatusResponse {
    pub id: Uuid,
    pub status: JobStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Document {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub document_type: DocumentType,
    pub author: String,
    pub products: Vec<String>,
    pub keywords: Option<Vec<String>>,
    pub pl_number: String,
    pub active_substances: Vec<String>,
    pub file_source: FileSource,
    pub file_path: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FileSource {
    #[serde(alias = "sentinel")]
    Sentinel,
}

impl Document {
    pub fn from(doc: XMLDocument) -> Self {
        Self {
            id: doc.id,
            name: doc.name,
            document_type: doc.document_type,
            author: doc.author,
            products: doc
                .products
                .iter()
                .map(move |active_substance| active_substance.name.clone())
                .collect::<Vec<String>>(),
            keywords: match doc.keywords {
                Some(kw) => Some(
                    kw.iter()
                        .map(move |keyword| keyword.name.clone())
                        .collect::<Vec<String>>(),
                ),
                None => None,
            },
            pl_number: doc.pl_number,
            active_substances: doc
                .active_substances
                .iter()
                .map(move |active_substance| active_substance.name.clone())
                .collect::<Vec<String>>(),
            file_source: doc.file_source,
            file_path: doc.file_path,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    #[serde(default)]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keyword {
    #[serde(default)]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveSubstance {
    #[serde(default)]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XMLDocument {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub document_type: DocumentType,
    pub author: String,
    pub products: Vec<Product>,
    pub keywords: Option<Vec<Keyword>>,
    pub pl_number: String,
    pub active_substances: Vec<ActiveSubstance>,
    pub file_source: FileSource,
    pub file_path: String,
}

#[derive(Serialize)]
#[serde(rename = "job")]
pub struct XMLJobStatusResponse {
    id: String,
    status: String,
}

impl Into<XMLJobStatusResponse> for JobStatusResponse {
    fn into(self) -> XMLJobStatusResponse {
        XMLJobStatusResponse {
            id: self.id.to_string(),
            status: self.status.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DocumentType {
    #[serde(rename = "SPC")]
    Spc,
    #[serde(rename = "PIL")]
    Pil,
    #[serde(rename = "PAR")]
    Par,
}

impl fmt::Display for DocumentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                DocumentType::Spc => "Spc",
                DocumentType::Pil => "Pil",
                DocumentType::Par => "Par",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CreateMessage {
    pub job_id: Uuid,
    pub document: Document,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DeleteMessage {
    pub job_id: Uuid,
    pub document_content_id: String,
}

pub trait Message: Sized + FromStr {
    fn to_json_string(&self) -> Result<String, serde_json::Error>;
}

impl FromStr for CreateMessage {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_slice::<CreateMessage>(s.as_bytes())?)
    }
}

impl FromStr for DeleteMessage {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_slice::<DeleteMessage>(s.as_bytes())?)
    }
}

impl Message for CreateMessage {
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_string(&self)?)
    }
}

impl Message for DeleteMessage {
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("Accepted", Ok(JobStatus::Accepted))]
    #[test_case("Done", Ok(JobStatus::Done))]
    #[test_case("Error", Ok(JobStatus::Error {message:"Error status".to_owned(), code:"0x0".to_owned()}))]
    #[test_case("Bedro", Err("Status unknown: Bedro".to_owned()))]
    fn test_parse_job_status(input: &str, output: Result<JobStatus, String>) {
        assert_eq!(input.parse::<JobStatus>(), output);
    }
}
