//! Data Transfer Objects e validazione

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Dto: Sized + Send + Sync {
fn validate(&self) -> Result<(), String>;
}

pub trait Validatable {
fn validate(&self) -> Result<(), String>;
}

// Esempio di DTO per creazione utente
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
pub username: String,
pub email: String,
pub age: Option<u8>,
}

impl Validatable for CreateUserDto {
fn validate(&self) -> Result<(), String> {
if self.username.len() < 3 {
return Err("Username troppo corto (min 3 caratteri)".to_string());
}
if !self.email.contains('@') {
return Err("Email non valida".to_string());
}
if let Some(age) = self.age {
if age < 18 {
return Err("Età minima 18 anni".to_string());
}
}
Ok(())
}
}

// DTO per risposta
#[derive(Debug, Serialize)]
pub struct UserResponse {
pub id: i64,
pub username: String,
pub email: String,
pub created_at: chrono::DateTime<chrono::Utc>,
}

