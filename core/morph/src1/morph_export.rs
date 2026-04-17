//! Eseguibile per esportare la morfologia in .sson

use sapri_morph::*;
use std::time::Instant;

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║              SAPRI MORPH EXPORTER v0.1.0                   ║");
    println!("║  Estrae verbi, nomi e aggettivi da file Hunspell          ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    let start = Instant::now();
    
    let aff_path = "data/italiano_2_4_2007_09_01/it_IT.aff";
    let dic_path = "data/italiano_2_4_2007_09_01/it_IT.dic";
    let output_dir = "ai/sson/grammar";
    
    // 1. Parsa .aff
    println!("📖 Parsing .aff...");
    let aff = match AffData::from_file(aff_path) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("❌ Errore parsing .aff: {}", e);
            return;
        }
    };
    println!("  ✅ Encoding: {}", aff.encoding);
    println!("  ✅ Suffix flags: {}", aff.suffixes.len());
    println!("  ✅ Prefix flags: {}", aff.prefixes.len());
    
    // 2. Parsa .dic
    println!("\n📖 Parsing .dic...");
    let entries = match DicEntry::from_file(dic_path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("❌ Errore parsing .dic: {}", e);
            return;
        }
    };
    println!("  ✅ Parole trovate: {}", entries.len());
    
    // 3. Estrai verbi
    println!("\n📖 Estrazione verbi...");
    let verbs = verb_extractor::extract(&entries, &aff);
    println!("  ✅ Verbi trovati: {}", verbs.len());
    
    // 4. Estrai nomi
    println!("\n📖 Estrazione nomi...");
    let nouns = noun_extractor::extract(&entries);
    println!("  ✅ Nomi trovati: {}", nouns.len());
    
    // 5. Estrai aggettivi
    println!("\n📖 Estrazione aggettivi...");
    let adjectives = adj_extractor::extract(&entries);
    println!("  ✅ Aggettivi trovati: {}", adjectives.len());
    
    // 6. Esporta in .sson
    println!("\n📖 Esportazione in .sson...");
    std::fs::create_dir_all(output_dir).unwrap();
    
    sson_export::export_verbs(&verbs, &format!("{}/verbs.sson", output_dir));
    sson_export::export_nouns(&nouns, &format!("{}/nouns.sson", output_dir));
    sson_export::export_adjectives(&adjectives, &format!("{}/adjectives.sson", output_dir));
    
    let elapsed = start.elapsed();
    
    println!("\n✅ Esportazione completata in: {}", output_dir);
    println!("   Tempo: {:.2?}", elapsed);
}
