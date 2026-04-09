//! Middleware per validazione, autenticazione, logging

use axum::{
response::{IntoResponse, Response},
http::{Request, StatusCode},
Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type ValidationResult = Response;

pub trait Middleware: Send + Sync {
fn handle(&self, req: Request) -> Result<Request, ValidationResult>;
}

// Middleware di logging
pub struct Logger;

impl Middleware for Logger {
fn handle(&self, req: Request) -> Result<Request, ValidationResult> {
println!("📝 {} {}", req.method(), req.uri());
Ok(req)
}
}

// Middleware di autenticazione con token
pub struct Auth {
token: String,
}

impl Auth {
pub fn new(token: impl Into<String>) -> Self {
Self {
token: token.into(),
}
}
}

impl Middleware for Auth {
fn handle(&self, mut req: Request) -> Result<Request, ValidationResult> {
let headers = req.headers();

if let Some(auth) = headers.get("Authorization") {
if let Ok(auth_str) = auth.to_str() {
if auth_str == format!("Bearer {}", self.token) {
return Ok(req);
}
}
}

Err((
StatusCode::UNAUTHORIZED,
Json(ErrorResponse {
error: "Unauthorized".to_string(),
message: "Token non valido o mancante".to_string(),
}),
).into_response())
}
}

// Middleware di validazione parametri
pub struct ValidateParam<T> {
name: String,
validator: Arc<dyn Fn(&T) -> bool + Send + Sync>,
}

impl<T> ValidateParam<T> {
pub fn new(name: impl Into<String>, validator: impl Fn(&T) -> bool + Send + Sync + 'static) -> Self {
Self {
name: name.into(),
validator: Arc::new(validator),
}
}
}

impl<T> Middleware for ValidateParam<T>
where
T: 'static + Send + Sync,
{
fn handle(&self, req: Request) -> Result<Request, ValidationResult> {
// Qui andrebbe estratta la logica per ottenere il parametro
// Per ora passiamo sempre
Ok(req)
}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
pub error: String,
pub message: String,
}

