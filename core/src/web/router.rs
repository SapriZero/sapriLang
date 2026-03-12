//! Gestione rotte e builder

use axum::{
Router,
routing::{get, post, put, delete},
response::IntoResponse,
extract::Request,
};
use std::collections::HashMap;
use crate::web::middleware::{Middleware, ValidationResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
GET,
POST,
PUT,
DELETE,
PATCH,
}

impl Method {
fn as_str(&self) -> &'static str {
match self {
Method::GET => "GET",
Method::POST => "POST",
Method::PUT => "PUT",
Method::DELETE => "DELETE",
Method::PATCH => "PATCH",
}
}
}

pub struct Route {
pub path: String,
pub method: Method,
pub handler: Box<dyn Fn(Request) -> Result<impl IntoResponse, ValidationResult> + Send + Sync>,
pub middlewares: Vec<Box<dyn Middleware>>,
}

impl Route {
pub fn new<H, R>(path: impl Into<String>, method: Method, handler: H) -> Self
where
H: Fn(Request) -> Result<R, ValidationResult> + Send + Sync + 'static,
R: IntoResponse,
{
Self {
path: path.into(),
method,
handler: Box::new(move |req| handler(req).map(|r| r.into_response())),
middlewares: Vec::new(),
}
}

pub fn with_middleware(mut self, mw: impl Middleware + 'static) -> Self {
self.middlewares.push(Box::new(mw));
self
}
}

pub struct RouterBuilder {
router: Router,
routes: Vec<Route>,
}

impl RouterBuilder {
pub fn new(router: Router) -> Self {
Self {
router,
routes: Vec::new(),
}
}

pub fn add_route(mut self, route: Route) -> Self {
self.routes.push(route);
self
}

pub fn build(self) -> Router {
let mut router = self.router;

for route in self.routes {
let path = route.path.clone();
let handler = move |req| {
// Applica middleware in ordine
let mut req = req;
for mw in &route.middlewares {
match mw.handle(req) {
Ok(r) => req = r,
Err(e) => return e.into_response(),
}
}
(route.handler)(req).unwrap_or_else(|e| e.into_response())
};

router = match route.method {
Method::GET => router.route(&path, get(handler)),
Method::POST => router.route(&path, post(handler)),
Method::PUT => router.route(&path, put(handler)),
Method::DELETE => router.route(&path, delete(handler)),
Method::PATCH => router.route(&path, delete(handler)),
};
}

router
}
}

