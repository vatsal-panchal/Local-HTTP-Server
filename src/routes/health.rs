use axum::{routing::get, Json, Router};

use crate::models::response::{HealthResponse, StatusResponse};

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        uptime: "running".to_string(),
    })
}

async fn status() -> Json<StatusResponse> {
    Json(StatusResponse {
        server: "local-http-server".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        rust_version: "2021 edition".to_string(),
    })
}

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/status", get(status))
}
