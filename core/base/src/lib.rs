//! Sapri Base: Primitive di stato, contesto e valutazione pura.
//! Zero dipendenze esterne pesanti. Compile-time < 3s.

pub mod error;
pub mod atom;
pub mod atom_impl;
pub mod macros;
pub mod bucket;
pub mod fp;

pub use error::{BaseError, Result};
pub use atom::Atom;
pub use atom_impl::{PromiseState, ExternalSource};
pub use bucket::{BucketArray, counting_sort_u8, counting_sort_u16};
pub use fp::{
    Either,
    bind, fmap, tap, mask, identity,
    or_else, unwrap_or_else, opt_ref_or_else, opt_or_else, opt_as_ref_or_else,
    get_or_default, get_or_default_with, set_or_default, set_or_default_with,
    get_curried, set_curried,
    eval, eval_lazy,
};
// pub use bucket::BucketStrategy;
