use crate::{
    auth_manager,
    models::{ParUploadRequest, ParUploadResponse},
};
use azure_jwt;
use warp::{reply::Json, Filter, Rejection, Reply};

pub fn upload_par_file() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("uploadpar")
        .and(warp::post())
        .and(auth_manager::with_basic_auth())
        .and(warp::header::exact_ignore_case(
            "accept",
            "application/json",
        ))
        .and(warp::body::json())
        .and_then(upload_par_handler)
}

async fn upload_par_handler(par_upload: ParUploadRequest) -> Result<Json, Rejection> {
    let authenticated = auth(par_upload.jwt_token);

    Ok(warp::reply::json(&par_upload))
}

fn auth(token: &str) -> ParUploadResponse {
    let mut az_auth = AzureAuth::new("c690f864-0c73-45a5-a1d0-0267b031f722").unwrap();

    let decoded_token = az_auth.validate_token(&token).expect("validated");
    ParUploadResponse {
        email: decoded_token.claims.preferred_username,
        authenticated: true,
    }
}
