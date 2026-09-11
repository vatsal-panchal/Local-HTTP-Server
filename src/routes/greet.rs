use axum::{extract::Path, routing::get, Json, Router};

use crate::models::response::GreetResponse;

/// Greeting handler — GET /hello/:name
async fn hello(Path(name): Path<String>) -> Json<GreetResponse> {
    let message = format!("Hello, {}!", name);
    Json(GreetResponse { message, name })
}

/// Returns the greet-related routes.
pub fn routes() -> Router {
    Router::new().route("/hello/{name}", get(hello))
}
