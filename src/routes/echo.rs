use axum::{routing::post, Json, Router};

use crate::models::request::EchoRequest;
use crate::models::response::EchoResponse;

/// Echo handler — POST /echo
/// Accepts a JSON body and echoes it back.
async fn echo(Json(payload): Json<EchoRequest>) -> Json<EchoResponse> {
    Json(EchoResponse {
        you_said: payload.message,
    })
}

/// Returns the echo-related routes.
pub fn routes() -> Router {
    Router::new().route("/echo", post(echo))
}
