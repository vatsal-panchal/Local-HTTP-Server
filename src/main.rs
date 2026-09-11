use local_http_server::app;
use local_http_server::config::Config;

#[tokio::main]
async fn main() {
    // Initialize tracing (logging)
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Load configuration
    let config = Config::default();

    // Build the application
    let app = app();

    // Bind and serve
    let addr = config.address();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("🚀 Server running on http://{}", addr);
    tracing::info!("📋 Routes:");
    tracing::info!("   GET  /");
    tracing::info!("   GET  /about");
    tracing::info!("   GET  /health");
    tracing::info!("   GET  /status");
    tracing::info!("   GET  /hello/:name");
    tracing::info!("   POST /echo");

    axum::serve(listener, app).await.unwrap();
}
