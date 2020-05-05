use crate::{
    auth_manager,
    models::{ParUploadRequest, ParUploadResponse},
};
use azure_jwt::AzureAuth;

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
    let authenticated = auth(par_upload.jwt_token.clone());

    Ok(warp::reply::json(&authenticated))
}

fn auth(token: String) -> ParUploadResponse {
    let mut az_auth = AzureAuth::new("c690f864-0c73-45a5-a1d0-0267b031f722").unwrap();

    let decoded_token = az_auth.validate_token(token.as_str());
    match decoded_token {
        Ok(x) => ParUploadResponse {
            email: x.claims.preferred_username,
            authenticated: true,
            error: None,
        },
        Err(x) => ParUploadResponse {
            email: None,
            authenticated: false,
            error: Some(format!("{}", x)),
        },
    }
}
