<<<<<<< HEAD
use azure_sdk_service_bus::prelude::Client;
use doc_index_updater::{delete_manager, document_manager, health, state_manager};
=======
use doc_index_updater::{document_manager, health, state_manager};
use document_manager::ServiceBusCredentials;
>>>>>>> Implement error handling, pass credentials into routes instead of Client, make it compile.
use state_manager::get_client;
use std::{env, error, net::SocketAddr};
use tracing::Level;
use warp::Filter;

const PORT: u16 = 8000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(Level::INFO)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing_log::LogTracer::init()
        .expect("error redirecting normal log messages to the tracing subscriber");

    let addr = format!("0.0.0.0:{}", get_env_or_default("PORT", PORT.to_string()))
        .parse::<SocketAddr>()?;

    let redis_server = get_env_or_default("REDIS_SERVER", "127.0.0.1".to_string());
    let redis_port = get_env_or_default("REDIS_PORT", "6379".to_string());
    let redis_key = get_env_or_default("REDIS_KEY", "".to_string());
    let redis_addr = create_redis_url(redis_server, redis_port, redis_key);

    let azure_sb_namespace = get_env_or_default("SERVICE_BUS_NAMESPACE", "".to_string());
    let azure_sb_create_queue_name =
        get_env_or_default("SERVICE_BUS_CREATE_QUEUE_NAME", "".to_string());
    let azure_sb_create_queue_policy_name =
        get_env_or_default("SERVICE_BUS_CREATE_QUEUE_POLICY_NAME", "".to_string());
    let azure_sb_create_queue_policy_key =
        get_env_or_default("SERVICE_BUS_CREATE_QUEUE_POLICY_KEY", "".to_string());
    let azure_sb_delete_queue_name =
        get_env_or_default("SERVICE_BUS_DELETE_QUEUE_NAME", "".to_string());
    let azure_sb_delete_queue_policy_name =
        get_env_or_default("SERVICE_BUS_DELETE_QUEUE_POLICY_NAME", "".to_string());
    let azure_sb_delete_queue_policy_key =
        get_env_or_default("SERVICE_BUS_DELETE_QUEUE_POLICY_KEY", "".to_string());

    let state = state_manager::StateManager::new(get_client(redis_addr.clone())?);
    tracing::info!("StateManager config: {:?}", state);

    let azure_sb_creds = ServiceBusCredentials {
        namespace: azure_sb_namespace,
        create_queue_name: azure_sb_create_queue_name,
        create_queue_policy_name: azure_sb_create_queue_policy_name,
        create_queue_policy_key: azure_sb_create_queue_policy_key,
        delete_queue_name: azure_sb_delete_queue_name,
        delete_queue_policy_name: azure_sb_delete_queue_policy_name,
        delete_queue_policy_key: azure_sb_delete_queue_policy_key,
    };

    let _ = tokio::join!(
        tokio::spawn(async move {
            warp::serve(
                health::get_health()
                    .or(state_manager::get_job_status(state.clone()))
                    .or(state_manager::set_job_status(state.clone()))
                    .or(document_manager::check_in_document(
                        state.clone(),
                        azure_sb_client.clone(),
                    ))
                    .or(document_manager::del_document(
                        state.clone(),
                        azure_sb_client.clone(),
                    ))
                    .with(warp::log("doc_index_updater")),
            )
            .run(addr.clone())
            .await;
        }),
        tokio::spawn(delete_manager::delete_service_worker())
    );
    Ok(())
}

pub fn get_env_or_default(key: &str, default: String) -> String {
    env::var(key).unwrap_or_else(|e| {
        tracing::warn!(r#"defaulting {} to "{}" ({})"#, key, &default, e);
        default.to_string()
    })
}

fn create_redis_url(server: String, port: String, key: String) -> String {
    if key == "" {
        format!("redis://{}:{}", server, port)
    } else {
        format!("redis://:{}@{}:{}", key, server, port)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_redis_url_without_key() {
        assert_eq!(
            create_redis_url("127.0.0.1".to_string(), "6379".to_string(), "".to_string()),
            "redis://127.0.0.1:6379"
        )
    }
    #[test]
    fn create_redis_url_with_key() {
        assert_eq!(
            create_redis_url(
                "127.0.0.1".to_string(),
                "6379".to_string(),
                "mykey".to_string()
            ),
            "redis://:mykey@127.0.0.1:6379"
        )
    }
}
