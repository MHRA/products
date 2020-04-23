use anyhow::anyhow;
use core::fmt::Display;
use std::{env, str::FromStr};
use tracing::Level;
use warp::{self, http::StatusCode, Filter, Rejection, Reply};

mod azure_context;
mod pagination;
mod product;
mod schema;
mod substance;

const PORT: u16 = 8000;

use crate::{azure_context::create_context, schema::create_schema};

pub fn healthz() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    warp::path!("healthz")
        .and(warp::get())
        .map(warp::reply)
        .map(|reply| warp::reply::with_status(reply, StatusCode::NO_CONTENT))
}

#[tokio::main]
async fn main() {
    if get_env_or_default("JSON_LOGS", true) {
        use_json_log_subscriber()
    } else {
        use_unstructured_log_subscriber()
    }

    let log = warp::log("medicines-api");

    tracing::info!("hi");

    let schema = create_schema();
    let context = warp::any().map(create_context);

    let graphql_filter = juniper_warp::make_graphql_filter(schema, context.boxed());

    warp::serve(
        warp::get()
            .and(warp::path("graphiql").and(juniper_warp::graphiql_filter("/graphql", None)))
            .or(healthz())
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], PORT))
    .await
}

fn use_json_log_subscriber() {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .json()
        .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc3339())
        .with_max_level(Level::INFO)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn use_unstructured_log_subscriber() {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc3339())
        .with_max_level(Level::DEBUG)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

pub fn get_env_or_default<T>(key: &str, default: T) -> T
where
    T: FromStr + Display,
{
    get_env(key).unwrap_or_else(|e| {
        tracing::warn!(r#"defaulting {} to "{}" ({})"#, key, &default, e);
        default
    })
}

pub fn get_env<T>(key: &str) -> anyhow::Result<T>
where
    T: FromStr,
{
    env::var(key)?
        .parse::<T>()
        .map_err(|_| anyhow!("failed to parse for {}", key))
}
