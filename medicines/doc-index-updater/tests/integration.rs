extern crate doc_index_updater;

mod support;
use doc_index_updater::{
    document_manager,
    models::{Document, DocumentType, JobStatus, JobStatusResponse},
    state_manager,
};
use support::TestContext;
use test_case::test_case;
use tokio_test::block_on;
use uuid::Uuid;

#[test_case(JobStatus::Done)]
#[test_case(JobStatus::Accepted)]
fn set_get_compatibility_on_state_manager(status: JobStatus) {
    let ctx = TestContext::default();

    let state = state_manager::StateManager::new(ctx.client);
    let id = Uuid::new_v4();

    let response = block_on(state.set_status(id, status.clone())).unwrap();

    assert_eq!(response.status, status.clone());

    let response = block_on(state.get_status(id)).unwrap();
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

#[test]
fn delete_endpoint_sets_state() {
    let ctx = TestContext::default();

    let state = state_manager::StateManager::new(ctx.client);
    let delete_filter = document_manager::del_document(state.clone());

    let r = block_on(
        warp::test::request()
            .method("DELETE")
            .path("/documents/hello-string")
            .reply(&delete_filter),
    );

    let response: JobStatusResponse = serde_json::from_slice(r.body()).unwrap();
    assert_eq!(response.status, JobStatus::Accepted);
    let id = response.id;
    let response = block_on(state.get_status(id)).unwrap();
    assert_eq!(response.status, JobStatus::Accepted);
}

#[test]
fn create_endpoint_sets_state() {
    let ctx = TestContext::default();

    let state = state_manager::StateManager::new(ctx.client);
    let create_filter = document_manager::check_in_document(state.clone());

    let document_json = serde_json::to_string(&Document {
        id: "id".to_string(),
        name: "name".to_string(),
        document_type: DocumentType::Pil,
        author: "author".to_string(),
        products: vec!["products".to_string()],
        keywords: Some(vec!["keywords".to_string()]),
        pl_number: "pl_number".to_string(),
        active_substances: vec!["active_substances".to_string()],
        file_source: "file_source".to_string(),
        file_path: "file_path".to_string(),
    })
    .unwrap();

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
    let response = block_on(state.get_status(id)).unwrap();
    assert_eq!(response.status, JobStatus::Accepted);
}
