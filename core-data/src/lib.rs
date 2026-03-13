//! core-data - Libreria di elaborazione dati per URCM/SAPRI
//!
//! Fornisce:
//! - Flattening JSON → formato tabellare
//! - Inferenza e validazione schema
//! - Gestione dati massivi ad alte performance
//! - Integrazione con il sistema di definizioni di core

pub mod flatten;
pub mod schema;
pub mod validation;
pub mod table;
pub mod stream;
pub mod registry;

// Re-export principali
pub use flatten::{Flattener, FlattenedData, FlattenOptions};
pub use schema::{SchemaInferrer, Schema, FieldType, FieldDef};
pub use validation::{Validator, ValidationResult, ValidationError};
pub use table::{Table, Row, TableIterator, Query};
pub use stream::{StreamProcessor, StreamConfig, Priority};
pub use registry::{DataRegistry, DataDefinition};

// Prelude per uso comune
pub mod prelude {
    pub use crate::flatten::Flattener;
    pub use crate::schema::SchemaInferrer;
    pub use crate::validation::Validator;
    pub use crate::table::Table;
    pub use crate::registry::DataRegistry;
}
