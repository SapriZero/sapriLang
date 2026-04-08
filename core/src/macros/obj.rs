//! Macro per creare oggetti con path e valori
//! Versione consolidata: include path! e path_arr! inline

use serde_json::Value as JsonValue;
use std::collections::HashMap;

// ============================================================================
// STRUTTURA DATI: Obj
// ============================================================================

/// Oggetto URCM minimale per test e uso generico
/// 
/// Memorizza valori JSON in una mappa piatta con chiavi dot-separated.
#[derive(Debug, Clone, Default)]
pub struct Obj {
    /// Mappa interna: chiave "a.b.c" → valore JSON
    pub data: HashMap<String, JsonValue>,
}

impl Obj {
    /// Crea un nuovo oggetto vuoto
    pub fn new() -> Self { 
        Self::default() 
    }
    
    /// Imposta un valore a un path di segmenti (fluent style)
    /// 
    /// # Arguments
    /// * `path` - Slice di segmenti `&[&str]` (es. `&["user", "profile", "name"]`)
    /// * `value` - Qualsiasi tipo convertibile in `JsonValue`
    /// 
    /// # Returns
    /// L'oggetto modificato (per chaining)
    pub fn set(mut self, path: &[&str], value: impl Into<JsonValue>) -> Self {
        let key = path.join(".");
        self.data.insert(key, value.into());
        self
    }
    
    /// Ottiene un valore da un path di segmenti
    /// 
    /// # Arguments
    /// * `path` - Slice di segmenti `&[&str]`
    /// 
    /// # Returns
    /// `Some(&JsonValue)` se la chiave esiste, `None` altrimenti
    pub fn get(&self, path: &[&str]) -> Option<&JsonValue> {
        let key = path.join(".");
        self.data.get(&key)
    }
}

// ============================================================================
// MACRO: obj!
// ============================================================================

/// Crea un oggetto `Obj` con coppie chiave-valore.
/// 
/// # Sintassi
/// - `obj!({ key: value, ... })` — oggetto nuovo
/// - `obj!(default => { key: value, ... })` — estende un oggetto esistente
/// 
/// # Esempi
/// ```ignore
/// use sapri_core::macros::obj::Obj;
/// use sapri_core::obj;
///
/// // Oggetto semplice
/// let o = obj!({
///     count: 42,
///     name: "test",
///     active: true
/// });
///
/// // Con default
/// let base = obj!({ theme: "dark" });
/// let extended = obj!(base => { theme: "light", debug: true });
/// ```
#[macro_export]
macro_rules! obj {
    // Sintassi base: obj!({ key: value, ... })
    ({ $( $key:ident : $val:expr ),* $(,)? }) => {
        {
            let mut __obj = $crate::macros::obj::Obj::new();
            $(
                // Wrap singolo ident in slice per matchare la signature di set
                __obj = __obj.set(&[stringify!($key)], $val);
            )*
            __obj
        }
    };

    // Con default: obj!(default_obj => { key: value, ... })
    ($default:expr => { $( $key:ident : $val:expr ),* $(,)? }) => {
        {
            let mut __obj = $default.clone();
            $(
                __obj = __obj.set(&[stringify!($key)], $val);
            )*
            __obj
        }
    };
}

// ============================================================================
// MACRO: path_arr! (space-separated, robusta)
// ============================================================================

/// Converte identificatori space-separated in array di stringhe.
/// 
/// # Sintassi
/// - `path_arr!(a b c)` → `vec!["a", "b", "c"]` (consigliato)
/// - `path_arr!("stringa")` → `vec!["stringa"]` (letterale)
/// - `path_arr!(single)` → `vec!["single"]` (singolo ident)
/// 
/// # Esempi
/// ```ignore
/// use sapri_core::path_arr;
///
/// let p1 = path_arr!(user profile name);  // ["user", "profile", "name"]
/// let p2 = path_arr!(count);              // ["count"]
/// let p3 = path_arr!("dynamic.path");     // ["dynamic.path"]
/// ```
#[macro_export]
macro_rules! path_arr {
    // Stringa letterale
    ($s:literal) => { vec![$s] };
    
    // Space-separated identifiers: path_arr!(a b c) → ["a", "b", "c"]
    ($($part:ident)+) => {
        vec![$(stringify!($part)),+]
    };
}

// ============================================================================
// MACRO: path! (alias retrocompatibile)
// ============================================================================

/// Alias per `path_arr!` con sintassi alternativa.
/// 
/// # Esempi
/// ```ignore
/// use sapri_core::path;
///
/// let p1 = path!(user profile name);  // ["user", "profile", "name"]
/// let p2 = path!(count);              // ["count"]
/// let p3 = path!("dynamic.path");     // ["dynamic.path"]
/// ```
#[macro_export]
macro_rules! path {
    ($s:literal) => { $crate::path_arr!($s) };
    ($($part:ident)+) => { $crate::path_arr!($($part)+) };
}

// ============================================================================
// TEST UNITARI
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_path_arr_macro() {
        // Space-separated: path_arr!(a b c) → Vec<&str>
        let p = path_arr!(user profile name);
        assert_eq!(p, vec!["user", "profile", "name"]);
        
        let p = path_arr!(count);
        assert_eq!(p, vec!["count"]);
        
        let p = path_arr!("dynamic.path");
        assert_eq!(p, vec!["dynamic.path"]);
    }

    #[test]
    fn test_path_alias() {
        // L'alias path! deve comportarsi come path_arr!
        let p1 = path!(app config debug);
        let p2 = path_arr!(app config debug);
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_obj_simple() {
        let o = obj!({
            count: 42,
            name: "test",
            active: true
        });
        
        assert_eq!(o.get(&["count"]), Some(&json!(42)));
        assert_eq!(o.get(&["name"]), Some(&json!("test")));
        assert_eq!(o.get(&["active"]), Some(&json!(true)));
        assert_eq!(o.get(&["missing"]), None);
    }

    #[test]
    fn test_obj_with_default() {
        let base = obj!({ theme: "dark", lang: "en" });
        
        let extended = obj!(base => {
            theme: "light",  // override
            debug: true      // nuovo campo
        });
        
        assert_eq!(extended.get(&["theme"]), Some(&json!("light")));
        assert_eq!(extended.get(&["lang"]), Some(&json!("en")));
        assert_eq!(extended.get(&["debug"]), Some(&json!(true)));
    }

    #[test]
    fn test_obj_with_path_arr() {
        let o = Obj::new()
            .set(&path_arr!(user profile name), "Mario")
            .set(&path_arr!(user profile age), 30);
        
        assert_eq!(o.get(&["user", "profile", "name"]), Some(&json!("Mario")));
        assert_eq!(o.get(&["user", "profile", "age"]), Some(&json!(30)));
    }

    #[test]
    fn test_obj_fluent_chain() {
        let o = Obj::new()
            .set(&["a"], 1)
            .set(&["b"], 2)
            .set(&["c"], 3);
        
        assert_eq!(o.get(&["a"]), Some(&json!(1)));
        assert_eq!(o.get(&["b"]), Some(&json!(2)));
        assert_eq!(o.get(&["c"]), Some(&json!(3)));
    }

    #[test]
    fn test_obj_complex_values() {
        let o = obj!({
            num: 42,
            float: 3.14,
            str: "hello",
            bool: false,
            arr: json!([1, 2, 3]),
            nested: json!({ "key": "value" })
        });
        
        assert_eq!(o.get(&["num"]), Some(&json!(42)));
        assert_eq!(o.get(&["float"]), Some(&json!(3.14)));
        assert_eq!(o.get(&["str"]), Some(&json!("hello")));
        assert_eq!(o.get(&["bool"]), Some(&json!(false)));
        assert_eq!(o.get(&["arr"]), Some(&json!([1, 2, 3])));
        assert_eq!(o.get(&["nested"]), Some(&json!({ "key": "value" })));
    }
}
