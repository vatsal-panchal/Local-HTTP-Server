pub mod config;
pub mod errors;
pub mod models;
pub mod routes;

use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// Builds and returns the complete application router with all routes and middleware.
pub fn app() -> Router {
    Router::new()
        .merge(routes::home::routes())
        .merge(routes::greet::routes())
        .merge(routes::health::routes())
        .merge(routes::echo::routes())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
