const PORT: u16 = 8000;
use actix_cors::Cors;
use actix_web::{http, middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use listenfd::ListenFd;
use std::{io, sync::Arc};

mod azure_search;
mod pagination;
mod product;
mod schema;
mod substance;

use crate::{
    azure_search::{create_context, AzureContext},
    schema::{create_schema, Schema},
};

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
    env_logger::init();

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
