//! Lettore autonomo di Wikipedia
//! Costruisce conoscenza senza regole predefinite

use crate::knowledge::KnowledgeBase;
use std::collections::HashMap;

/// Lettore di Wikipedia
#[derive(Debug)]
pub struct WikipediaReader {
    knowledge: KnowledgeBase,
    stats: Statistics,
}

/// Statistiche di lettura
#[derive(Debug, Default)]
pub struct Statistics {
    pub total_articles: usize,
    pub total_categories: usize,
    pub word_frequency: HashMap<String, usize>,
    pub cooccurences: HashMap<(String, String), usize>,
}

impl WikipediaReader {
    pub fn new() -> Self {
        Self {
            knowledge: KnowledgeBase::new(),
            stats: Statistics::default(),
        }
    }

    pub fn with_knowledge(knowledge: KnowledgeBase) -> Self {
        Self {
            knowledge,
            stats: Statistics::default(),
        }
    }

    /// Legge il file articoli riga per riga
    pub fn read_articles(&mut self, path: &str) -> Result<usize, String> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut count = 0;

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| e.to_string())?;
            if line.is_empty() { continue; }

            self.process_article(&line)?;
            count += 1;

            if count % 10000 == 0 {
                println!("📚 Processati {} articoli...", count);
            }
        }

        self.stats.total_articles = count;
        println!("✅ Letti {} articoli totali", count);
        Ok(count)
    }

    fn process_article(&mut self, line: &str) -> Result<(), String> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 5 {
            return Ok(());
        }

        let id = parts[0];
        let title = parts[1];
        let categories_str = parts[4];

        // Aggiungi articolo alla knowledge base
        self.knowledge.add_article(id, title, categories_str)?;

        // Estrai parole per statistiche
        let title_words = crate::utils::extract_words(title);
        let categories: Vec<String> = categories_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let category_words: Vec<String> = categories
            .iter()
            .flat_map(|cat| crate::utils::extract_words(cat))
            .collect();

        // Aggiorna statistiche
        for word in &title_words {
            *self.stats.word_frequency.entry(word.clone()).or_insert(0) += 1;
        }
        for word in &category_words {
            *self.stats.word_frequency.entry(word.clone()).or_insert(0) += 1;
        }

        // Registra co-occorrenze
        for title_word in &title_words {
            for cat_word in &category_words {
                if title_word != cat_word {
                    let key = (title_word.clone(), cat_word.clone());
                    *self.stats.cooccurences.entry(key).or_insert(0) += 1;
                }
            }
        }

        Ok(())
    }

    pub fn get_knowledge(&self) -> &KnowledgeBase {
        &self.knowledge
    }

    pub fn into_knowledge(self) -> KnowledgeBase {
        self.knowledge
    }

    pub fn stats(&self) -> &Statistics {
        &self.stats
    }

    pub fn print_stats(&self) {
        println!("\n📊 STATISTICHE LETTURA");
        println!("Articoli processati: {}", self.stats.total_articles);
        println!("Parole distinte: {}", self.stats.word_frequency.len());
        println!("Co-occorrenze distinte: {}", self.stats.cooccurences.len());

        // Mostra parole più frequenti
        let mut sorted: Vec<_> = self.stats.word_frequency.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        println!("\n🏆 Top 10 parole più frequenti:");
        for (word, count) in sorted.iter().take(10) {
            println!("  {}: {}", word, count);
        }
    }
}

impl Default for WikipediaReader {
    fn default() -> Self {
        Self::new()
    }
}
