use uuid::Uuid;
use warp::Filter;

#[tokio::main]
pub async fn serve() {
    let hello = warp::path!("jobs" / Uuid).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
