//! # sapri_rust_dsl
//!
//! DSL embedded in Rust per scrivere espressioni in modo testuale.
//!
//! ## Esempio
//!
//! ```
//! use sapri_rust_dsl::prelude::*;
//! use sapri_rust_dsl::{define, scan};
//!
//! let ctx = define! {
//!     a = 10;
//!     b = 20;
//! };
//!
//! let c = scan!("a * b", &ctx).unwrap();
//! assert_eq!(c.get().as_number(), Some(200.0));
//! ```

pub mod atom_value;
pub mod context;
pub mod scan;
pub mod scanner;
pub mod prelude;

// Includi il file delle macro (non come modulo pub)
#[macro_use]
mod define;

// Re-export solo tipi e funzioni, non macro
pub use atom_value::AtomValue;
pub use context::Context;
pub use scan::scan;
