//! Implementazione validatore

use super::validator::{Validate, ValidationError, Validator};
use crate::error::{DbError, Result};
use crate::schema::{Constraint, FieldDef, FieldType, TableDef};
use sapri_obj::Value;
use std::collections::HashMap;
// RIMUOVI: use sapri_base::eval_lazy;

impl Validate for Validator {
    fn new(schema: TableDef) -> Self {
        Self { schema }
    }

    fn validate(&self, record: &HashMap<String, Value>) -> Result<()> {
        let errors: Vec<ValidationError> = self
            .schema
            .fields
            .iter()
            .filter_map(|field| {
                let value = record.get(&field.name);
                if value.is_none() && !field.optional {
                    Some(ValidationError {
                        field: field.name.clone(),
                        message: format!("Missing required field: {}", field.name),
                    })
                } else if let Some(v) = value {
                    if let Err(e) = self.validate_field(&field.name, v, field) {
                        Some(ValidationError {
                            field: field.name.clone(),
                            message: e.to_string(),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        if !errors.is_empty() {
            Err(DbError::Validator(format!("{:?}", errors)))
        } else {
            Ok(())
        }
    }

    fn validate_field(&self, field_name: &str, value: &Value, field_def: &FieldDef) -> Result<()> {
        let _ = field_name;
        self.validate_type(value, field_def)?;
        self.validate_constraints(value, field_def)?;
        Ok(())
    }
}

impl Validator {
    fn validate_type(&self, value: &Value, field_def: &FieldDef) -> Result<()> {
        match (&field_def.field_type, value) {
            (FieldType::String, Value::String(_)) => Ok(()),
            (FieldType::Int, Value::Number(_)) => Ok(()),
            (FieldType::Float, Value::Number(_)) => Ok(()),
            (FieldType::Bool, Value::Bool(_)) => Ok(()),
            (FieldType::Date, Value::String(_)) => Ok(()),
            (FieldType::Ref { .. }, Value::Number(_)) => Ok(()),
            (FieldType::Enum(values), Value::String(s)) => {
                if values.contains(s) {
                    Ok(())
                } else {
                    Err(DbError::Validator(format!("Value '{}' not in enum {:?}", s, values)))
                }
            }
            _ => Err(DbError::Validator(format!(
                "Type mismatch for field {}: expected {:?}",
                field_def.name, field_def.field_type
            ))),
        }
    }

    fn validate_constraints(&self, value: &Value, field_def: &FieldDef) -> Result<()> {
        for c in &field_def.constraints {
            match (c, value) {
                (Constraint::MaxLength(max), Value::String(s)) => {
                    if s.len() > *max {
                        return Err(DbError::Validator(format!("String too long: {} > {}", s.len(), max)));
                    }
                }
                (Constraint::Min(min), Value::Number(n)) => {
                    if *n < *min {
                        return Err(DbError::Validator(format!("Value {} below min {}", n, min)));
                    }
                }
                (Constraint::Max(max), Value::Number(n)) => {
                    if *n > *max {
                        return Err(DbError::Validator(format!("Value {} above max {}", n, max)));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
