//! Registry per definizioni dati
//!
//! Integrazione con il sistema di naming di core

use std::collections::HashMap;
use urcm_core::registry::Registry as CoreRegistry;

#[derive(Debug, Clone)]
pub struct DataDefinition {
    pub name: String,
    pub schema: Option<Schema>,
    pub table: Option<Table>,
    pub flags: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
}

pub struct DataRegistry {
    core_registry: Option<CoreRegistry>,
    definitions: HashMap<String, DataDefinition>,
    aliases: HashMap<String, String>,  // alias → nome completo
}

impl DataRegistry {
    pub fn new() -> Self {
        Self {
            core_registry: None,
            definitions: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    pub fn with_core_registry(registry: CoreRegistry) -> Self {
        Self {
            core_registry: Some(registry),
            definitions: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    /// Registra una definizione
    pub fn register(&mut self, name: &str, def: DataDefinition) -> Result<(), RegistryError> {
        self.definitions.insert(name.to_string(), def);

        // Genera alias automatici
        self.generate_aliases(name)?;

        Ok(())
    }

    /// Genera alias automatici (1-2-3 caratteri)
    fn generate_aliases(&mut self, name: &str) -> Result<(), RegistryError> {
        let parts: Vec<&str> = name.split('.').collect();

        // Alias 1 carattere: prime lettere
        let alias1: String = parts.iter().map(|p| p.chars().next().unwrap()).collect();

        if !self.aliases.contains_key(&alias1) {
            self.aliases.insert(alias1, name.to_string());
        } else {
            // Alias 2 caratteri: prime due lettere
            let alias2: String = parts.iter().map(|p| {
                let mut chars = p.chars();
                let first = chars.next().unwrap();
                let second = chars.next().unwrap_or(first);
                format!("{}{}", first, second)
            }).collect();

            if !self.aliases.contains_key(&alias2) {
                self.aliases.insert(alias2, name.to_string());
            } else {
                // Alias 3 caratteri
                let alias3: String = parts.iter().map(|p| {
                    let mut chars = p.chars();
                    let first = chars.next().unwrap();
                    let second = chars.next().unwrap_or(first);
                    let third = chars.next().unwrap_or(second);
                    format!("{}{}{}", first, second, third)
                }).collect();

                if !self.aliases.contains_key(&alias3) {
                    self.aliases.insert(alias3, name.to_string());
                } else {
                    return Err(RegistryError::AliasCollision(alias3));
                }
            }
        }

        Ok(())
    }

    /// Ottiene definizione per nome o alias
    pub fn get(&self, name_or_alias: &str) -> Option<&DataDefinition> {
        // Cerca prima come nome diretto
        if let Some(def) = self.definitions.get(name_or_alias) {
            return Some(def);
        }

        // Poi come alias
        if let Some(real_name) = self.aliases.get(name_or_alias) {
            return self.definitions.get(real_name);
        }

        None
    }

    /// Crea definizione automatica da percorso non registrato
    pub fn ensure_path(&mut self, path: &str) -> Result<&DataDefinition, RegistryError> {
        if self.definitions.contains_key(path) {
            return Ok(self.definitions.get(path).unwrap());
        }

        // Crea definizione di default
        let def = DataDefinition {
            name: path.to_string(),
            schema: None,
            table: None,
            flags: HashMap::new(),
            metadata: HashMap::new(),
        };

        self.definitions.insert(path.to_string(), def);
        Ok(self.definitions.get(path).unwrap())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("Definition already exists: {0}")]
    AlreadyExists(String),

    #[error("Alias collision: {0}")]
    AliasCollision(String),

    #[error("Definition not found: {0}")]
    NotFound(String),
}

impl Default for DataRegistry {
    fn default() -> Self {
        Self::new()
    }
}
