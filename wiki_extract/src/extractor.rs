//! Estrazione parole da testo Wikipedia

use regex::Regex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref WORD_RE: Regex = Regex::new(r"[a-zàèéìòù]+").unwrap();
}

pub fn extract_from_text(text: &str) -> ExtractedData {
    let mut words = HashMap::new();
    let mut verbs = HashMap::new();
    let mut nouns = HashMap::new();
    
    let text_lower = text.to_lowercase();
    
    for word_match in WORD_RE.find_iter(&text_lower) {
        let w = word_match.as_str();
        if w.len() > 2 {
            *words.entry(w.to_string()).or_insert(0) += 1;
            
            if is_verb(w) {
                let infinitive = get_infinitive(w);
                *verbs.entry(infinitive).or_insert(0) += 1;
            }
            
            if is_noun(w) {
                *nouns.entry(w.to_string()).or_insert(0) += 1;
            }
        }
    }
    
    ExtractedData { words, verbs, nouns }
}

fn is_verb(word: &str) -> bool {
    word.ends_with("are") || word.ends_with("ere") || word.ends_with("ire")
}

fn is_noun(word: &str) -> bool {
    word.ends_with('a') || word.ends_with('o') || word.ends_with('e')
}

fn get_infinitive(word: &str) -> String {
    if word.ends_with("are") || word.ends_with("ere") || word.ends_with("ire") {
        word.to_string()
    } else if word.ends_with('o') {
        format!("{}are", &word[..word.len()-1])
    } else if word.ends_with('i') {
        format!("{}are", &word[..word.len()-1])
    } else {
        word.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ExtractedData {
    pub words: HashMap<String, u32>,
    pub verbs: HashMap<String, u32>,
    pub nouns: HashMap<String, u32>,
}

pub fn extract_from_page(text: &str, _title: &str) -> ExtractedData {
    extract_from_text(text)
}
