extern crate doc_index_updater;

mod support;
use crate::support::get_test_delete_message;
use doc_index_updater::{
    document_manager,
    models::{CreateMessage, DeleteMessage, JobStatus, JobStatusResponse},
    service_bus_client::{create_factory, delete_factory},
    state_manager,
};
use support::{get_ok, get_test_create_message, get_test_document, TestContext};
use test_case::test_case;
use tokio_test::block_on;
use uuid::Uuid;

#[test_case(JobStatus::Done)]
#[test_case(JobStatus::Accepted)]
fn set_get_compatibility_on_state_manager(status: JobStatus) {
    let ctx = TestContext::default();

    let state = state_manager::StateManager::new(ctx.client);
    let id = Uuid::new_v4();

    let response = get_ok(state.set_status(id, status.clone()));

    assert_eq!(response.status, status.clone());

    let response = get_ok(state.get_status(id));
    assert_eq!(response.status, status);
}

#[test_case(JobStatus::Done)]
#[test_case(JobStatus::Accepted)]
fn set_get_on_state_manager_endpoints(status: JobStatus) {
    let ctx = TestContext::default();

    let state = state_manager::StateManager::new(ctx.client);
    let id = Uuid::new_v4();

    let r = block_on(
        warp::test::request()
            .method("POST")
            .path(&format!("/jobs/{}/{}", id, status))
            .reply(&state_manager::set_job_status(state.clone())),
    );

    let response: JobStatusResponse = serde_json::from_slice(r.body()).unwrap();

    assert_eq!(response.status, status.clone());

    let r = block_on(
        warp::test::request()
            .method("GET")
            .path(&format!("/jobs/{}", id))
            .reply(&state_manager::get_job_status(state)),
    );

    let response: JobStatusResponse = serde_json::from_slice(r.body()).unwrap();
    assert_eq!(response.status, status);
}

#[ignore]
#[test]
fn delete_endpoint_sets_state() {
    let ctx = TestContext::default();

    let state = state_manager::StateManager::new(ctx.client);
    let delete_filter = document_manager::delete_document(state.clone());

    let r = block_on(
        warp::test::request()
            .method("DELETE")
            .path("/documents/hello-string")
            .reply(&delete_filter),
    );

    let response: JobStatusResponse = serde_json::from_slice(r.body()).unwrap();
    assert_eq!(response.status, JobStatus::Accepted);
    let id = response.id;
    let response = get_ok(state.get_status(id));
    assert_eq!(response.status, JobStatus::Accepted);

    let mut delete_client = get_ok(delete_factory());

    let mut received_message = get_ok(delete_client.receive::<DeleteMessage>()).message;
    let expected = get_test_delete_message(id, "hello-string".to_string());

    loop {
        if received_message.job_id == id {
            assert_eq!(received_message, expected);
            return;
        }
        received_message = get_ok(delete_client.receive::<DeleteMessage>()).message;
    }
}

#[ignore]
#[test]
fn create_endpoint_sets_state() {
    let ctx = TestContext::default();

    let state = state_manager::StateManager::new(ctx.client);
    let create_filter = document_manager::check_in_document(state.clone());

    let document_json = serde_json::to_string(&get_test_document()).unwrap();

    let r = block_on(
        warp::test::request()
            .method("POST")
            .body(document_json)
            .path("/documents")
            .reply(&create_filter),
    );

    let response: JobStatusResponse = serde_json::from_slice(r.body()).unwrap();
    assert_eq!(response.status, JobStatus::Accepted);
    let id = response.id;
    let response = get_ok(state.get_status(id.clone()));
    assert_eq!(response.status, JobStatus::Accepted);

    let mut create_client = get_ok(create_factory());

    let mut received_message = get_ok(create_client.receive::<CreateMessage>()).message;
    let expected = get_test_create_message(id);

    loop {
        if received_message.job_id == id {
            assert_eq!(received_message, expected);
            return;
        }
        received_message = get_ok(create_client.receive::<CreateMessage>()).message;
    }
}
