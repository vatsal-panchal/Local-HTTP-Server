use axum::{routing::get, Router};

async fn root() -> &'static str {
    "Hello from Rust!"
}

async fn about() -> &'static str {
    "This is a simple local HTTP server built with Rust and Axum. It demonstrates routing, JSON responses, error handling, and modular project structure."
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/about", get(about))
}
