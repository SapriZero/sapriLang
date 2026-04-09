//! Preludio per sapri_rust_dsl

pub use crate::atom_value::AtomValue;
pub use crate::context::Context;
pub use crate::scan::scan;

// Le macro sono disponibili globalmente (grazie a #[macro_export])
// Non serve re-exportarle, basta documentare:
// use sapri_rust_dsl::{define, define_expr};

#[cfg(feature = "global-context")]
pub use crate::context::{set_global_context, get_global_context, with_global_context};
