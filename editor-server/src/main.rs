//! URCM Editor Server - Entry point

mod router;
mod handlers;
mod services;
mod websocket;
mod execute;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::info;

#[derive(Debug, Clone)]
pub struct AppState {
    pub scripts_dir: std::path::PathBuf,
    pub output_tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let scripts_dir = std::path::Path::new("./scripts");
    if !scripts_dir.exists() {
        std::fs::create_dir_all(scripts_dir).expect("Failed to create scripts directory");
    }

    let (tx, _) = broadcast::channel(100);

    let state = Arc::new(AppState {
        scripts_dir: scripts_dir.to_path_buf(),
        output_tx: tx,
    });

    let app = router::create_router(state);

    let addr: SocketAddr = format!("{}:{}", "127.0.0.1", 3000).parse().unwrap();
    info!("Editor server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
