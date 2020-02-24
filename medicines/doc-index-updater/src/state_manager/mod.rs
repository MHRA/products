pub use self::redis::get_client;
use self::redis::{get_from_redis, set_in_redis, MyRedisError};
use ::redis::aio::MultiplexedConnection;
use models::{JobStatus, JobStatusResponse};
use uuid::Uuid;
use warp::{reply::Json, Filter, Rejection, Reply};

mod models;
mod redis;

async fn handler(id: Uuid, connection: MultiplexedConnection) -> Result<Json, Rejection> {
    let status = get_from_redis(connection, id)
        .await
        .map_err(MyRedisError::from)?;

    Ok(warp::reply::json(&JobStatusResponse { id, status }))
}

fn set_status_handler(id: Uuid, job_status: JobStatus) -> Result<impl Reply, MyRedisError> {
    let mut connection = get_client("redis://127.0.0.1:6379/".to_owned())
        .unwrap()
        .get_connection()
        .unwrap();
    set_in_redis(&mut connection, id, job_status)?;
    Ok(warp::reply())
}

pub fn jobs(
    connection: MultiplexedConnection,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid)
        .and(warp::get())
        .and(with_connection(connection))
        .and_then(handler)
}

pub fn with_connection(
    connection: MultiplexedConnection,
) -> impl Filter<Extract = (MultiplexedConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || connection.clone())
}

pub fn set_status() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid / JobStatus)
        .and(warp::post())
        .map(|id, status| set_status_handler(id, status).unwrap())
}
