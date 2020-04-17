extern crate doc_index_updater;

mod support;
use doc_index_updater::{
    models::{DeleteMessage, Document, DocumentType, FileSource, JobStatus, JobStatusResponse},
    service_bus_client::{delete_factory, Removeable},
};
use reqwest::Error;

use support::{get_message_safely, get_ok, get_test_delete_message};
use tokio_test::block_on;
use uuid::Uuid;

#[test]
#[ignore]
fn delete_queue_works() {
    let id = Uuid::new_v4();
    let sent_message = get_test_delete_message(id, format!("doc-{}", id));
    let mut queue = get_ok(delete_factory());
    get_ok(queue.send(sent_message.clone(), time::Duration::seconds(1)));

    let mut retrieval = block_on(get_message_safely::<DeleteMessage>(&mut queue));
    while retrieval.message != sent_message {
        retrieval = block_on(get_message_safely::<DeleteMessage>(&mut queue));
    }

    assert_eq!(retrieval.message, sent_message);

    let queue_removal_response = block_on(retrieval.remove());
    assert!(queue_removal_response.is_ok());
    assert_eq!(queue_removal_response.unwrap(), "");
}

fn delete_document(document_id: String) -> Result<JobStatusResponse, Error> {
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

fn create_document(document_id: String, file_path: String) -> Result<JobStatusResponse, Error> {
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

    println!("{:?}", metadata);

    let response = get_ok(
        client
            .post(format!("http://localhost:8000/documents").as_str())
            .basic_auth("username".to_string(), Some("password".to_string()))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&metadata).unwrap())
            .send(),
    );

    println!("{:?}", response);
    let job_status_response: JobStatusResponse = get_ok(response.json());
    println!("{:?}", job_status_response);
    Ok(job_status_response)
}

fn get_job_status(job_id: Uuid) -> JobStatus {
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

#[test]
fn document_not_found_error_sets_error_state() {
    let document_id = Uuid::new_v4();
    let job_status_response = delete_document(document_id.to_string()).unwrap();

    let job_id = job_status_response.id;
    println!("sleeping 5 seconds");
    std::thread::sleep(std::time::Duration::from_secs(5));
    let status = get_job_status(job_id);
    assert_eq!(
        status,
        JobStatus::Error {
            message: format!("Cannot find document with ID {}", document_id.to_string()),
            code: "".to_string()
        }
    );
}

#[test]
fn delete_created_document_succeeds() {
    let document_id = Uuid::new_v4();

    let create_response = create_document(document_id.to_string(), "example.txt".into()).unwrap();

    let mut i = 0;
    loop {
        let status = get_job_status(create_response.id);

        if status == JobStatus::Done {
            break;
        } else if i > 10 {
            panic!("Create job status is not Done after 10 seconds: {}", status);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        i += 1;
    }

    let job_status_response = delete_document(document_id.to_string()).unwrap();

    let job_id = job_status_response.id;

    let mut i = 0;
    loop {
        let status = get_job_status(job_id);

        if status == JobStatus::Done {
            break;
        } else if i > 10 {
            panic!("Delete job status is not Done after 10 seconds: {}", status);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        i += 1;
    }
}
