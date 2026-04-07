use std::fmt;
use thiserror::Error;

#[derive(thiserror::Error, Debug)]
pub enum SsonError {
    #[error("Parse error at line {line}: {message}")]
    Parse { line: usize, message: String },

    #[error("Circular reference detected: {path}")]
    CircularRef { path: String },

    #[error("Unknown type code: '{0}'")]
    UnknownTypeCode(String),

    #[error("Validation failed in strict mode: {0}")]
    StrictViolation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl fmt::Display for SsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SsonError::Parse { line, message } => write!(f, "Parse error at line {}: {}", line, message),
            SsonError::CircularRef { path } => write!(f, "Circular reference: {}", path),
            SsonError::UnknownTypeCode(code) => write!(f, "Unknown type code: '{}'", code),
            SsonError::StrictViolation(msg) => write!(f, "Strict violation: {}", msg),
            SsonError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for SsonError {}

pub type Result<T> = std::result::Result<T, SsonError>;
