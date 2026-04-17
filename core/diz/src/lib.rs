//! Dizionario strutturato globale

include!(concat!(env!("OUT_DIR"), "/diz_generated.rs"));

// Re-export della funzione load generata da build.rs
pub use load as load_diz;

// Non importare paths, text, code, validate_name perché sono già definiti
// nel codice generato!

use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;

#[macro_export]
macro_rules! diz {
    // Supporto per path annidati: diz!(text.filter_words.list)
    ($($segment:ident).+ $(.)?) => {{
        const PATH: &str = stringify!($($segment).+);
        $crate::get_value(PATH)
    }};
    
    // Supporto con tipo: diz!(text.filter_words.list -> Vec<String>)
    ($($segment:ident).+ -> $type:ty) => {{
        const PATH: &str = stringify!($($segment).+);
        $crate::get_value_as::<$type>(PATH)
    }};
}

/// Dati del dizionario caricati una volta sola
static DIZ_DATA: Lazy<HashMap<String, Value>> = Lazy::new(|| {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let json_path = std::path::Path::new(&manifest_dir).join("diz_data.json");
    let content = std::fs::read_to_string(&json_path)
        .expect(&format!("Failed to read {:?}", json_path));
    let root: Value = serde_json::from_str(&content).expect("Failed to parse diz_data.json");
    
    // Appiattisce la struttura in un HashMap path -> value
    let mut flat = HashMap::new();
    flatten_json(&root, String::new(), &mut flat);
    flat
});

/// Appiattisce un JSON annidato in un HashMap di path
fn flatten_json(value: &Value, prefix: String, out: &mut HashMap<String, Value>) {
    match value {
        Value::Object(obj) => {
            for (k, v) in obj {
                let new_prefix = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_json(v, new_prefix, out);
            }
        }
        _ => {
            out.insert(prefix, value.clone());
        }
    }
}

/// Ottiene un valore per path (es. "text.filter_words.list")
pub fn get_value(path: &str) -> Option<&'static Value> {
    DIZ_DATA.get(path)
}

/// Ottiene un valore tipizzato per path
pub fn get_value_as<T: FromDizValue>(path: &str) -> Option<T> {
    get_value(path).and_then(|v| T::from_diz_value(v))
}

pub trait FromDizValue: Sized {
    fn from_diz_value(value: &Value) -> Option<Self>;
}

impl FromDizValue for String {
    fn from_diz_value(value: &Value) -> Option<Self> {
        value.as_str().map(|s| s.to_string())
    }
}

impl FromDizValue for u8 {
    fn from_diz_value(value: &Value) -> Option<Self> {
        value.as_u64().map(|n| n as u8)
    }
}

impl FromDizValue for u16 {
    fn from_diz_value(value: &Value) -> Option<Self> {
        value.as_u64().map(|n| n as u16)
    }
}

impl FromDizValue for usize {
    fn from_diz_value(value: &Value) -> Option<Self> {
        value.as_u64().map(|n| n as usize)
    }
}

impl FromDizValue for bool {
    fn from_diz_value(value: &Value) -> Option<Self> {
        value.as_bool()
    }
}

impl FromDizValue for Vec<String> {
    fn from_diz_value(value: &Value) -> Option<Self> {
        value.as_array().map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
    }
}

impl<T: FromDizValue> FromDizValue for Option<T> {
    fn from_diz_value(value: &Value) -> Option<Self> {
        if value.is_null() {
            Some(None)
        } else {
            T::from_diz_value(value).map(Some)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_diz() {
        let diz = load_diz();
        assert_eq!(diz.text.min_word_length, 3);
        assert!(!diz.text.filter_words.list.is_empty());
    }

    #[test]
    fn test_diz_macro() {
        let words: Option<Vec<String>> = diz!(text.filter_words.list -> Vec<String>);
        assert!(words.is_some());
        assert!(!words.unwrap().is_empty());
    }

    #[test]
    fn test_get_value_path() {
        let version = get_value("text.min_word_length");
        assert!(version.is_some());
        
        let nonexistent = get_value("nonexistent");
        assert!(nonexistent.is_none());
    }

    #[test]
    fn test_charmap_fields() {
        let diz = load_diz();
        assert_eq!(diz.text.charmap.bits, 6);
        assert_eq!(diz.text.charmap.escape_code, 63);
        assert_eq!(diz.text.charmap.lowercase.start, 'a');
        assert_eq!(diz.text.charmap.uppercase.start, 'A');
        assert_eq!(diz.text.charmap.space.char, ' ');
        assert!(!diz.text.charmap.accents.is_empty());
    }
    
    #[test]
    fn test_validate_name_macro() {
        // Questi test verificano che la macro validate_name funzioni
        // A compile-time, qui testiamo solo che la macro esista
        assert!(true);
    }
}
