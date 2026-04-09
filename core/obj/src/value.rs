//! Valori che possono essere contenuti in un oggetto

use std::fmt;
use crate::obj::Obj;

/// Valori supportati da Obj
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Valore nullo
    Null,
    /// Booleano
    Bool(bool),
    /// Numero (f64)
    Number(f64),
    /// Stringa
    String(String),
    /// Oggetto annidato
    Obj(Obj),
}

impl Value {
    /// Converte in Option<f64>
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Converte in Option<bool>
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Converte in Option<&str>
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Converte in Option<&Obj>
    pub fn as_obj(&self) -> Option<&Obj> {
        match self {
            Value::Obj(obj) => Some(obj),
            _ => None,
        }
    }

    /// Converte in Option<Obj> (con consumo)
    pub fn into_obj(self) -> Option<Obj> {
        match self {
            Value::Obj(obj) => Some(obj),
            _ => None,
        }
    }

    /// Verifica se è Null
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Verifica se è un oggetto
    pub fn is_obj(&self) -> bool {
        matches!(self, Value::Obj(_))
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

// Conversioni da tipi semplici
impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Value::Number(n as f64)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<Obj> for Value {
    fn from(obj: Obj) -> Self {
        Value::Obj(obj)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Obj(obj) => write!(f, "{:?}", obj),
        }
    }
}
