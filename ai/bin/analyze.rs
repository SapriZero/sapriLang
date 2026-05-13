//! Analisi della conoscenza esistente

use sapri_ai::Brain;

fn main() -> Result<(), String> {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║              SAPRI AI - ANALISI CONOSCENZA                 ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    let mut brain = Brain::new()?;
    
    // Carica conoscenza esistente
    match brain.load("data/knowledge") {
        Ok(_) => println!("✅ Conoscenza caricata da data/knowledge/\n"),
        Err(e) => {
            println!("⚠️ Nessuna conoscenza trovata: {}", e);
            println!("   Esegui prima 'ai_learn' per addestrare l'AI.\n");
            return Ok(());
        }
    }
    
    println!("{}", brain.stats());
    
    Ok(())
}
