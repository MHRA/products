use bytes::BufMut;
use futures::future::join_all;
use futures::TryStreamExt;
use warp::multipart::{FormData, Part};

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub value: UploadFieldValue,
}

pub async fn collect_fields(form_data: FormData) -> anyhow::Result<Vec<Field>> {
    let parts: Vec<Part> = form_data.try_collect().await?;

    join_all(parts.into_iter().map(Field::from_upload_part))
        .await
        .into_iter()
        .collect::<Result<_, _>>()
}

impl Field {
    async fn from_upload_part(part: Part) -> anyhow::Result<Self> {
        let name = part.name().to_string();

        let file_name = part.filename().map(|s| s.to_string());

        let data = part
            .stream()
            .try_fold(Vec::new(), |mut vec, data| {
                vec.put(data);
                async move { Ok(vec) }
            })
            .await?;

        let value = match file_name {
            Some(file_name) => UploadFieldValue::File { file_name, data },
            None => UploadFieldValue::Text {
                value: std::str::from_utf8(&data)?.to_string(),
            },
        };

        Ok(Field { name, value })
    }
}

#[derive(Debug)]
pub enum UploadFieldValue {
    Text { value: String },
    File { file_name: String, data: Vec<u8> },
}

impl UploadFieldValue {
    pub fn value(&self) -> Option<&str> {
        match self {
            UploadFieldValue::Text { value } => Some(value),
            _ => None,
        }
    }

    pub fn file_name(&self) -> Option<&str> {
        match self {
            UploadFieldValue::File { file_name, data: _ } => Some(file_name),
            _ => None,
        }
    }

    pub fn into_file_data(self) -> Option<Vec<u8>> {
        match self {
            UploadFieldValue::File { file_name: _, data } => Some(data),
            _ => None,
        }
    }
}
