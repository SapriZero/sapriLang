//! Controller (handlers) che chiamano i servizi

use axum::{
extract::{Path, Query, State},
Json,
response::IntoResponse,
http::StatusCode,
};
use std::sync::Arc;
use serde::Deserialize;
use crate::web::dto::{CreateUserDto, UserResponse};

// Esempio di stato condiviso
pub struct AppState {
pub db: Arc<dyn crate::io::Connector + Send + Sync>,
pub urcm: crate::core::UrcmCtx<()>,
}

// Controller per GET /users/:id
pub async fn get_user(
Path(id): Path<i32>,
State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
// Logica con URCM
// let comfort = state.urcm.get('C').unwrap_or(0.0);

// TODO: chiamare servizio
(
StatusCode::OK,
Json(UserResponse {
id: id as i64,
username: format!("user_{}", id),
email: format!("user{}@example.com", id),
created_at: chrono::Utc::now(),
}),
)
}

// Controller per POST /users
pub async fn create_user(
State(state): State<Arc<AppState>>,
Json(payload): Json<CreateUserDto>,
) -> impl IntoResponse {
// Valida il DTO
if let Err(e) = payload.validate() {
return (
StatusCode::BAD_REQUEST,
Json(serde_json::json!({ "error": e })),
);
}

// TODO: chiamare servizio per creare utente

(
StatusCode::CREATED,
Json(serde_json::json!({ "id": 123, "username": payload.username })),
)
}

// Controller per GET /health
pub async fn health_check() -> impl IntoResponse {
(
StatusCode::OK,
Json(serde_json::json!({ "status": "ok", "timestamp": chrono::Utc::now() })),
)
}

