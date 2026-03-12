use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use std::sync::Arc;

use crate::handlers::*;
use crate::AppState;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/files", get(list_files_handler))
        .route("/api/files/:name", get(get_file_handler))
        .route("/api/files/:name", post(save_file_handler))
        .route("/api/execute", post(execute_handler))
        .route("/ws", get(websocket_handler))
        .route("/", get(index_handler))
        .route("/editor", get(editor_handler))
        .route("/api/chat/save", post(save_chat_handler))
        .nest_service("/static", ServeDir::new("editor-server/static"))
        .fallback_service(ServeDir::new("editor-server/static"))
        .with_state(state)
}

