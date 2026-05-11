//! Implementazione database

use super::database::{Database, DatabaseOps};
use super::table::Table;
use super::schema::TableDef;
use crate::error::Result;
use sapri_obj::{obj, Obj};
use std::collections::HashMap;

pub fn default_config() -> Obj {
    obj! {
        version: "1.0",
        default_index_bits: 16,
        max_table_bits: 32,
        options: obj! {
            auto_flat: true,
            auto_index: true,
            strict_mode: false
        }
    }
}

impl DatabaseOps for Database {
    fn new() -> Self {
        Self::with_config(default_config())
    }

    fn with_config(config: Obj) -> Self {
        Self {
            tables: HashMap::new(),
            schemas: HashMap::new(),
            config,
        }
    }

    fn add_table(&mut self, header: &str, table: Table) -> Result<()> {
        self.tables.insert(header.to_string(), table);
        Ok(())
    }

    fn add_schema(&mut self, name: &str, schema: TableDef) -> Result<()> {
        self.schemas.insert(name.to_string(), schema);
        Ok(())
    }

    fn get_table(&self, header: &str) -> Option<&Table> {
        self.tables.get(header)
    }

    fn get_table_mut(&mut self, header: &str) -> Option<&mut Table> {
        self.tables.get_mut(header)
    }

    fn get_schema(&self, name: &str) -> Option<&TableDef> {
        self.schemas.get(name)
    }

    fn save_binary(&self, _path: &str) -> Result<()> {
        // TODO: implementare
        Ok(())
    }

    fn load_binary(&mut self, _path: &str) -> Result<()> {
        // TODO: implementare
        Ok(())
    }
}
