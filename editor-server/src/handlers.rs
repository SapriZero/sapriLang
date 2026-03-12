//! Handler delle richieste HTTP

use axum::{
    extract::{Path, State, WebSocketUpgrade},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde_json::json;
use std::fs;
use std::sync::Arc;

use crate::services::file_service;
use crate::execute::execute_script;
use crate::websocket::handle_socket;
use crate::AppState;

pub async fn index_handler() -> impl IntoResponse {
    match fs::read_to_string("editor-server/static/index.html") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Errore: {}", e)),
    }
}

pub async fn editor_handler() -> impl IntoResponse {
    match fs::read_to_string("editor-server/static/editor.html") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Errore: {}", e)),
    }
}

pub async fn list_files_handler(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match file_service::list_files(&state.scripts_dir) {
        Ok(files) => (StatusCode::OK, Json(files)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))).into_response(),
    }
}

pub async fn save_file_handler(
    Path(name): Path<String>,
    State(state): State<Arc<AppState>>,
    body: String,
) -> impl IntoResponse {
    match file_service::write_file(&state.scripts_dir, &name, &body) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_file_handler(
    Path(name): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match file_service::read_file(&state.scripts_dir, &name) {
        Ok(content) => (StatusCode::OK, content),
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()),
    }
}

pub async fn execute_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<serde_json::Value>,
) -> impl IntoResponse {
    let code = req["code"].as_str().unwrap_or("");

    match execute_script(code) {
        Ok(output) => {
            for line in &output {
                let _ = state.output_tx.send(line.clone());
            }
            Json(json!({ "success": true, "output": output }))
        }
        Err(e) => Json(json!({ "success": false, "error": e.to_string() })),
    }
}

pub async fn save_chat_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<serde_json::Value>,
) -> impl IntoResponse {
    let content = req["content"].as_str().unwrap_or("");
    let filename = req["filename"].as_str().unwrap_or("chat.txt");
    
    // Crea cartella chats se non esiste
    let chat_dir = std::path::Path::new("./chats");
    if !chat_dir.exists() {
        std::fs::create_dir_all(chat_dir).unwrap();
    }
    
    let path = chat_dir.join(filename);
    match std::fs::write(&path, content) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}
