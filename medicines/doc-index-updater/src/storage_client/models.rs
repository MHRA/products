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
