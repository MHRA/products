use crate::{
    get_env_or_default,
    models::{JobStatus, Message},
    state_manager::{JobStatusClient, StateManager},
    storage_client::models::StorageClientError,
};
use anyhow::anyhow;
use async_trait::async_trait;
use azure_sdk_core::errors::AzureError;
use azure_sdk_service_bus::{event_hub::PeekLockResponse, prelude::Client};
use hyper::StatusCode;
use thiserror::Error;
use time::Duration;
use tracing_futures::Instrument;

pub async fn delete_factory() -> Result<DocIndexUpdaterQueue, AzureError> {
    let service_bus_namespace = std::env::var("SERVICE_BUS_NAMESPACE")
        .expect("Set env variable SERVICE_BUS_NAMESPACE first!");

    let queue_name =
        std::env::var("DELETE_QUEUE_NAME").expect("Set env variable DELETE_QUEUE_NAME first!");

    let policy_name = std::env::var("DELETE_QUEUE_POLICY_NAME")
        .expect("Set env variable DELETE_QUEUE_POLICY_NAME first!");

    let policy_key = std::env::var("DELETE_QUEUE_POLICY_KEY")
        .expect("Set env variable DELETE_QUEUE_POLICY_KEY first!");

    let service_bus = Client::new(service_bus_namespace, queue_name, policy_name, policy_key)?;
    Ok(DocIndexUpdaterQueue::new(service_bus))
}

pub async fn create_factory() -> Result<DocIndexUpdaterQueue, AzureError> {
    let service_bus_namespace = std::env::var("SERVICE_BUS_NAMESPACE")
        .expect("Set env variable SERVICE_BUS_NAMESPACE first!");

    let queue_name =
        std::env::var("CREATE_QUEUE_NAME").expect("Set env variable CREATE_QUEUE_NAME first!");

    let policy_name = std::env::var("CREATE_QUEUE_POLICY_NAME")
        .expect("Set env variable CREATE_QUEUE_POLICY_NAME first!");

    let policy_key = std::env::var("CREATE_QUEUE_POLICY_KEY")
        .expect("Set env variable CREATE_QUEUE_POLICY_KEY first!");

    let service_bus = Client::new(service_bus_namespace, queue_name, policy_name, policy_key)?;
    Ok(DocIndexUpdaterQueue::new(service_bus))
}

pub async fn create_clean_up_factory() -> Result<DocIndexUpdaterQueue, AzureError> {
    let service_bus_namespace = std::env::var("SERVICE_BUS_NAMESPACE")
        .expect("Set env variable SERVICE_BUS_NAMESPACE first!");

    let queue_name =
        std::env::var("CREATE_QUEUE_NAME").expect("Set env variable CREATE_QUEUE_NAME first!");

    let dead_letter_queue_name = format!("{}/$DeadLetterQueue", queue_name);

    let policy_name = std::env::var("CREATE_QUEUE_POLICY_NAME")
        .expect("Set env variable CREATE_QUEUE_POLICY_NAME first!");

    let policy_key = std::env::var("CREATE_QUEUE_POLICY_KEY")
        .expect("Set env variable CREATE_QUEUE_POLICY_KEY first!");

    let service_bus = Client::new(
        service_bus_namespace,
        dead_letter_queue_name,
        policy_name,
        policy_key,
    )?;
    Ok(DocIndexUpdaterQueue::new(service_bus))
}

#[derive(Error, Debug)]
pub enum RetrieveFromQueueError {
    #[error(transparent)]
    AzureError(AzureError),
    #[error("Parsing error: {0}")]
    ParseError(String),
    #[error("No Messages Found In Queue")]
    NotFoundError,
    #[error("Error reading Queue")]
    ErrorReadingQueue,
}

pub struct RetrievedMessage<T: Message> {
    pub message: T,
    peek_lock: PeekLockResponse,
}
pub trait RemovableMessage<T: Message>: Removable {
    fn get_message(&self) -> T;
}

impl<T> RemovableMessage<T> for RetrievedMessage<T>
where
    T: Message + Sync + Send,
{
    fn get_message(&self) -> T {
        self.message.clone()
    }
}

#[async_trait]
pub trait Removable {
    async fn remove(&mut self) -> Result<String, anyhow::Error>;
}

#[async_trait]
impl<T> Removable for RetrievedMessage<T>
where
    T: Message + Send + Sync,
{
    async fn remove(&mut self) -> Result<String, anyhow::Error> {
        let queue_removal_result = self.peek_lock.delete_message().await.map_err(|e| {
            tracing::error!("{:?}", e);
            anyhow!("Queue Removal Error")
        })?;
        tracing::debug!("Removed job from ServiceBus ({:?})", queue_removal_result);
        Ok(queue_removal_result)
    }
}

#[derive(Error, Debug)]
pub enum ProcessMessageError {
    #[error(transparent)]
    StorageClientError(#[from] StorageClientError),
    #[error("Cannot find document with ID {0}")]
    DocumentNotFoundInIndex(String),
    #[error("Cannot delete blob with ID {0}: {1}")]
    FailedDeletingBlob(String, String),
    #[error("Cannot restore index for blob with ID {0}: {1}")]
    FailedRestoringIndex(String, String),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Generic(#[from] anyhow::Error),
}

#[async_trait]
pub trait ProcessRetrievalError {
    async fn handle_processing_error(
        &mut self,
        e: ProcessMessageError,
        state_manager: &impl JobStatusClient,
    ) -> anyhow::Result<()>;
}

pub struct DocIndexUpdaterQueue {
    service_bus: Client,
    lock_timeout: time::Duration,
}

impl DocIndexUpdaterQueue {
    fn new(service_bus: Client) -> Self {
        let lock_timeout = get_env_or_default("SERVICE_BUS_MESSAGE_LOCK_TIMEOUT", 10);
        let lock_timeout = time::Duration::seconds(lock_timeout);
        Self {
            service_bus,
            lock_timeout,
        }
    }

    pub async fn receive<T: Message>(
        &mut self,
    ) -> Result<RetrievedMessage<T>, RetrieveFromQueueError> {
        let peek_lock = self
            .service_bus
            .peek_lock_full(time::Duration::days(1), Some(self.lock_timeout))
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e);
                RetrieveFromQueueError::AzureError(e)
            })?;

        if !peek_lock.status().is_success() {
            tracing::error!("{} when reading queue.", peek_lock.status(),);
            return Err(RetrieveFromQueueError::ErrorReadingQueue);
        }

        if peek_lock.status() == StatusCode::NO_CONTENT {
            tracing::debug!("No new messages found.");
            return Err(RetrieveFromQueueError::NotFoundError);
        }

        let body = peek_lock.body();

        match body.parse::<T>() {
            Ok(message) => {
                tracing::debug!(
                    "Message found perfectly parseable ({:?}).\n{:?}",
                    body,
                    message.to_json_string()
                );
                Ok(RetrievedMessage { message, peek_lock })
            }
            Err(_) => {
                tracing::error!(
                    "Message found could not be parsed ({:?}). Deleting it.",
                    body
                );
                let _ = peek_lock.delete_message().await;
                Err(RetrieveFromQueueError::ParseError(body))
            }
        }
    }

    pub async fn send<T: Message>(
        &mut self,
        message: T,
        duration: Duration,
    ) -> Result<(), AzureError> {
        let evt = message.to_json_string()?;
        Ok(self.service_bus.send_event(evt.as_str(), duration).await?)
    }

    pub async fn try_process_from_queue<T>(
        &mut self,
        state_manager: &StateManager,
    ) -> anyhow::Result<()>
    where
        T: Message,
        RetrievedMessage<T>: ProcessRetrievalError + Removable,
    {
        tracing::debug!("Checking for messages.");
        let retrieved_result: Result<RetrievedMessage<T>, RetrieveFromQueueError> =
            self.receive().await;

        if let Ok(retrieval) = retrieved_result {
            let correlation_id = retrieval.message.get_id().to_string();
            let correlation_id = correlation_id.as_str();

            process(retrieval, state_manager)
                .instrument(tracing::info_span!(
                    "try_process_from_queue",
                    correlation_id
                ))
                .await?
        }
        Ok(())
    }

    pub async fn try_process_from_dead_letter_queue<T>(
        &mut self,
        state_manager: &StateManager,
    ) -> anyhow::Result<()>
    where
        T: Message,
        RetrievedMessage<T>: ProcessRetrievalError + Removable,
    {
        tracing::debug!("Checking for messages.");
        let retrieved_result: Result<RetrievedMessage<T>, RetrieveFromQueueError> =
            self.receive().await;

        if let Ok(retrieval) = retrieved_result {
            let correlation_id = retrieval.message.get_id().to_string();
            let correlation_id = correlation_id.as_str();

            process_dead_letter(retrieval, state_manager)
                .instrument(tracing::info_span!(
                    "try_process_dead_letter_from_queue",
                    correlation_id
                ))
                .await?
        }
        Ok(())
    }
}

async fn process<T>(
    mut retrieval: RetrievedMessage<T>,
    state_manager: &impl JobStatusClient,
) -> anyhow::Result<()>
where
    T: Message,
    RetrievedMessage<T>: ProcessRetrievalError + Removable,
{
    let processing_result = retrieval.message.clone().process().await;

    match processing_result {
        Ok(job_id) => {
            state_manager.set_status(job_id, JobStatus::Done).await?;
            retrieval.remove().await?;
        }
        Err(e) => {
            tracing::error!(message = format!("Error {:?}", e).as_str());
            retrieval.handle_processing_error(e, state_manager).await?;
        }
    };
    Ok(())
}

async fn process_dead_letter<T>(
    mut retrieval: RetrievedMessage<T>,
    state_manager: &impl JobStatusClient,
) -> anyhow::Result<()>
where
    T: Message,
    RetrievedMessage<T>: ProcessRetrievalError + Removable,
{
    state_manager
        .set_status(
            retrieval.message.get_id(),
            JobStatus::Error {
                message: "Max number of retries exceeded - removed from dead letter queue"
                    .to_string(),
                code: "500".to_string(),
            },
        )
        .await?;
    let _ = retrieval.remove().await?;
    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    pub struct TestRemovableMessage<T: Message> {
        pub remove_was_called: bool,
        pub message: T,
    }

    #[async_trait]
    impl<T> Removable for TestRemovableMessage<T>
    where
        T: Message + Send + Sync,
    {
        async fn remove(&mut self) -> Result<String, anyhow::Error> {
            self.remove_was_called = true;
            Ok("success".to_owned())
        }
    }

    #[async_trait]
    impl<T> RemovableMessage<T> for TestRemovableMessage<T>
    where
        T: Message + Sync + Send,
    {
        fn get_message(&self) -> T {
            self.message.clone()
        }
    }
}
