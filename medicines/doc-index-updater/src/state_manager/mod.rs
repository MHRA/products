use serde_derive::Serialize;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

#[derive(Serialize)]
enum JobStatus {
    Accepted,
    _Done,
    _NotFound,
    _Error { message: String, code: String },
}

#[derive(Serialize)]
struct JobStatusResponse {
    id: Uuid,
    status: JobStatus,
}

fn handler(id: Uuid) -> impl Reply {
    warp::reply::json(&JobStatusResponse {
        id,
        status: JobStatus::Accepted,
    })
}

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid).map(handler)
}
