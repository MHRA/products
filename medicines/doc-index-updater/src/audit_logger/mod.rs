use crate::{
    models::Message,
    storage_client::{AzureBlobStorage, StorageClient},
};
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use core::fmt::Debug;
pub struct AuditLogger {}

// pub trait RemovableMessage<T: Message>: Removable {
//     fn get_message(&self) -> T;
// }

// impl<T> RemovableMessage<T> for RetrievedMessage<T>
// where
//     T: Message + Sync + Send,
// {
//     fn get_message(&self) -> T {
//         self.message.clone()
//     }
// }

#[async_trait]
pub trait LogTransaction<'a, T: Message> {
    async fn log_transaction(blob_name: &str, log_contents: &'a T) -> Result<(), anyhow::Error>;
}

#[async_trait]
impl<'a, T> LogTransaction<'a, T> for AuditLogger
where
    T: Message + Sync + Send + Debug,
{
    async fn log_transaction(blob_name: &str, log_contents: &'a T) -> Result<(), anyhow::Error> {
        let log_storage_client = AzureBlobStorage::log();
        let datetime_now = Utc::now();
        let file_name = get_log_file_name(&datetime_now);
        let body = get_log_body(blob_name, log_contents, &datetime_now);
        log_storage_client
            .append_to_file(file_name, &body.as_bytes())
            .await
            .map_err(|e| {
                eprintln!("Error appending to blob: {:?}", e);
                anyhow!("Error appending to blob")
            })
    }
}

fn get_log_body<Message>(
    blob_name: &str,
    log_contents: Message,
    datetime_now: &DateTime<Utc>,
) -> String {
    format!(
        "{},{},{:?}\n",
        blob_name,
        datetime_now.format("%Y-%m-%d %H:%M:%S"),
        log_contents
    )
}

fn get_log_file_name(date: &DateTime<Utc>) -> String {
    date.format("file-change-log-%Y-%m").to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::{CreateMessage, DeleteMessage, Document, FileSource};
    use search_client::models::DocumentType;
    use uuid::Uuid;

    #[test]
    fn test_get_log_file_name() {
        let date = chrono::DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339("1996-12-19T16:39:57-00:00").unwrap(),
        );
        let log_file_name = get_log_file_name(&date);
        assert_eq!(log_file_name, "file-change-log-1996-12".to_string());
    }

    fn get_create_message() -> CreateMessage {
        let job_id = Uuid::parse_str(&"739b7840-a1e9-42eb-8013-0120cdf066bc").unwrap();
        let document = Document {
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
        let initiator_email = Some("example@email.com".to_string());
        CreateMessage {
            document,
            job_id,
            initiator_email,
        }
    }

    fn get_delete_message() -> DeleteMessage {
        let job_id = Uuid::parse_str(&"739b7840-a1e9-42eb-8013-0120cdf066bc").unwrap();
        let document_content_id = "CON123456789".to_string();
        let initiator_email = Some("example@email.com".to_string());
        DeleteMessage {
            job_id,
            document_content_id,
            initiator_email,
        }
    }

    #[test]
    fn test_get_log_line_for_create_message() {
        let blob_name = "1kdlkjd1229ui09askjsadkl12da".to_string();
        let date = chrono::DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339("1996-12-19T16:39:57-00:00").unwrap(),
        );
        let message = get_create_message();
        let expected = "1kdlkjd1229ui09askjsadkl12da,1996-12-19 16:39:57,CreateMessage { job_id: 739b7840-a1e9-42eb-8013-0120cdf066bc, document: Document { id: \"CON123456\", name: \"Paracetamol Plus PL 12345/6789\", document_type: Spc, author: \"JRR Tolkien\", products: [\"Effective product 1\", \"Effective product 2\"], keywords: Some([\"Very good for you\", \"Cures headaches\", \"PL 12345/6789\"]), pl_number: \"PL 12345/6789\", active_substances: [\"Paracetamol\", \"Caffeine\"], file_source: TemporaryAzureBlobStorage, file_path: \"location/on/disk\" }, initiator_email: Some(\"example@email.com\") }\n".to_string();

        let actual = get_log_body(&blob_name, message, &date);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_log_line_for_delete_message() {
        let blob_name = "1kdlkjd1229ui09askjsadkl12da".to_string();
        let date = chrono::DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339("1996-12-19T16:39:57-00:00").unwrap(),
        );
        let message = get_delete_message();
        let expected = "1kdlkjd1229ui09askjsadkl12da,1996-12-19 16:39:57,DeleteMessage { job_id: 739b7840-a1e9-42eb-8013-0120cdf066bc, document_content_id: \"CON123456789\", initiator_email: Some(\"example@email.com\") }\n".to_string();

        let actual = get_log_body(&blob_name, message, &date);

        assert_eq!(actual, expected);
    }
}
