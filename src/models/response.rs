use serde::Serialize;

/// Response for the GET /hello/:name endpoint.
#[derive(Debug, Serialize)]
pub struct GreetResponse {
    pub message: String,
    pub name: String,
}

/// Response for the GET /health endpoint.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub uptime: String,
}

/// Response for the GET /status endpoint.
#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub server: String,
    pub version: String,
    pub rust_version: String,
}

/// Response for the POST /echo endpoint.
#[derive(Debug, Serialize)]
pub struct EchoResponse {
    pub you_said: String,
}
