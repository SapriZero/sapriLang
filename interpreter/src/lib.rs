pub mod parser;
pub mod vm;
pub mod live;
pub mod fp;

pub use parser::Parser;
pub use vm::VM;
pub use live::LiveInterpreter;

pub use fp::{eval, bind, fmap, tap, mask, pipe, compose, Either};

pub const PHI: f64 = 1.618033988749895;
pub const PI: f64 = 3.141592653589793;
pub const SQRT2: f64 = 1.4142135623730951;
pub const H: f64 = 6.62607015e-34;
pub const ALPHA: f64 = 0.0072973525693;
pub const MU: f64 = 1.0;

