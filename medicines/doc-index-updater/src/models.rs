use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum JobStatus {
    Accepted,
    Done,
    NotFound,
    Error { message: String, code: String },
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    id: String,
    name: String,
    #[serde(rename = "type")]
    document_type: DocumentType,
    author: String,
    products: Vec<String>,
    keywords: Option<Vec<String>>,
    pl_number: String,
    active_substances: Vec<String>,
    file_source: String,
    file_path: String,
}

#[derive(Serialize, Debug, Clone)]
pub enum DocumentType {
    SPC,
    PIL,
    PAR,
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("Accepted", Ok(JobStatus::Accepted))]
    #[test_case("Done", Ok(JobStatus::Done))]
    #[test_case("Error", Ok(JobStatus::Error {message:"Error status".to_owned(), code:"0x0".to_owned()}))]
    #[test_case("Bedro", Err("Status unknown: Bedro".to_owned()))]
    fn test_parse(input: &str, output: Result<JobStatus, String>) {
        assert_eq!(input.parse::<JobStatus>(), output);
    }
}
