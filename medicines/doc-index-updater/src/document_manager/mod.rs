use crate::{
    models::{CreateMessage, DeleteMessage, Document, JobStatus},
    service_bus_client::{create_factory, delete_factory},
    state_manager::{with_state, StateManager},
};
use time::Duration;
use uuid::Uuid;
use warp::{reply::Json, Filter, Rejection, Reply};

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
    if let Ok(mut queue) = delete_factory().await {
        let id = Uuid::new_v4();
        let message = DeleteMessage {
            job_id: id,
            document_content_id,
        };
        let duration = Duration::days(1);

        match queue.send(message, duration).await {
            Ok(_) => Ok(warp::reply::json(
                &state_manager.set_status(id, JobStatus::Accepted).await?,
            )),
            Err(_) => Err(warp::reject::custom(FailedToDispatchToQueue)),
        }
    } else {
        Err(warp::reject::custom(FailedToDispatchToQueue))
    }
}

async fn check_in_document_handler(
    doc: Document,
    state_manager: StateManager,
) -> Result<Json, Rejection> {
    if let Ok(mut queue) = create_factory().await {
        let id = Uuid::new_v4();
        let message = CreateMessage {
            job_id: id,
            document: doc,
        };
        let duration = Duration::days(1);
        match queue.send(message, duration).await {
            Ok(_) => Ok(warp::reply::json(
                &state_manager.set_status(id, JobStatus::Accepted).await?,
            )),
            Err(_) => Err(warp::reject::custom(FailedToDispatchToQueue)),
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
