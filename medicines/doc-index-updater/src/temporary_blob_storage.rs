pub trait StorageClient {
    fn add_file(self, file_data: &Vec<u8>) -> StorageFile;
    fn get_file(self, storage_file_identifier: StorageFile) -> Vec<u8>;
}

pub struct TemporaryBlobStorage {}

pub struct StorageFile {
    pub path: String,
}

impl StorageClient for TemporaryBlobStorage {}
