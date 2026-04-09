//! Server HTTP principale

use crate::web::router::{Route, RouterBuilder};
use axum::{Router, Server};
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ServerConfig {
pub host: String,
pub port: u16,
pub workers: Option<usize>,
pub backlog: Option<u32>,
}

impl Default for ServerConfig {
fn default() -> Self {
Self {
host: "127.0.0.1".to_string(),
port: 8080,
workers: None,
backlog: None,
}
}
}

pub struct WebServer {
config: ServerConfig,
router: Router,
routes: Vec<Route>,
}

impl WebServer {
pub fn new(config: ServerConfig) -> Self {
Self {
config,
router: Router::new(),
routes: Vec::new(),
}
}

pub fn with_port(port: u16) -> Self {
Self::new(ServerConfig {
port,
..Default::default()
})
}

pub fn route(mut self, route: Route) -> Self {
self.routes.push(route);
self
}

pub fn routes(mut self, routes: Vec<Route>) -> Self {
self.routes.extend(routes);
self
}

pub fn build_router(self) -> Router {
let mut builder = RouterBuilder::new(self.router);
for route in self.routes {
builder = builder.add_route(route);
}
builder.build()
}

pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
let router = self.build_router();
let addr = format!("{}:{}", self.config.host, self.config.port)
.parse::<SocketAddr>()?;

println!("🚀 Server in ascolto su http://{}", addr);

Server::bind(&addr)
.serve(router.into_make_service())
.await?;

Ok(())
}
}

