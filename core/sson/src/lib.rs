//! SSON Parser - Validatori per formato .sson

use std::collections::HashMap;
use sapri_obj::{Obj, Value};
use regex::Regex;

// ============================================
// TIPI BASE
// ============================================

pub type SScore = f64;
pub type FieldPath = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParserMode {
    #[default]
    Strict,
    Generative,
}

#[derive(Debug, Clone)]
pub struct ValidationContext {
    pub mode: ParserMode,
    pub obj: Obj,
    pub s_score: SScore,
}

impl ValidationContext {
    pub fn new(mode: ParserMode, obj: Obj) -> Self {
        Self { mode, obj, s_score: 1.0 }
    }
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub name: String,
    pub target: String,
    pub params: Obj,
}

// ============================================
// HELPER PER ESTRARRE VALORI DA Option<&Value>
// ============================================

fn opt_as_number(opt: Option<&Value>) -> Option<f64> {
    opt.and_then(|v| v.as_number())
}

fn opt_as_str(opt: Option<&Value>) -> Option<&str> {
    opt.and_then(|v| v.as_str())
}

// opt_as_array non usata per ora, commentata
// fn opt_as_array(opt: Option<&Value>) -> Option<Vec<Value>> {
//     opt.and_then(|v| {
//         if let Some(obj) = v.as_obj() {
//             let mut arr = Vec::new();
//             let mut i = 0;
//             while let Some(val) = obj.get(&i.to_string()) {
//                 arr.push(val.clone());
//                 i += 1;
//             }
//             if !arr.is_empty() {
//                 Some(arr)
//             } else {
//                 None
//             }
//         } else {
//             None
//         }
//     })
// }

// ============================================
// VALIDATORE CON REGISTRY INTERNO
// ============================================

type ValidatorFn = Box<dyn Fn(&ValidationContext, &Constraint) -> bool + Send + Sync>;

pub struct Validator {
    validators: HashMap<String, ValidatorFn>,
}

impl Validator {
    pub fn new() -> Self {
        let mut v = Self {
            validators: HashMap::new(),
        };
        v.register_builtins();
        v
    }
    
    fn register(&mut self, name: &str, f: ValidatorFn) {
        self.validators.insert(name.to_string(), f);
    }
    
    fn register_builtins(&mut self) {
        // 1. req: campo obbligatorio
        self.register("req", Box::new(|ctx, c| {
            ctx.obj.get(&c.target).is_some()
        }));
        
        // 2. min: valore minimo numerico
        self.register("min", Box::new(|ctx, c| {
            let val = ctx.obj.get(&c.target);
            let min = opt_as_number(c.params.get("value"))
                .or_else(|| opt_as_number(c.params.get("min")))
                .unwrap_or(f64::MIN);
            
            val.and_then(|v| v.as_number()).map_or(false, |v| v >= min)
        }));
        
        // 3. max: valore massimo numerico
        self.register("max", Box::new(|ctx, c| {
            let val = ctx.obj.get(&c.target);
            let max = opt_as_number(c.params.get("value"))
                .or_else(|| opt_as_number(c.params.get("max")))
                .unwrap_or(f64::MAX);
            
            val.and_then(|v| v.as_number()).map_or(false, |v| v <= max)
        }));
        
        // 4. pattern: espressione regolare
        self.register("pattern", Box::new(|ctx, c| {
            let val = ctx.obj.get(&c.target);
            let pattern = opt_as_str(c.params.get("regex"))
                .or_else(|| opt_as_str(c.params.get("pattern")))
                .unwrap_or("");
            
            if pattern.is_empty() {
                return true;
            }
            
            let text = val.and_then(|v| v.as_str()).unwrap_or("");
            
            thread_local! {
                static REGEX_CACHE: std::cell::RefCell<HashMap<String, Regex>> = 
                    std::cell::RefCell::new(HashMap::new());
            }
            
            REGEX_CACHE.with(|cache| {
                let mut cache = cache.borrow_mut();
                let regex = cache.entry(pattern.to_string())
                    .or_insert_with(|| Regex::new(pattern).unwrap_or_else(|_| Regex::new("^$").unwrap()));
                regex.is_match(text)
            })
        }));
        
        // 5. enum: valore in lista consentita
        self.register("enum", Box::new(|ctx, c| {
            let val = ctx.obj.get(&c.target);
            let allowed = c.params.get("values")
                .and_then(|v| v.as_obj())
                .or_else(|| c.params.get("enum").and_then(|v| v.as_obj()));
            
            if let (Some(val), Some(allowed_obj)) = (val, allowed) {
                let mut i = 0;
                while let Some(v) = allowed_obj.get(&i.to_string()) {
                    if v == val {
                        return true;
                    }
                    i += 1;
                }
            }
            false
        }));
        
        // 6. mutex: esattamente uno dei campi presente
        self.register("mutex", Box::new(|ctx, c| {
            let fields = c.params.get("fields")
                .and_then(|v| v.as_obj());
            
            fields.map_or(false, |fields_obj| {
                let mut active_count = 0;
                let mut i = 0;
                while let Some(field_val) = fields_obj.get(&i.to_string()) {
                    if let Some(field_name) = field_val.as_str() {
                        if ctx.obj.get(field_name).is_some() {
                            active_count += 1;
                        }
                    }
                    i += 1;
                }
                active_count == 1
            })
        }));
        
        // 7. at_least_one: almeno uno dei campi presente
        self.register("at_least_one", Box::new(|ctx, c| {
            let fields = c.params.get("fields")
                .and_then(|v| v.as_obj());
            
            fields.map_or(false, |fields_obj| {
                let mut i = 0;
                while let Some(field_val) = fields_obj.get(&i.to_string()) {
                    if let Some(field_name) = field_val.as_str() {
                        if ctx.obj.get(field_name).is_some() {
                            return true;
                        }
                    }
                    i += 1;
                }
                false
            })
        }));
        
        // 8. exactly: conteggio esatto di campi presenti
        self.register("exactly", Box::new(|ctx, c| {
            let fields = c.params.get("fields")
                .and_then(|v| v.as_obj());
            let expected = opt_as_number(c.params.get("count"))
                .or_else(|| opt_as_number(c.params.get("exactly")))
                .unwrap_or(0.0) as usize;
            
            fields.map_or(false, |fields_obj| {
                let mut active_count = 0;
                let mut i = 0;
                while let Some(field_val) = fields_obj.get(&i.to_string()) {
                    if let Some(field_name) = field_val.as_str() {
                        if ctx.obj.get(field_name).is_some() {
                            active_count += 1;
                        }
                    }
                    i += 1;
                }
                active_count == expected
            })
        }));
        
        // 9. guard: condizione booleana su un campo
        self.register("guard", Box::new(|ctx, c| {
            let field = opt_as_str(c.params.get("field"));
            let expected = c.params.get("value");
            
            if let (Some(field), Some(expected)) = (field, expected) {
                if let Some(val) = ctx.obj.get(field) {
                    return val == expected;
                }
            }
            false
        }));
        
        // 10. sum: somma di campi
        self.register("sum", Box::new(|ctx, c| {
            let fields = c.params.get("fields")
                .and_then(|v| v.as_obj());
            let target = opt_as_number(c.params.get("target"))
                .or_else(|| opt_as_number(c.params.get("value")))
                .unwrap_or(0.0);
            
            fields.map_or(false, |fields_obj| {
                let mut sum = 0.0;
                let mut i = 0;
                while let Some(field_val) = fields_obj.get(&i.to_string()) {
                    if let Some(field_name) = field_val.as_str() {
                        if let Some(val) = ctx.obj.get(field_name) {
                            if let Some(num) = val.as_number() {
                                sum += num;
                            }
                        }
                    }
                    i += 1;
                }
                (sum - target).abs() < 1e-9
            })
        }));
        
        // 11. type: verifica tipo del valore
        self.register("type", Box::new(|ctx, c| {
            let field = opt_as_str(c.params.get("field"));
            let expected_type = opt_as_str(c.params.get("expected"))
                .or_else(|| opt_as_str(c.params.get("type")));
            
            if let (Some(field), Some(expected)) = (field, expected_type) {
                let val = ctx.obj.get(field);
                match expected {
                    "null" => val.map_or(true, |v| v.is_null()),
                    "bool" => val.map_or(false, |v| v.as_bool().is_some()),
                    "number" => val.map_or(false, |v| v.as_number().is_some()),
                    "string" => val.map_or(false, |v| v.as_str().is_some()),
                    "object" => val.map_or(false, |v| v.as_obj().is_some()),
                    _ => false,
                }
            } else {
                false
            }
        }));
        
        // 12. length: lunghezza di stringa
        self.register("length", Box::new(|ctx, c| {
            let field = opt_as_str(c.params.get("field"));
            let min = opt_as_number(c.params.get("min")).unwrap_or(0.0) as usize;
            let max = opt_as_number(c.params.get("max")).unwrap_or(usize::MAX as f64) as usize;
            let exactly = opt_as_number(c.params.get("exactly")).map(|n| n as usize);
            
            if let Some(field) = field {
                if let Some(val) = ctx.obj.get(field) {
                    if let Some(s) = val.as_str() {
                        let len = s.len();
                        if let Some(ex) = exactly {
                            return len == ex;
                        }
                        return len >= min && len <= max;
                    }
                }
            }
            false
        }));
    }
    
    pub fn validate(&self, ctx: &ValidationContext, constraint: &Constraint) -> bool {
        self.validators
            .get(&constraint.name)
            .map_or(false, |f| f(ctx, constraint))
    }
    
    pub fn validate_all(&self, ctx: &mut ValidationContext, constraints: &[Constraint]) -> SScore {
        let total = constraints.len();
        if total == 0 {
            ctx.s_score = 1.0;
            return 1.0;
        }
        
        let valid = constraints.iter()
            .filter(|c| self.validate(ctx, c))
            .count();
        
        let s = calculate_s_score(valid, total, ctx.mode);
        ctx.s_score = s;
        s
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// HELPERS
// ============================================

pub fn calculate_s_score(valid: usize, total: usize, mode: ParserMode) -> SScore {
    let k = match mode {
        ParserMode::Strict => 1.0,
        ParserMode::Generative => 1.5,
    };
    if total == 0 { return 1.0; }
    (valid as f64) / (total as f64 * k)
}

pub fn is_exportable(s_score: SScore) -> bool {
    s_score >= 0.9
}

// ============================================
// MACRO PER COSTRUIRE CONSTRAINT
// ============================================

#[macro_export]
macro_rules! constraint {
    ($name:expr, $target:expr) => {
        Constraint {
            name: $name.to_string(),
            target: $target.to_string(),
            params: Obj::new(),
        }
    };
    ($name:expr, $target:expr, $($key:ident : $value:expr),* $(,)?) => {{
        let mut params = Obj::new();
        $(params.set(stringify!($key), $value);)*
        Constraint {
            name: $name.to_string(),
            target: $target.to_string(),
            params,
        }
    }};
}
