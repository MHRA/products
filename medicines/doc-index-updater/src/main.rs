use doc_index_updater::{document_manager, health, state_manager};
use state_manager::get_client;
use std::{env, error, net::SocketAddr};
use warp::Filter;

const PORT: u16 = 8000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=doc_index_updater=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "doc_index_updater=info");
    }
    pretty_env_logger::init();

    let addr = format!("0.0.0.0:{}", get_env_or_default("PORT", PORT.to_string()))
        .parse::<SocketAddr>()?;

    let redis_server = get_env_or_default("REDIS_SERVER", "127.0.0.1".to_string());
    let redis_port = get_env_or_default("REDIS_PORT", "6379".to_string());
    let redis_key = get_env_or_default("REDIS_KEY", "".to_string());
    let redis_addr = create_redis_url(redis_server, redis_port, redis_key);

    let state = state_manager::StateManager::new(get_client(redis_addr.clone())?);
    let _ = tokio::join!(tokio::spawn(async move {
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
    }),);

    Ok(())
}

fn get_env_or_default(key: &str, default: String) -> String {
    env::var(key).unwrap_or_else(|e| {
        log::error!("defaulting {} to {} ({})", key, &default, e);
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
