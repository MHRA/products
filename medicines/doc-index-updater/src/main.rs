use doc_index_updater::{
    auth_manager::AuthenticationFailed, create_manager, delete_manager, document_manager,
    get_env_or_default, health, pars_upload, state_manager,
};
use state_manager::get_client;
use std::{convert::Infallible, error, net::SocketAddr, time::Duration};
use tracing::Level;
use warp::{http::StatusCode, Filter};

const PORT: u16 = 8000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    if get_env_or_default("JSON_LOGS", true) {
        use_json_log_subscriber()
    } else {
        use_unstructured_log_subscriber()
    }

    tracing_log::LogTracer::init()
        .expect("error redirecting normal log messages to the tracing subscriber");

    let addr = format!("0.0.0.0:{}", get_env_or_default("PORT", PORT.to_string()))
        .parse::<SocketAddr>()?;

    let redis_server = get_env_or_default("REDIS_SERVER", "127.0.0.1".to_string());
    let redis_port = get_env_or_default("REDIS_PORT", "6379".to_string());
    let redis_key = get_env_or_default("REDIS_KEY", "".to_string());
    let redis_addr = create_redis_url(redis_server, redis_port, redis_key);

    let time_to_wait = Duration::from_secs(get_env_or_default("SECONDS_TO_WAIT", 5));
    let clean_up_time_to_wait = time_to_wait * 10;
    let state = state_manager::StateManager::new(get_client(redis_addr.clone())?);

    let create_state = state.clone();
    let delete_state = state.clone();
    let create_clean_up_state = state.clone();
    let delete_clean_up_state = state.clone();

    let pars_origin = get_env_or_default(
        "PARS_UPLOAD_SITE_ORIGIN",
        "http://localhost:3000".to_string(),
    );

    let _ = tokio::join!(
        tokio::spawn(async move {
            warp::serve(
                health::get_health()
                    .or(state_manager::get_job_status_xml(state.clone()))
                    .or(state_manager::get_job_status(state.clone()))
                    .or(state_manager::set_job_status(state.clone()))
                    .or(document_manager::check_in_xml_document(state.clone()))
                    .or(document_manager::check_in_document(state.clone()))
                    .or(document_manager::delete_document_xml(state.clone()))
                    .or(document_manager::delete_document(state.clone()))
                    .or(pars_upload::handler(state.clone(), &pars_origin))
                    .or(pars_upload::update_handler(state.clone(), &pars_origin))
                    .recover(handle_rejection)
                    .with(warp::log("doc_index_updater")),
            )
            .run(addr)
            .await;
        }),
        tokio::spawn(delete_manager::delete_service_worker(
            time_to_wait,
            delete_state
        )),
        tokio::spawn(create_manager::create_service_worker(
            time_to_wait,
            create_state
        )),
        tokio::spawn(
            create_manager::clean_up_worker::create_queue_clean_up_worker(
                clean_up_time_to_wait,
                create_clean_up_state
            )
        ),
        tokio::spawn(
            delete_manager::clean_up_worker::delete_queue_clean_up_worker(
                clean_up_time_to_wait,
                delete_clean_up_state
            )
        ),
    );
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

fn create_redis_url(server: String, port: String, key: String) -> String {
    if key == "" {
        format!("redis://{}:{}", server, port)
    } else {
        format!("redis://:{}@{}:{}", key, server, port)
    }
}

#[derive(serde::Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(warp::reject::MethodNotAllowed { .. }) = err.find() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else if let Some(warp::reject::UnsupportedMediaType { .. }) = err.find() {
        code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
        message = "UNSUPPORTED_MEDIA_TYPE";
    } else if let Some(AuthenticationFailed) = err.find() {
        code = StatusCode::UNAUTHORIZED;
        message = "AUTHENTICATION_FAILED";
    } else {
        tracing::error!("Internal server error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
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
