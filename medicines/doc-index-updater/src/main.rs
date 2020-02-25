use doc_index_updater::state_manager;
use state_manager::get_client;
use std::{env, error, net::SocketAddr, time::Duration};
use tokio::time;
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

    let redis_addr = get_env_or_default("REDIS_ADDR", "redis://127.0.0.1:6379/".to_string());
    let con1 = loop {
        match get_client(redis_addr.clone()) {
            Ok(c) => match c.get_multiplexed_tokio_connection().await {
                Ok(c) => {
                    log::info!("Got connection to Redis at {:?}", redis_addr);
                    break c;
                }
                Err(e) => log::error!("Failed to get multiplexed connection: {:?}", e),
            },
            Err(e) => log::error!("Failed to get redis client: {:?}", e),
        }
        time::delay_for(Duration::from_secs(10)).await;
    };
    let con2 = con1.clone();

    let _ = tokio::join!(
        tokio::spawn(async move {
            warp::serve(
                state_manager::get_job_status(con1.clone())
                    .or(state_manager::set_job_status(con1))
                    .with(warp::log("doc_index_updater")),
            )
            .run(addr.clone())
            .await;
        }),
        tokio::spawn(async move {
            let mut addr2 = addr;
            addr2.set_port(addr2.port() + 1);
            warp::serve(state_manager::get_job_status(con2).with(warp::log("doc_index_updater")))
                .run(addr2)
                .await;
        })
    );

    Ok(())
}

fn get_env_or_default(key: &str, default: String) -> String {
    env::var(key).unwrap_or_else(|e| {
        eprintln!("defaulting {} to {} ({})", key, &default, e);
        default.to_string()
    })
}
