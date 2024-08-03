use apis::crud::get_apis;
use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use proxy::proxy_request;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs::File, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};

mod apis;
mod proxy;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct API {
    pub identifier: String,
    pub metadata: String,
    pub api_key: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let apis_path = env::var("APIS_PATH").unwrap_or("apis.json".to_string());

    // Read APIs from file and create a hashmap
    let file = File::open(&apis_path).expect(format!("Failed to open file: {}", apis_path).as_str());
    let apis: HashMap<String, API> =
        serde_json::from_reader(file).expect(format!("Failed to read file: {}", apis_path).as_str());

    // Create the router and add the routes
    let app = Router::new()
        .route("/", get(get_apis))
        .route("/proxy", post(proxy_request))
        .with_state(apis)
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::DELETE])
                .allow_headers(Any),
        );

    let certificate_path = env::var("SSL_CERT_PATH");
    let key_path = env::var("SSL_KEY_PATH");
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());

    log::info!("Starting server on: {}", host);

    // Check if the SSL certificate and key paths are valid
    match (certificate_path, key_path) {
        (Ok(certificate_path), Ok(key_path)) => {
            log::info!("Attempting to start server with SSL");
            let config = RustlsConfig::from_pem_file(certificate_path, key_path)
                .await
                .unwrap();
            axum_server::bind_rustls(
                format!("{}:{}", host, port).parse::<SocketAddr>().unwrap(),
                config,
            );
        }
        _ => {
            log::info!(
                "Invalid or missing SSL certificate and key paths. Starting server without SSL"
            );
            let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
                .await
                .unwrap();
            axum::serve(listener, app).await.unwrap();
        }
    }
}
