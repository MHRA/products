use uuid::Uuid;
use warp::{Filter, Rejection, Reply};
mod models;
use models::{JobStatus, JobStatusResponse};

mod redis;
use self::redis::{get_client, get_from_redis, set_in_redis, MyRedisError};

fn handler(id: Uuid) -> impl Reply {
    let client = get_client("redis://127.0.0.1:6379/".to_owned()).unwrap();
    let status = get_from_redis(client, id).unwrap();

    warp::reply::json(&JobStatusResponse { id, status })
}

fn set_status_handler(id: Uuid, job_status: JobStatus) -> Result<impl Reply, MyRedisError> {
    let client = get_client("redis://127.0.0.1:6379/".to_owned()).unwrap();
    set_in_redis(client, id, job_status)?;
    Ok(warp::reply())
}

pub fn jobs() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid).and(warp::get()).map(handler)
}

pub fn set_status() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid / JobStatus)
        .and(warp::post())
        .map(|id, status| set_status_handler(id, status).unwrap())
}
