use anyhow::anyhow;
use core::fmt::Display;
use doc_index_updater::{create_manager, delete_manager, document_manager, health, state_manager};
use state_manager::get_client;
use std::{env, error, net::SocketAddr, time::Duration};
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

    let time_to_wait = Duration::from_secs(get_env_or_default("SECONDS_TO_WAIT", 5));
    let storage_container_name =
        env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");

    let state = state_manager::StateManager::new(get_client(redis_addr.clone())?);
    tracing::info!("StateManager config: {:?}", state);

    let create_state = state.clone();

    let _ = tokio::join!(
        tokio::spawn(async move {
            warp::serve(
                health::get_health()
                    .or(state_manager::get_job_status(state.clone()))
                    .or(state_manager::set_job_status(state.clone()))
                    .or(document_manager::check_in_document(state.clone()))
                    .or(document_manager::del_document(state.clone()))
                    .with(warp::log("doc_index_updater")),
            )
            .run(addr.clone())
            .await;
        }),
        tokio::spawn(delete_manager::delete_service_worker(
            storage_container_name.clone()
        )),
        tokio::spawn(create_manager::create_service_worker(
            storage_container_name.clone(),
            time_to_wait,
            create_state
        )),
    );
    Ok(())
}

pub fn get_env_or_default<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr + Display,
{
    get_env(key).unwrap_or_else(|e| {
        tracing::warn!(r#"defaulting {} to "{}" ({})"#, key, &default, e);
        return default;
    })
}

pub fn get_env<T>(key: &str) -> Result<T, anyhow::Error>
where
    T: std::str::FromStr,
{
    env::var(key)?
        .parse::<T>()
        .map_err(|_| anyhow!("failed to parse for {}", key))
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
