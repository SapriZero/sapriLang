//! Base di conoscenza strutturata
//!
//! Utilizza sapri_db per memorizzare e ricercare conoscenza in O(1)

use sapri_db::{Database, DatabaseOps, Table, Record, Validator, Validate};
use sapri_obj::Value;
use std::collections::HashMap;

/// Base di conoscenza strutturata
#[derive(Debug)]
pub struct KnowledgeBase {
    db: Database,
}

/// Informazioni su una parola/concept
#[derive(Debug, Clone)]
pub struct ConceptInfo {
    pub name: String,
    pub category: String,
    pub frequency: usize,
    pub relations: HashMap<String, Vec<String>>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self {
            db: Database::new(),
        }
    }

    pub fn with_config(config: &sapri_obj::Obj) -> Self {
        Self {
            db: Database::with_config(config.clone()),
        }
    }

    /// Aggiunge una relazione tra due concetti
    pub fn add_relation(&mut self, subject: &str, predicate: &str, object: &str) -> Result<(), String> {
        let mut table = self.get_or_create_table("relations")?;

        let mut record = Record::new(0);
        record.set("subject", Value::from(subject));
        record.set("predicate", Value::from(predicate));
        record.set("object", Value::from(object));

        table.insert(record);
        Ok(())
    }

    /// Cerca concetti per relazione
    pub fn find_by_relation(&self, subject: &str, predicate: &str) -> Vec<String> {
        let mut results = Vec::new();

        if let Some(table) = self.db.get_table("relations") {
            for record in table.records() {
                let record_subject = record.get("subject").and_then(|v| v.as_str()).unwrap_or("");
                let record_predicate = record.get("predicate").and_then(|v| v.as_str()).unwrap_or("");

                if record_subject == subject && record_predicate == predicate {
                    if let Some(obj) = record.get("object").and_then(|v| v.as_str()) {
                        results.push(obj.to_string());
                    }
                }
            }
        }

        results
    }

    /// Verifica se esiste una relazione
    pub fn has_relation(&self, subject: &str, predicate: &str, object: &str) -> bool {
        self.find_by_relation(subject, predicate).contains(&object.to_string())
    }

    /// Aggiunge una parola al vocabolario
    pub fn add_word(&mut self, word: &str, category: &str) -> Result<(), String> {
        let mut table = self.get_or_create_table("vocabulary")?;

        let mut record = Record::new(0);
        record.set("word", Value::from(word));
        record.set("category", Value::from(category));

        table.insert(record);
        Ok(())
    }

    /// Ottiene la categoria di una parola
    pub fn get_category(&self, word: &str) -> Option<String> {
        if let Some(table) = self.db.get_table("vocabulary") {
            for record in table.records() {
                let record_word = record.get("word").and_then(|v| v.as_str()).unwrap_or("");
                if record_word == word {
                    return record.get("category").and_then(|v| v.as_str()).map(|s| s.to_string());
                }
            }
        }
        None
    }

    /// Importa conoscenza da Wikipedia (formato articoli)
    pub fn import_wikipedia_articles(&mut self, path: &str) -> Result<usize, String> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut count = 0;

        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            if line.is_empty() { continue; }

            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 5 {
                let id = parts[0];
                let title = parts[1];
                let categories = parts[4];

                self.add_article(id, title, categories)?;
                count += 1;
            }

            if count % 10000 == 0 {
                println!("📚 Importati {} articoli...", count);
            }
        }

        Ok(count)
    }

    fn add_article(&mut self, id: &str, title: &str, categories: &str) -> Result<(), String> {
        let mut table = self.get_or_create_table("articles")?;

        let mut record = Record::new(0);
        record.set("id", Value::from(id));
        record.set("title", Value::from(title));
        record.set("categories", Value::from(categories));

        table.insert(record);

        // Aggiungi anche come parola nel vocabolario
        self.add_word(title, "article")?;

        Ok(())
    }

    fn get_or_create_table(&mut self, name: &str) -> Result<&mut Table, String> {
        if self.db.get_table(name).is_none() {
            // Header temporaneo (poi migliorabile)
            self.db.add_table(name, Table::new("10", 16))?;
        }
        Ok(self.db.get_table_mut(name).unwrap())
    }

    pub fn stats(&self) -> String {
        let articles = self.db.get_table("articles").map(|t| t.len()).unwrap_or(0);
        let vocabulary = self.db.get_table("vocabulary").map(|t| t.len()).unwrap_or(0);
        let relations = self.db.get_table("relations").map(|t| t.len()).unwrap_or(0);

        format!(
            "📊 Knowledge Base:\n  - Articoli: {}\n  - Vocabolario: {}\n  - Relazioni: {}",
            articles, vocabulary, relations
        )
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        self.db.save_binary(path)
    }

    pub fn load(&mut self, path: &str) -> Result<(), String> {
        self.db.load_binary(path)
    }
}

impl Default for KnowledgeBase {
    fn default() -> Self {
        Self::new()
    }
}