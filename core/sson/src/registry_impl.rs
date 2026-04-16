// ============================================
// core/sson/src/registry_impl.rs
// Implementazione del trait FunctionRegistry
// Validatori chiamabili per nome (es. "req", "min", "pattern")
// ============================================

use crate::*;
use core::obj::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::fmt;
use regex::Regex;
use once_cell::sync::Lazy;

// ============================================
// TIPO FUNZIONE VALIDATORE
// ============================================

/// Tipo delle funzioni validatrici registrate
/// Prendono contesto e constraint, restituiscono bool
pub type ValidatorFn = Box<dyn Fn(&ValidationContext, &Constraint) -> bool + Send + Sync>;

/// Tipo delle funzioni di trasformazione (per post-processing)
pub type TransformFn = Box<dyn Fn(&mut ValidationContext, &Constraint) -> Result<()> + Send + Sync>;

// ============================================
// FUNCTION REGISTRY IMPLEMENTATION
// ============================================

/// Registry di funzioni validatrici
/// Thread-safe, con cache dei risultati opzionale
#[derive(Debug, Clone)]
pub struct FunctionRegistryImpl {
    /// Validatori registrati (nome → funzione)
    validators: Arc<RwLock<HashMap<String, ValidatorFn>>>,
    
    /// Trasformazioni registrate (nome → funzione)
    transforms: Arc<RwLock<HashMap<String, TransformFn>>>,
    
    /// Cache dei risultati di validazione (opzionale)
    /// Formato: (path, constraint_name, params_hash) → result
    result_cache: Arc<RwLock<HashMap<(String, String, u64), bool>>>,
    
    /// Statistiche
    stats: Arc<RwLock<RegistryStats>>,
}

/// Statistiche del registry
#[derive(Debug, Clone, Default)]
pub struct RegistryStats {
    pub total_calls: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub registered_validators: usize,
    pub registered_transforms: usize,
}

impl FunctionRegistryImpl {
    /// Crea un nuovo registry vuoto
    pub fn new() -> Self {
        Self {
            validators: Arc::new(RwLock::new(HashMap::new())),
            transforms: Arc::new(RwLock::new(HashMap::new())),
            result_cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(RegistryStats::default())),
        }
    }
    
    /// Crea un registry con i validatori built-in pre-registrati
    pub fn with_builtins() -> Self {
        let registry = Self::new();
        registry.register_builtins();
        registry
    }
    
    /// Registra i validatori built-in
    fn register_builtins(&self) {
        // req: campo obbligatorio
        self.register("req", Box::new(|ctx, constraint| {
            ctx.obj.get(&constraint.target).is_some()
        }));
        
        // min: valore minimo numerico
        self.register("min", Box::new(|ctx, constraint| {
            let value = ctx.obj.get(&constraint.target);
            let min = constraint.params.get("value").as_f64()
                .or_else(|| constraint.params.get("min").as_f64())
                .unwrap_or(f64::MIN);
            value.as_f64().map_or(false, |v| v >= min)
        }));
        
        // max: valore massimo numerico
        self.register("max", Box::new(|ctx, constraint| {
            let value = ctx.obj.get(&constraint.target);
            let max = constraint.params.get("value").as_f64()
                .or_else(|| constraint.params.get("max").as_f64())
                .unwrap_or(f64::MAX);
            value.as_f64().map_or(false, |v| v <= max)
        }));
        
        // pattern: regex
        self.register("pattern", Box::new(|ctx, constraint| {
            let value = ctx.obj.get(&constraint.target).as_string().unwrap_or("");
            let pattern = constraint.params.get("regex").as_string()
                .or_else(|| constraint.params.get("pattern").as_string())
                .unwrap_or("");
            
            // Regex con cache thread-local per performance
            thread_local! {
                static REGEX_CACHE: std::cell::RefCell<HashMap<String, Regex>> = 
                    std::cell::RefCell::new(HashMap::new());
            }
            
            REGEX_CACHE.with(|cache| {
                let mut cache = cache.borrow_mut();
                let regex = cache.entry(pattern.to_string())
                    .or_insert_with(|| Regex::new(pattern).unwrap_or_else(|_| Regex::new("^$").unwrap()));
                regex.is_match(value)
            })
        }));
        
        // enum: valore in lista consentita
        self.register("enum", Box::new(|ctx, constraint| {
            let value = ctx.obj.get(&constraint.target);
            let allowed = constraint.params.get("values").as_array()
                .or_else(|| constraint.params.get("enum").as_array());
            
            allowed.map_or(false, |arr| {
                arr.iter().any(|v| v == &value)
            })
        }));
        
        // range: intervallo [min, max]
        self.register("range", Box::new(|ctx, constraint| {
            let value = ctx.obj.get(&constraint.target).as_f64();
            let min = constraint.params.get("min").as_f64().unwrap_or(f64::MIN);
            let max = constraint.params.get("max").as_f64().unwrap_or(f64::MAX);
            value.map_or(false, |v| v >= min && v <= max)
        }));
        
        // mutex: esclusione mutua (esattamente uno attivo)
        self.register("mutex", Box::new(|ctx, constraint| {
            let fields = constraint.params.get("fields").as_array();
            fields.map_or(false, |arr| {
                let active_count: usize = arr.iter()
                    .filter_map(|f| f.as_string())
                    .filter(|field| ctx.obj.get(field).is_some())
                    .count();
                active_count == 1
            })
        }));
        
        // at_least_one: almeno uno attivo
        self.register("at_least_one", Box::new(|ctx, constraint| {
            let fields = constraint.params.get("fields").as_array();
            fields.map_or(false, |arr| {
                arr.iter()
                    .filter_map(|f| f.as_string())
                    .any(|field| ctx.obj.get(field).is_some())
            })
        }));
        
        // exactly: conteggio esatto di campi attivi
        self.register("exactly", Box::new(|ctx, constraint| {
            let fields = constraint.params.get("fields").as_array();
            let count = constraint.params.get("count").as_i64().unwrap_or(0) as usize;
            fields.map_or(false, |arr| {
                let active_count = arr.iter()
                    .filter_map(|f| f.as_string())
                    .filter(|field| ctx.obj.get(field).is_some())
                    .count();
                active_count == count
            })
        }));
        
        // guard: condizione booleana
        self.register("guard", Box::new(|ctx, constraint| {
            let field = constraint.params.get("field").as_string();
            let expected = constraint.params.get("value");
            
            field.map_or(false, |f| {
                ctx.obj.get(f).as_ref() == expected.as_ref()
            })
        }));
        
        // sum: somma di campi
        self.register("sum", Box::new(|ctx, constraint| {
            let fields = constraint.params.get("fields").as_array();
            let target = constraint.params.get("target").as_f64()
                .or_else(|| constraint.params.get("value").as_f64());
            
            if let (Some(arr), Some(target_val)) = (fields, target) {
                let sum: f64 = arr.iter()
                    .filter_map(|f| f.as_string())
                    .filter_map(|field| ctx.obj.get(field).as_f64())
                    .sum();
                (sum - target_val).abs() < 1e-9
            } else {
                false
            }
        }));
        
        // compare: confronto tra campi
        self.register("compare", Box::new(|ctx, constraint| {
            let field1 = constraint.params.get("field1").as_string();
            let field2 = constraint.params.get("field2").as_string();
            let op = constraint.params.get("op").as_string().unwrap_or("eq");
            
            match (field1, field2) {
                (Some(f1), Some(f2)) => {
                    let v1 = ctx.obj.get(f1);
                    let v2 = ctx.obj.get(f2);
                    match op {
                        "eq" => v1 == v2,
                        "ne" => v1 != v2,
                        "lt" => v1.as_f64() < v2.as_f64(),
                        "le" => v1.as_f64() <= v2.as_f64(),
                        "gt" => v1.as_f64() > v2.as_f64(),
                        "ge" => v1.as_f64() >= v2.as_f64(),
                        _ => false,
                    }
                }
                _ => false,
            }
        }));
        
        // type_check: verifica tipo del valore
        self.register("type", Box::new(|ctx, constraint| {
            let field = constraint.params.get("field").as_string();
            let expected_type = constraint.params.get("expected").as_string();
            
            match (field, expected_type) {
                (Some(f), Some(exp)) => {
                    let value = ctx.obj.get(f);
                    match exp {
                        "null" => value.is_null(),
                        "bool" => value.is_bool(),
                        "number" => value.is_number(),
                        "string" => value.is_string(),
                        "array" => value.is_array(),
                        "object" => value.is_object(),
                        _ => false,
                    }
                }
                _ => false,
            }
        }));
        
        // length: lunghezza di stringa o array
        self.register("length", Box::new(|ctx, constraint| {
            let field = constraint.params.get("field").as_string();
            let min = constraint.params.get("min").as_i64();
            let max = constraint.params.get("max").as_i64();
            let exactly = constraint.params.get("exactly").as_i64();
            
            if let Some(f) = field {
                let value = ctx.obj.get(f);
                let len = if value.is_string() {
                    value.as_string().unwrap_or("").len() as i64
                } else if value.is_array() {
                    value.as_array().unwrap_or(&vec![]).len() as i64
                } else {
                    return false;
                };
                
                if let Some(ex) = exactly {
                    return len == ex;
                }
                
                let mut ok = true;
                if let Some(m) = min { ok = ok && len >= m; }
                if let Some(m) = max { ok = ok && len <= m; }
                ok
            } else {
                false
            }
        }));
    }
    
    /// Calcola un hash dei parametri per la cache
    fn params_hash(&self, constraint: &Constraint) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        constraint.name.hash(&mut hasher);
        constraint.target.hash(&mut hasher);
        // Serializza params in modo stabile per l'hash
        if let Ok(json) = serde_json::to_string(&constraint.params) {
            json.hash(&mut hasher);
        }
        hasher.finish()
    }
    
    /// Aggiorna statistiche
    fn update_stats(&self, is_cache_hit: bool) {
        let mut stats = self.stats.write().unwrap();
        stats.total_calls += 1;
        if is_cache_hit {
            stats.cache_hits += 1;
        } else {
            stats.cache_misses += 1;
        }
    }
    
    /// Ottieni statistiche
    pub fn get_stats(&self) -> RegistryStats {
        self.stats.read().unwrap().clone()
    }
    
    /// Pulisci la cache dei risultati
    pub fn clear_cache(&self) {
        self.result_cache.write().unwrap().clear();
    }
    
    /// Resetta le statistiche
    pub fn reset_stats(&self) {
        let mut stats = self.stats.write().unwrap();
        *stats = RegistryStats::default();
        let mut cache = self.result_cache.write().unwrap();
        cache.clear();
        
        // Re-count validators
        let validators_len = self.validators.read().unwrap().len();
        let transforms_len = self.transforms.read().unwrap().len();
        stats.registered_validators = validators_len;
        stats.registered_transforms = transforms_len;
    }
}

impl Default for FunctionRegistryImpl {
    fn default() -> Self {
        Self::with_builtins()
    }
}

impl FunctionRegistry for FunctionRegistryImpl {
    fn register(&mut self, name: &str, f: ValidatorFn) {
        let mut validators = self.validators.write().unwrap();
        validators.insert(name.to_string(), f);
        
        let mut stats = self.stats.write().unwrap();
        stats.registered_validators = validators.len();
    }
    
    fn call(&self, name: &str, ctx: &ValidationContext, constraint: &Constraint) -> Option<bool> {
        // Genera chiave cache
        let params_hash = self.params_hash(constraint);
        let cache_key = (constraint.target.clone(), name.to_string(), params_hash);
        
        // Controlla cache
        {
            let cache = self.result_cache.read().unwrap();
            if let Some(&result) = cache.get(&cache_key) {
                self.update_stats(true);
                return Some(result);
            }
        }
        
        self.update_stats(false);
        
        // Cerca il validator
        let validators = self.validators.read().unwrap();
        if let Some(validator) = validators.get(name) {
            let result = validator(ctx, constraint);
            
            // Salva in cache
            let mut cache = self.result_cache.write().unwrap();
            cache.insert(cache_key, result);
            
            Some(result)
        } else {
            None
        }
    }
    
    fn contains(&self, name: &str) -> bool {
        self.validators.read().unwrap().contains_key(name)
    }
}

// ============================================
// EXTENSION: REGISTRY PER TRASFORMAZIONI
// ============================================

impl FunctionRegistryImpl {
    /// Registra una funzione di trasformazione
    pub fn register_transform(&mut self, name: &str, f: TransformFn) {
        let mut transforms = self.transforms.write().unwrap();
        transforms.insert(name.to_string(), f);
        
        let mut stats = self.stats.write().unwrap();
        stats.registered_transforms = transforms.len();
    }
    
    /// Chiama una trasformazione registrata
    pub fn call_transform(&self, name: &str, ctx: &mut ValidationContext, constraint: &Constraint) -> Result<()> {
        let transforms = self.transforms.read().unwrap();
        if let Some(transform) = transforms.get(name) {
            transform(ctx, constraint)
        } else {
            Err(Error::new(&format!("Transform '{}' not found", name)))
        }
    }
}

// ============================================
// BUILT-IN TRASFORMAZIONI
// ============================================

impl FunctionRegistryImpl {
    /// Registra le trasformazioni built-in
    pub fn register_builtin_transforms(&self) {
        // default: imposta valore di default se campo assente
        let mut registry = self.clone();
        registry.register_transform("default", Box::new(|ctx, constraint| {
            let field = &constraint.target;
            if !ctx.obj.get(field).is_some() {
                let default_value = constraint.params.get("value").clone();
                ctx.obj.set_path(field, default_value)?;
            }
            Ok(())
        }));
        
        // coerce: forza un tipo
        registry.register_transform("coerce", Box::new(|ctx, constraint| {
            let field = &constraint.target;
            let target_type = constraint.params.get("type").as_string().unwrap_or("string");
            let value = ctx.obj.get(field);
            
            let converted = match target_type {
                "string" => Value::String(value.to_string()),
                "number" => Value::Number(value.as_f64().unwrap_or(0.0)),
                "bool" => Value::Bool(value.as_bool().unwrap_or(false)),
                _ => value,
            };
            
            ctx.obj.set_path(field, converted)?;
            Ok(())
        }));
        
        // trim: rimuove spazi da stringa
        registry.register_transform("trim", Box::new(|ctx, constraint| {
            let field = &constraint.target;
            if let Some(Value::String(s)) = ctx.obj.get(field) {
                ctx.obj.set_path(field, Value::String(s.trim().to_string()))?;
            }
            Ok(())
        }));
        
        // lowercase: converte in minuscolo
        registry.register_transform("lowercase", Box::new(|ctx, constraint| {
            let field = &constraint.target;
            if let Some(Value::String(s)) = ctx.obj.get(field) {
                ctx.obj.set_path(field, Value::String(s.to_lowercase()))?;
            }
            Ok(())
        }));
        
        // uppercase: converte in maiuscolo
        registry.register_transform("uppercase", Box::new(|ctx, constraint| {
            let field = &constraint.target;
            if let Some(Value::String(s)) = ctx.obj.get(field) {
                ctx.obj.set_path(field, Value::String(s.to_uppercase()))?;
            }
            Ok(())
        }));
    }
}

// ============================================
// LAZY SINGLETON (opzionale)
// ============================================

/// Registry globale (opzionale, per comodità)
pub static GLOBAL_REGISTRY: Lazy<FunctionRegistryImpl> = Lazy::new(|| {
    FunctionRegistryImpl::with_builtins()
});

/// Ottieni il registry globale
pub fn global_registry() -> &'static FunctionRegistryImpl {
    &GLOBAL_REGISTRY
}

// ============================================
// TEST
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    use core::obj::obj;
    
    fn create_test_constraint(name: &str, target: &str, params: Obj) -> Constraint {
        Constraint {
            name: name.to_string(),
            target: target.to_string(),
            params,
        }
    }
    
    #[test]
    fn test_registry_req() {
        let registry = FunctionRegistryImpl::with_builtins();
        
        let obj = obj! {
            name: "Alice"
        };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        let constraint = create_test_constraint("req", "name", Obj::new());
        
        assert_eq!(registry.call("req", &ctx, &constraint), Some(true));
        
        let obj2 = Obj::new();
        let ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
        assert_eq!(registry.call("req", &ctx2, &constraint), Some(false));
    }
    
    #[test]
    fn test_registry_min() {
        let registry = FunctionRegistryImpl::with_builtins();
        
        let obj = obj! {
            age: 25
        };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let mut params = Obj::new();
        params.set("value", Value::Number(18.0));
        let constraint = create_test_constraint("min", "age", params);
        
        assert_eq!(registry.call("min", &ctx, &constraint), Some(true));
        
        let mut params2 = Obj::new();
        params2.set("value", Value::Number(30.0));
        let constraint2 = create_test_constraint("min", "age", params2);
        
        assert_eq!(registry.call("min", &ctx, &constraint2), Some(false));
    }
    
    #[test]
    fn test_registry_pattern() {
        let registry = FunctionRegistryImpl::with_builtins();
        
        let obj = obj! {
            email: "alice@example.com"
        };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let mut params = Obj::new();
        params.set("regex", Value::String(r"^[^@]+@[^@]+\.[^@]+$".to_string()));
        let constraint = create_test_constraint("pattern", "email", params);
        
        assert_eq!(registry.call("pattern", &ctx, &constraint), Some(true));
        
        let obj2 = obj! {
            email: "not-an-email"
        };
        let ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
        
        assert_eq!(registry.call("pattern", &ctx2, &constraint), Some(false));
    }
    
    #[test]
    fn test_registry_mutex() {
        let registry = FunctionRegistryImpl::with_builtins();
        
        let obj = obj! {
            a: 10
        };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let mut params = Obj::new();
        params.set("fields", Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]));
        let constraint = create_test_constraint("mutex", "", params);
        
        assert_eq!(registry.call("mutex", &ctx, &constraint), Some(true));
        
        let obj2 = obj! {
            a: 10,
            b: 20
        };
        let ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
        
        assert_eq!(registry.call("mutex", &ctx2, &constraint), Some(false));
    }
    
    #[test]
    fn test_registry_cache() {
        let registry = FunctionRegistryImpl::with_builtins();
        
        let obj = obj! {
            name: "Alice"
        };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        let constraint = create_test_constraint("req", "name", Obj::new());
        
        // Prima chiamata (cache miss)
        let result1 = registry.call("req", &ctx, &constraint);
        assert_eq!(result1, Some(true));
        
        let stats = registry.get_stats();
        assert_eq!(stats.cache_misses, 1);
        assert_eq!(stats.cache_hits, 0);
        
        // Seconda chiamata (cache hit)
        let result2 = registry.call("req", &ctx, &constraint);
        assert_eq!(result2, Some(true));
        
        let stats2 = registry.get_stats();
        assert_eq!(stats2.cache_misses, 1);
        assert_eq!(stats2.cache_hits, 1);
    }
    
    #[test]
    fn test_registry_contains() {
        let registry = FunctionRegistryImpl::with_builtins();
        
        assert!(registry.contains("req"));
        assert!(registry.contains("min"));
        assert!(registry.contains("max"));
        assert!(registry.contains("pattern"));
        assert!(!registry.contains("non_existent"));
    }
    
    #[test]
    fn test_registry_custom_validator() {
        let mut registry = FunctionRegistryImpl::new();
        
        // Registra un validator personalizzato
        registry.register("is_even", Box::new(|ctx, constraint| {
            ctx.obj.get(&constraint.target).as_f64().map_or(false, |v| v % 2.0 == 0.0)
        }));
        
        let obj = obj! {
            value: 42
        };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        let constraint = create_test_constraint("is_even", "value", Obj::new());
        
        assert_eq!(registry.call("is_even", &ctx, &constraint), Some(true));
        
        let obj2 = obj! {
            value: 43
        };
        let ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
        
        assert_eq!(registry.call("is_even", &ctx2, &constraint), Some(false));
    }
    
    #[test]
    fn test_registry_sum() {
        let registry = FunctionRegistryImpl::with_builtins();
        
        let obj = obj! {
            a: 10,
            b: 20
        };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let mut params = Obj::new();
        params.set("fields", Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]));
        params.set("target", Value::Number(30.0));
        let constraint = create_test_constraint("sum", "", params);
        
        assert_eq!(registry.call("sum", &ctx, &constraint), Some(true));
        
        let mut params2 = Obj::new();
        params2.set("fields", Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]));
        params2.set("target", Value::Number(40.0));
        let constraint2 = create_test_constraint("sum", "", params2);
        
        assert_eq!(registry.call("sum", &ctx, &constraint2), Some(false));
    }
    
    #[test]
    fn test_registry_type_check() {
        let registry = FunctionRegistryImpl::with_builtins();
        
        let obj = obj! {
            name: "Alice",
            age: 30
        };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let mut params = Obj::new();
        params.set("field", Value::String("name".to_string()));
        params.set("expected", Value::String("string".to_string()));
        let constraint = create_test_constraint("type", "", params);
        
        assert_eq!(registry.call("type", &ctx, &constraint), Some(true));
        
        let mut params2 = Obj::new();
        params2.set("field", Value::String("age".to_string()));
        params2.set("expected", Value::String("string".to_string()));
        let constraint2 = create_test_constraint("type", "", params2);
        
        assert_eq!(registry.call("type", &ctx, &constraint2), Some(false));
    }
}
