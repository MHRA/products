use serde_derive::Serialize;
use uuid::Uuid;
use warp::Filter;

#[derive(Serialize)]
enum JobStatus {
    Accepted,
    _Done,
    _NotFound,
}

#[derive(Serialize)]
struct JobStatusResponse {
    id: Uuid,
    status: JobStatus,
}

#[tokio::main]
pub async fn serve() {
    let job_status = warp::path!("jobs" / Uuid).map(|id: Uuid| {
        warp::reply::json(&JobStatusResponse {
            id,
            status: JobStatus::Accepted,
        })
    });

    warp::serve(job_status).run(([127, 0, 0, 1], 3030)).await;
}
