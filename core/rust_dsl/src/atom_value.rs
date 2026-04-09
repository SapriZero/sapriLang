//! Valori che possono essere contenuti in un atomo nel DSL

use std::ops::Mul;
use sapri_obj::Obj;  // nuova dipendenza

#[derive(Debug, Clone, PartialEq)]
pub enum AtomValue {
    Number(f64),
    String(String),
    Bool(bool),
    Obj(Obj),  // nuova!
}

impl AtomValue {
    /// Converte in f64 se possibile (Number), altrimenti None
    pub fn as_number(&self) -> Option<f64> {
        match self {
            AtomValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Converte in String se possibile
    pub fn as_string(&self) -> Option<&str> {
        match self {
            AtomValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Converte in bool se possibile
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            AtomValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    
    pub fn as_obj(&self) -> Option<&Obj> {
        match self {
            AtomValue::Obj(obj) => Some(obj),
            _ => None,
        }
    }
    
    pub fn into_obj(self) -> Option<Obj> {
        match self {
            AtomValue::Obj(obj) => Some(obj),
            _ => None,
        }
    }
}

// Moltiplicazione: solo per Number * Number → Number
// Altri tipi producono Number(0.0) o si può gestire con errore
impl Mul for AtomValue {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (AtomValue::Number(a), AtomValue::Number(b)) => AtomValue::Number(a * b),
            _ => AtomValue::Number(0.0), // per ora, in futuro errore
        }
    }
}

// Conversioni
impl From<Obj> for AtomValue {
    fn from(obj: Obj) -> Self {
        AtomValue::Obj(obj)
    }
}

// Conversioni da tipi semplici
impl From<f64> for AtomValue {
    fn from(n: f64) -> Self {
        AtomValue::Number(n)
    }
}

impl From<i32> for AtomValue {
    fn from(n: i32) -> Self {
        AtomValue::Number(n as f64)
    }
}

impl From<String> for AtomValue {
    fn from(s: String) -> Self {
        AtomValue::String(s)
    }
}

impl From<&str> for AtomValue {
    fn from(s: &str) -> Self {
        AtomValue::String(s.to_string())
    }
}

impl From<bool> for AtomValue {
    fn from(b: bool) -> Self {
        AtomValue::Bool(b)
    }
}
