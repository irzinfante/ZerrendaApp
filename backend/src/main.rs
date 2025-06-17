mod api;
mod config;
mod db;
mod errors;
mod helpers;
mod models;
mod services;

use axum::{middleware, routing::get, Router};
use config::Config;
use couch_rs::Client;
use helpers::auth_helpers::enforce_auth;
use services::auth_service::AuthService;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    let db_client = Arc::new(Mutex::new(
        Client::new(&config.db_host, &config.db_user, &config.db_password)
            .expect("Failed to open database"),
    ));

    let health_chek = Router::new()
        .route("/healthcheck", get(async || -> &'static str { "Running!" }));

    let auth_service = Arc::new(AuthService::new(db_client.clone()).await);

    let protected_endpoints = Router::new()
        .merge(api::dummy_api())
        .route_layer(middleware::from_fn(enforce_auth));

    let app = Router::new()
        .merge(health_chek)
        .merge(api::auth_api(auth_service.clone()))
        .merge(protected_endpoints);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.app_host, config.app_port))
            .await
            .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
