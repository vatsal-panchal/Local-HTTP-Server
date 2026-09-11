use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GreetResponse {
    pub message: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub uptime: String,
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub server: String,
    pub version: String,
    pub rust_version: String,
}

#[derive(Debug, Serialize)]
pub struct EchoResponse {
    pub you_said: String,
}
