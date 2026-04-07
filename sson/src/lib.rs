//! Sapri Structured Object Notation (SSON) Core
//!
//! Parser dichiarativo, dizionario piatto e validatore strutturale.
//! Progettato per essere AI-ready, deterministico e compatibile con URCM.

pub mod ast;
pub mod token;
pub mod parser;
pub mod dict;
pub mod error;

pub use ast::{SsonDocument, FieldNode, SsonMode, TypeCode};
pub use dict::FieldDict;
pub use error::SsonError;
pub use parser::parse_sson;
