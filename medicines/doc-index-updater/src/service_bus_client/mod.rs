use crate::{
    get_env_or_default,
    models::{JobStatus, Message},
    state_manager::StateManager,
};
use anyhow::anyhow;
use async_trait::async_trait;
use azure_sdk_core::errors::AzureError;
use azure_sdk_service_bus::{event_hub::PeekLockResponse, prelude::Client};
use hyper::StatusCode;
use std::error::Error;
use time::Duration;

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

#[derive(Debug)]
pub enum RetrieveFromQueueError {
    AzureError(AzureError),
    ParseError(String),
    NotFoundError,
}

impl std::fmt::Display for RetrieveFromQueueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A")
    }
}

impl Error for RetrieveFromQueueError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub struct RetrievedMessage<T: Message> {
    pub message: T,
    peek_lock: PeekLockResponse,
}

impl<T: Message> RetrievedMessage<T> {
    pub async fn remove(&self) -> Result<String, anyhow::Error> {
        let queue_removal_result = self.peek_lock.delete_message().await.map_err(|e| {
            tracing::error!("{:?}", e);
            anyhow!("Queue Removal Error")
        })?;
        tracing::debug!("Removed job from ServiceBus ({:?})", queue_removal_result);
        Ok(queue_removal_result)
    }
}

#[async_trait]
pub trait ProcessRetrievalError {
    async fn handle_processing_error(
        self,
        e: anyhow::Error,
        state_manager: &StateManager,
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
    ) -> Result<(), anyhow::Error>
    where
        T: Message,
        RetrievedMessage<T>: ProcessRetrievalError,
    {
        tracing::debug!("Checking for messages.");
        let retrieved_result: Result<RetrievedMessage<T>, RetrieveFromQueueError> =
            self.receive().await;

        if let Ok(retrieval) = retrieved_result {
            let correlation_id = retrieval.message.get_id().to_string();
            let correlation_id = correlation_id.as_str();

            let processing_result = retrieval.message.clone().process().await;

            match processing_result {
                Ok(job_id) => {
                    state_manager.set_status(job_id, JobStatus::Done).await?;
                    retrieval.remove().await?;
                }
                Err(e) => {
                    tracing::error!(message = format!("Error {:?}", e).as_str(), correlation_id);
                    retrieval.handle_processing_error(e, &state_manager).await?;
                }
            };
        }
        Ok(())
    }
}
