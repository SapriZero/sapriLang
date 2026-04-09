//! Schema e catalogo delle definizioni URCM.
//! Mappa entità → campi, tipi e vincoli strutturali.
//! Zero dipendenze esterne. Puro e leggero.

use std::collections::HashMap;
use crate::error::{BaseError, Result};

/// Definizione di un singolo campo
#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub type_name: String,   // es. "n", "s", "bool"
    pub required: bool,
    pub constraints: Vec<String>,
}

/// Catalogo immutabile di entità registrate
#[derive(Debug, Clone, Default)]
pub struct SchemaCatalog {
    entries: HashMap<String, Vec<FieldDef>>,
}

impl SchemaCatalog {
    /// Registra un'entità con i suoi campi
    pub fn register(&mut self, entity: impl Into<String>, fields: Vec<FieldDef>) {
        self.entries.insert(entity.into(), fields);
    }

    /// Ottieni la definizione di un'entità
    pub fn get(&self, entity: &str) -> Result<&[FieldDef]> {
        self.entries.get(entity)
            .map(|v| v.as_slice())
            .ok_or_else(|| BaseError::NotFound { path: entity.into() })
    }

    /// Verifica se un'entità è registrata
    pub fn contains(&self, entity: &str) -> bool {
        self.entries.contains_key(entity)
    }

    /// Itera su tutte le entità registrate
    pub fn iter(&self) -> impl Iterator<Item = (&String, &[FieldDef])> {
        self.entries.iter().map(|(k, v)| (k, v.as_slice()))
    }
}
