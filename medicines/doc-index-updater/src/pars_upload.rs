use crate::{
    create_manager::{create_blob, models::BlobMetadata},
    models::DocumentType,
    storage_client,
};
use bytes::BufMut;
use futures::future::join_all;
use futures::TryStreamExt;
use serde::Serialize;
use warp::{
    filters::multipart::{FormData, Part},
    Filter, Rejection, Reply,
};

pub fn handler() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("pars")
        .and(warp::post())
        .and(warp::multipart::form().max_length(100 * 1024 * 1024))
        .and_then(upload_pars_handler)
}

async fn upload_pars_handler(form_data: FormData) -> Result<impl Reply, Rejection> {
    tracing::debug!("Received PARS submission");

    let storage_client = storage_client::factory()
        .map_err(|e| {
            tracing::error!("Error creating storage client: {:?}", e);
            warp::reject::custom(SubmissionError::UploadError {
                message: format!("Couldn't create storage client: {:?}", e),
            })
        })?
        .azure_client;

    let (metadata, file_data) = read_pars_upload(form_data).await.map_err(|e| {
        tracing::debug!("Error reading PARS upload: {:?}", e);
        warp::reject::custom(e)
    })?;

    dbg!(&metadata);

    let blob = create_blob(&storage_client, &file_data, metadata)
        .await
        .map_err(|e| {
            tracing::error!("Error uploading file to blob storage: {:?}", e);
            warp::reject::custom(SubmissionError::UploadError {
                message: format!("Error uploading to blob storage: {:?}", e),
            })
        })?;

    dbg!(&blob);

    Ok(warp::reply::json(&UploadResponse {
        name: &blob.name,
        path: &blob.path,
    }))
}

#[derive(Debug, Serialize)]
struct UploadResponse<'a> {
    name: &'a str,
    path: &'a str,
}

async fn read_pars_upload(form_data: FormData) -> Result<(BlobMetadata, Vec<u8>), SubmissionError> {
    let parts: Vec<Part> =
        form_data
            .try_collect()
            .await
            .map_err(|e| SubmissionError::UploadError {
                message: format!("Error receiving data: {}", e),
            })?;

    let fields: Vec<(String, UploadFieldValue)> = join_all(parts.into_iter().map(read_upload_part))
        .await
        .into_iter()
        .collect::<Result<_, _>>()?;

    let file_name = fields
        .iter()
        .find(|(name, _)| name == "file")
        .and_then(|(_, field)| field.file_name())
        .ok_or(SubmissionError::MissingField { name: "file" })?
        .into();

    let active_substances = fields
        .iter()
        .filter(|(name, _)| name == "active_substances")
        .filter_map(|(_, field)| field.value())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let title = fields
        .iter()
        .find(|(name, _)| name == "title")
        .and_then(|(_, field)| field.value())
        .ok_or(SubmissionError::MissingField { name: "title" })?
        .into();

    let author = fields
        .iter()
        .find(|(name, _)| name == "author")
        .and_then(|(_, field)| field.value())
        .ok_or(SubmissionError::MissingField { name: "author" })?
        .into();

    let pl_number = fields
        .iter()
        .find(|(name, _)| name == "pl_number")
        .and_then(|(_, field)| field.value())
        .ok_or(SubmissionError::MissingField { name: "pl_number" })?
        .into();

    let metadata = BlobMetadata::new(
        file_name,
        DocumentType::Par,
        title,
        pl_number,
        vec![],
        active_substances,
        author,
        None,
    );

    let file_data = fields
        .into_iter()
        .find(|(name, _)| name == "file")
        .and_then(|(_, field)| field.into_file_data())
        .ok_or(SubmissionError::MissingField { name: "file" })?;

    Ok((metadata, file_data))
}

async fn read_upload_part(part: Part) -> Result<(String, UploadFieldValue), SubmissionError> {
    let name = part.name().to_string();

    let file_name = part.filename().map(|s| s.to_string());

    let data = part
        .stream()
        .try_fold(Vec::new(), |mut vec, data| {
            vec.put(data);
            async move { Ok(vec) }
        })
        .await
        .map_err(|e| SubmissionError::UploadError {
            message: format!("Error receiving data: {}", e),
        })?;

    let field = match file_name {
        Some(file_name) => UploadFieldValue::File { file_name, data },
        None => UploadFieldValue::Text {
            value: std::str::from_utf8(&data)
                .map_err(|e| SubmissionError::UploadError {
                    message: format!("Error decoding field to utf-8: {}", e),
                })?
                .to_string(),
        },
    };

    Ok((name, field))
}

#[derive(Debug)]
enum UploadFieldValue {
    Text { value: String },
    File { file_name: String, data: Vec<u8> },
}

impl UploadFieldValue {
    fn value(&self) -> Option<&str> {
        match self {
            UploadFieldValue::Text { value } => Some(value),
            _ => None,
        }
    }

    fn file_name(&self) -> Option<&str> {
        match self {
            UploadFieldValue::File { file_name, data: _ } => Some(file_name),
            _ => None,
        }
    }

    fn into_file_data(self) -> Option<Vec<u8>> {
        match self {
            UploadFieldValue::File { file_name: _, data } => Some(data),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum SubmissionError {
    UploadError { message: String },
    MissingField { name: &'static str },
}

impl warp::reject::Reject for SubmissionError {}
