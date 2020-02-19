use serde_derive::Serialize;
use uuid::Uuid;
use warp::Filter;

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

fn handler(id: Uuid) -> impl warp::reply::Reply {
    warp::reply::json(&JobStatusResponse {
        id,
        status: JobStatus::Accepted,
    })
}

#[tokio::main]
pub async fn serve() {
    let routes = warp::path!("jobs" / Uuid)
        .map(handler)
        .with(warp::log("doc_index_updater"));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
