use crate::{
    create_manager::models::BlobMetadata,
    document_manager::{accept_job, check_in_document_handler},
    models::{Document, FileSource},
    state_manager::{with_state, JobStatusClient, StateManager},
    storage_client::{models::StorageFile, AzureBlobStorage, StorageClient},
};
use bytes::BufMut;
use futures::future::join_all;
use futures::TryStreamExt;
use search_client::models::DocumentType;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;
use warp::{
    filters::multipart::{FormData, Part},
    http::{header, Method},
    Filter, Rejection, Reply,
};

pub fn handler(
    state_manager: StateManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin() // TODO restrict to a specific domain once we know what it is
        .allow_headers(&[header::AUTHORIZATION])
        .allow_methods(&[Method::POST])
        .build();

    warp::path!("pars")
        .and(warp::post())
        // Max upload size is set to a very high limit here as the actual limit should be managed using istio
        .and(warp::multipart::form().max_length(1000 * 1024 * 1024))
        .and(with_state(state_manager))
        .and_then(upload_pars_handler)
        .with(cors)
}

async fn add_file_to_temporary_blob_storage(
    _job_id: Uuid,
    file_data: &[u8],
) -> Result<StorageFile, SubmissionError> {
    let storage_client = AzureBlobStorage::temporary();
    let storage_file = storage_client
        .add_file(file_data, HashMap::new())
        .await
        .map_err(|e| SubmissionError::UploadError {
            message: format!("Problem talking to temporary blob storage: {:?}", e),
        })?;
    Ok(storage_file)
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
        file_path: storage_file.name,
    }
}

async fn queue_pars_upload(
    job_id: Uuid,
    form_data: FormData,
    state_manager: impl JobStatusClient,
) -> Result<(), Rejection> {
    let (metadatas, file_data) = read_pars_upload(form_data).await.map_err(|e| {
        tracing::debug!("Error reading PARS upload: {:?}", e);
        warp::reject::custom(e)
    })?;

    for metadata in metadatas {
        let storage_file = add_file_to_temporary_blob_storage(job_id, &file_data)
            .await
            .map_err(warp::reject::custom)?;

        let document = document_from_form_data(storage_file, metadata);

        check_in_document_handler(document, &state_manager).await?;
    }

    Ok(())
}

async fn upload_pars_handler(
    form_data: FormData,
    state_manager: StateManager,
) -> Result<impl Reply, Rejection> {
    tracing::debug!("Received PARS submission");

    let job_id = accept_job(&state_manager).await?.id;

    let span = tracing::info_span!("Queueing PARS upload", job_id = job_id.to_string().as_str());
    let _enter = span.enter();

    queue_pars_upload(job_id, form_data, state_manager).await?;

    Ok(warp::reply::json(&UploadResponse {
        job_id: &job_id.to_string(),
    }))
}

#[derive(Debug, Serialize)]
struct UploadResponse<'a> {
    job_id: &'a str,
}

async fn read_pars_upload(
    form_data: FormData,
) -> Result<(Vec<BlobMetadata>, Vec<u8>), SubmissionError> {
    let parts: Vec<Part> =
        form_data
            .try_collect()
            .await
            .map_err(|e| SubmissionError::UploadError {
                message: format!("Error receiving data: {}", e),
            })?;

    let fields: Vec<Field> = join_all(parts.into_iter().map(read_upload_part))
        .await
        .into_iter()
        .collect::<Result<_, _>>()?;

    let GroupedFields {
        products,
        file_data,
    } = groups_fields_by_product(fields)?;

    let metadatas = products
        .into_iter()
        .map(product_form_data_to_blob_metadata)
        .collect::<Result<_, _>>()?;

    Ok((metadatas, file_data))
}

#[derive(Debug)]
struct Field {
    name: String,
    value: UploadFieldValue,
}

#[derive(Debug)]
struct GroupedFields {
    products: Vec<Vec<Field>>,
    file_data: Vec<u8>,
}

fn groups_fields_by_product(fields: Vec<Field>) -> Result<GroupedFields, SubmissionError> {
    let mut products = Vec::new();
    let mut file_field = None;

    for field in fields {
        if field.name == "file" {
            file_field = Some(field.value);
            continue;
        }

        if field.name == "product_name" {
            products.push(vec![]);
        }

        match products.last_mut() {
            Some(group) => {
                group.push(field);
            }
            None => {
                let group = vec![field];
                products.push(group);
            }
        }
    }

    let file_data = file_field
        .and_then(|field| field.into_file_data())
        .ok_or(SubmissionError::MissingField { name: "file" })?;

    Ok(GroupedFields {
        products,
        file_data,
    })
}

fn product_form_data_to_blob_metadata(fields: Vec<Field>) -> Result<BlobMetadata, SubmissionError> {
    let file_name = fields
        .iter()
        .find(|field| field.name == "file")
        .and_then(|field| field.value.file_name())
        .ok_or(SubmissionError::MissingField { name: "file" })?
        .into();

    let product_name = fields
        .iter()
        .find(|field| field.name == "product_name")
        .and_then(|field| field.value.value())
        .ok_or(SubmissionError::MissingField {
            name: "product_name",
        })?
        .into();

    let product_names = vec![product_name];

    let active_substances = fields
        .iter()
        .filter(|field| field.name == "active_substance")
        .filter_map(|field| field.value.value())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let title = fields
        .iter()
        .find(|field| field.name == "title")
        .and_then(|field| field.value.value())
        .ok_or(SubmissionError::MissingField { name: "title" })?
        .into();

    let pl_number = fields
        .iter()
        .find(|field| field.name == "license_number")
        .and_then(|field| field.value.value())
        .ok_or(SubmissionError::MissingField {
            name: "license_number",
        })?
        .into();

    let author = "".to_string();

    Ok(BlobMetadata::new(
        file_name,
        DocumentType::Par,
        title,
        pl_number,
        product_names,
        active_substances,
        author,
        None,
    ))
}

async fn read_upload_part(part: Part) -> Result<Field, SubmissionError> {
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

    let value = match file_name {
        Some(file_name) => UploadFieldValue::File { file_name, data },
        None => UploadFieldValue::Text {
            value: std::str::from_utf8(&data)
                .map_err(|e| SubmissionError::UploadError {
                    message: format!("Error decoding field to utf-8: {}", e),
                })?
                .to_string(),
        },
    };

    Ok(Field { name, value })
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
