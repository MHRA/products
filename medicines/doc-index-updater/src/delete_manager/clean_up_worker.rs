use crate::{
    models::DeleteMessage, service_bus_client::delete_clean_up_factory, state_manager::StateManager,
};
use anyhow::anyhow;
use std::time::Duration;
use tokio::time::delay_for;

pub async fn delete_queue_clean_up_worker(
    time_to_wait: Duration,
    state_manager: StateManager,
) -> Result<(), anyhow::Error> {
    tracing::info!("Starting delete queue clean up worker");
    let mut delete_clean_up_client = delete_clean_up_factory()
        .await
        .map_err(|e| anyhow!("Couldn't create service bus client: {:?}", e))?;

    loop {
        match delete_clean_up_client
            .try_process_from_dead_letter_queue::<DeleteMessage>(&state_manager)
            .await
        {
            Ok(found_message) => {
                if !found_message {
                    delay_for(time_to_wait).await;
                }
            }
            Err(e) => tracing::error!("{:?}", e),
        }
    }
}
