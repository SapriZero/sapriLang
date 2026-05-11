//! Definizione schema tabelle

use std::collections::HashMap;
use sapri_obj::{Obj, Value}; 
use crate::error::Result;


/// Tipo di campo
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    String,
    Int,
    Float,
    Bool,
    Date,
    Ref { table: String, field: String },
    Enum(Vec<String>),
}

/// Vincolo di campo
#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    Min(f64),
    Max(f64),
    MaxLength(usize),
    Pattern(String),
    Ref(String, String),
    Enum(Vec<String>),
}

/// Definizione di un campo
#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub table: String,
    pub field_type: FieldType,
    pub constraints: Vec<Constraint>,
    pub optional: bool,
}

impl FieldDef {
    /// Crea FieldDef da un oggetto Obj
    pub fn from_obj(obj: &Obj) -> Result<Self> {
        let name = obj.get_string("name").map_err(|e| crate::error::DbError::Schema(e))?;
        let table = obj.get_string("table").unwrap_or_default();
        let optional = obj.get_bool("optional").unwrap_or(false);
        
        let field_type = Self::parse_field_type(obj)?;
        
        Ok(Self {
            name,
            table,
            field_type,
            constraints: Vec::new(), // TODO: parse constraints
            optional,
        })
    }
    
    fn parse_field_type(obj: &Obj) -> Result<FieldType> {
        let type_str = obj.get_string("type").map_err(|e| crate::error::DbError::Schema(e))?;
        match type_str.as_str() {
            "string" => Ok(FieldType::String),
            "int" => Ok(FieldType::Int),
            "float" => Ok(FieldType::Float),
            "bool" => Ok(FieldType::Bool),
            "date" => Ok(FieldType::Date),
            "ref" => {
                let table = obj.get_string("ref_table").map_err(|e| crate::error::DbError::Schema(e))?;
                let field = obj.get_string("ref_field").unwrap_or_else(|_| "id".to_string());
                Ok(FieldType::Ref { table, field })
            }
            "enum" => {
                // Usa get_array se disponibile, altrimenti pattern matching
                let values = match obj.get("values") {
                    Some(Value::Array(arr)) => arr,
                    _ => return Err(crate::error::DbError::Schema("Invalid enum values".into())),
                };
                let values = values
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                Ok(FieldType::Enum(values))
            }
            _ => Err(crate::error::DbError::Schema(format!("Unknown field type: {}", type_str))),
        }
    }
}

/// Definizione di una tabella
#[derive(Debug, Clone)]
pub struct TableDef {
    pub name: String,
    pub header: String,
    pub index_bits: u8,
    pub fields: Vec<FieldDef>,
    pub indexes: HashMap<String, IndexDef>,
}

impl TableDef {
    /// Crea TableDef da un oggetto Obj
    pub fn from_obj(name: &str, obj: &Obj) -> Result<Self> {
        let header = obj.get_string("header").map_err(|e| crate::error::DbError::Schema(e))?;
        let index_bits = obj.get("index_bits")
            .and_then(|v| v.as_number())
            .map(|n| n as u8)
            .unwrap_or(16);
        
        Ok(Self {
            name: name.to_string(),
            header,
            index_bits,
            fields: Vec::new(),
            indexes: HashMap::new(),
        })
    }
}

/// Definizione di un indice secondario
#[derive(Debug, Clone)]
pub struct IndexDef {
    pub name: String,
    pub fields: Vec<String>,
    pub index_type: IndexType,
}

/// Tipo di indice
#[derive(Debug, Clone, PartialEq)]
pub enum IndexType {
    Hash,
    Sorted,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_type_debug() {
        let ft = FieldType::String;
        assert_eq!(format!("{:?}", ft), "String");
        
        let ft = FieldType::Ref { table: "users".to_string(), field: "id".to_string() };
        assert!(format!("{:?}", ft).contains("Ref"));
    }

    #[test]
    fn test_constraint_debug() {
        let c = Constraint::Min(10.0);
        assert_eq!(format!("{:?}", c), "Min(10.0)");
        
        let c = Constraint::MaxLength(100);
        assert_eq!(format!("{:?}", c), "MaxLength(100)");
    }
}
