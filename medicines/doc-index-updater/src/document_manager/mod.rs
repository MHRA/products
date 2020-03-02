use crate::{
    models::{Document, JobStatus},
    state_manager::{with_state, StateManager},
};
use azure_sdk_service_bus::prelude::*;
use std::time::duration::Duration;
use uuid::Uuid;
use warp::{reply::Json, Filter, Rejection, Reply};

mod service_bus;

struct CreateMessage {
    job_id: Uuid,
    document: Document,
}

struct DeleteMessage {
    job_id: Uuid,
    document_content_id: String,
}

async fn del_document_handler(
    document_content_id: String,
    state_manager: StateManager,
    client: Client,
) -> Result<Json, Rejection> {
    let id = Uuid::new_v4();
    let message = DeleteMessage {
        job_id: id,
        document_content_id,
    };

    match client.send_event(message, Duration::days(1)) {
        Ok(_) => Ok(warp::reply::json(
            &state_manager.set_status(id, JobStatus::Accepted).await?,
        )),
        // TODO: Handle errors
        Err(_) => Ok(warp::reply::json(
            &state_manager.set_status(id, JobStatus::Accepted).await?,
        )),
    }
}

async fn check_in_document_handler(
    doc: Document,
    state_manager: StateManager,
    client: Client,
) -> Result<Json, Rejection> {
    let id = Uuid::new_v4();
    let message = CreateMessage {
        job_id: id,
        document: doc,
    };

    match client.send_event(message, Duration::days(1) {
        Ok(_) => Ok(warp::reply::json(
            &state_manager.set_status(id, JobStatus::Accepted).await?,
        )),
        // TODO: Handle errors
        Err(_) => Ok(warp::reply::json(
            &state_manager.set_status(id, JobStatus::Accepted).await?,
        )),
    }
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
