//! Parser per il formato .sson (Sapri Script Object Notation)
//! Utilizza il sistema a bucket 65535 per tokenizzazione ultra-veloce

mod parser;
mod token;
mod ast;
mod error;

pub use parser::{SsonParser, parse_sson};
pub use ast::{SsonDocument, Table, Field, Value};
pub use error::SsonError;

/// Versione semplice: parse diretto da stringa
pub fn from_str(input: &str) -> Result<SsonDocument, SsonError> {
    let mut parser = SsonParser::new();
    parser.parse(input)
}

/// Versione da file
pub fn from_file(path: &str) -> Result<SsonDocument, SsonError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| SsonError::IoError(e.to_string()))?;
    from_str(&content)
}
