//! Validazione dati contro schema
//!
//! Usa jsonschema per validazione ultra-veloce (20-470x più veloce).

use jsonschema::{Draft, JSONSchema};
use serde_json::Value;
use std::sync::Arc;

use serde_json::json;
use std::collections::HashMap;

pub struct Validator {
    schemas: HashMap<String, Arc<JSONSchema>>,
    options: ValidationOptions,
}

#[derive(Debug, Clone)]
pub struct ValidationOptions {
    pub draft: Draft,
    pub fail_fast: bool,
    pub format_assertions: bool,
}

impl Default for ValidationOptions {
    fn default() -> Self {
        Self {
            draft: Draft::Draft7,
            fail_fast: false,
            format_assertions: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    #[error("Schema error: {0}")]
    SchemaError(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Type mismatch for field '{field}': expected {expected}, got {actual}")]
    TypeMismatch {
        field: String,
        expected: String,
        actual: String,
    },

    #[error("Missing required field: {0}")]
    MissingField(String),
}

impl Validator {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
            options: ValidationOptions::default(),
        }
    }

    pub fn with_options(options: ValidationOptions) -> Self {
        Self {
            schemas: HashMap::new(),
            options,
        }
    }

    /// Registra uno schema JSON per validazione
    pub fn register_schema(&mut self, name: &str, schema: Value) -> Result<(), ValidationError> {
        let compiled = JSONSchema::options()
            .with_draft(self.options.draft)
            .compile(&schema)
            .map_err(|e| ValidationError::SchemaError(e.to_string()))?;

        self.schemas.insert(name.to_string(), Arc::new(compiled));
        Ok(())
    }

    /// Valida un'istanza contro lo schema registrato
    pub fn validate(&self, schema_name: &str, instance: &Value) -> Result<ValidationResult, ValidationError> {
        let schema = self.schemas.get(schema_name)
            .ok_or_else(|| ValidationError::SchemaError(format!("Schema '{}' not found", schema_name)))?;

        let result = schema.validate(instance);

        match result {
            Ok(_) => Ok(ValidationResult {
                valid: true,
                errors: Vec::new(),
                warnings: Vec::new(),
            }),
            Err(errors) => {
                let mut validation_errors = Vec::new();
                for error in errors {
                    validation_errors.push(ValidationError::ValidationFailed(error.to_string()));
                }

                Ok(ValidationResult {
                    valid: false,
                    errors: validation_errors,
                    warnings: Vec::new(),
                })
            }
        }
    }

    /// Valida dati flattened
    pub fn validate_flattened(&self, data: &FlattenedData, schema: &Schema) -> Result<ValidationResult, ValidationError> {
        // Convert flattened data to JSON object
        let mut obj = serde_json::Map::new();
        for (path, val) in &data.fields {
            obj.insert(path.clone(), val.clone());
        }
        let instance = Value::Object(obj);

        // Convert schema to JSON Schema
        let schema_json = self.schema_to_json(schema);

        // Validate
        let compiled = JSONSchema::options()
            .with_draft(self.options.draft)
            .compile(&schema_json)
            .map_err(|e| ValidationError::SchemaError(e.to_string()))?;

        match compiled.validate(&instance) {
            Ok(_) => Ok(ValidationResult {
                valid: true,
                errors: Vec::new(),
                warnings: Vec::new(),
            }),
            Err(errors) => {
                let mut validation_errors = Vec::new();
                for error in errors {
                    validation_errors.push(ValidationError::ValidationFailed(error.to_string()));
                }

                Ok(ValidationResult {
                    valid: false,
                    errors: validation_errors,
                    warnings: Vec::new(),
                })
            }
        }
    }

    /// Converte il nostro Schema in JSON Schema
    fn schema_to_json(&self, schema: &Schema) -> Value {
        let mut properties = serde_json::Map::new();
        let mut required = Vec::new();

        for field in &schema.fields {
            let field_schema = self.field_to_json(&field);
            properties.insert(field.name.clone(), field_schema);

            if !field.nullable {
                required.push(Value::String(field.name.clone()));
            }
        }

        json!({
            "type": "object",
            "properties": properties,
            "required": required,
            "additionalProperties": false
        })
    }

    fn field_to_json(&self, field: &FieldDef) -> Value {
        match &field.field_type {
            FieldType::String => json!({ "type": "string" }),
            FieldType::Integer => json!({ "type": "integer" }),
            FieldType::Float => json!({ "type": "number" }),
            FieldType::Boolean => json!({ "type": "boolean" }),
            FieldType::Array(elem) => json!({
                "type": "array",
                "items": self.field_type_to_json(elem)
            }),
            FieldType::Object(fields) => {
                let mut props = serde_json::Map::new();
                for (k, t) in fields {
                    props.insert(k.clone(), self.field_type_to_json(t));
                }
                json!({
                    "type": "object",
                    "properties": props
                })
            }
            FieldType::Null => json!({ "type": "null" }),
            FieldType::Unknown => json!({}),
        }
    }

    fn field_type_to_json(&self, field_type: &FieldType) -> Value {
        match field_type {
            FieldType::String => json!({ "type": "string" }),
            FieldType::Integer => json!({ "type": "integer" }),
            FieldType::Float => json!({ "type": "number" }),
            FieldType::Boolean => json!({ "type": "boolean" }),
            FieldType::Array(elem) => json!({
                "type": "array",
                "items": self.field_type_to_json(elem)
            }),
            FieldType::Object(fields) => {
                let mut props = serde_json::Map::new();
                for (k, t) in fields {
                    props.insert(k.clone(), self.field_type_to_json(t));
                }
                json!({
                    "type": "object",
                    "properties": props
                })
            }
            _ => json!({}),
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}
