use crate::state_manager::{with_state, StateManager};
use anyhow::anyhow;
use bytes::BufMut;
use futures::future::join_all;
use futures::{stream, StreamExt, TryStreamExt};
use std::collections::HashMap;
use warp::{
    filters::multipart::{FormData, Part},
    reject,
    reply::{Json, Xml},
    Filter, Rejection, Reply,
};

pub fn handler(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("pars")
        .and(warp::post())
        .and(warp::multipart::form())
        .and(with_state(state_manager))
        .and_then(upload_pars_handler)
}

async fn upload_pars_handler(
    form_data: FormData,
    state_manager: StateManager,
) -> Result<impl Reply, Rejection> {
    dbg!(&form_data);

    let parts: Vec<Part> = form_data.try_collect().await.map_err(|e| {
        warp::reject::custom(UploadError {
            message: format!("Error receiving data: {}", e),
        })
    })?;

    let fields: Vec<UploadField> = join_all(parts.into_iter().map(read_upload_part))
        .await
        .into_iter()
        .collect::<Result<_, _>>()
        .map_err(|e| warp::reject::custom(e))?;

    dbg!(fields);

    Ok(warp::reply())
}

async fn read_upload_part(part: Part) -> Result<UploadField, UploadError> {
    let name = part.name().to_string();

    let file_name = part.filename().map(|s| s.to_string());

    let data = part
        .stream()
        .try_fold(Vec::new(), |mut vec, data| {
            vec.put(data);
            async move { Ok(vec) }
        })
        .await
        .map_err(|e| UploadError {
            message: format!("Error receiving data: {}", e),
        })?;

    let field = match file_name {
        Some(file_name) => UploadField::File {
            name,
            file_name: file_name.into(),
            data,
        },
        None => UploadField::Text {
            name,
            value: std::str::from_utf8(&data)
                .map_err(|e| UploadError {
                    message: format!("Error decoding field to utf-8: {}", e),
                })?
                .to_string(),
        },
    };

    Ok(field)
}

#[derive(Debug)]
enum UploadField {
    Text {
        name: String,
        value: String,
    },
    File {
        name: String,
        file_name: String,
        data: Vec<u8>,
    },
}

#[derive(Debug)]
struct UploadError {
    message: String,
}

impl warp::reject::Reject for UploadError {}
