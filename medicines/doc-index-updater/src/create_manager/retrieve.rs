use crate::{
    models::FileSource,
    storage_client::{models::StorageClientError, AzureBlobStorage, GetBlob, SftpClient},
};

pub async fn retrieve(source: FileSource, filepath: String) -> Result<Vec<u8>, StorageClientError> {
    let a = match source {
        FileSource::Sentinel => SftpClient::sentinel().await.get_blob(&filepath).await?,
        FileSource::TemporaryAzureBlobStorage { uploader_email: _ } => {
            AzureBlobStorage::temporary().get_blob(&filepath).await?
        }
    };
    Ok(a.data)
}
