use local_http_server::app;
use local_http_server::config::Config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = Config::default();
    let app = app();

    let addr = config.address();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("🚀 Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
