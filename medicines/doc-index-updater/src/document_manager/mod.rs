use crate::{
    auth_manager,
    models::{
        CreateMessage, DeleteMessage, Document, JobStatus, JobStatusResponse, Message,
        UniqueDocumentIdentifier, XMLJobStatusResponse,
    },
    service_bus_client::{create_factory, delete_factory, DocIndexUpdaterQueue},
    state_manager::{with_state, JobStatusClient, MyRedisError, StateManager},
};
use bytes::{buf::BufExt, Buf};
use chrono::Duration;
use hyper::Body;
use serde::de::DeserializeOwned;
use tracing_futures::Instrument;
use uuid::Uuid;
use warp::{
    body::aggregate, http::HeaderValue, http::Response, http::StatusCode, reject, reply::Json,
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

pub async fn delete_document_handler(
    document_id: UniqueDocumentIdentifier,
    state_manager: &impl JobStatusClient,
    initiator_email: Option<String>,
) -> Result<JobStatusResponse, Rejection> {
    if let Ok(mut queue) = delete_factory().await {
        let id = accept_job(state_manager).await?.id;
        let correlation_id = id.to_string();
        let correlation_id = correlation_id.as_str();

        let message = DeleteMessage {
            job_id: id,
            document_id,
            initiator_email,
        };

        queue_job(&mut queue, state_manager, message)
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
    document_id: String,
    state_manager: StateManager,
) -> Result<Response<Body>, Rejection> {
    let r: XMLJobStatusResponse = delete_document_handler(document_id.into(), &state_manager, None)
        .await?
        .into();
    match serde_xml_rs::to_string(&r) {
        Ok(xml_body) => {
            let mut res = Response::new(xml_body.into());
            res.headers_mut()
                .insert("Content-type", HeaderValue::from_static("text/xml"));
            Ok(res)
        }
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}

async fn delete_document_json_handler(
    document_id: String,
    state_manager: StateManager,
) -> Result<Json, Rejection> {
    let r = delete_document_handler(document_id.into(), &state_manager, None).await?;
    Ok(warp::reply::json(&r))
}

pub async fn check_in_document_handler(
    doc: Document,
    state_manager: &impl JobStatusClient,
    initiator_email: Option<String>,
) -> Result<JobStatusResponse, Rejection> {
    if let Ok(mut queue) = create_factory().await {
        let id = accept_job(state_manager).await?.id;
        let correlation_id = id.to_string();
        let correlation_id = correlation_id.as_str();

        let message = CreateMessage {
            job_id: id,
            document: doc,
            initiator_email,
        };

        queue_job(&mut queue, state_manager, message)
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
) -> Result<Response<Body>, Rejection> {
    let r: XMLJobStatusResponse = check_in_document_handler(doc, &state_manager, None)
        .await?
        .into();
    match serde_xml_rs::to_string(&r) {
        Ok(xml_body) => {
            let mut res = Response::new(xml_body.into());
            res.headers_mut()
                .insert("Content-type", HeaderValue::from_static("text/xml"));
            Ok(res)
        }
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}

async fn check_in_document_json_handler(
    doc: Document,
    state_manager: StateManager,
) -> Result<Json, Rejection> {
    let r = check_in_document_handler(doc, &state_manager, None).await?;
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

pub async fn convert_xml(buf: impl Buf) -> anyhow::Result<T> {
    serde_xml_rs::from_reader(&mut buf.reader()).map_err(|err| Err(warp::reject::reject()))
}

pub fn check_in_xml_document(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("documents")
        .and(warp::post())
        .and(auth_manager::with_basic_auth())
        .and(warp::header::exact_ignore_case("accept", "application/xml"))
        .and(warp::header::exact_ignore_case(
            "content-type",
            "application/xml",
        ))
        .and(aggregate())
        .and(|buf| async move { serde_xml_rs::from_reader(buf.reader()).map_err(Into::into) })
        // need to add a recover filter to turn the 500 error into a Reply otherwise it gets returned as a 500 internal server error
        .map(Into::<Document>::into)
        .and(with_state(state_manager))
        .and_then(check_in_document_xml_handler)
}
