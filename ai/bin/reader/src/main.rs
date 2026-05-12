use ai_reader::WikipediaReader;

fn main() -> Result<(), String> {
    println!("🧠 AVVIO LETTURA AUTONOMA DI WIKIPEDIA");
    println!("========================================");
    
    let mut reader = WikipediaReader::new();
    
    // 1. Leggi gli articoli
    println!("\n📚 Fase 1: Lettura articoli...");
    reader.read_articles("data/wikipedia_articles.txt")?; // il tuo file
    
    // 2. Analizza correlazioni
    reader.analyze_correlations();
    
    // 3. Calcola MSAS
    let msas = reader.compute_msas();
    
    // 4. Statistiche finali
    reader.print_stats();
    
    println!("\n✅ Lettura completata. MSAS = {:.4}", msas);
    
    Ok(())
}
