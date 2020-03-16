use crate::{
    models::{CreateMessage, Document, FileProcessStatus, JobStatus},
    service_bus_client::{
        create_factory, DocIndexUpdaterQueue, RetrieveFromQueueError, RetrievedMessage,
    },
    state_manager::StateManager,
};
use anyhow::anyhow;
use std::{collections::HashMap, time::Duration};
use tokio::time::delay_for;

mod metadata;
mod sftp_client;

#[tracing::instrument]
pub async fn create_service_worker(
    time_to_wait: Duration,
    state_manager: StateManager,
) -> Result<String, anyhow::Error> {
    tracing::info!("Starting create service worker");
    let mut create_client = create_factory().await.map_err(|e| {
        tracing::error!("{:?}", e);
        anyhow!("Couldn't create the Create Queue")
    })?;

    loop {
        match try_process_from_queue(&mut create_client).await {
            Ok(status) => match status {
                FileProcessStatus::Success(job_id) => {
                    let _ = state_manager.set_status(job_id, JobStatus::Done).await?;
                }
                FileProcessStatus::NothingToProcess => {}
            },
            Err(e) => {
                tracing::error!("{:?}", e);
            }
        }
        delay_for(time_to_wait).await;
    }
}

async fn create_file_in_blob(file: Vec<u8>) -> String {
    format!("create file in blob: {:?}", file)
}

async fn add_to_search_index(blob: String) {
    tracing::debug!("update the index for {}", blob);
}

async fn try_process_from_queue(
    create_client: &mut DocIndexUpdaterQueue,
) -> Result<FileProcessStatus, anyhow::Error> {
    tracing::info!("Checking for create messages");
    let retrieved_result: Result<RetrievedMessage<CreateMessage>, RetrieveFromQueueError> =
        create_client.receive().await;

    if let Ok(retrieval) = retrieved_result {
        let message = retrieval.message.clone();
        tracing::info!("{:?} message receive!", message);
        let file = sftp_client::retrieve(
            message.document.file_source.clone(),
            message.document.file_path.clone(),
        )
        .await?;
        let _metadata = derive_metadata_from_message(&message.document);
        let blob = create_file_in_blob(file).await;
        add_to_search_index(blob).await;
        retrieval.remove().await?;

        Ok(FileProcessStatus::Success(message.job_id))
    } else {
        Ok(FileProcessStatus::NothingToProcess)
    }
}

pub fn derive_metadata_from_message(document: &Document) -> HashMap<String, String> {
    let mut metadata: HashMap<String, String> = HashMap::new();

    let file_name = metadata::sanitize(&document.id);
    metadata.insert("file_name".to_string(), file_name);
    metadata.insert("doc_type".to_string(), document.document_type.to_string());

    let title = metadata::sanitize(&document.name);
    let pl_numbers = metadata::extract_product_licences(&title);

    metadata.insert("title".to_string(), title);
    metadata.insert("pl_number".to_string(), pl_numbers);

    let product_names = metadata::to_json(document.products.clone());
    let product_names_csv = document.products.join(", ");
    metadata.insert("product_name".to_string(), product_names);

    let active_substances = metadata::to_json(document.active_substances.clone());
    metadata.insert("substance_name".to_string(), active_substances);

    let facets = metadata::to_json(metadata::create_facets_by_active_substance(
        &product_names_csv,
        document.active_substances.clone(),
    ));
    metadata.insert("facets".to_string(), facets);

    let author = metadata::sanitize(&document.author);
    metadata.insert("author".to_string(), author);

    if let Some(keywords) = &document.keywords {
        metadata.insert("keywords".to_string(), keywords.join(" "));
    }

    metadata
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::{DocumentType, FileSource};

    #[test]
    fn derive_metadata() {
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
            file_source: FileSource::Sentinel,
        };

        let expected_file_name = "CON123456".to_string();
        let expected_doc_type = "Spc".to_string();
        let expected_title = "Paracetamol Plus PL 12345/6789".to_string();
        let expected_author = "JRR Tolkien".to_string();
        let expected_product_name = "[\"Effective product 1\",\"Effective product 2\"]".to_string();
        let expected_substance_name = "[\"Paracetamol\",\"Caffeine\"]".to_string();
        let expected_keywords = "Very good for you Cures headaches PL 12345/6789".to_string();
        let expected_pl_number = "[\"PL123456789\"]".to_string();

        let output_metadata = derive_metadata_from_message(&doc);

        assert_eq!(output_metadata["file_name"], expected_file_name);
        assert_eq!(output_metadata["doc_type"], expected_doc_type);
        assert_eq!(output_metadata["title"], expected_title);
        assert_eq!(output_metadata["author"], expected_author);
        assert_eq!(output_metadata["product_name"], expected_product_name);
        assert_eq!(output_metadata["substance_name"], expected_substance_name);
        assert_eq!(output_metadata["keywords"], expected_keywords);
        assert_eq!(output_metadata["pl_number"], expected_pl_number);
    }
}
