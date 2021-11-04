use crate::{
    create_manager::models::BlobMetadata,
    document_manager::{accept_job, check_in_document_handler, delete_document_handler},
    models::{Document, FileSource, JobStatusResponse, UniqueDocumentIdentifier},
    multipart_form_data::{collect_fields, Field},
    state_manager::{with_state, JobStatusClient, StateManager},
    storage_client::{models::StorageFile, AzureBlobStorage, StorageClient},
};
use search_client::models::{DocumentType, TerritoryType, TerritoryTypeParseError};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use uuid::Uuid;
use warp::{
    http::{header, Method},
    multipart::FormData,
    Filter, Rejection, Reply,
};

pub fn update_handler(
    state_manager: StateManager,
    pars_origin: &str,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let cors = warp::cors()
        .allow_origin(pars_origin)
        .allow_headers(&[header::AUTHORIZATION])
        .allow_methods(&[Method::POST])
        .build();

    warp::path!("pars" / String)
        .and(warp::post())
        // Max upload size is set to a very high limit here as the actual limit should be managed using istio
        .and(warp::multipart::form().max_length(1000 * 1024 * 1024))
        .and(with_state(state_manager))
        .and(warp::header("username"))
        .and_then(update_pars_handler)
        .with(cors)
}

pub fn handler(
    state_manager: StateManager,
    pars_origin: &str,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let cors = warp::cors()
        .allow_origin(pars_origin)
        .allow_headers(&[
            header::AUTHORIZATION,
            header::HeaderName::from_bytes(b"username").unwrap(),
        ])
        .allow_methods(&[Method::POST])
        .build();

    warp::path!("pars")
        .and(warp::post())
        // Max upload size is set to a very high limit here as the actual limit should be managed using istio
        .and(warp::multipart::form().max_length(1000 * 1024 * 1024))
        .and(with_state(state_manager))
        .and(warp::header("username"))
        .and_then(upload_pars_handler)
        .with(cors)
}

async fn add_file_to_temporary_blob_storage(
    _job_id: Uuid,
    file_data: &[u8],
    licence_number: &str,
) -> Result<StorageFile, SubmissionError> {
    let storage_client = AzureBlobStorage::temporary();
    let storage_file = storage_client
        .add_file(file_data, licence_number, HashMap::new())
        .await
        .map_err(|e| SubmissionError::BlobStorageError {
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
        territory: metadata.territory,
        active_substances: metadata.active_substances.to_vec_string(),
        file_source: FileSource::TemporaryAzureBlobStorage,
        file_path: storage_file.name,
    }
}

async fn queue_pars_upload(
    form_data: FormData,
    uploader_email: String,
    state_manager: impl JobStatusClient,
) -> Result<Vec<Uuid>, Rejection> {
    let (metadatas, file_data) = read_pars_upload(form_data).await.map_err(|e| {
        tracing::debug!("Error reading PARS upload: {:?}", e);
        warp::reject::custom(e)
    })?;

    let mut job_ids = Vec::with_capacity(metadatas.len());

    for metadata in metadatas {
        let job_id = accept_job(&state_manager).await?.id;

        job_ids.push(job_id);

        let storage_file =
            add_file_to_temporary_blob_storage(job_id, &file_data, &metadata.pl_number)
                .await
                .map_err(warp::reject::custom)?;

        let document = document_from_form_data(storage_file, metadata);

        check_in_document_handler(document, &state_manager, Some(uploader_email.clone())).await?;
    }

    Ok(job_ids)
}

async fn update_pars_handler(
    existing_par_identifier: String,
    form_data: FormData,
    state_manager: StateManager,
    username: String,
) -> Result<impl Reply, Rejection> {
    let delete = delete_document_handler(
        UniqueDocumentIdentifier::MetadataStorageName(existing_par_identifier),
        &state_manager,
        Some(username.clone()),
    )
    .await?;

    let upload = queue_upload_pars_job(form_data, state_manager, username).await?;

    Ok(warp::reply::json(&UpdateResponse { delete, upload }))
}

async fn upload_pars_handler(
    form_data: FormData,
    state_manager: StateManager,
    username: String,
) -> Result<impl Reply, Rejection> {
    let job_ids = queue_upload_pars_job(form_data, state_manager, username).await?;
    Ok(warp::reply::json(&UploadResponse { job_ids }))
}

async fn queue_upload_pars_job(
    form_data: FormData,
    state_manager: StateManager,
    username: String,
) -> Result<Vec<Uuid>, Rejection> {
    let request_id = Uuid::new_v4();
    let span = tracing::info_span!("PARS upload", request_id = request_id.to_string().as_str());
    let _enter = span.enter();
    tracing::debug!("Received PARS submission");

    tracing::info!("Uploader email: {}", username);

    Ok(queue_pars_upload(form_data, username, state_manager).await?)
}

#[derive(Debug, Serialize)]
struct UploadResponse {
    job_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
struct UpdateResponse {
    delete: JobStatusResponse,
    upload: Vec<Uuid>,
}

async fn read_pars_upload(
    form_data: FormData,
) -> Result<(Vec<BlobMetadata>, Vec<u8>), SubmissionError> {
    let fields = collect_fields(form_data)
        .await
        .map_err(|error| SubmissionError::UploadError { error })?;

    let GroupedFields {
        products,
        file_name,
        file_data,
    } = groups_fields_by_product(fields)?;

    let metadatas = products
        .into_iter()
        .map(|fields| product_form_data_to_blob_metadata(file_name.clone(), fields))
        .collect::<Result<_, _>>()?;

    Ok((metadatas, file_data))
}

#[derive(Debug)]
struct GroupedFields {
    products: Vec<Vec<Field>>,
    file_name: String,
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

    let file_name = file_field
        .as_ref()
        .and_then(|field| field.file_name())
        .ok_or(SubmissionError::MissingField { name: "file" })?
        .to_string();

    let file_data = file_field
        .and_then(|field| field.into_file_data())
        .ok_or(SubmissionError::MissingField { name: "file" })?;

    Ok(GroupedFields {
        products,
        file_name,
        file_data,
    })
}

fn product_form_data_to_blob_metadata(
    file_name: String,
    fields: Vec<Field>,
) -> Result<BlobMetadata, SubmissionError> {
    let product_name = get_field_as_uppercase_string(&fields, "product_name")?;

    let product_names = vec![product_name];

    let title = get_field_as_uppercase_string(&fields, "title")?;
    let pl_number = get_field_as_uppercase_string(&fields, "licence_number")?;

    let active_substances = fields
        .iter()
        .filter(|field| field.name == "active_substance")
        .filter_map(|field| field.value.value())
        .map(|s| s.to_uppercase())
        .collect::<Vec<String>>();

    let territory = fields
        .iter()
        .find(|field| field.name == "territory")
        .and_then(|field| field.value.value())
        .map(|s| TerritoryType::from_str(s))
        .transpose()?;

    let author = "".to_string();

    Ok(BlobMetadata::new(
        file_name,
        DocumentType::Par,
        title,
        pl_number,
        territory,
        product_names,
        active_substances,
        author,
        None,
    ))
}

fn get_field_as_uppercase_string(
    fields: &[Field],
    field_name: &'static str,
) -> Result<String, SubmissionError> {
    fields
        .iter()
        .find(|field| field.name == field_name)
        .and_then(|field| field.value.value())
        .ok_or(SubmissionError::MissingField { name: field_name })
        .map(|s| s.to_uppercase())
}

#[derive(Debug)]
enum SubmissionError {
    UploadError {
        error: anyhow::Error,
    },
    BlobStorageError {
        message: String, // should maybe be StorageClientError but that is not
                         // Send + Sync so then we can't implement warp::reject::Reject
    },
    MissingField {
        name: &'static str,
    },
    UnknownTerritoryType {
        error: TerritoryTypeParseError,
    },
}

impl From<TerritoryTypeParseError> for SubmissionError {
    fn from(error: TerritoryTypeParseError) -> Self {
        SubmissionError::UnknownTerritoryType { error }
    }
}

impl warp::reject::Reject for SubmissionError {}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    preferred_username: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multipart_form_data::UploadFieldValue;
    use pretty_assertions::assert_eq;

    fn text_field(name: &str, value: &str) -> Field {
        Field {
            name: name.into(),
            value: UploadFieldValue::Text {
                value: value.into(),
            },
        }
    }

    #[test]
    fn converts_form_data_to_metadata() {
        let file_name = "file";
        let result = product_form_data_to_blob_metadata(
            file_name.into(),
            vec![
                text_field("product_name", "Feel good pills"),
                text_field("active_substance", "Ibuprofen"),
                text_field("active_substance", "Temazepam"),
                text_field(
                    "title",
                    "Feel good pills Really Strong High Dose THR 12345/1234",
                ),
                text_field("licence_number", "THR 12345/1234"),
                text_field("territory", "UK"),
            ],
        )
        .unwrap();

        assert_eq!(
            result,
            BlobMetadata {
                file_name: file_name.into(),
                doc_type: DocumentType::Par,
                title: "FEEL GOOD PILLS REALLY STRONG HIGH DOSE THR 12345/1234".into(),
                pl_number: "THR 12345/1234".into(),
                territory: Some(TerritoryType::UK),
                product_names: vec!["FEEL GOOD PILLS".into()].into(),
                active_substances: vec!["IBUPROFEN".into(), "TEMAZEPAM".into()].into(),
                author: "".into(),
                keywords: None
            }
        )
    }
}
