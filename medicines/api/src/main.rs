const PORT: u16 = 8000;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use listenfd::ListenFd;
use std::{io, sync::Arc};

mod schema;
mod pagination;

use crate::schema::{create_schema, Schema};

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

async fn healthz() -> impl actix_web::Responder {
    "OK"
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info, actix_server=info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    let mut server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
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
