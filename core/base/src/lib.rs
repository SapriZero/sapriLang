//! Sapri Base: Primitive di stato, contesto e valutazione pura.
//! Zero dipendenze esterne pesanti. Compile-time < 3s.

pub mod error;
pub mod atom;
pub mod atom_impl;
pub mod macros;
pub mod bucket;  // ← aggiunto

pub use error::{BaseError, Result};
pub use atom::Atom;
pub use atom_impl::{PromiseState, ExternalSource};
pub use bucket::{BucketArray, counting_sort_u8, counting_sort_u16};
