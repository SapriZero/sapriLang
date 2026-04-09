//! Errori fondamentali per le primitive di base.
//! 
//! Questo modulo definisce solo errori legati a:
//! - Manipolazione di array/slice
//! - Valutazione/espressione (eval)
//! - Transizioni di stato e atomi
//! - Argomenti non validi o operazioni non supportate
//! 
//! Nessun riferimento a `.sson`, bucket, CRUD o I/O.

use thiserror::Error;

/// Errore base per le primitive del sistema.
#[derive(Error, Debug, PartialEq)]
pub enum BaseError {
    #[error("Index {index} out of bounds (length {len})")]
    OutOfBounds { index: usize, len: usize },

    #[error("Invalid argument: {msg}")]
    InvalidArg { msg: String },

    #[error("Evaluation failed: {msg}")]
    EvalFailed { msg: String },

    #[error("State transition invalid: {from} → {to}")]
    InvalidTransition { from: String, to: String },

    #[error("Atomic operation failed: {msg}")]
    AtomError { msg: String },

    #[error("Unsupported operation: {op}")]
    Unsupported { op: String },

    #[error("Resource not found: {path}")]
    NotFound { path: String },
}

/// Alias standard per Result<T> nel layer base.
pub type Result<T> = std::result::Result<T, BaseError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = BaseError::OutOfBounds { index: 5, len: 3 };
        assert_eq!(format!("{}", err), "Index 5 out of bounds (length 3)");
    }

    #[test]
    fn test_result_alias() {
        fn maybe_fail(ok: bool) -> Result<i32> {
            if ok { Ok(42) } else { Err(BaseError::InvalidArg { msg: "bad".into() }) }
        }
        assert_eq!(maybe_fail(true), Ok(42));
        assert!(matches!(maybe_fail(false), Err(BaseError::InvalidArg { .. })));
    }
}
