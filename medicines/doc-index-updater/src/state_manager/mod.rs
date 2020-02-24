use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

mod models;
use models::{JobStatus, JobStatusResponse};

mod redis;
use self::redis::{get_from_redis, set_in_redis, MyRedisError};

fn handler(id: Uuid) -> impl Reply {
    let status = get_from_redis(id).unwrap();

    warp::reply::json(&JobStatusResponse { id, status })
}

fn set_status_handler(id: Uuid, job_status: JobStatus) -> Result<impl Reply, MyRedisError> {
    set_in_redis(id, job_status)?;
    Ok(warp::reply())
}

pub fn jobs() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid).map(handler)
}

pub fn set_status() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid / JobStatus)
        .and(warp::post())
        .map(|id, status| set_status_handler(id, status).unwrap())
}
