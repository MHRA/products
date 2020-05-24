use crate::{
    create_manager::models::BlobMetadata,
    storage_client::{AzureBlobStorage, StorageClient},
};
// use anyhow::anyhow;
use chrono::{DateTime, Utc};

pub async fn log_file_upload(
    blob_name: &String,
    uploader_source: &String,
    metadata: &BlobMetadata,
) -> Result<(), anyhow::Error> {
    let log_storage_client = AzureBlobStorage::log();
    let file_name = get_log_file_name(Utc::now());
    let contents = get_log_line(blob_name, uploader_source, metadata);
    let _ = log_storage_client.append_to_file(file_name, &contents.as_bytes());
    Ok(())
}

fn get_log_line(blob_name: &String, uploader_source: &String, metadata: &BlobMetadata) -> String {
    format!("{},{},{:?}", blob_name, uploader_source, metadata)
}

fn get_log_file_name(date: DateTime<Utc>) -> String {
    date.format("file-change-log-%Y-%m").to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_get_log_file_name() {
        let date = chrono::DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap(),
        );
        let log_file_name = get_log_file_name(date);
        assert_eq!(log_file_name, "file-change-log-1996-12".to_string());
    }

    #[test]
    fn test_get_log_line() {}
}
