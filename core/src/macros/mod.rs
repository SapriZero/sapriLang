//! Macro per URCM - Sintassi compatta per Obj e path

pub mod obj;
mod struct_with_keys;
mod cascade;
mod reactive;

pub use obj::*;
pub use struct_with_keys::*;
pub use cascade::*;
pub use reactive::*;
