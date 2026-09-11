use axum::{routing::get, Router};

/// Home page handler — GET /
async fn root() -> &'static str {
    "Hello from Rust!"
}

/// About page handler — GET /about
async fn about() -> &'static str {
    "This is a simple local HTTP server built with Rust and Axum. It demonstrates routing, JSON responses, error handling, and modular project structure."
}

/// Returns the home-related routes.
pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/about", get(about))
}
