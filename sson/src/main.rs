use std::fs;
use sson::{parse_sson, FieldDict};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Uso: sson <file.sson> [--mode strict|generative]");
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1])?;
    let mut doc = parse_sson(&input)?;

    // Override modalità da CLI se presente
    if args.contains(&"--mode".to_string()) {
        if let Some(pos) = args.iter().position(|a| a == "--mode") {
            if let Some(mode_str) = args.get(pos + 1) {
                doc.mode = if mode_str == "strict" { sson::SsonMode::Strict } else { sson::SsonMode::Generative };
            }
        }
    }

    let dict = FieldDict::from_doc(doc);
    println!("🔍 Dizionario generato: {} campi", dict.nodes.len());
    println!("⚖️  Equilibrio S = {:.3}", dict.s_global);
    println!("🚫 Violazioni: {} | ⚠️  Warning: {}", dict.violations, dict.warnings);

    if dict.s_global >= 0.9 {
        println!("✅ Esportabile: true");
    } else {
        println!("❌ Esportabile: false (S < 0.9)");
    }

    Ok(())
}
