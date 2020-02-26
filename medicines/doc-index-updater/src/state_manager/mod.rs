pub use self::redis::get_client;
use self::redis::{get_from_redis, set_in_redis, MyRedisError};
use crate::models::{JobStatus, JobStatusResponse};
use ::redis::Client;
use uuid::Uuid;
use warp::{reply::Json, Filter, Rejection, Reply};

mod redis;

async fn get_status_handler(id: Uuid, client: Client) -> Result<Json, Rejection> {
    let status = get_from_redis(client, id)
        .await
        .map_err(MyRedisError::from)?;

    Ok(warp::reply::json(&JobStatusResponse { id, status }))
}

async fn set_status_handler(
    id: Uuid,
    status: JobStatus,
    client: Client,
) -> Result<Json, Rejection> {
    let status = set_in_redis(client, id, status)
        .await
        .map_err(MyRedisError::from)?;

    Ok(warp::reply::json(&JobStatusResponse { id, status }))
}

pub fn get_job_status(
    redis_client: Client,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid)
        .and(warp::get())
        .and(with_connection(redis_client))
        .and_then(get_status_handler)
}

pub fn set_job_status(
    redis_client: Client,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid / JobStatus)
        .and(warp::post())
        .and(with_connection(redis_client))
        .and_then(set_status_handler)
}

pub fn with_connection(
    redis_client: Client,
) -> impl Filter<Extract = (Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || redis_client.clone())
}
