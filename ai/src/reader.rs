//! Lettore autonomo di Wikipedia
//! Costruisce conoscenza senza regole predefinite

use sapri_db::{Database, DatabaseOps, Table, Record};
use sapri_obj::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct WikipediaReader {
    db: Database,
    stats: Statistics,
}

struct Statistics {
    total_articles: usize,
    total_categories: usize,
    word_frequency: HashMap<String, usize>,
    cooccurences: HashMap<(String, String), usize>,
}

impl WikipediaReader {
    pub fn new() -> Self {
        Self {
            db: Database::new(),
            stats: Statistics::default(),
        }
    }

    /// Legge il file articoli riga per riga
    pub fn read_articles(&mut self, path: &str) -> Result<(), String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| e.to_string())?;
            if line.is_empty() { continue; }
            
            self.process_article(&line, line_num)?;
            
            // Ogni 10000 articoli, mostra progresso
            if line_num % 10000 == 0 {
                println!("📚 Processati {} articoli...", line_num);
            }
        }
        
        println!("✅ Letti {} articoli totali", self.stats.total_articles);
        Ok(())
    }

    /// Processa un singolo articolo
    fn process_article(&mut self, line: &str, id: usize) -> Result<(), String> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 5 {
            return Ok(()); // Formato non valido, salta
        }

        let article_id = parts[0].to_string();
        let title = parts[1].to_string();
        let _primary_cat = parts[2].to_string();
        let _related_article = parts[3].to_string();
        let categories_str = parts[4];

        // Estrai parole dal titolo
        let title_words = self.extract_words(&title);
        
        // Estrai parole dalle categorie
        let categories: Vec<String> = categories_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        let category_words: Vec<String> = categories
            .iter()
            .flat_map(|cat| self.extract_words(cat))
            .collect();

        // Aggiorna statistiche
        self.stats.total_articles += 1;
        
        for word in &title_words {
            *self.stats.word_frequency.entry(word.clone()).or_insert(0) += 1;
        }
        for word in &category_words {
            *self.stats.word_frequency.entry(word.clone()).or_insert(0) += 1;
        }

        // Registra co-occorrenze (titolo ↔ categoria)
        for title_word in &title_words {
            for cat_word in &category_words {
                let key = (title_word.clone(), cat_word.clone());
                *self.stats.cooccurences.entry(key).or_insert(0) += 1;
            }
        }

        // Salva nel database
        self.save_to_db(&article_id, &title, &categories)?;
        
        Ok(())
    }

    /// Estrae parole da un testo (molto semplice: split per spazi)
    fn extract_words(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase())
            .filter(|s| s.len() > 2) // ignora parole troppo corte
            .collect()
    }

    /// Salva articolo nel database
    fn save_to_db(&mut self, id: &str, title: &str, categories: &[String]) -> Result<(), String> {
        // Tabella articoli
        let mut article_record = Record::new(0);
        article_record.set("id", Value::from(id.to_string()));
        article_record.set("title", Value::from(title.to_string()));
        
        // Trova o crea tabella "articles"
        // (implementazione dipende da come usi sapri_db)
        
        Ok(())
    }

    /// Calcola correlazioni tra parole (usando MSAS-like)
    pub fn analyze_correlations(&self) {
        println!("\n📊 ANALISI CORRELAZIONI");
        println!("{} parole distinte", self.stats.word_frequency.len());
        
        // Trova le parole più frequenti
        let mut sorted: Vec<_> = self.stats.word_frequency.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        
        println!("Top 10 parole più frequenti:");
        for (word, count) in sorted.iter().take(10) {
            println!("  {}: {}", word, count);
        }
        
        // Trova le co-occorrenze più forti
        let mut cooc_sorted: Vec<_> = self.stats.cooccurences.iter().collect();
        cooc_sorted.sort_by(|a, b| b.1.cmp(a.1));
        
        println!("\nTop 10 co-occorrenze (parola1, parola2):");
        for ((w1, w2), count) in cooc_sorted.iter().take(10) {
            println!("  ({}, {}): {}", w1, w2, count);
        }
    }

    /// Calcola l'indice MSAS per rilevare struttura
    pub fn compute_msas(&self) -> f64 {
        // Prepara sequenza delle frequenze
        let frequencies: Vec<f64> = self.stats.word_frequency
            .values()
            .map(|&v| v as f64)
            .collect();
        
        // Calcola MSAS (versione semplificata)
        let mean = frequencies.iter().sum::<f64>() / frequencies.len() as f64;
        let variance = frequencies.iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>() / frequencies.len() as f64;
        
        let msas = variance.sqrt();
        
        println!("\n📈 INDICE MSAS: {:.4}", msas);
        if msas > 0.05 {
            println!("   → Struttura significativa rilevata");
        } else {
            println!("   → Struttura debole o casuale");
        }
        
        msas
    }

    /// Mostra statistiche finali
    pub fn print_stats(&self) {
        println!("\n📊 STATISTICHE FINALI");
        println!("Articoli processati: {}", self.stats.total_articles);
        println!("Parole distinte: {}", self.stats.word_frequency.len());
        println!("Co-occorrenze distinte: {}", self.stats.cooccurences.len());
    }
}

impl Default for WikipediaReader {
    fn default() -> Self {
        Self::new()
    }
}

struct Statistics {
    total_articles: usize,
    total_categories: usize,
    word_frequency: HashMap<String, usize>,
    cooccurences: HashMap<(String, String), usize>,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            total_articles: 0,
            total_categories: 0,
            word_frequency: HashMap::new(),
            cooccurences: HashMap::new(),
        }
    }
}
