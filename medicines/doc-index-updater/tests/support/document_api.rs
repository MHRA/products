use super::get_ok;
use reqwest::RequestBuilder;

use doc_index_updater::{
    get_env,
    models::{Document, DocumentType, FileSource, JobStatus, JobStatusResponse},
};
use reqwest::Error;
use uuid::Uuid;

fn get_basic_auth_credentials() -> (String, Option<String>) {
    let username = get_env("BASIC_AUTH_USERNAME").unwrap();
    let password = if let Ok(p) = get_env("BASIC_AUTH_PASSWORD") {
        Some(p)
    } else {
        None
    };

    (username, password)
}

trait WithBasicAuth {
    fn with_basic_auth(self, _: (String, Option<String>)) -> Self;
}

impl WithBasicAuth for RequestBuilder {
    fn with_basic_auth(self, (username, password): (String, Option<String>)) -> Self {
        self.basic_auth(username, password)
    }
}

pub fn delete_document(document_id: String) -> Result<JobStatusResponse, Error> {
    let client = reqwest::Client::new();

    let response = get_ok(
        client
            .delete(format!("http://localhost:8000/documents/{}", document_id).as_str())
            .with_basic_auth(get_basic_auth_credentials())
            .send(),
    );

    println!("{:?}", response);
    let job_status_response: JobStatusResponse = get_ok(response.json());
    println!("{:?}", job_status_response);
    Ok(job_status_response)
}

pub fn create_document(document_id: String, file_path: String) -> Result<JobStatusResponse, Error> {
    let client = reqwest::Client::new();
    let metadata = Document {
        id: document_id,
        name: "Star Wars Trilogy".to_string(),
        document_type: DocumentType::Pil,
        author: "author".to_string(),
        products: vec!["products".to_string()],
        pl_number: "pl_number".to_string(),
        active_substances: vec!["active_substances".to_string()],
        file_path,
        file_source: FileSource::Sentinel,
        keywords: Some(vec!["keyword".to_string()]),
    };

    let response = get_ok(
        client
            .post("http://localhost:8000/documents")
            .with_basic_auth(get_basic_auth_credentials())
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&metadata).unwrap())
            .send(),
    );

    let job_status_response: JobStatusResponse = get_ok(response.json());
    Ok(job_status_response)
}

pub fn get_job_status(job_id: Uuid) -> JobStatus {
    let client = reqwest::Client::new();

    let response = get_ok(
        client
            .get(format!("http://localhost:8000/jobs/{}", job_id).as_str())
            .with_basic_auth(get_basic_auth_credentials())
            .send(),
    );
    let job_status_response: JobStatusResponse = get_ok(response.json());
    job_status_response.status
}
