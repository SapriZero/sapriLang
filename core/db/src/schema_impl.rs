//! Implementazione schema (validazione)

use super::schema::{TableDef, FieldDef};
use crate::error::{DbError, Result};
use sapri_base::eval_lazy;

/// Struttura per la validazione degli schemi
#[derive(Debug, Default)]
pub struct SchemaValidator;

impl SchemaValidator {
    /// Crea un nuovo validatore
    pub fn new() -> Self {
        Self
    }

    /// Valida un TableDef
    pub fn validate_table_def(&self, def: &TableDef) -> Result<()> {
        eval_lazy(
            def.header.len() > 7,
            || Err(DbError::InvalidHeader(format!("header too long: {}", def.header))),
            || self.validate_fields(def)
        )
    }

    /// Valida i campi di un TableDef
    fn validate_fields(&self, def: &TableDef) -> Result<()> {
        def.fields.iter().try_for_each(|f| self.validate_field(f))
    }

    /// Valida un singolo FieldDef
    fn validate_field(&self, field: &FieldDef) -> Result<()> {
        eval_lazy(
            field.name.is_empty(),
            || Err(DbError::Schema("Field name empty".into())),
            || Ok(())
        )
    }
}
