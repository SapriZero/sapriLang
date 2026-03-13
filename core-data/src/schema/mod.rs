//! Inferenza e gestione schema
//!
//! Usa genson-rs per inferenza ultra-veloce (25-75x più veloce di Python)
//! e jsonschema per validazione performante.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    Array(Box<FieldType>),
    Object(HashMap<String, FieldType>),
    Null,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDef {
    pub name: String,
    pub field_type: FieldType,
    pub nullable: bool,
    pub description: Option<String>,
    pub constraints: Option<Value>,  // JSON Schema constraints
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub fields: Vec<FieldDef>,
    pub metadata: HashMap<String, Value>,
}

pub struct SchemaInferrer {
    options: InferOptions,
}

#[derive(Debug, Clone)]
pub struct InferOptions {
    /// Campi obbligatori da cercare
    pub required_fields: Vec<String>,
    /// Inferisci enum da valori stringa
    pub infer_enums: bool,
    /// Inferisci array types
    pub infer_arrays: bool,
    /// Profondità massima
    pub max_depth: usize,
}

impl Default for InferOptions {
    fn default() -> Self {
        Self {
            required_fields: Vec::new(),
            infer_enums: true,
            infer_arrays: true,
            max_depth: 10,
        }
    }
}

impl SchemaInferrer {
    pub fn new() -> Self {
        Self {
            options: InferOptions::default(),
        }
    }

    pub fn with_options(options: InferOptions) -> Self {
        Self { options }
    }

    /// Inferisce schema da JSON value
    pub fn infer_from_value(&self, value: &Value) -> Result<Schema, SchemaError> {
        // Usa genson-rs o implementazione custom
        unimplemented!()
    }

    /// Inferisce schema da flattened data
    pub fn infer_from_flattened(&self, data: &FlattenedData) -> Result<Schema, SchemaError> {
        let mut fields = Vec::new();

        for (path, val) in &data.fields {
            let field_type = self.infer_type(val);
            fields.push(FieldDef {
                name: path.clone(),
                field_type,
                nullable: val.is_null(),
                description: None,
                constraints: None,
            });
        }

        Ok(Schema {
            name: data.name.clone(),
            fields,
            metadata: HashMap::new(),
        })
    }

    /// Inferisce tipo da valore
    fn infer_type(&self, value: &Value) -> FieldType {
        match value {
            Value::Null => FieldType::Null,
            Value::Bool(_) => FieldType::Boolean,
            Value::Number(n) if n.is_i64() => FieldType::Integer,
            Value::Number(n) if n.is_f64() => FieldType::Float,
            Value::String(_) => FieldType::String,
            Value::Array(arr) => {
                if arr.is_empty() {
                    FieldType::Array(Box::new(FieldType::Unknown))
                } else {
                    // Prendi il tipo del primo elemento
                    let elem_type = self.infer_type(&arr[0]);
                    FieldType::Array(Box::new(elem_type))
                }
            }
            Value::Object(obj) => {
                let mut fields = HashMap::new();
                for (k, v) in obj {
                    fields.insert(k.clone(), self.infer_type(v));
                }
                FieldType::Object(fields)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SchemaError {
    #[error("Invalid schema: {0}")]
    InvalidSchema(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Type mismatch: {0}")]
    TypeError(String),
}

impl Default for SchemaInferrer {
    fn default() -> Self {
        Self::new()
    }
}
