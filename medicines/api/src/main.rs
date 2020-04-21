use actix_cors::Cors;
use actix_web::{http, middleware, web, App, Error, HttpResponse, HttpServer};
use anyhow::anyhow;
use core::fmt::Display;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use listenfd::ListenFd;
use std::{env, str::FromStr};
use std::{io, sync::Arc};
use tracing::Level;

mod azure_search;
mod pagination;
mod product;
mod schema;
mod substance;

const PORT: u16 = 8000;

use crate::{
    azure_search::AzureContext,
    schema::{create_schema, Schema},
};
use azure_search::create_context;

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    context: web::Data<Arc<AzureContext>>,
) -> Result<HttpResponse, Error> {
    let res = data.execute_async(&st, &context).await;
    let body = serde_json::to_string(&res)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

async fn healthz() -> impl actix_web::Responder {
    "OK"
}

fn cors_middleware() -> actix_cors::CorsFactory {
    Cors::new()
        .allowed_methods(vec!["POST"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .max_age(3600)
        .finish()
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    if get_env_or_default("JSON_LOGS", true) {
        use_json_log_subscriber()
    } else {
        use_unstructured_log_subscriber()
    }

    let mut listenfd = ListenFd::from_env();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());
    let context = std::sync::Arc::new(create_context());

    // Start http server
    let mut server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(context.clone())
            .wrap(middleware::Logger::default())
            .wrap(cors_middleware())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .service(web::resource("/healthz").route(web::get().to(healthz)))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(format!(
            "0.0.0.0:{}",
            std::env::var("PORT").unwrap_or_else(|e| {
                eprintln!(
                    "Error reading $PORT env var (defaulting to {}): {}",
                    PORT, e
                );
                PORT.to_string()
            })
        ))?
    };

    server.run().await
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
