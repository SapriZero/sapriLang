//! Web server module for URCM
//! Fornisce un server HTTP configurabile via Rust o DSL

pub mod server;
pub mod router;
pub mod middleware;
pub mod controller;
pub mod dto;

pub use server::{WebServer, ServerConfig};
pub use router::{Route, Method, RouterBuilder};
pub use middleware::{Middleware, ValidationResult};
pub use dto::{Dto, Validatable};

