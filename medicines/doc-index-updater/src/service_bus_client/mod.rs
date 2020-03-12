use crate::models::Message;
use anyhow::anyhow;
use azure_sdk_core::errors::AzureError;
use azure_sdk_service_bus::{event_hub::PeekLockResponse, prelude::Client};
use hyper::StatusCode;
use std::error::Error;
use time::Duration;
use AzureError::UnexpectedHTTPResult;

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

pub struct DocIndexUpdaterQueue {
    service_bus: Client,
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

pub struct RetrievedMessage<T> {
    pub message: T,
    peek_lock: PeekLockResponse,
}

impl<T> RetrievedMessage<T> {
    pub async fn remove(&self) -> Result<String, anyhow::Error> {
        let queue_removal_result = self.peek_lock.delete_message().await.map_err(|e| {
            tracing::error!("{:?}", e);
            anyhow!("Queue Removal Error")
        });
        tracing::info!("Removed job from ServiceBus ({:?})", queue_removal_result);
        queue_removal_result
    }
}

impl DocIndexUpdaterQueue {
    fn new(service_bus: Client) -> Self {
        Self { service_bus }
    }

    pub async fn receive<T: Message>(
        &mut self,
    ) -> Result<RetrievedMessage<T>, RetrieveFromQueueError> {
        let peek_lock = self
            .service_bus
            .peek_lock_full(time::Duration::days(1), Some(time::Duration::seconds(10)))
            .await
            .map_err(|e| match e {
                UnexpectedHTTPResult(a) if a.status_code() == StatusCode::NO_CONTENT => {
                    tracing::info!("No new messages found. ({:?})", a);
                    RetrieveFromQueueError::NotFoundError
                }
                _ => {
                    tracing::error!("{:?}", e);
                    RetrieveFromQueueError::AzureError(e)
                }
            })?;

        let body = peek_lock.body();

        match body.parse::<T>() {
            Ok(message) => {
                tracing::info!(
                    "Message found perfectly parseable ({:?}).\n{:?}",
                    body,
                    message.to_json_string()
                );
                Ok(RetrievedMessage { message, peek_lock })
            }
            Err(_) => {
                tracing::error!(
                    "Message found could not be parsed ({:?}). Gonna go ahead and delete it.",
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
}
