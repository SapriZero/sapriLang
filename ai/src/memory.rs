//! Memoria olografica (solo variazioni)

use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub timestamp: u64,
    pub input: String,
    pub response: String,
}

impl MemoryEntry {
    pub fn to_sson(&self) -> String {
        format!(
            "[entry_{}]\ntimestamp: {}\ninput: {}\nresponse: {}\n",
            self.timestamp, self.timestamp, self.input, self.response
        )
    }
}

#[derive(Debug, Clone)]
pub struct HolographicMemory {
    entries: VecDeque<MemoryEntry>,
    max_size: usize,
}

impl HolographicMemory {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
            max_size: 100,
        }
    }
    
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_size,
        }
    }
    
    pub fn remember(&mut self, input: &str, response: &str) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let entry = MemoryEntry {
            timestamp,
            input: input.to_string(),
            response: response.to_string(),
        };
        
        self.entries.push_back(entry);
        while self.entries.len() > self.max_size {
            self.entries.pop_front();
        }
    }
    
    pub fn recall(&self, input: &str) -> Option<String> {
        for entry in self.entries.iter().rev() {
            if entry.input == input {
                return Some(entry.response.clone());
            }
        }
        None
    }
    
    pub fn recent(&self, n: usize) -> Vec<&MemoryEntry> {
        self.entries.iter().rev().take(n).collect()
    }
    
    pub fn save(&self, path: &str) -> Result<(), String> {
        let mut content = String::new();
        content.push_str("# Holographic Memory in formato .sson\n\n");
        
        for entry in &self.entries {
            content.push_str(&entry.to_sson());
            content.push('\n');
        }
        
        fs::write(path, content).map_err(|e| e.to_string())
    }
    
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for HolographicMemory {
    fn default() -> Self {
        Self::new()
    }
}
