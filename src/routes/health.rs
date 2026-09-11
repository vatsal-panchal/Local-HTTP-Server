use axum::{routing::get, Json, Router};

use crate::models::response::{HealthResponse, StatusResponse};

/// Health check handler — GET /health
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        uptime: "running".to_string(),
    })
}

/// Status handler — GET /status
async fn status() -> Json<StatusResponse> {
    Json(StatusResponse {
        server: "local-http-server".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        rust_version: "2021 edition".to_string(),
    })
}

/// Returns the health-related routes.
pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/status", get(status))
}
