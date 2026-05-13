//! Memoria olografica basata su IRCM
//!
//! Implementa i principi della memoria olografica:
//! - Salva solo gli scarti (S ≠ 1)
//! - Ricostruisce il dato implicito dalla struttura

use sapri_db::{Database, DatabaseOps, Table, Record};
use sapri_obj::Value;

/// Memoria olografica
#[derive(Debug)]
pub struct HolographicMemory {
    db: Database,
    /// Fattore K (tolleranza per considerare S ≈ 1)
    tolerance: f64,
}

/// Entry di memoria con S-score
#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub input: String,
    pub output: String,
    pub s_score: f64,
    pub timestamp: u64,
}

impl HolographicMemory {
    pub fn new() -> Self {
        Self {
            db: Database::new(),
            tolerance: 0.1, // S ≈ 1 se |S-1| < 0.1
        }
    }

    pub fn with_tolerance(tolerance: f64) -> Self {
        Self {
            db: Database::new(),
            tolerance,
        }
    }

    /// Ricorda una nuova informazione (calcola S-score)
    pub fn remember(&mut self, input: &str, output: &str) -> Result<f64, String> {
        // Calcola S-score (versione semplificata)
        // S = (v * i) / (t * k) dove:
        // v = importanza (1 per default)
        // i = novità (1 per default)
        // t = target (1 per default)
        // k = tolerance
        let s_score = self.compute_s_score(input, output);

        // Salva solo se S ≠ 1 (scarto significativo)
        if (s_score - 1.0).abs() > self.tolerance {
            let mut table = self.get_or_create_table("memory")?;
            let mut record = Record::new(0);
            record.set("input", Value::from(input));
            record.set("output", Value::from(output));
            record.set("s_score", Value::from(s_score));
            table.insert(record);
        }

        Ok(s_score)
    }

    /// Recupera un ricordo per input
    pub fn recall(&self, input: &str) -> Option<String> {
        // Cerca prima nella memoria diretta
        if let Some(table) = self.db.get_table("memory") {
            for record in table.records() {
                let record_input = record.get("input").and_then(|v| v.as_str()).unwrap_or("");
                if record_input == input {
                    return record.get("output").and_then(|v| v.as_str()).map(|s| s.to_string());
                }
            }
        }

        // TODO: Se non trova, cerca per similarità strutturale (IRCM)
        None
    }

    /// Calcola S-score per una coppia input-output
    fn compute_s_score(&self, input: &str, output: &str) -> f64 {
        // Formula semplificata: S = (len(input) * novelty) / (target * tolerance)
        let v = input.len() as f64 / 100.0; // normalizzato
        let i = 1.0; // novità (da implementare)
        let t = 1.0; // target
        let k = self.tolerance;

        if t * k == 0.0 { 1.0 } else { (v * i) / (t * k) }
    }

    /// Ricostruisce un dato mancante dalla struttura relazionale
    pub fn reconstruct(&self, input: &str) -> Option<String> {
        // TODO: Implementare ricostruzione olografica
        // Dati i vicini (relazioni), deduci il dato mancante
        self.recall(input)
    }

    fn get_or_create_table(&mut self, name: &str) -> Result<&mut Table, String> {
        if self.db.get_table(name).is_none() {
            self.db.add_table(name, Table::new("11", 16))?;
        }
        Ok(self.db.get_table_mut(name).unwrap())
    }

    pub fn len(&self) -> usize {
        self.db.get_table("memory").map(|t| t.len()).unwrap_or(0)
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        self.db.save_binary(path)
    }

    pub fn load(&mut self, path: &str) -> Result<(), String> {
        self.db.load_binary(path)
    }
}

impl Default for HolographicMemory {
    fn default() -> Self {
        Self::new()
    }
}