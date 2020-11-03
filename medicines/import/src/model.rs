use azure_sdk_core::errors::AzureError;
use thiserror::Error;

#[derive(Debug)]
pub enum DocType {
    Par,
    Pil,
    Spc,
}

#[derive(Error, Debug)]
pub enum ImportError {
    #[error(transparent)]
    AzureError(#[from] AzureError),
    #[error("Could not open worksheet: {0}")]
    WorkbookOpenError(String),
    #[error("Could not open file: {0}")]
    FileOpenError(String),
}
