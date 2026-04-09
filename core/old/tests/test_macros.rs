//! Test per le macro URCM

use sapri_core::{obj, path_arr};
use serde_json::json;

#[test]
fn test_path_basic() {
    // Sintassi space-separated (richiesta per evitare ambiguità con field access)
    let p = path_arr!(a b c);
    assert_eq!(p, vec!["a", "b", "c"]);
    
    let p = path_arr!(single);
    assert_eq!(p, vec!["single"]);
    
    let p = path_arr!("string.path");
    assert_eq!(p, vec!["string.path"]);
}

#[test]
fn test_obj_creation() {
    // Creazione semplice
    let o = obj!({
        count: 100,
        name: "test",
        active: true
    });
    
    assert_eq!(o.get(&["count"]), Some(&json!(100)));
    assert_eq!(o.get(&["name"]), Some(&json!("test")));
    assert_eq!(o.get(&["active"]), Some(&json!(true)));
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
fn test_obj_with_path_segments() {
    // Uso esplicito di path_arr! per chiavi gerarchiche
    let o = obj!({
        user: "Alice",
        role: "admin"
    }).set(&path_arr!(settings theme), "dark")
     .set(&path_arr!(settings lang), "it");
     
    assert_eq!(o.get(&["settings", "theme"]), Some(&json!("dark")));
    assert_eq!(o.get(&["settings", "lang"]), Some(&json!("it")));
}

#[test]
fn test_obj_mixed_values() {
    let o = obj!({
        num: 42,
        float: 3.14,
        str: "hello",
        bool: false,
        arr: json!([1, 2, 3]),
        obj: json!({ "nested": true })
    });
    
    assert_eq!(o.get(&["num"]), Some(&json!(42)));
    assert_eq!(o.get(&["float"]), Some(&json!(3.14)));
    assert_eq!(o.get(&["str"]), Some(&json!("hello")));
    assert_eq!(o.get(&["bool"]), Some(&json!(false)));
}
