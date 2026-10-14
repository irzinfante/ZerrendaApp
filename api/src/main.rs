mod api;
mod config;
mod db;
mod errors;
mod helpers;
mod models;
mod services;

use api::middleware::enforce_auth;
use axum::{
    Router,
    http::{HeaderValue, Method, header},
    middleware,
    routing::get,
};
use config::Config;
use couch_rs::Client;
use helpers::auth_helpers::signing_key_from_hex;
use services::auth_service::AuthService;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    let db_client = Client::new(&config.db_host, &config.db_user, &config.db_password)
        .expect("Failed to open database");

    let health_check =
        Router::new().route("/healthcheck", get(async || -> &'static str { "Running!" }));

    let signing_key = signing_key_from_hex(&config.auth_private_key)
        .expect("AUTH_PRIVATE_KEY must be a hex-encoded P-256 private key");

    let auth_service = Arc::new(AuthService::new(&db_client, signing_key).await);

    let protected_endpoints =
        Router::new()
            .merge(api::dummy_api())
            .route_layer(middleware::from_fn_with_state(
                auth_service.clone(),
                enforce_auth,
            ));

    let allowed_origins = config
        .cors_allowed_origins
        .iter()
        .map(|origin| {
            HeaderValue::from_str(origin)
                .expect("CORS_ALLOWED_ORIGINS must be a comma-separated list of origins")
        })
        .collect::<Vec<HeaderValue>>();
    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let app = Router::new()
        .merge(health_check)
        .merge(api::auth_api(auth_service.clone()))
        .merge(protected_endpoints)
        .layer(cors);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.app_host, config.app_port))
            .await
            .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
