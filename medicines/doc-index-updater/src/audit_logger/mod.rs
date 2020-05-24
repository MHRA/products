use crate::{
    models::Document,
    storage_client::{AzureBlobStorage, StorageClient},
};
// use anyhow::anyhow;
use chrono::{DateTime, Utc};

pub async fn log_file_upload(blob_name: &String, document: Document) -> Result<(), anyhow::Error> {
    let log_storage_client = AzureBlobStorage::log();
    let datetime_now = Utc::now();
    let file_name = get_log_file_name(&datetime_now);
    let body = get_log_file_upload_body(blob_name, document, &datetime_now);
    let _ = log_storage_client.append_to_file(file_name, &body.as_bytes());
    Ok(())
}

fn get_log_file_upload_body(
    blob_name: &String,
    document: Document,
    datetime_now: &DateTime<Utc>,
) -> String {
    format!(
        "{},{},{},{:?}",
        blob_name,
        datetime_now.format("%Y-%m-%d %H:%M:%S"),
        "CREATED",
        document
    )
}

// TODO: need to work out how to get source into delete message

pub async fn log_file_delete(blob_name: &String, document: Document) -> Result<(), anyhow::Error> {
    let log_storage_client = AzureBlobStorage::log();
    let datetime_now = Utc::now();
    let file_name = get_log_file_name(&datetime_now);
    let body = get_log_file_upload_body(blob_name, document, &datetime_now);
    let _ = log_storage_client.append_to_file(file_name, &body.as_bytes());
    Ok(())
}

fn get_log_file_name(date: &DateTime<Utc>) -> String {
    date.format("file-change-log-%Y-%m").to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::FileSource;
    use search_client::models::DocumentType;

    #[test]
    fn test_get_log_file_name() {
        let date = chrono::DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339("1996-12-19T16:39:57-00:00").unwrap(),
        );
        let log_file_name = get_log_file_name(&date);
        assert_eq!(log_file_name, "file-change-log-1996-12".to_string());
    }

    #[test]
    fn test_get_log_line_with_temporary_file_source() {
        let doc = Document {
            id: "CON123456".to_string(),
            name: "Paracetamol Plus PL 12345/6789".to_string(),
            document_type: DocumentType::Spc,
            author: "JRR Tolkien".to_string(),
            products: vec![
                "Effective product 1".to_string(),
                "Effective product 2".to_string(),
            ],
            keywords: Some(vec![
                "Very good for you".to_string(),
                "Cures headaches".to_string(),
                "PL 12345/6789".to_string(),
            ]),
            pl_number: "PL 12345/6789".to_string(),
            active_substances: vec!["Paracetamol".to_string(), "Caffeine".to_string()],
            file_path: "location/on/disk".to_string(),
            file_source: FileSource::TemporaryAzureBlobStorage,
        };
        let blob_name = "1kdlkjd1229ui09askjsadkl12da".to_string();
        let date = chrono::DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339("1996-12-19T16:39:57-00:00").unwrap(),
        );
        let expected = "1kdlkjd1229ui09askjsadkl12da,1996-12-19 16:39:57,Document { id: \"CON123456\", name: \"Paracetamol Plus PL 12345/6789\", document_type: Spc, author: \"JRR Tolkien\", products: [\"Effective product 1\", \"Effective product 2\"], keywords: Some([\"Very good for you\", \"Cures headaches\", \"PL 12345/6789\"]), pl_number: \"PL 12345/6789\", active_substances: [\"Paracetamol\", \"Caffeine\"], file_source: TemporaryAzureBlobStorage { uploader_email: \"example@email.com\" }, file_path: \"location/on/disk\" }".to_string();

        let actual = get_log_file_upload_body(&blob_name, doc, &date);

        assert_eq!(actual, expected);
    }
}
