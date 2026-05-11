//! Definizione database

use crate::error::Result;
use crate::schema::TableDef;
use crate::table::Table;
use std::collections::HashMap;
use sapri_obj::Obj;

/// Database collezione di tabelle
#[derive(Debug, Default)]
pub struct Database {
    /// Tabelle indicizzate per header
    pub tables: HashMap<String, Table>,
    /// Schemi delle tabelle
    pub schemas: HashMap<String, TableDef>,
    /// Configurazione (fornita da sapri_core)
    pub config: Obj,
}

/// Trait per le operazioni sul database
pub trait DatabaseOps {
    /// Crea un nuovo database vuoto con configurazione di default
    fn new() -> Self;
    
    /// Crea un nuovo database con configurazione fornita
    fn with_config(config: Obj) -> Self;
    
    /// Aggiunge una tabella già caricata
    fn add_table(&mut self, header: &str, table: Table) -> Result<()>;
    
    /// Aggiunge uno schema
    fn add_schema(&mut self, name: &str, schema: TableDef) -> Result<()>;
    
    /// Ottiene una tabella per header
    fn get_table(&self, header: &str) -> Option<&Table>;
    
    /// Ottiene una tabella mutabile per header
    fn get_table_mut(&mut self, header: &str) -> Option<&mut Table>;
    
    /// Ottiene uno schema per nome
    fn get_schema(&self, name: &str) -> Option<&TableDef>;
    
    /// Salva il database in formato binario
    fn save_binary(&self, path: &str) -> Result<()>;
    
    /// Carica il database da formato binario
    fn load_binary(&mut self, path: &str) -> Result<()>;
}
