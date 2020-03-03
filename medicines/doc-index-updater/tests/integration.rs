extern crate doc_index_updater;

mod support;
use doc_index_updater::{
    document_manager,
    models::{JobStatus, JobStatusResponse},
    state_manager,
};
use support::TestContext;
use tokio_test::block_on;
use uuid::Uuid;

#[test]
fn set_get_compatibility_on_state_manager() {
    let ctx = TestContext::new();

    let state = state_manager::StateManager::new(ctx.client);
    let id = Uuid::new_v4();

    let response = block_on(state.set_status(id, JobStatus::Accepted)).unwrap();

    assert_eq!(response.status, JobStatus::Accepted);

    let response = block_on(state.get_status(id)).unwrap();
    assert_eq!(response.status, JobStatus::Accepted);
}

#[test]
fn delete_endpoint_sets() {
    let ctx = TestContext::new();

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
