use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EchoRequest {
    pub message: String,
}
