//! Flattening JSON → formato tabellare
//!
//! Trasforma JSON complessi in strutture piatte con dot notation,
//! separando automaticamente array in tabelle distinte.

use serde_json::Value;
use std::collections::HashMap;
use urcm_core::registry::Registry;

#[derive(Debug, Clone)]
pub struct FlattenOptions {
    /// Massima profondità di flattening (None = illimitato)
    pub max_depth: Option<usize>,
    /// Separa array in tabelle distinte
    pub separate_arrays: bool,
    /// Genera automaticamente definizioni mancanti
    pub auto_define: bool,
    /// Prefisso per tabelle figlie
    pub child_prefix: Option<String>,
}

impl Default for FlattenOptions {
    fn default() -> Self {
        Self {
            max_depth: None,
            separate_arrays: true,
            auto_define: true,
            child_prefix: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FlattenedData {
    /// Nome della tabella principale
    pub name: String,
    /// Campi con i loro valori (dot notation)
    pub fields: HashMap<String, Value>,
    /// Tabelle figlie (per array separati)
    pub children: Vec<Table>,
    /// Definizioni create/aggiornate
    pub definitions: Vec<DataDefinition>,
}

pub struct Flattener {
    registry: Option<Registry>,
    options: FlattenOptions,
}

impl Flattener {
    pub fn new() -> Self {
        Self {
            registry: None,
            options: FlattenOptions::default(),
        }
    }

    pub fn with_registry(registry: Registry) -> Self {
        Self {
            registry: Some(registry),
            options: FlattenOptions::default(),
        }
    }

    pub fn with_options(mut self, options: FlattenOptions) -> Self {
        self.options = options;
        self
    }

    /// Flattena un JSON value
    pub fn flatten(&self, value: &Value) -> Result<FlattenedData, FlattenError> {
        // TODO: implementare flattening ricorsivo
        // Usa json-unflattening o simd-json
        unimplemented!()
    }

    /// Flattena un JSON da stringa
    pub fn flatten_str(&self, json_str: &str) -> Result<FlattenedData, FlattenError> {
        let value: Value = serde_json::from_str(json_str)?;
        self.flatten(&value)
    }

    /// Flattena con inferenza schema
    pub fn flatten_with_schema(&self, value: &Value) -> Result<(FlattenedData, Schema), FlattenError> {
        let flattened = self.flatten(value)?;
        let schema = SchemaInferrer::new().infer_from_flattened(&flattened)?;
        Ok((flattened, schema))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FlattenError {
    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Invalid structure: {0}")]
    InvalidStructure(String),

    #[error("Max depth exceeded")]
    MaxDepthExceeded,

    #[error("Registry error: {0}")]
    RegistryError(String),
}

impl Default for Flattener {
    fn default() -> Self {
        Self::new()
    }
}
