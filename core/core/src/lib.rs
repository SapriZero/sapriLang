//! SAPRI CORE - Nucleo del sistema
//! 
//! Coordina:
//! - Configurazione da file .sson
//! - Esecuzione IRCM (sapri_rust_dsl)
//! - Hot-reload a runtime
//! - Comandi interattivi

pub mod config;
pub mod hot_reload;
pub mod executor;
pub mod runtime;
pub mod command;

pub use config::Config;
pub use hot_reload::HotReload;
pub use executor::Executor;
pub use runtime::Runtime;
pub use command::Command;

