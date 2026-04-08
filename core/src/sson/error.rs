use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SsonError {
    /// Errore di sintassi nel lexer
    LexerError { line: usize, col: usize, message: String },

    /// Token non atteso durante il parsing
    UnexpectedToken { line: usize, col: usize, expected: String, found: String },

    /// Path malformato o con caratteri non validi
    InvalidPath { path: String, reason: String },

    /// Campo con nome non valido
    InvalidFieldName { name: String, reason: String },

    /// Tipo sconosciuto o non parsabile
    UnknownTypeCode { code: String },

    /// Proprietà `_: ` non riconosciuta
    UnknownProperty { prop: String, context: String },

    /// Riferimento circolare rilevato
    CircularReference { path: String, cycle: Vec<String> },

    /// Violazione vincolo in strict mode
    StrictViolation { constraint: String, message: String },

    /// Errore I/O durante il caricamento file
    IoError(String),

    /// Errore generico con messaggio
    Other(String),
}

impl fmt::Display for SsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SsonError::LexerError { line, col, message } => {
                write!(f, "Lexer error at {}:{}: {}", line, col, message)
            }
            SsonError::UnexpectedToken { line, col, expected, found } => {
                write!(f, "Unexpected token at {}:{}: expected '{}', found '{}'", line, col, expected, found)
            }
            SsonError::InvalidPath { path, reason } => {
                write!(f, "Invalid path '{}': {}", path, reason)
            }
            SsonError::InvalidFieldName { name, reason } => {
                write!(f, "Invalid field name '{}': {}", name, reason)
            }
            SsonError::UnknownTypeCode { code } => {
                write!(f, "Unknown type code: '{}'", code)
            }
            SsonError::UnknownProperty { prop, context } => {
                write!(f, "Unknown property '_:{}' in context '{}'", prop, context)
            }
            SsonError::CircularReference { path, cycle } => {
                write!(f, "Circular reference detected: {} → [{}]", path, cycle.join(" → "))
            }
            SsonError::StrictViolation { constraint, message } => {
                write!(f, "Strict violation [{}]: {}", constraint, message)
            }
            SsonError::IoError(msg) => write!(f, "IO error: {}", msg),
            SsonError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for SsonError {}

pub type Result<T> = std::result::Result<T, SsonError>;
