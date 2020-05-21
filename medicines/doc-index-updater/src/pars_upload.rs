use crate::{
    create_manager::models::BlobMetadata,
    document_manager::{accept_job, check_in_document_handler},
    models::{Document, FileSource, JsonWebToken},
    state_manager::{with_state, JobStatusClient, StateManager},
    storage_client::{models::StorageFile, AzureBlobStorage, StorageClient},
};
use serde::{Deserialize, Serialize};

use bytes::BufMut;
use futures::future::join_all;
use futures::TryStreamExt;
use jsonwebtoken::{dangerous_unsafe_decode, errors::Error};
use search_client::models::DocumentType;

use std::collections::HashMap;
use uuid::Uuid;
use warp::{
    filters::multipart::{FormData, Part},
    http::{header, Method},
    Filter, Rejection, Reply,
};

pub fn handler(
    state_manager: StateManager,
    pars_origin: &str,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let cors = warp::cors()
        .allow_origin(pars_origin)
        .allow_headers(&[header::AUTHORIZATION])
        .allow_methods(&[Method::POST])
        .build();

    warp::path!("pars")
        .and(warp::post())
        // Max upload size is set to a very high limit here as the actual limit should be managed using istio
        .and(warp::multipart::form().max_length(1000 * 1024 * 1024))
        .and(with_state(state_manager))
        .and(warp::header("Authorization"))
        .and_then(upload_pars_handler)
        .with(cors)
}

async fn add_file_to_temporary_blob_storage(
    _job_id: Uuid,
    file_data: &[u8],
    license_number: &str,
) -> Result<StorageFile, SubmissionError> {
    let storage_client = AzureBlobStorage::temporary();
    let storage_file = storage_client
        .add_file(file_data, license_number, HashMap::new())
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
    form_data: FormData,
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

        check_in_document_handler(document, &state_manager).await?;
    }

    Ok(job_ids)
}

async fn upload_pars_handler(
    form_data: FormData,
    state_manager: StateManager,
    authorization_header: String,
) -> Result<impl Reply, Rejection> {
    let request_id = Uuid::new_v4();
    let span = tracing::info_span!("PARS upload", request_id = request_id.to_string().as_str());
    let _enter = span.enter();
    tracing::debug!("Received PARS submission");

    let json_web_token = decode_token_from_authorization_header(authorization_header);

    match json_web_token {
        Ok(token) => {
            tracing::info!("Uploader email: {}", token.email);
        }
        Err(e) => {
            tracing::error!("Error decoding token: {:?}", e);
        }
    }

    let job_ids = queue_pars_upload(form_data, state_manager).await?;

    Ok(warp::reply::json(&UploadResponse { job_ids }))
}

#[derive(Debug, Serialize)]
struct UploadResponse {
    job_ids: Vec<Uuid>,
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
struct Field {
    name: String,
    value: UploadFieldValue,
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
    let product_name = get_field_as_string(&fields, "product_name")?;

    let product_names = vec![product_name];

    let title = get_field_as_string(&fields, "title")?;
    let pl_number = get_field_as_string(&fields, "license_number")?;

    let active_substances = fields
        .iter()
        .filter(|field| field.name == "active_substance")
        .filter_map(|field| field.value.value())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

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

fn get_field_as_string(
    fields: &[Field],
    field_name: &'static str,
) -> Result<String, SubmissionError> {
    fields
        .iter()
        .find(|field| field.name == field_name)
        .and_then(|field| field.value.value())
        .ok_or(SubmissionError::MissingField { name: field_name })
        .map(|s| s.to_string())
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

fn decode_token_from_authorization_header(
    authorization_header: String,
) -> Result<JsonWebToken, Error> {
    let token = authorization_header.split("Bearer ").collect::<Vec<&str>>()[1];
    let token_message = dangerous_unsafe_decode::<Claims>(&token)?;
    tracing::debug!("{:?}", token_message);

    Ok(JsonWebToken {
        email: token_message.claims.preferred_username,
    })
}
#[derive(Debug)]
enum SubmissionError {
    UploadError { message: String },
    MissingField { name: &'static str },
}

impl warp::reject::Reject for SubmissionError {}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    preferred_username: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn decode_a_jwt_token_from_authorization_header() {
        let token = decode_token_from_authorization_header(String::from("Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiIyNmY5NWIyMS02M2IyLTQ3NWYtOGEzNS1kMzljZWE0Y2ZkNjEiLCJpc3MiOiJodHRwczovL2xvZ2luLm1pY3Jvc29mdG9ubGluZS5jb20vZTUyN2VhNWMtNjI1OC00Y2QyLWEyN2YtOGJkMjM3ZWM0YzI2L3YyLjAiLCJpYXQiOjE1ODk4MDg3MzEsIm5iZiI6MTU4OTgwODczMSwiZXhwIjoxNTkwMDU5NjI0LCJhaW8iOiJBVlFBcS84UEFBQUFLSFNQZXNxRHdzTzAvaDhpNHloMVAzeHBZN1NiRkNpVWNmVDlJeVQxUE1zNnVhdW5TQkE4aUdmZHFYU0tzTDQ5aUpaMFkzTXRtZm8vaVN5QXJkYklkL041MUhiSzI4R2tURm9mSmY1bGpOOD0iLCJoYXNncm91cHMiOiJ0cnVlIiwibmFtZSI6IkJsb2dncywgSm9lIiwibm9uY2UiOiJiNzI2Y2M4Zi03ODBlLTRhZmUtYTE2Yy0wYTlmMzUxN2ZiNDYiLCJvaWQiOiI5NGI3ODVhYi1mNDhmLTRmNmYtYjMyZi05ZWJlMjFmNmIwZWQiLCJwcmVmZXJyZWRfdXNlcm5hbWUiOiJKb2UuQmxvZ2dzQE1pY3Jvc29mdC5jb20iLCJzdWIiOiJtOFFYbXdKQVpGWDZDQ1k3NU1jV3pRUkMzcmVhOGRnbDJuNHRKbnNFNVowIiwidGlkIjoiZTUyN2VhNWMtNjI1OC00Y2QyLWEyN2YtOGJkMjM3ZWM0YzI2IiwidXRpIjoiT0JJQXhoLXFRa0dBb29zTVZRMU1BQSIsInZlciI6IjIuMCIsImp0aSI6ImNlMmM5NjA1LWM5MjAtNGJkMC04NWMwLTBlOTA0MGZlMTQ5MCJ9.KWRK8mVdljCG3hpYHMSVrhIipBIq4Z_wfM9zzKAlYPw"));
        assert_eq!(
            token.unwrap().email,
            String::from("Joe.Bloggs@Microsoft.com")
        );
    }

    #[test]
    fn decode_a_badly_formatted_jwt_token_from_authorization_header() {
        let token = decode_token_from_authorization_header(String::from("Bearer xxxx"));
        assert!(token.is_err());
    }
}
