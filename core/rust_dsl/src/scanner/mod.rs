//! Scanner per trasformare stringhe in espressioni su atomi

mod token;
mod parser;
mod compiler;

pub use token::{Token, tokenize};
pub use parser::{parse, Ast};
pub use compiler::compile;
