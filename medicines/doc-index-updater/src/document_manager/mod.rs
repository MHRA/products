use crate::{
    models::{Document, JobStatus},
    state_manager::{with_state, StateManager},
};
use azure_sdk_service_bus::prelude::*;
use uuid::Uuid;
use warp::{reply::Json, Filter, Rejection, Reply};

mod service_bus;

async fn del_document_handler(
    _document_content_id: String,
    state_manager: StateManager,
    client: Client,
) -> Result<Json, Rejection> {
    let id = Uuid::new_v4();

    Ok(warp::reply::json(
        &state_manager.set_status(id, JobStatus::Accepted).await?,
    ))
}

async fn check_in_document_handler(
    _doc: Document,
    state_manager: StateManager,
    client: Client,
) -> Result<Json, Rejection> {
    let id = Uuid::new_v4();

    Ok(warp::reply::json(
        &state_manager.set_status(id, JobStatus::Accepted).await?,
    ))
}

pub fn del_document(
    state_manager: StateManager,
    client: Client,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("documents" / String)
        .and(warp::delete())
        .and(with_state(state_manager))
        .and(with_client(client))
        .and_then(del_document_handler)
}

pub fn check_in_document(
    state_manager: StateManager,
    client: Client,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("documents")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state_manager))
        .and(with_client(client))
        .and_then(check_in_document_handler)
}

fn with_client(
    client: Client,
) -> impl Filter<Extract = (Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}
