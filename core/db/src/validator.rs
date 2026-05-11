//! Definizione validatore

use crate::error::Result;
use crate::schema::{FieldDef, TableDef};
use sapri_obj::{Obj, Value};
use std::collections::HashMap;

/// Validatore
#[derive(Debug)]
pub struct Validator {
    pub schema: TableDef,
}

/// Errore di validazione
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

/// Trait per la validazione
pub trait Validate {
    fn new(schema: TableDef) -> Self;
    fn validate(&self, record: &HashMap<String, Value>) -> Result<()>;
    fn validate_field(&self, field_name: &str, value: &Value, field_def: &FieldDef) -> Result<()>;
}

impl Validator {
    /// Valida un record rappresentato come Obj
    pub fn validate_obj(&self, obj: &Obj) -> Result<()> {
        let map: HashMap<String, Value> = obj
            .keys()
            .iter()
            .filter_map(|key| {
                obj.get(key).map(|value| (key.to_string(), value.clone()))
            })
            .collect();
        self.validate(&map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{FieldDef, FieldType, TableDef, Constraint};
    use sapri_obj::{obj, Value};
    use std::collections::HashMap;

    fn create_test_schema() -> TableDef {
        let fields = vec![
            FieldDef {
                name: "name".to_string(),
                table: "test".to_string(),
                field_type: FieldType::String,
                constraints: vec![Constraint::MaxLength(50)],
                optional: false,
            },
            FieldDef {
                name: "age".to_string(),
                table: "test".to_string(),
                field_type: FieldType::Int,
                constraints: vec![Constraint::Min(0.0), Constraint::Max(120.0)],
                optional: false,
            },
            FieldDef {
                name: "email".to_string(),
                table: "test".to_string(),
                field_type: FieldType::String,
                constraints: vec![],
                optional: true,
            },
        ];

        TableDef {
            name: "test".to_string(),
            header: "10".to_string(),
            index_bits: 16,
            fields,
            indexes: HashMap::new(),
        }
    }

    #[test]
    fn test_validator_new() {
        let schema = create_test_schema();
        let validator = Validator::new(schema);
        assert_eq!(validator.schema.name, "test");
    }

    #[test]
    fn test_validate_valid_record() {
        let schema = create_test_schema();
        let validator = Validator::new(schema);

        let mut record = HashMap::new();
        record.insert("name".to_string(), Value::from("Mario Rossi"));
        record.insert("age".to_string(), Value::from(30));

        let result = validator.validate(&record);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_missing_required_field() {
        let schema = create_test_schema();
        let validator = Validator::new(schema);

        let mut record = HashMap::new();
        record.insert("name".to_string(), Value::from("Mario Rossi"));
        // age manca (obbligatorio)

        let result = validator.validate(&record);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_optional_field_missing() {
        let schema = create_test_schema();
        let validator = Validator::new(schema);

        let mut record = HashMap::new();
        record.insert("name".to_string(), Value::from("Mario Rossi"));
        record.insert("age".to_string(), Value::from(30));
        // email manca ma è opzionale

        let result = validator.validate(&record);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_type_mismatch() {
        let schema = create_test_schema();
        let validator = Validator::new(schema);

        let mut record = HashMap::new();
        record.insert("name".to_string(), Value::from(123)); // dovrebbe essere stringa
        record.insert("age".to_string(), Value::from(30));

        let result = validator.validate(&record);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_constraint_violation() {
        let schema = create_test_schema();
        let validator = Validator::new(schema);

        let mut record = HashMap::new();
        record.insert("name".to_string(), Value::from("Mario Rossi"));
        record.insert("age".to_string(), Value::from(150)); // oltre max 120

        let result = validator.validate(&record);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_obj() {
        let schema = create_test_schema();
        let validator = Validator::new(schema);

        let obj = obj! {
            name: "Mario Rossi",
            age: 30
        };

        let result = validator.validate_obj(&obj);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_obj_invalid() {
        let schema = create_test_schema();
        let validator = Validator::new(schema);

        let obj = obj! {
            name: "Mario Rossi",
            age: 200  // oltre max
        };

        let result = validator.validate_obj(&obj);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError {
            field: "age".to_string(),
            message: "Value 150 above max 120".to_string(),
        };
        assert_eq!(err.field, "age");
        assert_eq!(err.message, "Value 150 above max 120");
    }
}
