//! Gestione checkpoint per ripresa elaborazione

use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub page_count: u64,
    pub byte_position: u64,
    pub last_page_title: String,
    pub timestamp: u64,
    pub word_counts: Vec<(String, u32)>,
    pub verb_counts: Vec<(String, u32)>,
    pub noun_counts: Vec<(String, u32)>,
}

impl Checkpoint {
    pub fn new() -> Self {
        Self {
            page_count: 0,
            byte_position: 0,
            last_page_title: String::new(),
            timestamp: 0,
            word_counts: Vec::new(),
            verb_counts: Vec::new(),
            noun_counts: Vec::new(),
        }
    }
    
    pub fn save(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }
    
    pub fn load(path: &str) -> Result<Self, String> {
        let json = fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
    
    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }
}

impl Default for Checkpoint {
    fn default() -> Self {
        Self::new()
    }
}
