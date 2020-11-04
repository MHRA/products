use crate::{
    models::CreateMessage, service_bus_client::create_clean_up_factory, state_manager::StateManager,
};
use anyhow::anyhow;
use std::time::Duration;
use tokio::time::sleep;

pub async fn create_queue_clean_up_worker(
    time_to_wait: Duration,
    state_manager: StateManager,
) -> Result<(), anyhow::Error> {
    tracing::info!("Starting create queue clean up worker");
    let mut create_clean_up_client = create_clean_up_factory()
        .await
        .map_err(|e| anyhow!("Couldn't create service bus client: {:?}", e))?;

    loop {
        match create_clean_up_client
            .try_process_from_dead_letter_queue::<CreateMessage>(&state_manager)
            .await
        {
            Ok(found_message) => {
                if !found_message {
                    sleep(time_to_wait).await;
                }
            }
            Err(e) => tracing::error!("{:?}", e),
        }
    }
}
