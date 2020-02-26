use warp::{reply::Json, Filter, Rejection, Reply};

use std::collections::HashMap;

async fn get_health_handler() -> Result<Json, Rejection> {
    let mut result = HashMap::new();
    result.insert("healthy".to_string(), true);

    Ok(warp::reply::json(&result))
}

pub fn get_health() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("health")
        .and(warp::get())
        .and_then(get_health_handler)
}
