//! Base di conoscenza (terzine, pesi)
//! Usa formato .sson per salvare/caricare

use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Terzina {
    pub soggetto: String,
    pub predicato: String,
    pub oggetto: String,
    pub peso: u8,
}

impl Terzina {
    pub fn new(soggetto: &str, predicato: &str, oggetto: &str, peso: u8) -> Self {
        Self {
            soggetto: soggetto.to_string(),
            predicato: predicato.to_string(),
            oggetto: oggetto.to_string(),
            peso,
        }
    }
    
    /// Converte in formato .sson
    pub fn to_sson(&self) -> String {
        format!(
            "[{}]\npredicato: {}\noggetto: {}\npeso: {}\n",
            self.soggetto, self.predicato, self.oggetto, self.peso
        )
    }
}

#[derive(Debug, Clone)]
pub struct KnowledgeBase {
    terzine: HashMap<String, Terzina>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self {
            terzine: HashMap::new(),
        }
    }
    
    pub fn add_terzina(&mut self, soggetto: &str, oggetto: &str) {
        let key = format!("{}→{}", soggetto, oggetto);
        self.terzine.insert(key, Terzina::new(soggetto, "è", oggetto, 100));
    }
    
    pub fn add(&mut self, terzina: Terzina) {
        let key = format!("{}→{}", terzina.soggetto, terzina.oggetto);
        self.terzine.insert(key, terzina);
    }
    
    pub fn answer(&self, question: &str) -> String {
        for (_, t) in &self.terzine {
            if question.contains(&t.soggetto) {
                return format!("{} {} {}", t.soggetto, t.predicato, t.oggetto);
            }
        }
        String::new()
    }
    
    pub fn get(&self, soggetto: &str) -> Vec<&Terzina> {
        self.terzine
            .values()
            .filter(|t| t.soggetto == soggetto)
            .collect()
    }
    
    /// ============================================
    /// CARICAMENTO DATI DA FILE
    /// ============================================
    
    /// Carica parole da file Wikipedia (formato: "parola peso")
    pub fn load_words(&mut self, path: &str) -> Result<usize, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut count = 0;
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 1 {
                let word = parts[0];
                let peso = if parts.len() >= 2 {
                    parts[1].parse::<u8>().unwrap_or(100)
                } else {
                    100
                };
                
                // Salta parole troppo corte
                if word.len() < 3 {
                    continue;
                }
                
                self.add(Terzina::new(word, "è", "parola italiana", peso));
                count += 1;
            }
        }
        
        Ok(count)
    }
    
    /// Carica dizionario Hunspell (formato .dic)
    pub fn load_dic(&mut self, path: &str) -> Result<usize, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut count = 0;
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Salta la prima riga che contiene il numero di parole
            if line.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }
            
            // Formato .dic: parola/opzioni
            let word = line.split('/').next().unwrap_or(line);
            if word.len() > 2 {
                self.add(Terzina::new(word, "è", "parola italiana", 80));
                count += 1;
            }
        }
        
        Ok(count)
    }
    
    /// Salva in formato .sson
    pub fn save(&self, path: &str) -> Result<(), String> {
        let mut content = String::new();
        content.push_str("# Knowledge Base in formato .sson\n\n");
        
        for terzina in self.terzine.values() {
            content.push_str(&terzina.to_sson());
            content.push('\n');
        }
        
        fs::write(path, content).map_err(|e| e.to_string())
    }
    
    /// Carica da formato .sson
    pub fn load(&mut self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        self.parse_sson(&content)
    }
    
    fn parse_sson(&mut self, content: &str) -> Result<(), String> {
        let mut current_section = String::new();
        let mut current_terzina: Option<(String, String, String, u8)> = None;
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if line.starts_with('[') && line.ends_with(']') {
                // Nuova sezione (soggetto)
                if let Some((soggetto, predicato, oggetto, peso)) = current_terzina.take() {
                    self.add(Terzina::new(&soggetto, &predicato, &oggetto, peso));
                }
                current_section = line[1..line.len()-1].to_string();
                current_terzina = Some((current_section.clone(), String::new(), String::new(), 100));
            } else if let Some((ref mut soggetto, ref mut predicato, ref mut oggetto, ref mut peso)) = current_terzina {
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim();
                    let value = value.trim();
                    match key {
                        "predicato" => *predicato = value.to_string(),
                        "oggetto" => *oggetto = value.to_string(),
                        "peso" => {
                            if let Ok(p) = value.parse::<u8>() {
                                *peso = p;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        
        // Ultima terzina
        if let Some((soggetto, predicato, oggetto, peso)) = current_terzina {
            self.add(Terzina::new(&soggetto, &predicato, &oggetto, peso));
        }
        
        Ok(())
    }
    
    pub fn len(&self) -> usize {
        self.terzine.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.terzine.is_empty()
    }
    
    /// Stampa statistiche
    pub fn stats(&self) -> String {
        format!("KnowledgeBase: {} terzine", self.terzine.len())
    }
}

impl Default for KnowledgeBase {
    fn default() -> Self {
        Self::new()
    }
}
