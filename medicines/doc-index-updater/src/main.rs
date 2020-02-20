use doc_index_updater::state_manager;
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

    let addr = format!(
        "0.0.0.0:{}",
        env::var("PORT").unwrap_or_else(|e| {
            eprintln!("defaulting PORT to {} ({})", PORT, e);
            PORT.to_string()
        })
    )
    .parse::<SocketAddr>()?;

    let _ = tokio::join!(
        tokio::spawn(async move {
            warp::serve(
                state_manager::jobs()
                    .or(state_manager::complete())
                    .with(warp::log("doc_index_updater")),
            )
            .run(addr.clone())
            .await;
        }),
        tokio::spawn(async move {
            let mut addr2 = addr;
            addr2.set_port(addr2.port() + 1);
            warp::serve(state_manager::jobs().with(warp::log("doc_index_updater")))
                .run(addr2)
                .await;
        })
    );

    Ok(())
}
