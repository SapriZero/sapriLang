//! Addestramento AI da Wikipedia (usa wiki_extract)

use sapri_ai::{Brain, WikiLoader};

fn main() -> Result<(), String> {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║              SAPRI AI - FASE DI APPRENDIMENTO               ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let mut brain = Brain::new()?;
    
    // Carica articoli dal file .sson di wiki_extract
    let articles_path = "wiki_extract/index_pages.sson";  // ← percorso corretto
    println!("📚 Caricamento articoli da {}...", articles_path);
    
    match WikiLoader::load_articles(brain.knowledge_mut(), articles_path) {
        Ok(count) => println!("✅ Caricati {} articoli", count),
        Err(e) => eprintln!("❌ Errore: {}", e),
    }
    
    // Carica anche le categorie
    let categories_path = "wiki_extract/categories.sson";
    match WikiLoader::load_categories(brain.knowledge_mut(), categories_path) {
        Ok(count) => println!("✅ Caricate {} categorie", count),
        Err(e) => eprintln!("⚠️ Categorie non trovate: {}", e),
    }
    
    // Mostra statistiche
    println!("\n{}", brain.stats());
    
    // Salva la conoscenza
    brain.save("data/knowledge")?;
    println!("💾 Conoscenza salvata in data/knowledge/");
    
    Ok(())
}
