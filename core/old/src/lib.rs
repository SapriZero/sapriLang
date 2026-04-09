pub mod error;
pub mod types;
pub mod fp;
pub mod fp_core;
pub mod array;

// Re-export per comodità
pub use error::{SapriError, Result};
pub use fp::{eval, mask, Either};
