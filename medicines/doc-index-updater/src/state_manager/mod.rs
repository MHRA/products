pub use self::redis::get_client;
use self::redis::{get_from_redis, set_in_redis, MyRedisError};
use crate::models::{JobStatus, JobStatusResponse};
use ::redis::Client;
use uuid::Uuid;
use warp::{reply::Json, Filter, Rejection, Reply};

mod redis;

#[derive(Clone)]
pub struct StateManager {
    pub client: Client,
}

impl StateManager {
    pub fn new(client: Client) -> Self {
        StateManager { client }
    }

    pub async fn get_status(&self, id: Uuid) -> Result<JobStatusResponse, MyRedisError> {
        let status = get_from_redis(self.client.clone(), id)
            .await
            .map_err(MyRedisError::from)?;

        Ok(JobStatusResponse { id, status })
    }

    pub async fn set_status(
        &self,
        id: Uuid,
        status: JobStatus,
    ) -> Result<JobStatusResponse, MyRedisError> {
        let status = set_in_redis(self.client.clone(), id, status)
            .await
            .map_err(MyRedisError::from)?;

        Ok(JobStatusResponse { id, status })
    }
}

pub fn get_job_status(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid)
        .and(warp::get())
        .and(with_state(state_manager))
        .and_then(get_status_handler)
}

async fn get_status_handler(id: Uuid, mgr: StateManager) -> Result<Json, Rejection> {
    let response = mgr.get_status(id).await?;
    match response.status {
        JobStatus::NotFound => Err(warp::reject::not_found()),
        _ => Ok(warp::reply::json(&response)),
    }
}

pub fn set_job_status(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid / JobStatus)
        .and(warp::post())
        .and(with_state(state_manager))
        .and_then(set_status_handler)
}

async fn set_status_handler(
    id: Uuid,
    status: JobStatus,
    mgr: StateManager,
) -> Result<Json, Rejection> {
    Ok(warp::reply::json(&mgr.set_status(id, status).await?))
}

pub fn with_state(
    mgr: StateManager,
) -> impl Filter<Extract = (StateManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || mgr.clone())
}
