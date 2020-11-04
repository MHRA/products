extern crate doc_index_updater;

mod support;
use chrono::Duration;
use doc_index_updater::{
    models::{DeleteMessage, JobStatus},
    service_bus_client::{delete_factory, Removable},
};

use support::{
    document_api::{create_document, delete_document, get_job_status},
    get_message_safely, get_ok, get_test_delete_message, repeatedly_check_until_result_is,
};
use tokio_test::block_on;
use uuid::Uuid;

#[test]
#[ignore]
fn delete_queue_works() {
    let id = Uuid::new_v4();
    let sent_message = get_test_delete_message(id, format!("doc-{}", id));
    let mut queue = get_ok(delete_factory());
    get_ok(queue.send(sent_message.clone(), Duration::seconds(1)));

    let mut retrieval = block_on(get_message_safely::<DeleteMessage>(&mut queue));
    while retrieval.message != sent_message {
        retrieval = block_on(get_message_safely::<DeleteMessage>(&mut queue));
    }

    assert_eq!(retrieval.message, sent_message);

    let queue_removal_response = block_on(retrieval.remove());
    assert!(queue_removal_response.is_ok());
    assert_eq!(queue_removal_response.unwrap(), "");
}

#[test]
#[ignore]
/*
    For this test to work, follow the README to setup your local environment as it needs to connect to you local sftp server with pub/priv key method.
    Also, place a file called example.txt in the root of you sftp user folder.
*/
fn document_not_found_error_sets_error_state() {
    let document_id = Uuid::new_v4();
    let job_status_response = delete_document(document_id.to_string()).unwrap();

    let job_id = job_status_response.id;

    repeatedly_check_until_result_is(
        JobStatus::Error {
            message: format!("Cannot find document with ID {}", document_id.to_string()),
            code: "".to_string(),
        },
        || get_job_status(job_id),
        10,
    );
}

#[test]
#[ignore]
/*
    For this test to work, follow the README to setup your local environment as it needs to connect to you local sftp server with pub/priv key method.
    Also, place a file called example.txt in the root of you sftp user folder.
*/
fn delete_created_document_succeeds() {
    let document_id = Uuid::new_v4();

    let create_response = create_document(document_id.to_string(), "example.txt".into()).unwrap();

    repeatedly_check_until_result_is(JobStatus::Done, || get_job_status(create_response.id), 10);

    let delete_response = delete_document(document_id.to_string()).unwrap();

    repeatedly_check_until_result_is(JobStatus::Done, || get_job_status(delete_response.id), 10);
}
