extern crate doc_index_updater;

mod support;
use doc_index_updater::{
    models::JobStatus,
    state_manager::{JobStatusClient, StateManager},
};
use support::{get_ok, TestContext};
use test_case::test_case;
use uuid::Uuid;

#[test_case(JobStatus::Done)]
#[test_case(JobStatus::Accepted)]
fn set_get_compatibility_on_state_manager(status: JobStatus) {
    let ctx = TestContext::default();

    let state = StateManager::new(ctx.client);
    let id = Uuid::new_v4();

    let response = get_ok(state.set_status(id, status.clone()));

    assert_eq!(response.status, status);

    let response = get_ok(state.get_status(id));
    assert_eq!(response.status, status);
}

#[test_case(JobStatus::Done)]
#[test_case(JobStatus::Error { message: "Bad error".into(), code: "".into() })]
fn set_job_status_to_accepted(original_status: JobStatus) {
    let ctx = TestContext::default();

    let state = StateManager::new(ctx.client);
    let id = Uuid::new_v4();

    get_ok(state.set_status(id, original_status.clone()));

    let response = get_ok(state.set_status(id, JobStatus::Accepted));

    assert_eq!(response.status, original_status);
}
