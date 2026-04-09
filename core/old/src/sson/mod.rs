//! Sapri Structured Object Notation (SSON) Core
//!
//! Parser dichiarativo, dizionario piatto e validatore strutturale.
//! Progettato per essere AI-ready, deterministico e compatibile con URCM.

pub mod ast;
pub mod token;
pub mod lexer;
pub mod parser;
pub mod dict;
pub mod error;
pub mod validator;
pub use ast::ValidationReport;

pub use ast::{SsonDocument, FieldNode, SsonMode, TypeCode, FieldProperty, ConstraintRule, ConstraintKind, FlatDict};
pub use token::Token;
pub use lexer::Lexer;
pub use parser::{parse_sson, ParseResult};
pub use dict::FieldDict;
pub use error::SsonError;

/// Versione del parser
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Parse rapido da stringa a dizionario validato
pub fn parse_and_validate(input: &str, mode: SsonMode) -> Result<FieldDict, SsonError> {
    let doc = parse_sson(input)?;
    let mut dict = FieldDict::from_document(doc, mode);
    dict.validate();
    Ok(dict)
}
