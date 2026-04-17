//! Estrazione aggettivi

use crate::DicEntry;

#[derive(Debug, Clone)]
pub struct AdjectiveInfo {
    pub word: String,
    pub gender: Option<String>,
    pub number: Option<String>,
}

impl AdjectiveInfo {
    pub fn from_entry(entry: &DicEntry) -> Option<Self> {
        let is_adj = entry.flags.contains(&"A".to_string());
        if !is_adj {
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
        
        Some(AdjectiveInfo {
            word: entry.word.clone(),
            gender,
            number,
        })
    }
}

pub fn extract(entries: &[DicEntry]) -> Vec<AdjectiveInfo> {
    entries.iter()
        .filter_map(AdjectiveInfo::from_entry)
        .collect()
}
