use crate::{
    models::{Document, JobStatus},
    state_manager::{with_state, StateManager},
};
use uuid::Uuid;
use warp::{reply::Json, Filter, Rejection, Reply};

async fn del_document_handler(
    _document_content_id: String,
    state_manager: StateManager,
) -> Result<Json, Rejection> {
    let id = Uuid::new_v4();

    Ok(warp::reply::json(
        &state_manager.set_status(id, JobStatus::Accepted).await?,
    ))
}

async fn check_in_document_handler(
    _doc: Document,
    state_manager: StateManager,
) -> Result<Json, Rejection> {
    let id = Uuid::new_v4();

    Ok(warp::reply::json(
        &state_manager.set_status(id, JobStatus::Accepted).await?,
    ))
}

pub fn del_document(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("documents" / String)
        .and(warp::delete())
        .and(with_state(state_manager))
        .and_then(del_document_handler)
}

pub fn check_in_document(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("documents")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state_manager))
        .and_then(check_in_document_handler)
}
