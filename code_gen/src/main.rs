mod config;
mod engine;
mod generator;

use config::GenConfig;
use engine::GenerationEngine;

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║              SAPRI CODE GENERATOR v0.1.0                   ║");
    println!("║  Genera codice Rust da file .sson                          ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    let args: Vec<String> = std::env::args().collect();
    
    // Directory dei file di configurazione del generatore (grammar, rules, etc.)
    // Di default, cerca nella directory dell'eseguibile
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let default_config_dir = exe_dir.join("sson");
    
    // Gestione argomenti:
    // - 1 argomento: lo usa come project_dir (cerca loader.sson lì)
    // - 2 argomenti: primo = generator_config_dir, secondo = project_dir
    // - 0 argomenti: default (generator_config_dir = ../sson, project_dir = .)
    let (generator_config_dir, project_dir) = match args.len() {
        1 => {
            // Solo il nome dell'eseguibile
            (default_config_dir.to_str().unwrap(), ".")
        }
        2 => {
            // Un argomento: usalo come project_dir
            (default_config_dir.to_str().unwrap(), args[1].as_str())
        }
        _ => {
            // Due o più argomenti: primo = config_dir, secondo = project_dir
            (args[1].as_str(), args[2].as_str())
        }
    };
    
    println!("📁 Generator config dir: {}", generator_config_dir);
    println!("📁 Project dir: {}", project_dir);
    println!();
    
    let config = match GenConfig::load(generator_config_dir, project_dir) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Errore caricamento configurazione: {}", e);
            return;
        }
    };
    
    let engine = match GenerationEngine::new(config) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("❌ Errore inizializzazione motore: {}", e);
            return;
        }
    };
    
    match engine.generate_all() {
        Ok(_) => println!("\n✅ Generazione completata!"),
        Err(e) => eprintln!("\n❌ Errore durante generazione: {}", e),
    }
}
