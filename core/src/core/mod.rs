//! Core module - atomi e contesto

pub mod atom_impl;
pub mod atom;
pub mod context;

pub use atom::Atom;
pub use atom_impl::{AtomImpl, PromiseState, ExternalSource};
pub use context::UrcmCtx;

