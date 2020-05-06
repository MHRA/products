use crate::models::{ParUploadRequest, ParUploadResponse};

use jsonwebtoken::dangerous_unsafe_decode;
use serde::{Deserialize, Serialize};
use warp::{reply::Json, Filter, Rejection, Reply};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    preferred_username: String, //company: String,
}

pub fn upload_par_file() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("par")
        .and(warp::post())
        .and(warp::header::exact_ignore_case(
            "accept",
            "application/json",
        ))
        .and(warp::body::json())
        .and_then(upload_par_handler)
}

async fn upload_par_handler(par_upload: ParUploadRequest) -> Result<Json, Rejection> {
    let authenticated = auth(par_upload.jwt_token);
    Ok(warp::reply::json(&authenticated))
}

fn auth(token: String) -> ParUploadResponse {
    // Claims is a struct that implements Deserialize
    let token_message = dangerous_unsafe_decode::<Claims>(&token);
    tracing::info!("{:?}", token_message);

    match token_message {
        Ok(t) => ParUploadResponse {
            email: Some(t.claims.preferred_username),
            error: None,
        },
        Err(e) => ParUploadResponse {
            email: None,
            error: Some(format!("{:?}", e)),
        },
    }
}
