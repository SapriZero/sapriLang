//! Gestione dati tabellari
//!
//! Fornisce strutture per rappresentare e interrogare dati in formato tabellare.

use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub schema: Option<Schema>,
    pub columns: Vec<String>,
    pub rows: Vec<Row>,
    pub metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub values: Vec<Value>,
    pub id: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Query {
    pub table: String,
    pub filter: Option<String>,
    pub limit: Option<usize>,
    pub offset: usize,
}

pub struct TableIterator {
    table: Table,
    position: usize,
}

impl Table {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            schema: None,
            columns: Vec::new(),
            rows: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_schema(mut self, schema: Schema) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn add_row(&mut self, values: Vec<Value>) -> Result<(), TableError> {
        if let Some(schema) = &self.schema {
            if values.len() != schema.fields.len() {
                return Err(TableError::ColumnCountMismatch {
                    expected: schema.fields.len(),
                    got: values.len(),
                });
            }
        }

        self.rows.push(Row {
            values,
            id: Some(self.rows.len()),
        });
        Ok(())
    }

    pub fn query(&self, query: Query) -> Result<Vec<Row>, TableError> {
        let mut results = Vec::new();

        for row in &self.rows {
            // Applica filtro (semplice implementazione)
            if let Some(filter) = &query.filter {
                if !self.evaluate_filter(row, filter)? {
                    continue;
                }
            }

            results.push(row.clone());

            if let Some(limit) = query.limit {
                if results.len() >= limit {
                    break;
                }
            }
        }

        Ok(results)
    }

    fn evaluate_filter(&self, row: &Row, filter: &str) -> Result<bool, TableError> {
        // TODO: implementare parser filtri
        // Per ora sempre true
        Ok(true)
    }

    pub fn iter(&self) -> TableIterator {
        TableIterator {
            table: self.clone(),
            position: 0,
        }
    }
}

impl Iterator for TableIterator {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.table.rows.len() {
            let row = self.table.rows[self.position].clone();
            self.position += 1;
            Some(row)
        } else {
            None
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TableError {
    #[error("Column count mismatch: expected {expected}, got {got}")]
    ColumnCountMismatch { expected: usize, got: usize },

    #[error("Invalid filter: {0}")]
    InvalidFilter(String),
}
