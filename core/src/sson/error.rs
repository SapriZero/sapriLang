//! Errori del parser .sson

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SsonError {
    /// Errore di sintassi
    SyntaxError { line: usize, col: usize, message: String },

    /// Token non atteso
    UnexpectedToken { line: usize, col: usize, expected: String, found: String },

    /// Sezione senza nome
    MissingSectionName { line: usize },

    /// Numero campi errato in una riga
    FieldCountMismatch { line: usize, expected: usize, got: usize },

    /// Carattere non valido
    InvalidChar { line: usize, c: char },

    /// Valore non valido
    InvalidValue { line: usize, value: String, expected_type: String },

    /// Errore I/O
    IoError(String),

    /// Errore generico
    Other(String),
}

impl fmt::Display for SsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SsonError::SyntaxError { line, col, message } => {
                write!(f, "Syntax error at {}:{}: {}", line, col, message)
            }
            SsonError::UnexpectedToken { line, col, expected, found } => {
                write!(f, "Unexpected token at {}:{}: expected {}, found {}", line, col, expected, found)
            }
            SsonError::MissingSectionName { line } => {
                write!(f, "Missing section name at line {}", line)
            }
            SsonError::FieldCountMismatch { line, expected, got } => {
                write!(f, "Field count mismatch at line {}: expected {}, got {}", line, expected, got)
            }
            SsonError::InvalidChar { line, c } => {
                write!(f, "Invalid character '{}' at line {}", c, line)
            }
            SsonError::InvalidValue { line, value, expected_type } => {
                write!(f, "Invalid value '{}' at line {}, expected {}", value, line, expected_type)
            }
            SsonError::IoError(msg) => write!(f, "IO error: {}", msg),
            SsonError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for SsonError {}
