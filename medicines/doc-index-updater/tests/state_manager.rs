extern crate doc_index_updater;

use doc_index_updater::{
    models::JobStatus,
    state_manager::{JobStatusClient, StateManager},
};
use support::{get_ok, TestContext};
use test_case::test_case;
use uuid::Uuid;

mod support;

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

#[test_case(JobStatus::Accepted)]
#[test_case(JobStatus::Done)]
#[test_case(JobStatus::Error{ message: "Bad error".into(), code: "".into() })]
fn sets_job_status_if_it_doesnt_exist(status: JobStatus) {
    let ctx = TestContext::default();

    let state = StateManager::new(ctx.client);
    let id = Uuid::new_v4();

    let response = get_ok(state.set_status(id, status.clone()));

    assert_eq!(response.status, status);
}

#[test_case(JobStatus::Accepted)]
#[test_case(JobStatus::Done)]
#[test_case(JobStatus::Error{ message: "Bad error".into(), code: "".into() })]
fn does_not_overwrite_job_status_if_already_marked_as_done(new_status: JobStatus) {
    let ctx = TestContext::default();

    let state = StateManager::new(ctx.client);
    let id = Uuid::new_v4();

    let original_status = JobStatus::Done;

    get_ok(state.set_status(id, original_status.clone()));

    let response = get_ok(state.set_status(id, new_status));

    assert_eq!(response.status, original_status);
}

#[test_case(JobStatus::Accepted)]
#[test_case(JobStatus::Done)]
#[test_case(JobStatus::Error{ message: "Bad error".into(), code: "".into() })]
fn does_not_overwrite_job_status_if_job_has_errored(new_status: JobStatus) {
    let ctx = TestContext::default();

    let state = StateManager::new(ctx.client);
    let id = Uuid::new_v4();

    let original_status = JobStatus::Error {
        message: "The original error".into(),
        code: "11".into(),
    };

    get_ok(state.set_status(id, original_status.clone()));

    let response = get_ok(state.set_status(id, new_status));

    assert_eq!(response.status, original_status);
}
