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

#[derive(Serialize, Deserialize)]
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
    Sentinel,
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
