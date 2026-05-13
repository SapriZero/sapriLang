//! Chat interattiva con l'AI

use sapri_ai::Brain;
use std::io::{self, Write};

fn main() -> Result<(), String> {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                    SAPRI AI v0.1.0                         ║");
    println!("║  Parla con me. Comandi: /exit, /save, /load, /stats        ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    let mut brain = match Brain::new() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("❌ Failed to initialize brain: {}", e);
            return Ok(());
        }
    };
    
    // Prova a caricare conoscenza esistente
    if let Err(e) = brain.load("data/knowledge") {
        println!("⚠️ Nessuna conoscenza preesistente ({}), inizio da zero.", e);
    } else {
        println!("✅ Conoscenza caricata da data/knowledge/");
    }
    
    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        match input {
            "/exit" | "/quit" => {
                println!("👋 Goodbye!");
                break;
            }
            "/stats" => {
                println!("{}", brain.stats());
                continue;
            }
            cmd if cmd.starts_with("/save ") => {
                let path = &cmd[6..];
                match brain.save(path) {
                    Ok(_) => println!("💾 Saved to {}", path),
                    Err(e) => eprintln!("❌ Save error: {}", e),
                }
                continue;
            }
            cmd if cmd.starts_with("/load ") => {
                let path = &cmd[6..];
                match brain.load(path) {
                    Ok(_) => println!("📂 Loaded from {}", path),
                    Err(e) => eprintln!("❌ Load error: {}", e),
                }
                continue;
            }
            _ => {}
        }
        
        let response = brain.talk(input);
        println!("🤖 {}", response);
        
        // Feedback opzionale
        println!("\n💬 (Corretto? y/n/skip)");
        let mut feedback = String::new();
        io::stdin().read_line(&mut feedback).unwrap();
        match feedback.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                brain.teach(input, &response)?;
                println!("✅ Imparato!");
            }
            "n" | "no" => {
                println!("📝 Dimmi la risposta corretta:");
                let mut correct = String::new();
                io::stdin().read_line(&mut correct).unwrap();
                let correct = correct.trim();
                if !correct.is_empty() {
                    brain.teach(input, correct)?;
                    println!("✅ Corretto!");
                }
            }
            _ => {}
        }
    }
    
    Ok(())
}
