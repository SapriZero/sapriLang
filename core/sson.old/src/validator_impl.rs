// ============================================
// core/sson/src/validator_impl.rs
// Implementazione del trait Validator
// ============================================

use crate::*;
use crate::registry_impl::FunctionRegistryImpl;
use core::obj::Value;
use core::base::fp::{compose, pipe, memoize};

use std::sync::Arc;
use std::collections::HashMap;
use regex::Regex;

// ============================================
// VALIDATORE BASE
// ============================================

/// Validatore che usa un FunctionRegistry per chiamare validatori per nome
#[derive(Debug, Clone)]
pub struct BaseValidator {
    registry: Arc<FunctionRegistryImpl>,
    cache: HashMap<(FieldPath, ConstraintName), bool>,  // Cache dei risultati
}

impl BaseValidator {
    /// Crea un nuovo validatore con registry predefinito
    pub fn new() -> Self {
        let mut registry = FunctionRegistryImpl::new();
        Self::register_builtin_validators(&mut registry);
        
        Self {
            registry: Arc::new(registry),
            cache: HashMap::new(),
        }
    }
    
    /// Crea un validatore con registry personalizzato
    pub fn with_registry(registry: FunctionRegistryImpl) -> Self {
        Self {
            registry: Arc::new(registry),
            cache: HashMap::new(),
        }
    }
    
    /// Registra i validatori built-in
    fn register_builtin_validators(registry: &mut FunctionRegistryImpl) {
        // req: campo obbligatorio
        registry.register("req", Box::new(|ctx, constraint| {
            ctx.obj.get(&constraint.target).is_some()
        }));
        
        // min: valore minimo numerico
        registry.register("min", Box::new(|ctx, constraint| {
            let value = ctx.obj.get(&constraint.target);
            let min = constraint.params.get("value").as_f64().unwrap_or(f64::MIN);
            value.as_f64().map_or(false, |v| v >= min)
        }));
        
        // max: valore massimo numerico
        registry.register("max", Box::new(|ctx, constraint| {
            let value = ctx.obj.get(&constraint.target);
            let max = constraint.params.get("value").as_f64().unwrap_or(f64::MAX);
            value.as_f64().map_or(false, |v| v <= max)
        }));
        
        // pattern: regex sulla stringa
        registry.register("pattern", Box::new(|ctx, constraint| {
            let value = ctx.obj.get(&constraint.target).as_string();
            let pattern = constraint.params.get("regex").as_string().unwrap_or("");
            
            // Memoizzazione della regex per performance
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
        registry.register("enum", Box::new(|ctx, constraint| {
            let value = ctx.obj.get(&constraint.target);
            let allowed = constraint.params.get("values").as_array();
            
            allowed.map_or(false, |arr| {
                arr.iter().any(|v| v == &value)
            })
        }));
        
        // range: intervallo [min, max]
        registry.register("range", Box::new(|ctx, constraint| {
            let value = ctx.obj.get(&constraint.target).as_f64();
            let min = constraint.params.get("min").as_f64().unwrap_or(f64::MIN);
            let max = constraint.params.get("max").as_f64().unwrap_or(f64::MAX);
            
            value.map_or(false, |v| v >= min && v <= max)
        }));
        
        // mutex: esclusione mutua tra campi
        registry.register("mutex", Box::new(|ctx, constraint| {
            let fields = constraint.params.get("fields").as_array();
            
            fields.map_or(false, |arr| {
                let active_count: usize = arr.iter()
                    .filter_map(|f| f.as_string())
                    .filter(|field| ctx.obj.get(field).is_some())
                    .count();
                active_count == 1
            })
        }));
        
        // guard: condizione booleana su un campo
        registry.register("guard", Box::new(|ctx, constraint| {
            let field = constraint.params.get("field").as_string();
            let expected = constraint.params.get("value");
            
            field.map_or(false, |f| {
                ctx.obj.get(f).as_ref() == expected.as_ref()
            })
        }));
        
        // sum: somma di campi uguale a un valore
        registry.register("sum", Box::new(|ctx, constraint| {
            let fields = constraint.params.get("fields").as_array();
            let target = constraint.params.get("target").as_f64();
            
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
    }
    
    /// Pulisce la cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    
    /// Valida un singolo constraint con caching
    fn validate_cached(&mut self, ctx: &ValidationContext, constraint: &Constraint) -> bool {
        let key = (constraint.target.clone(), constraint.name.clone());
        
        if let Some(&cached) = self.cache.get(&key) {
            return cached;
        }
        
        let result = self.registry.call(&constraint.name, ctx, constraint);
        self.cache.insert(key, result);
        result
    }
}

impl Default for BaseValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl Validator for BaseValidator {
    fn validate(&self, ctx: &ValidationContext, constraint: &Constraint) -> bool {
        // Usa il registry per chiamare il validator per nome
        self.registry.call(&constraint.name, ctx, constraint)
    }
    
    fn validate_all(&mut self, ctx: &mut ValidationContext, constraints: &[Constraint]) -> SScore {
        let total = constraints.len();
        if total == 0 {
            ctx.s_score = 1.0;
            return 1.0;
        }
        
        let mut valid_count = 0;
        
        for constraint in constraints {
            if self.validate_cached(ctx, constraint) {
                valid_count += 1;
            }
        }
        
        let s = calculate_s_score(valid_count, total, ctx.mode);
        ctx.s_score = s;
        s
    }
}

// ============================================
// VALIDATORE COMPOSTO (per pipeline)
// ============================================

/// Validatore che compone più validatori in sequenza
#[derive(Debug)]
pub struct CompositeValidator {
    validators: Vec<Box<dyn Validator + Send + Sync>>,
}

impl CompositeValidator {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }
    
    pub fn add<V: Validator + Send + Sync + 'static>(&mut self, validator: V) {
        self.validators.push(Box::new(validator));
    }
}

impl Default for CompositeValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl Validator for CompositeValidator {
    fn validate(&self, ctx: &ValidationContext, constraint: &Constraint) -> bool {
        // Tutti i validatori devono passare (AND logico)
        self.validators.iter().all(|v| v.validate(ctx, constraint))
    }
    
    fn validate_all(&mut self, ctx: &mut ValidationContext, constraints: &[Constraint]) -> SScore {
        let total = constraints.len();
        if total == 0 {
            ctx.s_score = 1.0;
            return 1.0;
        }
        
        // Applica tutti i validatori in sequenza
        let mut valid_count = 0;
        for constraint in constraints {
            let all_valid = self.validators.iter_mut().all(|v| v.validate(ctx, constraint));
            if all_valid {
                valid_count += 1;
            }
        }
        
        let s = calculate_s_score(valid_count, total, ctx.mode);
        ctx.s_score = s;
        s
    }
}

// ============================================
// VALIDATORE CON MEMOIZATION (per performance)
// ============================================

/// Validatore che memoizza i risultati per campo+constraint
#[derive(Debug)]
pub struct MemoizingValidator {
    inner: BaseValidator,
    memo: HashMap<(FieldPath, ConstraintName), bool>,
}

impl MemoizingValidator {
    pub fn new() -> Self {
        Self {
            inner: BaseValidator::new(),
            memo: HashMap::new(),
        }
    }
    
    pub fn with_registry(registry: FunctionRegistryImpl) -> Self {
        Self {
            inner: BaseValidator::with_registry(registry),
            memo: HashMap::new(),
        }
    }
    
    pub fn clear_memo(&mut self) {
        self.memo.clear();
        self.inner.clear_cache();
    }
}

impl Default for MemoizingValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl Validator for MemoizingValidator {
    fn validate(&self, ctx: &ValidationContext, constraint: &Constraint) -> bool {
        let key = (constraint.target.clone(), constraint.name.clone());
        
        if let Some(&cached) = self.memo.get(&key) {
            return cached;
        }
        
        // Nota: self è immutable, ma memo richiede mutabilità
        // In produzione si userebbe RefCell o DashMap
        let result = self.inner.validate(ctx, constraint);
        
        // Per ora restituiamo senza cache mutabile
        // TODO: usare RefCell per cache interna
        result
    }
}

// ============================================
// HELPER: VALIDA UN OGGETTO COMPLETO
// ============================================

/// Valida un intero oggetto contro uno schema di constraint
pub fn validate_object(
    validator: &mut impl Validator,
    obj: &mut Obj,
    constraints: &[Constraint],
    mode: ParserMode,
) -> Result<(bool, SScore)> {
    let mut ctx = ValidationContext::new(mode, obj.clone());
    let s = validator.validate_all(&mut ctx, constraints);
    let is_valid = s >= EXPORT_THRESHOLD;
    
    // Aggiorna l'oggetto con eventuali modifiche del contesto
    *obj = ctx.obj;
    
    Ok((is_valid, s))
}

// ============================================
// TEST
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    use core::obj::{obj, Obj};
    
    #[test]
    fn test_req_validator() {
        let validator = BaseValidator::new();
        
        let obj = obj! {
            name: "Alice"
        };
        
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let constraint = Constraint {
            name: "req".to_string(),
            target: "name".to_string(),
            params: Obj::new(),
        };
        
        assert!(validator.validate(&ctx, &constraint));
        
        let ctx2 = ValidationContext::new(ParserMode::Strict, Obj::new());
        assert!(!validator.validate(&ctx2, &constraint));
    }
    
    #[test]
    fn test_min_validator() {
        let validator = BaseValidator::new();
        
        let obj = obj! {
            age: 25
        };
        
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let mut params = Obj::new();
        params.set("value", Value::Number(18.0));
        
        let constraint = Constraint {
            name: "min".to_string(),
            target: "age".to_string(),
            params,
        };
        
        assert!(validator.validate(&ctx, &constraint));
        
        let mut params2 = Obj::new();
        params2.set("value", Value::Number(30.0));
        
        let constraint2 = Constraint {
            name: "min".to_string(),
            target: "age".to_string(),
            params: params2,
        };
        
        assert!(!validator.validate(&ctx, &constraint2));
    }
    
    #[test]
    fn test_pattern_validator() {
        let validator = BaseValidator::new();
        
        let obj = obj! {
            email: "alice@example.com"
        };
        
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let mut params = Obj::new();
        params.set("regex", Value::String(r"^[^@]+@[^@]+\.[^@]+$".to_string()));
        
        let constraint = Constraint {
            name: "pattern".to_string(),
            target: "email".to_string(),
            params,
        };
        
        assert!(validator.validate(&ctx, &constraint));
    }
    
    #[test]
    fn test_mutex_validator() {
        let validator = BaseValidator::new();
        
        let obj = obj! {
            a: 10
            // b non presente
        };
        
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let mut params = Obj::new();
        params.set("fields", Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]));
        
        let constraint = Constraint {
            name: "mutex".to_string(),
            target: "".to_string(),
            params,
        };
        
        // Solo a presente → OK (esattamente 1 attivo)
        assert!(validator.validate(&ctx, &constraint));
        
        let obj2 = obj! {
            a: 10,
            b: 20
        };
        
        let ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
        assert!(!validator.validate(&ctx2, &constraint));
    }
    
    #[test]
    fn test_sum_validator() {
        let validator = BaseValidator::new();
        
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
        
        let constraint = Constraint {
            name: "sum".to_string(),
            target: "".to_string(),
            params,
        };
        
        assert!(validator.validate(&ctx, &constraint));
    }
    
    #[test]
    fn test_calculate_s_score() {
        // Strict mode: k = 1.0
        let s = calculate_s_score(8, 10, ParserMode::Strict);
        assert_eq!(s, 0.8);
        
        // Generative mode: k = 1.5
        let s2 = calculate_s_score(8, 10, ParserMode::Generative);
        assert_eq!(s2, 0.5333333333333333);
        
        // Tutti validi
        let s3 = calculate_s_score(10, 10, ParserMode::Strict);
        assert_eq!(s3, 1.0);
        
        // Nessuno valido
        let s4 = calculate_s_score(0, 10, ParserMode::Strict);
        assert_eq!(s4, 0.0);
    }
    
    #[test]
    fn test_validate_all() {
        let mut validator = BaseValidator::new();
        let obj = Obj::new();
        let mut ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let constraints = vec![
            Constraint {
                name: "req".to_string(),
                target: "field1".to_string(),
                params: Obj::new(),
            },
            Constraint {
                name: "req".to_string(),
                target: "field2".to_string(),
                params: Obj::new(),
            },
        ];
        
        // Nessun campo presente → S = 0
        let s = validator.validate_all(&mut ctx, &constraints);
        assert_eq!(s, 0.0);
        
        // Aggiungi campi all'oggetto
        let mut obj2 = Obj::new();
        obj2.set("field1", Value::String("value".to_string()));
        obj2.set("field2", Value::String("value".to_string()));
        let mut ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
        
        let s2 = validator.validate_all(&mut ctx2, &constraints);
        assert_eq!(s2, 1.0);
    }
}
