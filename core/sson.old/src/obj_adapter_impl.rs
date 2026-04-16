// ============================================
// core/sson/src/obj_adapter_impl.rs
// Implementazione del trait ObjAdapter
// Adapter tra dizionario piatto e sapri_obj::Obj
// ============================================

use crate::*;
use core::obj::{Obj, Value};
use core::base::fp::memoize;

use std::collections::HashMap;
use std::sync::Arc;

// ============================================
// OBJ ADAPTER IMPLEMENTATION
// ============================================

/// Adapter per core/obj che implementa conversioni e accesso per path
#[derive(Debug, Clone, Default)]
pub struct ObjAdapterImpl {
    /// Separatore per path annidati (default: ".")
    separator: String,
    
    /// Gestione array nei path (es. "users[0].name")
    array_support: bool,
}

impl ObjAdapterImpl {
    /// Crea un nuovo adapter con configurazione default
    pub fn new() -> Self {
        Self {
            separator: ".".to_string(),
            array_support: true,
        }
    }
    
    /// Crea un adapter con separatore personalizzato
    pub fn with_separator(separator: &str) -> Self {
        Self {
            separator: separator.to_string(),
            array_support: true,
        }
    }
    
    /// Disabilita il supporto per array nei path
    pub fn without_array_support(mut self) -> Self {
        self.array_support = false;
        self
    }
    
    /// Splitta un path nei suoi componenti
    /// Esempio: "user.address.city" → ["user", "address", "city"]
    /// Esempio: "users[0].name" → ["users", "0", "name"] (se array_support)
    fn split_path(&self, path: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut in_bracket = false;
        
        for c in path.chars() {
            if self.array_support && c == '[' {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
                in_bracket = true;
            } else if self.array_support && c == ']' {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
                in_bracket = false;
            } else if c == '.' && !in_bracket {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            } else {
                current.push(c);
            }
        }
        
        if !current.is_empty() {
            parts.push(current);
        }
        
        parts
    }
    
    /// Converte un valore da flat a nested (chiamata ricorsiva)
    fn expand_value(&self, value: &Value, key_parts: &[String]) -> Value {
        if key_parts.is_empty() {
            return value.clone();
        }
        
        let mut obj = Obj::new();
        let first_key = &key_parts[0];
        let remaining = &key_parts[1..];
        
        let nested_value = self.expand_value(value, remaining);
        obj.set(first_key, nested_value);
        
        Value::Object(obj)
    }
    
    /// Estrae i componenti di un path flat
    /// "user.address.city" → ["user", "address", "city"]
    fn get_path_components(&self, path: &str) -> Vec<String> {
        path.split(&self.separator)
            .map(|s| s.to_string())
            .collect()
    }
}

impl ObjAdapter for ObjAdapterImpl {
    /// Da dizionario piatto (HashMap<String, Value>) a Obj annidato
    /// 
    /// Esempio input:
    /// {
    ///     "user.name": "Alice",
    ///     "user.age": 30,
    ///     "user.address.city": "Wonderland"
    /// }
    /// 
    /// Output:
    /// {
    ///     "user": {
    ///         "name": "Alice",
    ///         "age": 30,
    ///         "address": {
    ///             "city": "Wonderland"
    ///         }
    ///     }
    /// }
    fn from_flat_dict(&self, flat: &HashMap<FieldPath, Value>) -> Obj {
        let mut result = Obj::new();
        
        for (path, value) in flat {
            let components = self.get_path_components(path);
            if components.is_empty() {
                continue;
            }
            
            // Naviga o crea la struttura annidata
            let mut current = &mut result;
            
            for (i, comp) in components.iter().enumerate() {
                let is_last = i == components.len() - 1;
                
                if is_last {
                    // Imposta il valore all'ultimo livello
                    current.set(comp, value.clone());
                } else {
                    // Assicura che il livello intermedio esista
                    if !current.has(comp) {
                        current.set(comp, Value::Object(Obj::new()));
                    }
                    
                    // Spostati al livello successivo
                    if let Some(Value::Object(next)) = current.get_mut(comp) {
                        current = next;
                    } else {
                        // Se non è un oggetto, lo sostituiamo con uno nuovo
                        let new_obj = Obj::new();
                        current.set(comp, Value::Object(new_obj.clone()));
                        if let Some(Value::Object(next)) = current.get_mut(comp) {
                            current = next;
                        }
                    }
                }
            }
        }
        
        result
    }
    
    /// Da Obj annidato a dizionario piatto (HashMap<String, Value>)
    /// 
    /// Esempio input:
    /// {
    ///     "user": {
    ///         "name": "Alice",
    ///         "address": {
    ///             "city": "Wonderland"
    ///         }
    ///     }
    /// }
    /// 
    /// Output:
    /// {
    ///     "user.name": "Alice",
    ///     "user.address.city": "Wonderland"
    /// }
    fn to_flat_dict(&self, obj: &Obj) -> HashMap<FieldPath, Value> {
        let mut result = HashMap::new();
        self.flatten_obj(obj, "", &mut result);
        result
    }
    
    /// Get per path (es. "user.address.city")
    /// Supporta array: "users[0].name"
    fn get_path(&self, obj: &Obj, path: &str) -> Result<Value> {
        let parts = self.split_path(path);
        let mut current: &Value = &Value::Object(obj.clone());
        
        for part in parts {
            // Verifica se la parte è un indice numerico (per array)
            if self.array_support && part.parse::<usize>().is_ok() {
                let idx = part.parse::<usize>().unwrap();
                if let Some(Value::Array(arr)) = current.as_array() {
                    if idx < arr.len() {
                        current = &arr[idx];
                    } else {
                        return Err(Error::new(&format!("Array index {} out of bounds", idx)));
                    }
                } else {
                    return Err(Error::new(&format!("Expected array at path component '{}'", part)));
                }
            } else {
                if let Some(Value::Object(obj_ref)) = current.as_object() {
                    if let Some(value) = obj_ref.get(&part) {
                        current = value;
                    } else {
                        return Err(Error::new(&format!("Path '{}' not found", path)));
                    }
                } else {
                    return Err(Error::new(&format!("Expected object at path component '{}'", part)));
                }
            }
        }
        
        Ok(current.clone())
    }
    
    /// Set per path (es. "user.address.city")
    /// Crea i nodi intermedi se non esistono
    fn set_path(&self, obj: &mut Obj, path: &str, value: Value) -> Result<()> {
        let parts = self.split_path(path);
        if parts.is_empty() {
            return Err(Error::new("Empty path"));
        }
        
        let last_idx = parts.len() - 1;
        let mut current: &mut Value = &mut Value::Object(obj.clone());
        
        // Naviga fino all'ultimo componente
        for (i, part) in parts.iter().enumerate() {
            let is_last = i == last_idx;
            
            if is_last {
                // Imposta il valore
                if let Value::Object(ref mut obj_ref) = current {
                    obj_ref.set(part, value);
                    *obj = obj_ref.clone();
                    return Ok(());
                } else {
                    return Err(Error::new(&format!("Cannot set value on non-object at path '{}'", path)));
                }
            } else {
                // Naviga o crea il livello intermedio
                if let Value::Object(ref mut obj_ref) = current {
                    if !obj_ref.has(part) {
                        obj_ref.set(part, Value::Object(Obj::new()));
                    }
                    
                    // Prendi il riferimento al prossimo livello
                    if let Some(next) = obj_ref.get_mut(part) {
                        current = next;
                    } else {
                        return Err(Error::new(&format!("Failed to navigate to '{}'", part)));
                    }
                } else {
                    return Err(Error::new(&format!("Expected object at path component '{}'", part)));
                }
            }
        }
        
        Ok(())
    }
}

// ============================================
// HELPER FUNCTIONS (flatten)
// ============================================

impl ObjAdapterImpl {
    /// Funzione ricorsiva per appiattire un Obj
    fn flatten_obj(&self, obj: &Obj, prefix: &str, result: &mut HashMap<FieldPath, Value>) {
        for (key, value) in obj.iter() {
            let full_key = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}{}{}", prefix, self.separator, key)
            };
            
            match value {
                Value::Object(nested) => {
                    self.flatten_obj(nested, &full_key, result);
                }
                Value::Array(arr) => {
                    // Per array, usiamo notazione con indice
                    for (idx, item) in arr.iter().enumerate() {
                        let array_key = format!("{}[{}]", full_key, idx);
                        if let Value::Object(nested) = item {
                            self.flatten_obj(nested, &array_key, result);
                        } else {
                            result.insert(array_key, item.clone());
                        }
                    }
                }
                _ => {
                    result.insert(full_key, value.clone());
                }
            }
        }
    }
}

// ============================================
// OBJ ADAPTER CON CACHE (per performance)
// ============================================

/// Adapter con cache per operazioni ripetute
#[derive(Debug, Clone)]
pub struct CachedObjAdapter {
    inner: ObjAdapterImpl,
    get_cache: Arc<std::sync::RwLock<HashMap<(u64, String), Value>>>,
    flat_cache: Arc<std::sync::RwLock<HashMap<u64, HashMap<FieldPath, Value>>>>,
}

impl CachedObjAdapter {
    pub fn new() -> Self {
        Self {
            inner: ObjAdapterImpl::new(),
            get_cache: Arc::new(std::sync::RwLock::new(HashMap::new())),
            flat_cache: Arc::new(std::sync::RwLock::new(HashMap::new())),
        }
    }
    
    /// Calcola un hash per l'oggetto (semplice, basato sulla dimensione)
    fn obj_hash(&self, obj: &Obj) -> u64 {
        // Hash semplice: lunghezza + prime keys
        let mut hash = obj.len() as u64;
        for (i, key) in obj.keys().take(5).enumerate() {
            hash = hash.wrapping_mul(31).wrapping_add(key.len() as u64);
            hash = hash.wrapping_mul(31).wrapping_add(i as u64);
        }
        hash
    }
    
    /// Pulisce la cache
    pub fn clear_cache(&self) {
        self.get_cache.write().unwrap().clear();
        self.flat_cache.write().unwrap().clear();
    }
    
    /// Get con cache (memorizza risultati per (obj_hash, path))
    pub fn get_path_cached(&self, obj: &Obj, path: &str) -> Result<Value> {
        let obj_hash = self.obj_hash(obj);
        let cache_key = (obj_hash, path.to_string());
        
        // Prova cache
        {
            let cache = self.get_cache.read().unwrap();
            if let Some(value) = cache.get(&cache_key) {
                return Ok(value.clone());
            }
        }
        
        // Calcola e salva in cache
        let value = self.inner.get_path(obj, path)?;
        {
            let mut cache = self.get_cache.write().unwrap();
            cache.insert(cache_key, value.clone());
        }
        
        Ok(value)
    }
    
    /// To flat dict con cache
    pub fn to_flat_dict_cached(&self, obj: &Obj) -> HashMap<FieldPath, Value> {
        let obj_hash = self.obj_hash(obj);
        
        // Prova cache
        {
            let cache = self.flat_cache.read().unwrap();
            if let Some(flat) = cache.get(&obj_hash) {
                return flat.clone();
            }
        }
        
        // Calcola e salva in cache
        let flat = self.inner.to_flat_dict(obj);
        {
            let mut cache = self.flat_cache.write().unwrap();
            cache.insert(obj_hash, flat.clone());
        }
        
        flat
    }
}

impl Default for CachedObjAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjAdapter for CachedObjAdapter {
    fn from_flat_dict(&self, flat: &HashMap<FieldPath, Value>) -> Obj {
        self.inner.from_flat_dict(flat)
    }
    
    fn to_flat_dict(&self, obj: &Obj) -> HashMap<FieldPath, Value> {
        self.to_flat_dict_cached(obj)
    }
    
    fn get_path(&self, obj: &Obj, path: &str) -> Result<Value> {
        self.get_path_cached(obj, path)
    }
    
    fn set_path(&self, obj: &mut Obj, path: &str, value: Value) -> Result<()> {
        // La set_path invalida la cache (modifica l'oggetto)
        self.clear_cache();
        self.inner.set_path(obj, path, value)
    }
}

// ============================================
// HELPER: CONVERSIONI DA/FROM JSON
// ============================================

/// Converte un Obj in JSON string
pub fn obj_to_json(obj: &Obj) -> Result<String> {
    serde_json::to_string_pretty(obj)
        .map_err(|e| Error::new(&format!("Failed to serialize Obj to JSON: {}", e)))
}

/// Converte JSON string in Obj
pub fn json_to_obj(json_str: &str) -> Result<Obj> {
    let value: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| Error::new(&format!("Failed to parse JSON: {}", e)))?;
    
    // Converte serde_json::Value in sapri_obj::Value
    fn convert_json_value(v: serde_json::Value) -> Value {
        match v {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    Value::Number(f)
                } else {
                    Value::Null
                }
            }
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(arr) => {
                Value::Array(arr.into_iter().map(convert_json_value).collect())
            }
            serde_json::Value::Object(map) => {
                let mut obj = Obj::new();
                for (k, v) in map {
                    obj.set(&k, convert_json_value(v));
                }
                Value::Object(obj)
            }
        }
    }
    
    match convert_json_value(value) {
        Value::Object(obj) => Ok(obj),
        _ => Err(Error::new("JSON root must be an object")),
    }
}

// ============================================
// TEST
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    use core::obj::obj;
    
    #[test]
    fn test_from_flat_dict() {
        let adapter = ObjAdapterImpl::new();
        
        let mut flat = HashMap::new();
        flat.insert("user.name".to_string(), Value::String("Alice".to_string()));
        flat.insert("user.age".to_string(), Value::Number(30.0));
        flat.insert("user.address.city".to_string(), Value::String("Wonderland".to_string()));
        
        let obj = adapter.from_flat_dict(&flat);
        
        assert_eq!(obj.get("user.name").as_string(), Some("Alice"));
        assert_eq!(obj.get("user.age").as_f64(), Some(30.0));
        assert_eq!(obj.get("user.address.city").as_string(), Some("Wonderland"));
    }
    
    #[test]
    fn test_to_flat_dict() {
        let adapter = ObjAdapterImpl::new();
        
        let obj = obj! {
            user: {
                name: "Alice",
                age: 30,
                address: {
                    city: "Wonderland"
                }
            }
        };
        
        let flat = adapter.to_flat_dict(&obj);
        
        assert_eq!(flat.get("user.name").unwrap().as_string(), Some("Alice"));
        assert_eq!(flat.get("user.age").unwrap().as_f64(), Some(30.0));
        assert_eq!(flat.get("user.address.city").unwrap().as_string(), Some("Wonderland"));
    }
    
    #[test]
    fn test_get_path() {
        let adapter = ObjAdapterImpl::new();
        
        let obj = obj! {
            user: {
                name: "Alice",
                address: {
                    city: "Wonderland",
                    zip: 12345
                }
            }
        };
        
        assert_eq!(adapter.get_path(&obj, "user.name").unwrap().as_string(), Some("Alice"));
        assert_eq!(adapter.get_path(&obj, "user.address.city").unwrap().as_string(), Some("Wonderland"));
        assert_eq!(adapter.get_path(&obj, "user.address.zip").unwrap().as_f64(), Some(12345.0));
        
        // Path non esistente
        assert!(adapter.get_path(&obj, "user.nonexistent").is_err());
    }
    
    #[test]
    fn test_set_path() {
        let adapter = ObjAdapterImpl::new();
        
        let mut obj = Obj::new();
        
        // Imposta path annidato
        adapter.set_path(&mut obj, "user.name", Value::String("Alice".to_string())).unwrap();
        adapter.set_path(&mut obj, "user.address.city", Value::String("Wonderland".to_string())).unwrap();
        
        assert_eq!(obj.get("user.name").as_string(), Some("Alice"));
        assert_eq!(obj.get("user.address.city").as_string(), Some("Wonderland"));
    }
    
    #[test]
    fn test_array_support() {
        let adapter = ObjAdapterImpl::new();
        
        let obj = obj! {
            users: [
                { name: "Alice" },
                { name: "Bob" }
            ]
        };
        
        // Accesso con notazione array
        assert_eq!(adapter.get_path(&obj, "users[0].name").unwrap().as_string(), Some("Alice"));
        assert_eq!(adapter.get_path(&obj, "users[1].name").unwrap().as_string(), Some("Bob"));
    }
    
    #[test]
    fn test_split_path() {
        let adapter = ObjAdapterImpl::new();
        
        let parts = adapter.split_path("user.address.city");
        assert_eq!(parts, vec!["user", "address", "city"]);
        
        let parts2 = adapter.split_path("users[0].name");
        assert_eq!(parts2, vec!["users", "0", "name"]);
        
        let parts3 = adapter.split_path("nested.array[2].field");
        assert_eq!(parts3, vec!["nested", "array", "2", "field"]);
    }
    
    #[test]
    fn test_cached_adapter() {
        let adapter = CachedObjAdapter::new();
        
        let obj = obj! {
            test: {
                value: 42
            }
        };
        
        // Prima chiamata (cache miss)
        let val1 = adapter.get_path(&obj, "test.value").unwrap();
        assert_eq!(val1.as_f64(), Some(42.0));
        
        // Seconda chiamata (cache hit)
        let val2 = adapter.get_path(&obj, "test.value").unwrap();
        assert_eq!(val2.as_f64(), Some(42.0));
    }
    
    #[test]
    fn test_roundtrip() {
        let adapter = ObjAdapterImpl::new();
        
        let original = obj! {
            user: {
                name: "Alice",
                age: 30,
                active: true,
                tags: ["admin", "user"]
            }
        };
        
        // Converti in flat
        let flat = adapter.to_flat_dict(&original);
        
        // Riconverti in nested
        let reconstructed = adapter.from_flat_dict(&flat);
        
        // Verifica che siano uguali
        assert_eq!(original.get("user.name").as_string(), reconstructed.get("user.name").as_string());
        assert_eq!(original.get("user.age").as_f64(), reconstructed.get("user.age").as_f64());
        assert_eq!(original.get("user.active").as_bool(), reconstructed.get("user.active").as_bool());
    }
    
    #[test]
    fn test_json_conversion() {
        let original = obj! {
            name: "Alice",
            age: 30,
            active: true
        };
        
        let json_str = obj_to_json(&original).unwrap();
        let reconstructed = json_to_obj(&json_str).unwrap();
        
        assert_eq!(original.get("name").as_string(), reconstructed.get("name").as_string());
        assert_eq!(original.get("age").as_f64(), reconstructed.get("age").as_f64());
        assert_eq!(original.get("active").as_bool(), reconstructed.get("active").as_bool());
    }
}
