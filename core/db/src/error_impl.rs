//! Implementazione errori

use super::error::{DbError, ErrorDisplay};
use std::fmt;

impl ErrorDisplay for DbError {
    fn fmt_error(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::Schema(msg) => write!(f, "Schema error: {}", msg),
            DbError::Loader(msg) => write!(f, "Loader error: {}", msg),
            DbError::Validator(msg) => write!(f, "Validator error: {}", msg),
            DbError::Binary(msg) => write!(f, "Binary error: {}", msg),
            DbError::Table(msg) => write!(f, "Table error: {}", msg),
            DbError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DbError::InvalidHeader(msg) => write!(f, "Invalid header: {}", msg),
        }
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_error(f)
    }
}

// Implementazione di std::error::Error per DbError
impl std::error::Error for DbError {}
