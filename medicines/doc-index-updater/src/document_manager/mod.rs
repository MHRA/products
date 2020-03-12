use crate::{
    models::{CreateMessage, DeleteMessage, Document, JobStatus, XMLDocument},
    service_bus_client::{create_factory, delete_factory},
    state_manager::{with_state, StateManager},
};
use time::Duration;
use uuid::Uuid;
use warp::{
    reply::{Json, Reply as ReplyReply, Xml},
    Filter, Rejection, Reply,
};

#[derive(Debug)]
struct FailedToDispatchToQueue;
#[derive(Debug)]
struct FailedToDeserialize;
impl warp::reject::Reject for FailedToDispatchToQueue {}
impl warp::reject::Reject for FailedToDeserialize {}

enum ReplyType {
    Json,
    Xml,
}

fn choose_reply_content_type(accept_header: Option<String>) -> ReplyType {
    match accept_header {
        Some(v) => match v {
            "application/xml" => ReplyType::Xml,
            "text/xml" => ReplyType::Xml,
            _ => ReplyType::Json,
        },
        _ => ReplyType::Json,
    }
}

async fn del_document_handler(
    document_content_id: String,
    accept_header: Option<String>,
    state_manager: StateManager,
) -> Result<ReplyReply, Rejection> {
    if let Ok(mut queue) = delete_factory().await {
        let id = Uuid::new_v4();
        let message = DeleteMessage {
            job_id: id,
            document_content_id,
        };
        let duration = Duration::days(1);

        match queue.send(message, duration).await {
            Ok(_) => match choose_reply_content_type(accept_header) {
                ReplyType::Xml => Ok(warp::reply::xml(
                    &state_manager.set_status(id, JobStatus::Accepted).await?,
                )),
                ReplyType::Json => Ok(warp::reply::json(
                    &state_manager.set_status(id, JobStatus::Accepted).await?,
                )),
            },
            Err(_) => Err(warp::reject::custom(FailedToDispatchToQueue)),
        }
    } else {
        Err(warp::reject::custom(FailedToDispatchToQueue))
    }
}

async fn check_in_document_handler(
    doc: Document,
    accept_header: Option<String>,
    state_manager: StateManager,
) -> Result<ReplyReply, Rejection> {
    if let Ok(mut queue) = create_factory().await {
        let id = Uuid::new_v4();
        let message = CreateMessage {
            job_id: id,
            document: doc,
        };
        let duration = Duration::days(1);
        match queue.send(message, duration).await {
            Ok(_) => match choose_reply_content_type(accept_header) {
                ReplyType::Xml => Ok(warp::reply::xml(
                    &state_manager.set_status(id, JobStatus::Accepted).await?,
                )),
                ReplyType::Json => Ok(warp::reply::json(
                    &state_manager.set_status(id, JobStatus::Accepted).await?,
                )),
            },
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
        .and(warp::header::optional::<String>("accept"))
        .and(with_state(state_manager))
        .and_then(del_document_handler)
}

pub fn check_in_document(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("documents")
        .and(warp::post())
        .and(warp::body::xml_enforce_strict_content_type())
        .map(|doc: XMLDocument| Document::from(doc))
        .or(warp::body::json())
        .unify()
        .and(warp::header::optional::<String>("accept"))
        .and(with_state(state_manager))
        .and_then(check_in_document_handler)
}
