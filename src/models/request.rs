use serde::Deserialize;

/// Request body for the POST /echo endpoint.
#[derive(Debug, Deserialize)]
pub struct EchoRequest {
    pub message: String,
}
