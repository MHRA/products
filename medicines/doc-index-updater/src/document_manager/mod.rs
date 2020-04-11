use crate::{
    auth_manager,
    models::{
        CreateMessage, DeleteMessage, Document, JobStatus, JobStatusResponse, Message, XMLDocument,
        XMLJobStatusResponse,
    },
    service_bus_client::{create_factory, delete_factory, DocIndexUpdaterQueue},
    state_manager::{with_state, JobStatusClient, MyRedisError, StateManager},
};
use fehler::{throw, throws};
use time::Duration;
use tracing_futures::Instrument;
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

#[throws(MyRedisError)]
async fn accept_job(state_manager: &impl JobStatusClient) -> JobStatusResponse {
    let id = Uuid::new_v4();
    let correlation_id = id.to_string();
    let correlation_id = correlation_id.as_str();
    state_manager
        .set_status(id, JobStatus::Accepted)
        .instrument(tracing::info_span!("document_manager", correlation_id))
        .await?
}

#[throws(Rejection)]
async fn queue_job<T>(
    queue: &mut DocIndexUpdaterQueue,
    state_manager: &impl JobStatusClient,
    message: T,
) -> JobStatusResponse
where
    T: Message,
{
    let duration = Duration::days(1);

    match queue.send(message.clone(), duration).await {
        Ok(_) => state_manager.get_status(message.get_id()).await?,
        Err(e) => {
            tracing::error!(
                "Failed to dispatch to queue. Check environment variables align for queue names, policies and keys. Error: ({:?})",
                e
            );

            state_manager
                .set_status(
                    message.get_id(),
                    JobStatus::Error {
                        message: "Failed to dispatch to queue".to_owned(),
                        code: "".to_owned(),
                    },
                )
                .await?
        }
    }
}

#[throws(Rejection)]
async fn delete_document_handler(
    document_content_id: String,
    state_manager: StateManager,
) -> JobStatusResponse {
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
            .await?
    } else {
        throw!(warp::reject::custom(FailedToDispatchToQueue))
    }
}

#[throws(Rejection)]
async fn delete_document_xml_handler(
    document_content_id: String,
    state_manager: StateManager,
) -> Xml {
    warp::reply::xml::<XMLJobStatusResponse>(
        &delete_document_handler(document_content_id, state_manager)
            .await?
            .into(),
    )
}

#[throws(Rejection)]
async fn delete_document_json_handler(
    document_content_id: String,
    state_manager: StateManager,
) -> Json {
    warp::reply::json(&delete_document_handler(document_content_id, state_manager).await?)
}

#[throws(Rejection)]
async fn check_in_document_handler(
    doc: Document,
    state_manager: StateManager,
) -> JobStatusResponse {
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
            .await?
    } else {
        throw!(warp::reject::custom(FailedToDispatchToQueue))
    }
}

#[throws(Rejection)]
async fn check_in_document_xml_handler(doc: Document, state_manager: StateManager) -> Xml {
    warp::reply::xml::<XMLJobStatusResponse>(
        &check_in_document_handler(doc, state_manager).await?.into(),
    )
}

#[throws(Rejection)]
async fn check_in_document_json_handler(doc: Document, state_manager: StateManager) -> Json {
    warp::reply::json(&check_in_document_handler(doc, state_manager).await?)
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
