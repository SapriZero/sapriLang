mod parser;
mod vm;
mod live;

use std::env;
use live::LiveInterpreter;
use urcm_core::{UrcmCtx, urcm, pipe};

pub const PHI: f64 = 1.618033988749895;
pub const PI: f64 = 3.141592653589793;
pub const SQRT2: f64 = 1.4142135623730951;
pub const H: f64 = 6.62607015e-34;
pub const ALPHA: f64 = 0.0072973525693;
pub const MU: f64 = 1.0;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Uso: {} <file.urcm>", args[0]);
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    println!("🚀 URCM Interpreter v0.1");
    println!("📁 File: {}", file_path);
    
    let mut live = LiveInterpreter::new(file_path)?;
    live.run()?;
    
    Ok(())
}
