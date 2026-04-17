//! Eseguibile per esportare la morfologia in .sson
//! Usa sapri_diz per la configurazione

use sapri_morph::*;
use sapri_diz::load_diz;
use std::time::Instant;

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║              SAPRI MORPH EXPORTER v0.1.0                   ║");
    println!("║  Estrae verbi, nomi e aggettivi                           ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    let start = Instant::now();
    
    // Carica configurazione da diz
    let diz = load_diz();
    println!("📖 Configurazione caricata da diz");
    println!("  Filter words: {}", diz.text.filter_words.list.len());
    println!("  Min word length: {}", diz.text.min_word_length);
    
    // Esempio: lista di parole da processare
    let words = vec![
        "casa".to_string(),
        "gatto".to_string(),
        "bella".to_string(),
        "parlare".to_string(),
        "correre".to_string(),
        "sentire".to_string(),
    ];
    
    // Estrai verbi
    println!("\n📖 Estrazione verbi...");
    let verbs = verb::extract_verbs(&words);
    println!("  ✅ Verbi trovati: {}", verbs.len());
    
    // Estrai nomi
    println!("\n📖 Estrazione nomi...");
    let nouns = noun::extract_nouns(&words);
    println!("  ✅ Nomi trovati: {}", nouns.len());
    
    // Estrai aggettivi
    println!("\n📖 Estrazione aggettivi...");
    let adjectives = adj::extract_adjectives(&words);
    println!("  ✅ Aggettivi trovati: {}", adjectives.len());
    
    // Esporta
    println!("\n📖 Esportazione in .sson...");
    std::fs::create_dir_all("ai/sson/grammar").unwrap();
    
    export::export_verbs(&verbs, "ai/sson/grammar/verbs.sson");
    export::export_nouns(&nouns, "ai/sson/grammar/nouns.sson");
    export::export_adjectives(&adjectives, "ai/sson/grammar/adjectives.sson");
    
    let elapsed = start.elapsed();
    println!("\n✅ Esportazione completata in {:.2?}", elapsed);
}
