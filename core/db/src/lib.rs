//! # sapri_db
//! Database a indirizzamento diretto con header a bit variabili

// Tipi e definizioni
pub mod error;
pub mod header;
pub mod schema;
pub mod validator;
pub mod database;
pub mod table;

// Implementazioni
pub mod error_impl;
pub mod header_impl;
pub mod schema_impl;
pub mod validator_impl;
pub mod database_impl;
pub mod table_impl;

// Moduli da implementare (file vuoti per ora)
pub mod loader;
pub mod loader_impl;
pub mod binary;
pub mod binary_impl;
pub mod join;
pub mod join_impl;
pub mod flat;
pub mod flat_impl;

// Re-export principali
pub use database::{Database, DatabaseOps};
pub use error::{DbError, Result, ErrorDisplay};
pub use header::{Header, HeaderOps};
pub use schema::{FieldDef, FieldType, TableDef, IndexDef, IndexType, Constraint};
pub use schema_impl::SchemaValidator;
pub use table::{Table, Record};
pub use validator::{Validator, Validate, ValidationError};
