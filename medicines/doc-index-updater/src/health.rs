use crate::auth_manager;
use warp::{
    reply::{Json, Xml},
    Filter, Rejection, Reply,
};

use std::collections::HashMap;

fn get_health_handler() -> HashMap<String, bool> {
    let mut result = HashMap::new();
    result.insert("healthy".to_string(), true);

    result
}

async fn get_health_handler_json() -> Result<Json, Rejection> {
    Ok(warp::reply::json(&get_health_handler()))
}

async fn get_health_handler_xml() -> Result<Xml, Rejection> {
    Ok(warp::reply::xml(&get_health_handler()))
}

pub fn get_health() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    warp::path!("healthz")
        .and(warp::get())
        .and_then(get_health_handler_json)
}

pub fn get_health_xml() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    warp::path!("healthz")
        .and(warp::get())
        .and(warp::header::exact_ignore_case("accept", "application/xml"))
        .and_then(get_health_handler_xml)
}
