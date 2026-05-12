//! Punto di ingresso per l'AI

use sapri_ai::Brain;
use std::io::{self, Write};

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                    SAPRI AI v0.1.0                         ║");
    println!("║  Parla con me. Comandi: /exit, /save <dir>, /load <dir>,   ║");
    println!("║            /stats                                          ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    let mut brain = match Brain::new(None) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to initialize brain: {}", e);
            return;
        }
    };
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input == "/exit" {
            println!("Goodbye!");
            break;
        }
        
        if input == "/stats" {
            println!("{}", brain.stats());
            continue;
        }
        
        if input.starts_with("/save ") {
            let path = &input[6..];
            if let Err(e) = brain.save(path) {
                eprintln!("Save error: {}", e);
            } else {
                println!("Saved to {}", path);
            }
            continue;
        }
        
        if input.starts_with("/load ") {
            let path = &input[6..];
            if let Err(e) = brain.load(path) {
                eprintln!("Load error: {}", e);
            } else {
                println!("Loaded from {}", path);
            }
            continue;
        }
        
        let response = brain.talk(input);
        println!("{}", response);
        
        println!("(Was this response correct? y/n)");
        let mut feedback = String::new();
        io::stdin().read_line(&mut feedback).unwrap();
        if feedback.trim().to_lowercase() == "y" {
            brain.learn(input, &response);
        }
    }
}
