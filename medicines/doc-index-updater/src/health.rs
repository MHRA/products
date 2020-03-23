use warp::{http::StatusCode, Filter, Rejection, Reply};

pub fn get_health() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    warp::path!("healthz")
        .and(warp::get())
        .map(warp::reply)
        .map(|reply| warp::reply::with_status(reply, StatusCode::NO_CONTENT))
}
