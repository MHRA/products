use crate::models::Document;
use warp::{reply::Json, Filter, Rejection, Reply};

use std::collections::HashMap;

async fn get_health_handler() -> Result<Json, Rejection> {
    let mut result = HashMap::new();
    result.insert("healthy".to_string(), true);

    Ok(warp::reply::json(&result))
}

async fn del_document_handler(document_content_id: String) -> Result<Json, Rejection> {}

async fn check_in_document_handler(doc: Document) -> Result<Json, Rejection> {}

pub fn get_health() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("health")
        .and(warp::get())
        .and_then(get_health_handler)
}

pub fn del_document(
    document_content_id: String,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("documents" / String)
        .and(warp::delete())
        .and_then(del_document_handler)
}

pub fn check_in_document() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("documents")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(check_in_document_handler)
}
