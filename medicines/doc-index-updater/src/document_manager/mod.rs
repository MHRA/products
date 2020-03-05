use crate::{
    models::{Document, JobStatus},
    service_bus_client::{create_factory, delete_factory},
    state_manager::{with_state, StateManager},
};
use serde_derive::Serialize;
use time::Duration;
use uuid::Uuid;
use warp::{reply::Json, Filter, Rejection, Reply};

#[derive(Serialize)]
struct CreateMessage {
    job_id: Uuid,
    document: Document,
}

#[derive(Serialize)]
struct DeleteMessage {
    job_id: Uuid,
    document_content_id: String,
}

#[derive(Debug)]
struct FailedToDispatchToQueue;
#[derive(Debug)]
struct FailedToDeserialize;
impl warp::reject::Reject for FailedToDispatchToQueue {}
impl warp::reject::Reject for FailedToDeserialize {}

async fn del_document_handler(
    document_content_id: String,
    state_manager: StateManager,
) -> Result<Json, Rejection> {
    if let Ok(mut client) = delete_factory().await {
        let id = Uuid::new_v4();
        let message = DeleteMessage {
            job_id: id,
            document_content_id,
        };
        let duration = Duration::days(1);

        match serde_json::to_string(&message) {
            Ok(evt) => match client.send_event(evt.as_str(), duration).await {
                Ok(_) => Ok(warp::reply::json(
                    &state_manager.set_status(id, JobStatus::Accepted).await?,
                )),
                Err(_) => Err(warp::reject::custom(FailedToDispatchToQueue)),
            },
            Err(_) => Err(warp::reject::custom(FailedToDeserialize)),
        }
    } else {
        Err(warp::reject::custom(FailedToDispatchToQueue))
    }
}

async fn check_in_document_handler(
    doc: Document,
    state_manager: StateManager,
) -> Result<Json, Rejection> {
    if let Ok(mut client) = create_factory().await {
        let id = Uuid::new_v4();
        let message = CreateMessage {
            job_id: id,
            document: doc,
        };
        let duration = Duration::days(1);
        match serde_json::to_string(&message) {
            Ok(evt) => match client.send_event(evt.as_str(), duration).await {
                Ok(_) => Ok(warp::reply::json(
                    &state_manager.set_status(id, JobStatus::Accepted).await?,
                )),
                Err(_) => Err(warp::reject::custom(FailedToDispatchToQueue)),
            },
            Err(_) => Err(warp::reject::custom(FailedToDeserialize)),
        }
    } else {
        Err(warp::reject::custom(FailedToDispatchToQueue))
    }
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
