use azure_sdk_core::errors::AzureError;

pub struct StorageFile {
    pub name: String,
    pub path: String,
}

#[derive(Debug)]
pub enum StorageClientError {
    RetrievalError { message: String },
    UploadError { message: String },
    ClientError { message: String },
}

impl From<AzureError> for StorageClientError {
    fn from(e: AzureError) -> Self {
        Self::ClientError {
            message: format!("Azure error: {:?}", e),
        }
    }
}

pub struct BlobResponse {
    pub blob_name: String,
    pub data: Vec<u8>,
}
