//! Definizione errori per sapri_db

use std::fmt;

/// Risultato tipizzato
pub type Result<T> = std::result::Result<T, DbError>;

/// Errori del database
#[derive(Debug, Clone, PartialEq)]
pub enum DbError {
    Schema(String),
    Loader(String),
    Validator(String),
    Binary(String),
    Table(String),
    NotFound(String),
    InvalidHeader(String),
}

/// Trait per la gestione degli errori
pub trait ErrorDisplay {
    fn fmt_error(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dberror_display() {
        let err = DbError::Schema("test error".to_string());
        assert_eq!(err.to_string(), "Schema error: test error");

        let err = DbError::Loader("test error".to_string());
        assert_eq!(err.to_string(), "Loader error: test error");

        let err = DbError::Validator("test error".to_string());
        assert_eq!(err.to_string(), "Validator error: test error");

        let err = DbError::Binary("test error".to_string());
        assert_eq!(err.to_string(), "Binary error: test error");

        let err = DbError::Table("test error".to_string());
        assert_eq!(err.to_string(), "Table error: test error");

        let err = DbError::NotFound("test error".to_string());
        assert_eq!(err.to_string(), "Not found: test error");

        let err = DbError::InvalidHeader("test error".to_string());
        assert_eq!(err.to_string(), "Invalid header: test error");
    }
}
