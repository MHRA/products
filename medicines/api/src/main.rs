use crate::azure_context::create_context;
use anyhow::anyhow;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    QueryBuilder,
};
use async_graphql_warp::{BadRequest, GQLResponse};
use core::fmt::Display;
use std::{convert::Infallible, env, net::SocketAddr, str::FromStr};
use tracing::Level;
use warp::{
    self,
    http::{header, Method, Response, StatusCode},
    Filter, Rejection, Reply,
};

mod azure_context;
mod pagination;
mod query_objects;
mod schema;

const PORT: u16 = 8000;

pub fn healthz() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    warp::path!("healthz")
        .and(warp::get())
        .map(warp::reply)
        .map(|reply| warp::reply::with_status(reply, StatusCode::NO_CONTENT))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if get_env_or_default("JSON_LOGS", true) {
        use_json_log_subscriber()
    } else {
        use_unstructured_log_subscriber()
    }

    let log = warp::log("medicines-api");

    let products_index = get_env_or_default("AZURE_SEARCH_INDEX", "products-index".to_string());
    let bmgf_index = get_env_or_default("BMGF_AZURE_SEARCH_INDEX", "bmgf-index".to_string());
    let schema = schema::ApiSchema::new(create_context(products_index, bmgf_index));

    let cors = warp::cors()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .allow_any_origin();

    let addr = format!("0.0.0.0:{}", get_env_or_default("PORT", PORT.to_string()))
        .parse::<SocketAddr>()?;

    let graphql_post = async_graphql_warp::graphql(schema.0)
        .and_then(|(schema, builder): (_, QueryBuilder)| async move {
            let response = builder.execute(&schema).await;
            Ok::<_, Infallible>(GQLResponse::from(response))
        })
        .with(cors.clone());

    let graphql_options = warp::options()
        .map(warp::reply)
        .with(cors)
        .with(warp::log("cors-only"));

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let routes = healthz()
        .or(graphql_playground)
        .or(graphql_options)
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(BadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    let _ = tokio::join!(tokio::spawn(async move {
        warp::serve(routes.with(log)).run(addr).await
    }));

    Ok(())
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
