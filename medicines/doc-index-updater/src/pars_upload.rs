use crate::{
    create_manager::{create_blob, models::BlobMetadata, Blob},
    document_manager::{accept_job, check_in_document_handler},
    models::{Document, FileSource},
    state_manager::{with_state, JobStatusClient, StateManager},
    storage_client,
    temporary_blob_storage::TemporaryBlobStorage,
    temporary_blob_storage::{StorageClient, StorageFile},
};

use bytes::BufMut;
use futures::future::join_all;
use futures::TryStreamExt;
use search_client::models::DocumentType;
use serde::Serialize;
use storage_client::BlobClient;
use uuid::Uuid;
use warp::{
    filters::multipart::{FormData, Part},
    Filter, Rejection, Reply,
};

pub fn handler(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("pars")
        .and(warp::post())
        .and(warp::multipart::form().max_length(100 * 1024 * 1024))
        .and(with_state(state_manager))
        .and_then(upload_pars_handler)
}

fn storage_client_factory() -> Result<BlobClient, SubmissionError> {
    let client = storage_client::factory().map_err(|e| {
        tracing::error!("Error creating storage client: {:?}", e);
        SubmissionError::UploadError {
            message: format!("Couldn't create storage client: {:?}", e),
        }
    })?;

    Ok(client)
}

async fn add_form_to_temporary_blob_storage(
    _job_id: Uuid,
    form_data: FormData,
) -> Result<(StorageFile, BlobMetadata), SubmissionError> {
    let storage_client = storage_client_factory()?.azure_client;

    let (metadata, file_data) = read_pars_upload(form_data).await.map_err(|e| {
        tracing::debug!("Error reading PARS upload: {:?}", e);
        e
    })?;

    let storage_client = TemporaryBlobStorage {};
    let blob = storage_client.add_file(&file_data);
    // create_blob(
    //     &storage_client,
    //     &file_data,
    //     metadata,
    //     Some("temp".to_owned()),
    // )
    // .await
    // .map_err(|e| {
    //     tracing::error!("Error uploading file to blob storage: {:?}", e);
    //     SubmissionError::UploadError {
    //         message: format!("Couldn't create blob: {:?}", e),
    //     }
    // })?;

    Ok((blob, metadata))
}

fn document_from_form_data(storage_file: StorageFile, metadata: BlobMetadata) -> Document {
    Document {
        id: metadata.file_name.to_string(),
        name: metadata.title.to_string(),
        document_type: DocumentType::Par,
        author: metadata.author.to_string(),
        products: metadata.product_names.to_vec_string(),
        keywords: match metadata.keywords {
            Some(a) => Some(a.to_vec_string()),
            None => None,
        },
        pl_number: metadata.pl_number,
        active_substances: metadata.active_substances.to_vec_string(),
        file_source: FileSource::TemporaryAzureBlobStorage,
        file_path: storage_file.path,
    }
}

async fn queue_pars_upload(
    job_id: Uuid,
    form_data: FormData,
    state_manager: impl JobStatusClient,
) -> Result<(), SubmissionError> {
    let (blob, metadata) = add_form_to_temporary_blob_storage(job_id, form_data).await?;
    let document = document_from_form_data(blob, metadata);

    let _ = check_in_document_handler(document, state_manager).await;
    Ok(())
}

async fn upload_pars_handler(
    form_data: FormData,
    state_manager: StateManager,
) -> Result<impl Reply, Rejection> {
    tracing::debug!("Received PARS submission");

    let job_id = accept_job(&state_manager).await?.id;

    let _ = tokio::join!(tokio::spawn(queue_pars_upload(
        job_id,
        form_data,
        state_manager.clone(),
    )),);

    Ok(warp::reply::json(&UploadResponse {
        job_id: &job_id.to_string(),
    }))
}

#[derive(Debug, Serialize)]
struct UploadResponse<'a> {
    job_id: &'a str,
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
