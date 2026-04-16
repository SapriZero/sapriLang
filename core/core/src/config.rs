//! Gestione configurazione da file .sson

use std::fs;
// use std::path::Path;
use sapri_sson::{ParserMode, Constraint, Validator};
use sapri_obj::Obj;

/// Configurazione del sistema
#[derive(Debug, Clone)]
pub struct Config {
    pub mode: ParserMode,
    pub max_depth: usize,
    pub version: String,
    pub constraints: Vec<Constraint>,
    pub initial_atoms: Obj,
}

impl Config {
    /// Configurazione di default
    pub fn default() -> Self {
        Self {
            mode: ParserMode::Strict,
            max_depth: 100,
            version: env!("CARGO_PKG_VERSION").to_string(),
            constraints: Vec::new(),
            initial_atoms: Obj::new(),
        }
    }
    
    /// Carica configurazione da file .sson
    pub fn load(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path, e))?;
        
        // Per ora, parsing semplificato
        // TODO: Usare sapri_sson parser completo
        let mut config = Config::default();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                
                match key {
                    "mode" => {
                        config.mode = match value {
                            "strict" => ParserMode::Strict,
                            "generative" => ParserMode::Generative,
                            _ => ParserMode::Strict,
                        };
                    }
                    "max_depth" => {
                        config.max_depth = value.parse().unwrap_or(100);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(config)
    }
    
    /// Valida la configurazione
    pub fn validate(&self) -> Result<(), String> {
        let validator = Validator::new();
        
        for constraint in &self.constraints {
            let ctx = sapri_sson::ValidationContext::new(self.mode, self.initial_atoms.clone());
            if !validator.validate(&ctx, constraint) {
                return Err(format!("Constraint validation failed: {:?}", constraint));
            }
        }
        
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::default()
    }
}
