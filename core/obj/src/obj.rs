//! Struttura Obj dinamica con supporto path

use std::collections::HashMap;
use crate::value::Value;

type Key = String;

/// Oggetto dinamico stile JavaScript con supporto path annidati
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Obj {
    data: HashMap<Key, Value>,
}

impl Obj {
    /// Crea un nuovo oggetto vuoto
    pub fn new() -> Self {
        Self::default()
    }

    /// Imposta un valore nel campo (semplice)
    pub fn set(mut self, key: &str, value: impl Into<Value>) -> Self {
        self.data.insert(key.to_string(), value.into());
        self
    }

    /// Imposta un valore per path (crea oggetti intermedi se necessario)
    pub fn set_path(mut self, path: &[&str], value: impl Into<Value>) -> Self {
        if path.is_empty() {
            return self;
        }

        let value = value.into();

        if path.len() == 1 {
            self.data.insert(path[0].to_string(), value);
            return self;
        }

        let key = path[0].to_string();
        let remaining = &path[1..];

        let child = self
            .data
            .remove(&key)
            .and_then(|v| {
                if let Value::Obj(obj) = v {
                    Some(obj)
                } else {
                    None
                }
            })
            .unwrap_or_else(Obj::new);

        let new_child = child.set_path(remaining, value);
        self.data.insert(key, Value::Obj(new_child));
        self
    }

    /// Ottiene un valore per chiave semplice
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    /// Ottiene un valore per path array
    pub fn get_path(&self, path: &[&str]) -> Option<&Value> {
        if path.is_empty() {
            return None;
        }

        let first = path[0];
        let value = self.data.get(first)?;

        if path.len() == 1 {
            return Some(value);
        }

        match value {
            Value::Obj(obj) => obj.get_path(&path[1..]),
            _ => None,
        }
    }

    /// Ottiene un valore per dot notation (es. "a.b.c")
    pub fn get_dot(&self, dot_path: &str) -> Option<&Value> {
        let path: Vec<&str> = dot_path.split('.').collect();
        self.get_path(&path)
    }

    /// Imposta un valore per dot notation
    pub fn set_dot(self, dot_path: &str, value: impl Into<Value>) -> Self {
        let path: Vec<&str> = dot_path.split('.').collect();
        self.set_path(&path, value)
    }

    /// Rimuove un campo per chiave semplice
    pub fn remove(mut self, key: &str) -> Self {
        self.data.remove(key);
        self
    }

    /// Rimuove per path
    pub fn remove_path(mut self, path: &[&str]) -> Self {
        if path.is_empty() {
            return self;
        }

        if path.len() == 1 {
            self.data.remove(path[0]);
            return self;
        }

        let key = path[0];
        let remaining = &path[1..];

        if let Some(Value::Obj(obj)) = self.data.get(key).cloned() {
            let new_obj = obj.remove_path(remaining);
            self.data.insert(key.to_string(), Value::Obj(new_obj));
        }
        self
    }

    /// Verifica se contiene una chiave
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Unisce due oggetti (spread operator, superficiale)
    pub fn merge(mut self, other: Obj) -> Self {
        for (k, v) in other.data {
            self.data.insert(k, v);
        }
        self
    }

    /// Unisce per path (merge profondo, ricorsivo su oggetti annidati)
    pub fn merge_deep(mut self, other: Obj) -> Self {
        for (k, v) in other.data {
            match (self.data.remove(&k), v) {
                (Some(Value::Obj(existing)), Value::Obj(new)) => {
                    let merged = existing.merge_deep(new);
                    self.data.insert(k, Value::Obj(merged));
                }
                (_, v) => {
                    self.data.insert(k, v);
                }
            }
        }
        self
    }

    /// Restituisce il numero di campi
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Verifica se è vuoto
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Restituisce tutte le chiavi
    pub fn keys(&self) -> Vec<&str> {
        self.data.keys().map(|s| s.as_str()).collect()
    }

    /// Restituisce tutti i valori
    pub fn values(&self) -> Vec<&Value> {
        self.data.values().collect()
    }

    // ========== Metodi di conversione tipata ==========

    /// Estrae una stringa dal campo specificato
    pub fn get_string(&self, key: &str) -> Result<String, String> {
        self.get(key)
            .ok_or_else(|| format!("Campo '{}' mancante", key))?
            .as_str()
            .ok_or_else(|| format!("Campo '{}' non è una stringa", key))
            .map(|s| s.to_string())
    }

    /// Estrae un i32 dal campo specificato
    pub fn get_i32(&self, key: &str) -> Result<i32, String> {
        self.get(key)
            .ok_or_else(|| format!("Campo '{}' mancante", key))?
            .as_number()
            .ok_or_else(|| format!("Campo '{}' non è un numero", key))
            .map(|n| n as i32)
    }

    /// Estrae un u32 dal campo specificato
    pub fn get_u32(&self, key: &str) -> Result<u32, String> {
        self.get(key)
            .ok_or_else(|| format!("Campo '{}' mancante", key))?
            .as_number()
            .ok_or_else(|| format!("Campo '{}' non è un numero", key))
            .map(|n| n as u32)
    }

    /// Estrae un f64 dal campo specificato
    pub fn get_f64(&self, key: &str) -> Result<f64, String> {
        self.get(key)
            .ok_or_else(|| format!("Campo '{}' mancante", key))?
            .as_number()
            .ok_or_else(|| format!("Campo '{}' non è un numero", key))
    }

    /// Estrae un bool dal campo specificato
    pub fn get_bool(&self, key: &str) -> Result<bool, String> {
        self.get(key)
            .ok_or_else(|| format!("Campo '{}' mancante", key))?
            .as_bool()
            .ok_or_else(|| format!("Campo '{}' non è un booleano", key))
    }

    /// Estrae un oggetto annidato dal campo specificato
	pub fn get_obj(&self, key: &str) -> Result<Self, String> {
		Ok(self.get(key)
			.ok_or_else(|| format!("Campo '{}' mancante", key))?
			.as_obj()
			.ok_or_else(|| format!("Campo '{}' non è un oggetto", key))?
			.clone())
	}
}

impl From<HashMap<String, Value>> for Obj {
    fn from(data: HashMap<String, Value>) -> Self {
        Self { data }
    }
}

impl From<Obj> for HashMap<String, Value> {
    fn from(obj: Obj) -> Self {
        obj.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let obj = Obj::new();
        assert!(obj.is_empty());
    }

    #[test]
    fn test_set_get() {
        let obj = Obj::new().set("a", 10).set("b", "hello");
        assert_eq!(obj.get("a").unwrap().as_number(), Some(10.0));
        assert_eq!(obj.get("b").unwrap().as_str(), Some("hello"));
    }

    #[test]
    fn test_set_path() {
        let obj = Obj::new().set_path(&["a", "b", "c"], 42);
        assert_eq!(obj.get_dot("a.b.c").unwrap().as_number(), Some(42.0));
    }

    #[test]
    fn test_get_path() {
        let obj = Obj::new().set_path(&["a", "b", "c"], 42);
        assert_eq!(obj.get_path(&["a", "b", "c"]).unwrap().as_number(), Some(42.0));
        assert_eq!(obj.get_dot("a.b.c").unwrap().as_number(), Some(42.0));
    }

    #[test]
    fn test_contains() {
        let obj = Obj::new().set("a", 10);
        assert!(obj.contains("a"));
        assert!(!obj.contains("b"));
    }

    #[test]
    fn test_remove() {
        let obj = Obj::new().set("a", 10).set("b", 20);
        let obj = obj.remove("a");
        assert!(!obj.contains("a"));
        assert!(obj.contains("b"));
    }

    #[test]
    fn test_remove_path() {
        let obj = Obj::new()
            .set_path(&["a", "b"], 10)
            .set_path(&["a", "c"], 20);
        let obj = obj.remove_path(&["a", "b"]);
        assert!(obj.get_dot("a.b").is_none());
        assert!(obj.get_dot("a.c").is_some());
    }

    #[test]
    fn test_merge() {
        let obj1 = Obj::new().set("a", 10).set("b", 20);
        let obj2 = Obj::new().set("b", 30).set("c", 40);
        let merged = obj1.merge(obj2);
        assert_eq!(merged.get("a").unwrap().as_number(), Some(10.0));
        assert_eq!(merged.get("b").unwrap().as_number(), Some(30.0));
        assert_eq!(merged.get("c").unwrap().as_number(), Some(40.0));
    }

    #[test]
    fn test_merge_deep() {
        let obj1 = Obj::new().set_path(&["a", "b"], 10);
        let obj2 = Obj::new().set_path(&["a", "c"], 20);
        let merged = obj1.merge_deep(obj2);
        assert_eq!(merged.get_dot("a.b").unwrap().as_number(), Some(10.0));
        assert_eq!(merged.get_dot("a.c").unwrap().as_number(), Some(20.0));
    }

    #[test]
    fn test_keys() {
        let obj = Obj::new().set("a", 1).set("b", 2).set("c", 3);
        let mut keys: Vec<_> = obj.keys().into_iter().map(|s| s.to_string()).collect();
        keys.sort();
        assert_eq!(keys, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_values() {
        let obj = Obj::new().set("a", 10).set("b", 20);
        let values: Vec<f64> = obj
            .values()
            .into_iter()
            .filter_map(|v| v.as_number())
            .collect();
        assert_eq!(values.len(), 2);
        assert!(values.contains(&10.0));
        assert!(values.contains(&20.0));
    }

    // Test per i metodi di conversione tipata
    #[test]
    fn test_get_string() {
        let obj = Obj::new().set("name", "test");
        assert_eq!(obj.get_string("name").unwrap(), "test");
        assert!(obj.get_string("missing").is_err());
    }

    #[test]
    fn test_get_i32() {
        let obj = Obj::new().set("value", 42);
        assert_eq!(obj.get_i32("value").unwrap(), 42);
    }

    #[test]
    fn test_get_u32() {
        let obj = Obj::new().set("value", 42);
        assert_eq!(obj.get_u32("value").unwrap(), 42);
    }

    #[test]
    fn test_get_f64() {
        let obj = Obj::new().set("value", 3.14);
        assert_eq!(obj.get_f64("value").unwrap(), 3.14);
    }

    #[test]
    fn test_get_bool() {
        let obj = Obj::new().set("active", true);
        assert_eq!(obj.get_bool("active").unwrap(), true);
    }

    #[test]
    fn test_get_obj() {
        let inner = Obj::new().set("x", 10);
        let outer = Obj::new().set("inner", inner.clone());
        let retrieved = outer.get_obj("inner").unwrap();
        assert_eq!(retrieved.get("x").unwrap().as_number(), Some(10.0));
    }
}
