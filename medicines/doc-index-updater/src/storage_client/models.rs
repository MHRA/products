use azure_sdk_core::errors::AzureError;
use thiserror::Error;

pub struct StorageFile {
    pub name: String,
    pub path: String,
}

#[derive(Error, Debug)]
pub enum StorageClientError {
    #[error(transparent)]
    SftpError(#[from] SftpError),
    #[error("Could not retrieve: {0}")]
    RetrievalError(String),
    #[error("Could not upload: {0}")]
    UploadError(String),
    #[error("Could not create client: {0}")]
    ClientError(String),

    #[error(transparent)]
    OtherAzureError(#[from] AzureError),
    #[error(transparent)]
    Generic(#[from] anyhow::Error),
}

pub struct BlobResponse {
    pub blob_name: String,
    pub data: Vec<u8>,
}

#[derive(Error, Debug)]
pub enum SftpError {
    #[error("A TCP error connecting to server. ({0:?})")]
    TcpError(#[from] std::io::Error),
    #[error("An SSH error connecting to server. ({0:?})")]
    Ssh2Error(#[from] async_ssh2::Error),
    #[error("File could not be retrieved on server")]
    CouldNotRetrieveFile,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
