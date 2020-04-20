use super::get_ok;
use doc_index_updater::models::{Document, DocumentType, FileSource, JobStatus, JobStatusResponse};
use reqwest::Error;
use uuid::Uuid;

pub fn delete_document(document_id: String) -> Result<JobStatusResponse, Error> {
    let client = reqwest::Client::new();

    let response = get_ok(
        client
            .delete(format!("http://localhost:8000/documents/{}", document_id).as_str())
            .basic_auth("username".to_string(), Some("password".to_string()))
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
            .post(format!("http://localhost:8000/documents").as_str())
            .basic_auth("username".to_string(), Some("password".to_string()))
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
            .basic_auth("username".to_string(), Some("password".to_string()))
            .send(),
    );
    let job_status_response: JobStatusResponse = get_ok(response.json());
    job_status_response.status
}
