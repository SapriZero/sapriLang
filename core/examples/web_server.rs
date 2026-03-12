//! Esempio di utilizzo del web server

use urcm_core::web::{
server::{WebServer, ServerConfig},
router::{Route, Method},
middleware::{Logger, Auth},
controller::{get_user, create_user, health_check},
};
use urcm_core::web::dto::CreateUserDto;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
// Configurazione server
let config = ServerConfig {
port: 3000,
..Default::default()
};

// Definisci rotte
let server = WebServer::new(config)
.route(
Route::new("/users/:id", Method::GET, get_user)
.with_middleware(Logger)
.with_middleware(Auth::new("secret-token"))
)
.route(
Route::new("/users", Method::POST, create_user)
.with_middleware(Logger)
)
.route(
Route::new("/health", Method::GET, health_check)
.with_middleware(Logger)
);

// Avvia server
server.start().await?;

Ok(())
}

