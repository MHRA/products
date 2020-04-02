pub use self::redis::{get_client, MyRedisError};
use self::redis::{get_from_redis, set_in_redis};
use crate::{
    auth_manager,
    models::{JobStatus, JobStatusResponse, XMLJobStatusResponse},
};
use ::redis::Client;
use async_trait::async_trait;
use uuid::Uuid;
use warp::{http::StatusCode, reply::Json, Filter, Rejection, Reply};
mod redis;

#[derive(Clone, Debug)]
pub struct StateManager {
    pub client: Client,
}

#[async_trait]
pub trait JobStatusClient {
    async fn get_status(&self, id: Uuid) -> Result<JobStatusResponse, MyRedisError>;
    async fn set_status(
        &self,
        id: Uuid,
        status: JobStatus,
    ) -> Result<JobStatusResponse, MyRedisError>;
}

#[cfg(test)]
pub struct TestJobStatusClient {}

#[cfg(test)]
#[async_trait]
impl JobStatusClient for TestJobStatusClient {
    async fn get_status(
        &self,
        _id: Uuid,
    ) -> Result<crate::models::JobStatusResponse, crate::state_manager::MyRedisError> {
        unimplemented!()
    }
    async fn set_status(
        &self,
        id: Uuid,
        status: JobStatus,
    ) -> Result<crate::models::JobStatusResponse, crate::state_manager::MyRedisError> {
        Ok(JobStatusResponse { id, status })
    }
}

impl StateManager {
    pub fn new(client: Client) -> Self {
        StateManager { client }
    }
}

#[async_trait::async_trait]
impl JobStatusClient for StateManager {
    async fn get_status(&self, id: Uuid) -> Result<JobStatusResponse, MyRedisError> {
        let status = get_from_redis(self.client.clone(), id).await?;

        Ok(JobStatusResponse { id, status })
    }

    async fn set_status(
        &self,
        id: Uuid,
        status: JobStatus,
    ) -> Result<JobStatusResponse, MyRedisError> {
        let status = set_in_redis(self.client.clone(), id, status).await?;

        Ok(JobStatusResponse { id, status })
    }
}

pub fn get_job_status(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid)
        .and(warp::get())
        .and(auth_manager::with_basic_auth())
        .and(with_state(state_manager))
        .and_then(get_status_handler_json)
}

pub fn get_job_status_xml(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid)
        .and(warp::get())
        .and(auth_manager::with_basic_auth())
        .and(warp::header::exact_ignore_case("accept", "application/xml"))
        .and(with_state(state_manager))
        .and_then(get_status_handler_xml)
}

fn handle_known_errors(id: Uuid, e: MyRedisError) -> Result<JobStatusResponse, MyRedisError> {
    tracing::error!("{}", e);
    if let MyRedisError::IncompatibleType(_) = e {
        Ok(JobStatusResponse {
            id,
            status: JobStatus::NotFound,
        })
    } else {
        Err(e)
    }
}

fn to_response_with_status(response: JobStatusResponse) -> impl Reply {
    let json = warp::reply::json(&response);
    match response.status {
        JobStatus::NotFound => warp::reply::with_status(json, StatusCode::NOT_FOUND),
        _ => warp::reply::with_status(json, StatusCode::OK),
    }
}

fn to_response_with_status_xml(response: JobStatusResponse) -> impl Reply {
    let status = response.status.clone();
    let xml_response: XMLJobStatusResponse = response.into();
    let xml = warp::reply::xml(&xml_response);
    match status {
        JobStatus::NotFound => warp::reply::with_status(xml, StatusCode::NOT_FOUND),
        _ => warp::reply::with_status(xml, StatusCode::OK),
    }
}

#[tracing::instrument]
async fn get_status_handler(id: Uuid, mgr: StateManager) -> Result<JobStatusResponse, Rejection> {
    mgr.get_status(id)
        .await
        .or_else(|e| handle_known_errors(id, e))
        .map_err(Into::into)
}

async fn get_status_handler_json(id: Uuid, mgr: StateManager) -> Result<impl Reply, Rejection> {
    get_status_handler(id, mgr)
        .await
        .map(to_response_with_status)
}

async fn get_status_handler_xml(id: Uuid, mgr: StateManager) -> Result<impl Reply, Rejection> {
    get_status_handler(id, mgr)
        .await
        .map(to_response_with_status_xml)
}

pub fn set_job_status(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid / JobStatus)
        .and(warp::post())
        .and(auth_manager::with_basic_auth())
        .and(with_state(state_manager))
        .and_then(set_status_handler)
}

#[tracing::instrument]
async fn set_status_handler(
    id: Uuid,
    status: JobStatus,
    mgr: StateManager,
) -> Result<Json, Rejection> {
    mgr.set_status(id, status)
        .await
        .or_else(|e: MyRedisError| {
            tracing::error!("{}", e);
            Err(e)
        })
        .map_err(Into::into)
        .map(|r| warp::reply::json(&r))
}

pub fn with_state(
    mgr: StateManager,
) -> impl Filter<Extract = (StateManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || mgr.clone())
}
