//! Strutture dati per l'AST .sson

use std::collections::HashMap;

/// Documento .sson completo
#[derive(Debug, Clone, PartialEq)]
pub struct SsonDocument {
    /// Metadati globali (_META)
    pub metadata: HashMap<String, Value>,
    /// Tabelle definite
    pub tables: Vec<Table>,
}

/// Tabella .sson
#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    /// Nome completo (es. ["users"] o ["orders", "items"])
    pub name: Vec<String>,
    /// Campi della tabella
    pub fields: Vec<Field>,
    /// Righe di dati
    pub rows: Vec<Row>,
}

/// Campo di una tabella
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// Nome del campo
    pub name: String,
    /// Tipo opzionale (es. :string, :int)
    pub field_type: Option<String>,
}

/// Riga di dati (lista di valori)
pub type Row = Vec<Value>;

/// Valore in una cella
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl Value {
    /// Converte in stringa se possibile
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Converte in numero se possibile
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Converte in booleano se possibile
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// È null?
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}
