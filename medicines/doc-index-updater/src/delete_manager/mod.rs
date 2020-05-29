use crate::{
    models::{DeleteMessage, JobStatus, UniqueDocumentIdentifier},
    service_bus_client::{
        delete_factory, ProcessMessageError, ProcessRetrievalError, RemovableMessage,
        RetrievedMessage,
    },
    state_manager::{JobStatusClient, StateManager},
    storage_client,
};
use anyhow::anyhow;
use async_trait::async_trait;
use search_client::{
    models::{IndexEntry, IndexResult},
    CreateIndexEntry, DeleteIndexEntry, Search,
};
use std::time::Duration;
use storage_client::{AzureBlobStorage, DeleteBlob};
use tokio::time::delay_for;
use uuid::Uuid;

pub async fn delete_service_worker(
    time_to_wait: Duration,
    state_manager: StateManager,
) -> Result<String, anyhow::Error> {
    tracing::info!("Starting delete service worker");
    let mut delete_client = delete_factory()
        .await
        .map_err(|e| anyhow!("Couldn't create service bus client: {:?}", e))?;

    loop {
        match delete_client
            .try_process_from_queue::<DeleteMessage>(&state_manager)
            .await
        {
            Ok(()) => {}
            Err(e) => tracing::error!("{:?}", e),
        }
        delay_for(time_to_wait).await;
    }
}

#[async_trait]
impl ProcessRetrievalError for RetrievedMessage<DeleteMessage> {
    async fn handle_processing_error(
        &mut self,
        error: ProcessMessageError,
        state_manager: &impl JobStatusClient,
    ) -> anyhow::Result<()> {
        handle_processing_error_for_delete_message(self, error, state_manager).await
    }
}

async fn handle_processing_error_for_delete_message<T>(
    removable_message: &mut T,
    error: ProcessMessageError,
    state_manager: &impl JobStatusClient,
) -> anyhow::Result<()>
where
    T: RemovableMessage<DeleteMessage>,
{
    tracing::info!("Handling processing error. Setting error state in state manager");

    let error_message = error.to_string();

    match error {
        ProcessMessageError::DocumentNotFoundInIndex(id) => {
            tracing::warn!(
                "Document {} wasn't found during delete, removing message",
                id
            );

            state_manager
                .set_status(
                    removable_message.get_message().job_id,
                    JobStatus::Error {
                        message: error_message,
                        code: "".to_string(),
                    },
                )
                .await?;
            let _remove = removable_message.remove().await?;
        }
        ProcessMessageError::FailedRestoringIndex(_, _) => {
            tracing::error!("{}", error_message);
            state_manager
                .set_status(
                    removable_message.get_message().job_id,
                    JobStatus::Error {
                        message: error_message,
                        code: "".to_string(),
                    },
                )
                .await?;
            let _remove = removable_message.remove().await?;
        }
        ProcessMessageError::FailedDeletingBlob(_, _) => {
            tracing::error!("{}", error_message);
        }
        _ => {}
    }

    Ok(())
}

pub async fn process_message(message: DeleteMessage) -> Result<Uuid, ProcessMessageError> {
    tracing::info!("Message received: {:?} ", message);

    let search_client = search_client::factory();
    let storage_client = AzureBlobStorage::permanent();

    process_delete_message(message, storage_client, search_client).await
}

async fn process_delete_message(
    message: DeleteMessage,
    mut storage_client: impl DeleteBlob,
    search_client: impl Search + DeleteIndexEntry + CreateIndexEntry,
) -> Result<Uuid, ProcessMessageError> {
    let index_record: IndexResult =
        get_index_record_from_unique_identifier(&message.document_id, &search_client).await?;
    let blob_name = index_record.metadata_storage_name.clone();

    tracing::debug!(
        "Found blob name {} for document content ID {:?} from index",
        &blob_name,
        &message.document_id
    );

    search_client
        .delete_index_entry(&"metadata_storage_name".to_string(), &blob_name)
        .await?;
    tracing::debug!("Deleted blob {} from index", &blob_name);

    if let Err(e) = storage_client.delete_blob(&blob_name).await {
        tracing::debug!(
            "Error deleting blob: {:?}, re-creating index: {:?}",
            e,
            &index_record
        );

        search_client
            .create_index_entry(IndexEntry::from(index_record.clone()))
            .await
            .map_err(|err| {
                ProcessMessageError::FailedRestoringIndex(blob_name.clone(), err.to_string())
            })?;
        return Err(ProcessMessageError::FailedDeletingBlob(
            blob_name.clone(),
            format!("{:?}", e),
        ));
    }

    tracing::info!("Successfully deleted blob {}", &blob_name);

    Ok(message.job_id)
}

pub async fn get_index_record_from_unique_identifier(
    unique_document_identifier: &UniqueDocumentIdentifier,
    search_client: &impl Search,
) -> Result<IndexResult, ProcessMessageError> {
    match unique_document_identifier {
        UniqueDocumentIdentifier::ContentId(content_id) => {
            get_index_record_from_content_id(content_id, search_client).await
        }
        UniqueDocumentIdentifier::MetadataStorageName(metadata_storage_name) => {
            get_index_record_from_metadata_storage_name(metadata_storage_name, search_client).await
        }
    }
}

pub async fn get_index_record_from_content_id(
    content_id: &str,
    search_client: &impl Search,
) -> Result<IndexResult, ProcessMessageError> {
    let search_results = search_client.search(content_id).await?;
    for result in search_results.search_results {
        if result.file_name == content_id {
            return Ok(result);
        }
    }
    Err(ProcessMessageError::DocumentNotFoundInIndex(
        content_id.to_string(),
    ))
}

pub async fn get_index_record_from_metadata_storage_name(
    metadata_storage_name: &str,
    search_client: &impl Search,
) -> Result<IndexResult, ProcessMessageError> {
    let search_results = search_client.search(metadata_storage_name).await?;
    for result in search_results.search_results {
        if result.metadata_storage_name == metadata_storage_name {
            return Ok(result);
        }
    }
    Err(ProcessMessageError::DocumentNotFoundInIndex(
        metadata_storage_name.to_string(),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    use crate::{
        models::DeleteMessage, service_bus_client::test::TestRemovableMessage,
        state_manager::test::TestJobStatusClient,
    };
    use search_client::{
        models::{
            AzureIndexChangedResult, AzureIndexChangedResults, DocumentType, IndexResult,
            IndexResults,
        },
        Search,
    };
    use std::env;
    use storage_client::test::TestAzureStorageClient;
    use tokio_test::block_on;

    #[test]
    fn not_found_error_during_delete_removes_message_since_no_need_to_retry() {
        let state_manager = given_a_state_manager();
        let mut removable_message = given_we_have_a_delete_message();
        let error = given_document_not_found_in_index();

        block_on(handle_processing_error_for_delete_message(
            &mut removable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        assert_eq!(
            removable_message.remove_was_called, true,
            "Didn't remove message, but should"
        );
    }

    #[test]
    fn not_found_error_during_delete_sets_job_status_as_error() {
        let state_manager = given_a_state_manager();
        let mut removable_message = given_we_have_a_delete_message();
        let error = given_document_not_found_in_index();

        block_on(handle_processing_error_for_delete_message(
            &mut removable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        let result =
            block_on(state_manager.get_status(removable_message.get_message().job_id)).unwrap();
        assert_eq!(
            result.status,
            JobStatus::Error {
                message: String::from("Cannot find document with ID any id"),
                code: String::from(""),
            },
        );
    }

    #[test]
    fn recoverable_error_during_delete_does_not_remove_message_from_servicebus() {
        let state_manager = given_a_state_manager();
        let mut removable_message = given_we_have_a_delete_message();
        let error = given_an_unknown_error();

        block_on(handle_processing_error_for_delete_message(
            &mut removable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        assert_eq!(
            removable_message.remove_was_called, false,
            "Removed message, but shouldn't"
        );
    }

    #[test]
    fn recoverable_error_during_delete_leaves_job_status_as_accepted() {
        let state_manager = given_a_state_manager();
        let mut removable_message = given_we_have_a_delete_message();
        given_the_delete_job_is_accepted(removable_message.get_message().job_id, &state_manager);
        let error = given_an_unknown_error();

        block_on(handle_processing_error_for_delete_message(
            &mut removable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        let result =
            block_on(state_manager.get_status(removable_message.get_message().job_id)).unwrap();
        assert_eq!(result.status, JobStatus::Accepted);
    }

    #[test]
    fn failure_to_delete_blob_leaves_job_status_as_accepted() {
        let state_manager = given_a_state_manager();
        let mut removable_message = given_we_have_a_delete_message();
        given_the_delete_job_is_accepted(removable_message.get_message().job_id, &state_manager);
        let error = given_a_delete_blob_error();

        block_on(handle_processing_error_for_delete_message(
            &mut removable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        let result =
            block_on(state_manager.get_status(removable_message.get_message().job_id)).unwrap();
        assert_eq!(result.status, JobStatus::Accepted);
    }

    #[test]
    fn failure_to_delete_blob_does_not_remove_message_from_service_bus() {
        let state_manager = given_a_state_manager();
        let mut removable_message = given_we_have_a_delete_message();
        let error = given_a_delete_blob_error();

        block_on(handle_processing_error_for_delete_message(
            &mut removable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        assert_eq!(
            removable_message.remove_was_called, false,
            "Removed message, but shouldn't"
        );
    }

    #[test]
    fn failure_to_restore_index_removes_message_since_cannot_be_retried() {
        let state_manager = given_a_state_manager();
        let mut removable_message = given_we_have_a_delete_message();
        let blob_id = "Blob Id".to_string();
        let error = given_failure_to_restore_index(blob_id);

        block_on(handle_processing_error_for_delete_message(
            &mut removable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        assert_eq!(
            removable_message.remove_was_called, true,
            "Didn't remove message, but should"
        );
    }

    #[test]
    fn failure_to_restore_index_leaves_job_status_as_error() {
        let state_manager = given_a_state_manager();
        let mut removable_message = given_we_have_a_delete_message();
        let blob_id = "Blob Id".to_string();
        let error = given_failure_to_restore_index(blob_id);

        block_on(handle_processing_error_for_delete_message(
            &mut removable_message,
            error,
            &state_manager,
        ))
        .unwrap();

        let result =
            block_on(state_manager.get_status(removable_message.get_message().job_id)).unwrap();

        let expected = JobStatus::Error {
            message: String::from("Cannot restore index for blob with ID Blob Id: Error message"),
            code: String::from(""),
        };

        assert_eq!(result.status, expected);
    }

    #[test]
    fn index_with_doc_returns_success() {
        let removable_message = given_we_have_a_delete_message().message;
        let search_client = given_a_search_client_that_returns_results();
        let storage_client = given_a_storage_client();
        given_the_necessary_env_vars_are_initialised();

        let result = block_on(process_delete_message(
            removable_message,
            storage_client,
            search_client,
        ));

        assert_eq!(result.is_err(), false);
    }

    #[test]
    fn failure_to_delete_blob_returns_expected_error() {
        let removable_message = given_we_have_a_delete_message().message;
        let search_client = given_a_search_client_that_returns_results();
        let storage_client = given_a_storage_client_that_cannot_delete_blob();
        given_the_necessary_env_vars_are_initialised();

        let result = block_on(process_delete_message(
            removable_message,
            storage_client,
            search_client,
        ));

        match result {
            Ok(_) => panic!("Error expected"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    ProcessMessageError::FailedDeletingBlob(
                        "storage_name".to_string(),
                        "Generic(blob could not be deleted)".to_string()
                    )
                    .to_string()
                );
            }
        }
    }

    #[test]
    fn failure_to_restore_index_returns_expected_error() {
        let removable_message = given_we_have_a_delete_message().message;
        let search_client = given_a_search_client_that_cannot_restore_index();
        let storage_client = given_a_storage_client_that_cannot_delete_blob();
        given_the_necessary_env_vars_are_initialised();

        let result = block_on(process_delete_message(
            removable_message,
            storage_client,
            search_client,
        ));

        match result {
            Ok(_) => panic!("Error expected"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    ProcessMessageError::FailedRestoringIndex(
                        "storage_name".to_string(),
                        "Index could not be created".to_string()
                    )
                    .to_string()
                );
            }
        }
    }

    #[test]
    fn failure_to_delete_index_returns_expected_error() {
        let removable_message = given_we_have_a_delete_message().message;
        let search_client = given_a_search_client_that_cannot_delete_index();
        let storage_client = given_a_storage_client();
        given_the_necessary_env_vars_are_initialised();

        let result = block_on(process_delete_message(
            removable_message,
            storage_client,
            search_client,
        ));

        match result {
            Ok(_) => panic!("Error expected"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    ProcessMessageError::Generic(anyhow!("Index could not be deleted")).to_string()
                );
            }
        }
    }

    fn given_the_necessary_env_vars_are_initialised() {
        env::set_var("STORAGE_CONTAINER", "storage_container");
    }

    fn given_document_not_found_in_index() -> ProcessMessageError {
        ProcessMessageError::DocumentNotFoundInIndex("any id".to_owned())
    }

    fn given_an_unknown_error() -> ProcessMessageError {
        anyhow!("Any other error").into()
    }

    fn given_a_delete_blob_error() -> ProcessMessageError {
        ProcessMessageError::FailedDeletingBlob("Blob Id".to_string(), "Error message".to_string())
    }

    fn given_failure_to_restore_index(blob_id: String) -> ProcessMessageError {
        ProcessMessageError::FailedRestoringIndex(blob_id, "Error message".to_string())
    }

    fn given_an_index_search_result() -> IndexResult {
        IndexResult {
            doc_type: DocumentType::Spc,
            file_name: "our_id".to_string(),
            metadata_storage_name: "storage_name".to_string(),
            metadata_storage_path: "test/path".to_string(),
            product_name: Some("product".to_string()),
            substance_name: vec!["substance".to_string()],
            title: "title".to_string(),
            created: None,
            facets: vec!["facet".to_string()],
            keywords: None,
            metadata_storage_size: 300,
            release_state: None,
            rev_label: None,
            suggestions: vec!["suggestion".to_string()],
            score: 1.0,
            highlights: None,
        }
    }

    fn given_a_state_manager() -> TestJobStatusClient {
        TestJobStatusClient::accepted()
    }

    fn given_we_have_a_delete_message() -> TestRemovableMessage<DeleteMessage> {
        let delete_message = DeleteMessage {
            document_id: "our_id".to_string().into(),
            job_id: Uuid::new_v4(),
        };

        TestRemovableMessage::<DeleteMessage> {
            remove_was_called: false,
            message: delete_message,
        }
    }

    fn given_the_delete_job_is_accepted(id: Uuid, state_manager: &TestJobStatusClient) {
        let _ = block_on(state_manager.set_status(id, JobStatus::Accepted));
    }

    #[test]
    fn get_blob_name_from_content_id_raises_document_not_found_in_index_error_when_not_there() {
        let search_client = given_a_search_client_that_returns_no_results();
        let result = when_getting_blob_name_from_content_id(search_client);
        then_document_not_found_in_index_error_is_raised(result);
    }

    fn given_a_search_client_that_returns_no_results() -> impl Search {
        TestAzureSearchClient {
            can_insert_index: true,
            can_delete_index: true,
            search_results: vec![],
        }
    }

    fn given_a_search_client_that_returns_results(
    ) -> impl Search + CreateIndexEntry + DeleteIndexEntry {
        TestAzureSearchClient {
            can_insert_index: true,
            can_delete_index: true,
            search_results: vec![given_an_index_search_result()],
        }
    }

    fn given_a_search_client_that_cannot_restore_index(
    ) -> impl Search + CreateIndexEntry + DeleteIndexEntry {
        TestAzureSearchClient {
            can_insert_index: false,
            can_delete_index: true,
            search_results: vec![given_an_index_search_result()],
        }
    }

    fn given_a_search_client_that_cannot_delete_index(
    ) -> impl Search + CreateIndexEntry + DeleteIndexEntry {
        TestAzureSearchClient {
            can_insert_index: true,
            can_delete_index: false,
            search_results: vec![given_an_index_search_result()],
        }
    }

    fn given_a_storage_client() -> impl DeleteBlob {
        TestAzureStorageClient {
            can_delete_blob: true,
        }
    }

    fn given_a_storage_client_that_cannot_delete_blob() -> impl DeleteBlob {
        TestAzureStorageClient {
            can_delete_blob: false,
        }
    }

    fn when_getting_blob_name_from_content_id(
        search_client: impl Search,
    ) -> Result<String, ProcessMessageError> {
        Ok(when_getting_index_record_from_content_id(search_client)?.metadata_storage_name)
    }

    fn when_getting_index_record_from_content_id(
        search_client: impl Search,
    ) -> Result<IndexResult, ProcessMessageError> {
        block_on(get_index_record_from_content_id(
            "non existent content id",
            &search_client,
        ))
    }

    fn then_document_not_found_in_index_error_is_raised(
        result: Result<String, ProcessMessageError>,
    ) {
        assert_eq!(result.is_err(), true);

        assert!(
            if let Err(ProcessMessageError::DocumentNotFoundInIndex(_)) = result {
                true
            } else {
                false
            },
            format!(
                "Should have been an error with type: DocumentNotFoundInIndex, but was {:?}",
                result
            )
        );
    }

    struct TestAzureSearchClient {
        pub search_results: Vec<IndexResult>,
        pub can_insert_index: bool,
        pub can_delete_index: bool,
    }

    #[async_trait]
    impl Search for TestAzureSearchClient {
        async fn search(&self, _search_term: &str) -> Result<IndexResults, reqwest::Error> {
            Ok(IndexResults {
                search_results: self.search_results.clone(),
                context: String::from(""),
                count: None,
            })
        }
        async fn search_with_pagination(
            &self,
            _search_term: &str,
            _pagination: search_client::AzurePagination,
            _include_count: bool,
        ) -> Result<IndexResults, reqwest::Error> {
            unimplemented!()
        }
        async fn search_with_pagination_and_filter(
            &self,
            _search_term: &str,
            _pagination: search_client::AzurePagination,
            _include_count: bool,
            _filter: Option<&str>,
        ) -> Result<IndexResults, reqwest::Error> {
            unimplemented!()
        }
        async fn filter_by_collection_field(
            &self,
            _field_name: &str,
            _field_value: &str,
        ) -> Result<IndexResults, reqwest::Error> {
            unimplemented!()
        }
        async fn filter_by_non_collection_field(
            &self,
            _field_name: &str,
            _field_value: &str,
        ) -> Result<IndexResults, reqwest::Error> {
            unimplemented!()
        }
    }

    #[async_trait]
    impl DeleteIndexEntry for TestAzureSearchClient {
        async fn delete_index_entry(
            &self,
            key_name: &str,
            _value: &str,
        ) -> Result<AzureIndexChangedResults, anyhow::Error> {
            if !&self.can_delete_index {
                return Err(anyhow!("Index could not be deleted"));
            }

            let index_changed_result = AzureIndexChangedResult {
                key: key_name.to_string(),
                status: true,
                error_message: None,
                status_code: 200,
            };

            Ok(AzureIndexChangedResults::new(index_changed_result))
        }
    }

    #[async_trait]
    impl CreateIndexEntry for TestAzureSearchClient {
        async fn create_index_entry(
            &self,
            _key_values: IndexEntry,
        ) -> Result<AzureIndexChangedResults, anyhow::Error> {
            if !&self.can_insert_index {
                return Err(anyhow!("Index could not be created"));
            }

            let index_changed_result = AzureIndexChangedResult {
                key: "key".to_string(),
                status: true,
                error_message: None,
                status_code: 200,
            };

            Ok(AzureIndexChangedResults::new(index_changed_result))
        }
    }
}
