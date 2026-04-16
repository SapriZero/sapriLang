//! Punto di ingresso per l'eseguibile

use sapri_core::Runtime;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config_path = if args.len() > 1 {
        Some(args[1].as_str())
    } else {
        None
    };
    
    match Runtime::new(config_path) {
        Ok(mut runtime) => runtime.run(),
        Err(e) => eprintln!("Failed to start runtime: {}", e),
    }
}
