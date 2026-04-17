//! Parser per file .dic di Hunspell

use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DicEntry {
    pub word: String,
    pub flags: Vec<String>,
    pub morphological: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WordFlags {
    pub pos: Option<String>,      // Parte del discorso (N, V, A, etc.)
    pub gender: Option<String>,   // Genere (m, f)
    pub number: Option<String>,   // Numero (s, p)
    pub person: Option<String>,   // Persona (1, 2, 3)
    pub tense: Option<String>,    // Tempo (presente, passato, etc.)
    pub mood: Option<String>,     // Modo (indicativo, congiuntivo, etc.)
}

impl DicEntry {
    pub fn from_file(path: &str) -> Result<Vec<Self>, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::parse(&content)
    }
    
    pub fn parse(content: &str) -> Result<Vec<Self>, String> {
        let mut entries = Vec::new();
        let mut lines = content.lines();
        
        // Prima riga: numero di parole (ignoriamo)
        if let Some(count_line) = lines.next() {
            let _count: usize = count_line.trim().parse().unwrap_or(0);
        }
        
        for line in lines {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            
            let (word, rest) = if let Some(idx) = line.find('/') {
                (line[..idx].to_string(), Some(&line[idx+1..]))
            } else {
                (line.to_string(), None)
            };
            
            let flags = if let Some(rest_str) = rest {
                rest_str.split(',').map(|f| f.trim().to_string()).collect()
            } else {
                Vec::new()
            };
            
            entries.push(DicEntry {
                word,
                flags,
                morphological: None,
            });
        }
        
        Ok(entries)
    }
}

impl WordFlags {
    pub fn from_flags(flags: &[String]) -> Self {
        let mut pos = None;
        let mut gender = None;
        let mut number = None;
        let mut person = None;
        let mut tense = None;
        let mut mood = None;
        
        for flag in flags {
            match flag.as_str() {
                "N" => pos = Some("noun".to_string()),
                "V" => pos = Some("verb".to_string()),
                "A" => pos = Some("adjective".to_string()),
                "Av" => pos = Some("adverb".to_string()),
                "P" => pos = Some("pronoun".to_string()),
                "C" => pos = Some("conjunction".to_string()),
                "I" => pos = Some("interjection".to_string()),
                "E" => pos = Some("preposition".to_string()),
                "m" => gender = Some("masculine".to_string()),
                "f" => gender = Some("feminine".to_string()),
                "s" => number = Some("singular".to_string()),
                "p" => number = Some("plural".to_string()),
                "1" => person = Some("first".to_string()),
                "2" => person = Some("second".to_string()),
                "3" => person = Some("third".to_string()),
                "P" if flag.len() > 1 && flag.starts_with("P") => {
                    tense = Some("past".to_string());
                }
                "F" => tense = Some("future".to_string()),
                _ => {}
            }
        }
        
        Self { pos, gender, number, person, tense, mood }
    }
}
