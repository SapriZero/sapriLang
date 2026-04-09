//! Registry base per nomi e alias
//! Step 1: Implementazione base senza dipendenze complesse

use std::collections::HashMap;

/// Entry nel registry
#[derive(Debug, Clone)]
pub struct RegistryEntry {
    /// Nome completo (path.dot.separated)
    pub full_name: String,
    /// Alias automatico (1-3 caratteri per livello)
    pub alias: String,
    /// Valore associato (opzionale)
    pub value: Option<String>,
    /// Metadati aggiuntivi
    pub metadata: HashMap<String, String>,
}

/// Registry principale
#[derive(Debug, Default)]
pub struct Registry {
    /// Mappa nome completo → entry
    names: HashMap<String, RegistryEntry>,
    /// Mappa alias → nome completo
    aliases: HashMap<String, String>,
    /// Contatore per generazione alias
    counter: usize,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registra un nuovo nome
    pub fn register(&mut self, name: &str, value: Option<String>) -> Result<(), RegistryError> {
        if self.names.contains_key(name) {
            return Err(RegistryError::AlreadyExists(name.to_string()));
        }

        let alias = self.generate_alias(name)?;

        let entry = RegistryEntry {
            full_name: name.to_string(),
            alias: alias.clone(),
            value,
            metadata: HashMap::new(),
        };

        self.names.insert(name.to_string(), entry);
        self.aliases.insert(alias, name.to_string());

        Ok(())
    }

    /// Genera alias automatico (1-3 caratteri per livello)
    fn generate_alias(&self, name: &str) -> Result<String, RegistryError> {
        let parts: Vec<&str> = name.split('.').collect();
        let mut alias_parts = Vec::with_capacity(parts.len());

        for part in parts {
            // Prova con 1 carattere
            let alias1 = part.chars().next().unwrap().to_string();
            let candidate = if alias_parts.is_empty() {
                alias1.clone()
            } else {
                format!("{}.{}", alias_parts.join("."), alias1)
            };

            if !self.aliases.contains_key(&candidate) {
                alias_parts.push(alias1);
                continue;
            }

            // Collisione → prova con 2 caratteri
            if part.len() >= 2 {
                let alias2 = part[..2].to_string();
                let candidate = if alias_parts.is_empty() {
                    alias2.clone()
                } else {
                    format!("{}.{}", alias_parts.join("."), alias2)
                };

                if !self.aliases.contains_key(&candidate) {
                    alias_parts.push(alias2);
                    continue;
                }
            }

            // Ancora collisione → prova con 3 caratteri
            if part.len() >= 3 {
                let alias3 = part[..3].to_string();
                let candidate = if alias_parts.is_empty() {
                    alias3.clone()
                } else {
                    format!("{}.{}", alias_parts.join("."), alias3)
                };

                if !self.aliases.contains_key(&candidate) {
                    alias_parts.push(alias3);
                    continue;
                }
            }

            return Err(RegistryError::AliasCollision(name.to_string()));
        }

        Ok(alias_parts.join("."))
    }

    /// Ottiene entry per nome completo
    pub fn get(&self, name: &str) -> Option<&RegistryEntry> {
        self.names.get(name)
    }

    /// Ottiene entry per alias
    pub fn get_by_alias(&self, alias: &str) -> Option<&RegistryEntry> {
        self.aliases
            .get(alias)
            .and_then(|full| self.names.get(full))
    }

    /// Risolve un nome (accetta sia completo che alias)
    pub fn resolve(&self, name_or_alias: &str) -> Option<&RegistryEntry> {
        self.get(name_or_alias)
            .or_else(|| self.get_by_alias(name_or_alias))
    }

    /// Aggiunge metadata a un entry
    pub fn add_metadata(&mut self, name: &str, key: &str, value: &str) -> Result<(), RegistryError> {
        let entry = self.names.get_mut(name)
            .ok_or_else(|| RegistryError::NotFound(name.to_string()))?;
        entry.metadata.insert(key.to_string(), value.to_string());
        Ok(())
    }

    /// Lista tutti i nomi registrati
    pub fn list_names(&self) -> Vec<&str> {
        self.names.keys().map(|s| s.as_str()).collect()
    }

    /// Lista tutti gli alias
    pub fn list_aliases(&self) -> Vec<&str> {
        self.aliases.keys().map(|s| s.as_str()).collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("Name already exists: {0}")]
    AlreadyExists(String),

    #[error("Name not found: {0}")]
    NotFound(String),

    #[error("Cannot generate unique alias for: {0}")]
    AliasCollision(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_get() {
        let mut reg = Registry::new();
        reg.register("app.file.list", None).unwrap();

        let entry = reg.get("app.file.list").unwrap();
        assert_eq!(entry.full_name, "app.file.list");
        assert_eq!(entry.alias, "a.f.l");
    }

    #[test]
    fn test_alias_generation() {
        let mut reg = Registry::new();
        reg.register("app.file.list", None).unwrap();   // a.f.l
        reg.register("app.file.get", None).unwrap();    // a.f.g
        reg.register("app.file.save", None).unwrap();   // a.f.s

        assert!(reg.resolve("a.f.l").is_some());
        assert!(reg.resolve("a.f.g").is_some());
        assert!(reg.resolve("a.f.s").is_some());
    }

    #[test]
    fn test_alias_collision_2char() {
        let mut reg = Registry::new();
        reg.register("app.file.list", None).unwrap();    // a.f.l
        reg.register("app.file.light", None).unwrap();   // a.f.li (2 char)

        assert!(reg.resolve("a.f.l").is_some());
        assert!(reg.resolve("a.f.li").is_some());
    }

    #[test]
    fn test_metadata() {
        let mut reg = Registry::new();
        reg.register("app.file.list", Some("handler1".to_string())).unwrap();
        reg.add_metadata("app.file.list", "public", "true").unwrap();

        let entry = reg.get("app.file.list").unwrap();
        assert_eq!(entry.metadata.get("public").unwrap(), "true");
    }
}
