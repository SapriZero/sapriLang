//! Estrazione nomi con genere e numero

use crate::DicEntry;

#[derive(Debug, Clone)]
pub struct NounInfo {
    pub word: String,
    pub gender: Option<String>,
    pub number: Option<String>,
}

impl NounInfo {
    pub fn from_entry(entry: &DicEntry) -> Option<Self> {
        let is_noun = entry.flags.contains(&"N".to_string());
        if !is_noun {
            return None;
        }
        
        let mut gender = None;
        let mut number = None;
        
        for flag in &entry.flags {
            match flag.as_str() {
                "m" => gender = Some("masculine".to_string()),
                "f" => gender = Some("feminine".to_string()),
                "s" => number = Some("singular".to_string()),
                "p" => number = Some("plural".to_string()),
                _ => {}
            }
        }
        
        Some(NounInfo {
            word: entry.word.clone(),
            gender,
            number,
        })
    }
}

pub fn extract(entries: &[DicEntry]) -> Vec<NounInfo> {
    entries.iter()
        .filter_map(NounInfo::from_entry)
        .collect()
}
