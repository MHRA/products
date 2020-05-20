use crate::{
    auth_manager,
    models::{
        CreateMessage, DeleteMessage, Document, JobStatus, JobStatusResponse, Message, XMLDocument,
        XMLJobStatusResponse,
    },
    service_bus_client::{create_factory, delete_factory, DocIndexUpdaterQueue},
    state_manager::{with_state, JobStatusClient, MyRedisError, StateManager},
};
use time::Duration;
use tracing_futures::Instrument;
use uuid::Uuid;
use warp::{
    reject,
    reply::{Json, Xml},
    Filter, Rejection, Reply,
};

#[derive(Debug)]
pub struct FailedToDispatchToQueue;

impl warp::reject::Reject for FailedToDispatchToQueue {}

#[derive(Debug)]
pub struct FailedToDeserialize;

impl warp::reject::Reject for FailedToDeserialize {}

#[derive(Debug)]
pub struct FailedToAddToQueue;

impl warp::reject::Reject for FailedToAddToQueue {}

pub async fn accept_job(
    state_manager: &impl JobStatusClient,
) -> Result<JobStatusResponse, MyRedisError> {
    let id = Uuid::new_v4();
    let correlation_id = id.to_string();
    let correlation_id = correlation_id.as_str();
    state_manager
        .set_status(id, JobStatus::Accepted)
        .instrument(tracing::info_span!("document_manager", correlation_id))
        .await
}

async fn queue_job<T>(
    queue: &mut DocIndexUpdaterQueue,
    state_manager: &impl JobStatusClient,
    message: T,
) -> Result<JobStatusResponse, Rejection>
where
    T: Message,
{
    let id = message.get_id();
    let duration = Duration::days(1);

    match queue.send(message, duration).await {
        Ok(_) => Ok(state_manager.get_status(id).await?),
        Err(e) => {
            tracing::error!(
                "Failed to dispatch to queue. Check environment variables align for queue names, policies and keys. Error: ({:?})",
                e
            );
            let _state = state_manager
                .set_status(
                    id,
                    JobStatus::Error {
                        message: "Failed to dispatch to queue".to_owned(),
                        code: "".to_owned(),
                    },
                )
                .await?;

            Err(reject::custom(FailedToAddToQueue {}))
        }
    }
}

async fn delete_document_handler(
    document_content_id: String,
    state_manager: StateManager,
) -> Result<JobStatusResponse, Rejection> {
    if let Ok(mut queue) = delete_factory().await {
        let id = accept_job(&state_manager).await?.id;
        let correlation_id = id.to_string();
        let correlation_id = correlation_id.as_str();

        let message = DeleteMessage {
            job_id: id,
            document_content_id,
        };

        queue_job(&mut queue, &state_manager, message)
            .instrument(tracing::info_span!(
                "delete_document_handler::queue_job",
                correlation_id
            ))
            .await
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

pub async fn check_in_document_handler(
    doc: Document,
    state_manager: impl JobStatusClient,
) -> Result<JobStatusResponse, Rejection> {
    if let Ok(mut queue) = create_factory().await {
        let id = accept_job(&state_manager).await?.id;
        let correlation_id = id.to_string();
        let correlation_id = correlation_id.as_str();

        let message = CreateMessage {
            job_id: id,
            document: doc,
        };

        queue_job(&mut queue, &state_manager, message)
            .instrument(tracing::info_span!(
                "check_in_document_handler::queue_job",
                correlation_id
            ))
            .await
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
