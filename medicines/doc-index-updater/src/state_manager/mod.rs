pub use self::redis::get_client;
use self::redis::{get_from_redis, set_in_redis, MyRedisError};
use ::redis::aio::MultiplexedConnection;
use models::{JobStatus, JobStatusResponse};
use uuid::Uuid;
use warp::{reply::Json, Filter, Rejection, Reply};

mod models;
mod redis;

async fn get_status_handler(
    id: Uuid,
    connection: MultiplexedConnection,
) -> Result<Json, Rejection> {
    let status = get_from_redis(connection, id)
        .await
        .map_err(MyRedisError::from)?;

    Ok(warp::reply::json(&JobStatusResponse { id, status }))
}

async fn set_status_handler(
    id: Uuid,
    status: JobStatus,
    connection: MultiplexedConnection,
) -> Result<Json, Rejection> {
    let status = set_in_redis(connection, id, status)
        .await
        .map_err(MyRedisError::from)?;

    Ok(warp::reply::json(&JobStatusResponse { id, status }))
}

pub fn get_job_status(
    connection: MultiplexedConnection,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid)
        .and(warp::get())
        .and(with_connection(connection))
        .and_then(get_status_handler)
}

pub fn set_job_status(
    connection: MultiplexedConnection,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid / JobStatus)
        .and(warp::post())
        .and(with_connection(connection))
        .and_then(set_status_handler)
}

pub fn with_connection(
    connection: MultiplexedConnection,
) -> impl Filter<Extract = (MultiplexedConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || connection.clone())
}
