//! # sapri_obj
//!
//! Struttura dinamica a oggetti stile JavaScript per Rust.
//!
//! ## Esempio
//!
//! ```
//! use sapri_obj::{obj, Obj};
//!
//! let base = obj! {
//!     nome: "Sapri",
//!     eta: 42
//! };
//!
//! let esteso = obj! {
//!     using base,
//!     eta: 43,
//!     citta: "Roma"
//! };
//!
//! assert_eq!(esteso.get("nome").unwrap().as_str(), Some("Sapri"));
//! assert_eq!(esteso.get("eta").unwrap().as_number(), Some(43.0));
//! ```

pub mod obj;
pub mod value;
pub mod macros;

pub use obj::Obj;
pub use value::Value;
// pub use macros::from_obj;
// pub use macros::FromObjValue;

// La macro obj! è esportata alla radice del crate da #[macro_export]
