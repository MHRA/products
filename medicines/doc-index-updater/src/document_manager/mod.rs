use crate::{
    auth_manager,
    models::{
        CreateMessage, DeleteMessage, Document, JobStatus, JobStatusResponse, XMLDocument,
        XMLJobStatusResponse,
    },
    service_bus_client::{create_factory, delete_factory},
    state_manager::{with_state, StateManager},
};
use time::Duration;
use uuid::Uuid;
use warp::{
    reply::{Json, Xml},
    Filter, Rejection, Reply,
};

#[derive(Debug)]
pub struct FailedToDispatchToQueue;
#[derive(Debug)]
pub struct FailedToDeserialize;
impl warp::reject::Reject for FailedToDispatchToQueue {}
impl warp::reject::Reject for FailedToDeserialize {}

async fn delete_document_handler(
    document_content_id: String,
    state_manager: StateManager,
) -> Result<JobStatusResponse, Rejection> {
    if let Ok(mut queue) = delete_factory().await {
        let id = Uuid::new_v4();
        let message = DeleteMessage {
            job_id: id,
            document_content_id,
        };
        let duration = Duration::days(1);

        match queue.send(message, duration).await {
            Ok(_) => Ok(state_manager.set_status(id, JobStatus::Accepted).await?),
            Err(_) => Err(warp::reject::custom(FailedToDispatchToQueue)),
        }
    } else {
        Err(warp::reject::custom(FailedToDispatchToQueue))
    }
}

async fn delete_document_xml_handler(
    document_content_id: String,
    state_manager: StateManager,
) -> Result<Xml, Rejection> {
    let r: XMLJobStatusResponse = delete_document_handler(document_content_id, state_manager)
        .await?
        .into();
    Ok(warp::reply::xml(&r))
}

async fn delete_document_json_handler(
    document_content_id: String,
    state_manager: StateManager,
) -> Result<Json, Rejection> {
    let r = delete_document_handler(document_content_id, state_manager).await?;
    Ok(warp::reply::json(&r))
}

async fn check_in_document_handler(
    doc: Document,
    state_manager: StateManager,
) -> Result<JobStatusResponse, Rejection> {
    if let Ok(mut queue) = create_factory().await {
        let id = Uuid::new_v4();
        let message = CreateMessage {
            job_id: id,
            document: doc,
        };
        let duration = Duration::days(1);
        match queue.send(message, duration).await {
            Ok(_) => Ok(state_manager.set_status(id, JobStatus::Accepted).await?),
            Err(_) => Err(warp::reject::custom(FailedToDispatchToQueue)),
        }
    } else {
        Err(warp::reject::custom(FailedToDispatchToQueue))
    }
}

async fn check_in_document_xml_handler(
    doc: Document,
    state_manager: StateManager,
) -> Result<Xml, Rejection> {
    let r: XMLJobStatusResponse = check_in_document_handler(doc, state_manager).await?.into();
    Ok(warp::reply::xml(&r))
}

async fn check_in_document_json_handler(
    doc: Document,
    state_manager: StateManager,
) -> Result<Json, Rejection> {
    let r = check_in_document_handler(doc, state_manager).await?;
    Ok(warp::reply::json(&r))
}

pub fn delete_document(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("documents" / String)
        .and(warp::delete())
        .and(auth_manager::with_basic_auth())
        .and(with_state(state_manager))
        .and_then(delete_document_json_handler)
}

pub fn delete_document_xml(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("documents" / String)
        .and(warp::delete())
        .and(auth_manager::with_basic_auth())
        .and(warp::header::exact_ignore_case("accept", "application/xml"))
        .and(with_state(state_manager))
        .and_then(delete_document_xml_handler)
}

pub fn check_in_document(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("documents")
        .and(warp::post())
        .and(auth_manager::with_basic_auth())
        .and(warp::body::json())
        .and(with_state(state_manager))
        .and_then(check_in_document_json_handler)
}

pub fn check_in_xml_document(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("documents")
        .and(warp::post())
        .and(auth_manager::with_basic_auth())
        .and(warp::header::exact_ignore_case("accept", "application/xml"))
        .and(warp::body::xml_enforce_strict_content_type::<XMLDocument>())
        .map(Into::<Document>::into)
        .and(with_state(state_manager))
        .and_then(check_in_document_xml_handler)
}
