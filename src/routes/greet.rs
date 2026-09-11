use axum::{extract::Path, routing::get, Json, Router};

use crate::models::response::GreetResponse;

async fn hello(Path(name): Path<String>) -> Json<GreetResponse> {
    let message = format!("Hello, {}!", name);
    Json(GreetResponse { message, name })
}

pub fn routes() -> Router {
    Router::new().route("/hello/{name}", get(hello))
}
