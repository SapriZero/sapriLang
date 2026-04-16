//! Test semplici per sapri_sson

use sapri_sson::*;
use sapri_obj::{obj, Obj, Value};

#[test]
fn test_req_validator() {
    let validator = Validator::new();
    
    let obj = obj! { name: "Alice" };
    let ctx = ValidationContext::new(ParserMode::Strict, obj);
    
    let constraint = Constraint {
        name: "req".to_string(),
        target: "name".to_string(),
        params: Obj::new(),
    };
    
    assert!(validator.validate(&ctx, &constraint));
    
    let empty_ctx = ValidationContext::new(ParserMode::Strict, Obj::new());
    assert!(!validator.validate(&empty_ctx, &constraint));
}

#[test]
fn test_min_validator() {
    let validator = Validator::new();
    
    let obj = obj! { age: 25 };
    let ctx = ValidationContext::new(ParserMode::Strict, obj);
    
    let params = Obj::new()
        .set("value", Value::Number(18.0));
    
    let constraint = Constraint {
        name: "min".to_string(),
        target: "age".to_string(),
        params,
    };
    
    assert!(validator.validate(&ctx, &constraint));
    
    let params_fail = Obj::new()
        .set("value", Value::Number(30.0));
    
    let constraint_fail = Constraint {
        name: "min".to_string(),
        target: "age".to_string(),
        params: params_fail,
    };
    
    assert!(!validator.validate(&ctx, &constraint_fail));
}

#[test]
fn test_max_validator() {
    let validator = Validator::new();
    
    let obj = obj! { age: 25 };
    let ctx = ValidationContext::new(ParserMode::Strict, obj);
    
    let params = Obj::new()
        .set("value", Value::Number(30.0));
    
    let constraint = Constraint {
        name: "max".to_string(),
        target: "age".to_string(),
        params,
    };
    
    assert!(validator.validate(&ctx, &constraint));
    
    let params_fail = Obj::new()
        .set("value", Value::Number(20.0));
    
    let constraint_fail = Constraint {
        name: "max".to_string(),
        target: "age".to_string(),
        params: params_fail,
    };
    
    assert!(!validator.validate(&ctx, &constraint_fail));
}

#[test]
fn test_pattern_validator() {
    let validator = Validator::new();
    
    let obj = obj! { email: "alice@example.com" };
    let ctx = ValidationContext::new(ParserMode::Strict, obj);
    
    let params = Obj::new()
        .set("regex", Value::String(r"^[^@]+@[^@]+\.[^@]+$".to_string()));
    
    let constraint = Constraint {
        name: "pattern".to_string(),
        target: "email".to_string(),
        params,
    };
    
    assert!(validator.validate(&ctx, &constraint));
    
    let obj2 = obj! { email: "invalid" };
    let ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
    
    assert!(!validator.validate(&ctx2, &constraint));
}

#[test]
fn test_enum_validator() {
    let validator = Validator::new();
    
    let obj = obj! { status: "active" };
    let ctx = ValidationContext::new(ParserMode::Strict, obj);
    
    let allowed_obj = Obj::new()
        .set("0", Value::String("active".to_string()))
        .set("1", Value::String("inactive".to_string()));
    
    let params = Obj::new()
        .set("values", Value::Obj(allowed_obj));
    
    let constraint = Constraint {
        name: "enum".to_string(),
        target: "status".to_string(),
        params,
    };
    
    assert!(validator.validate(&ctx, &constraint));
    
    let obj2 = obj! { status: "deleted" };
    let ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
    
    assert!(!validator.validate(&ctx2, &constraint));
}

#[test]
fn test_mutex_validator() {
    let validator = Validator::new();
    
    let obj = obj! { a: 10 };
    let ctx = ValidationContext::new(ParserMode::Strict, obj);
    
    let fields_obj = Obj::new()
        .set("0", Value::String("a".to_string()))
        .set("1", Value::String("b".to_string()));
    
    let params = Obj::new()
        .set("fields", Value::Obj(fields_obj));
    
    let constraint = Constraint {
        name: "mutex".to_string(),
        target: "".to_string(),
        params,
    };
    
    assert!(validator.validate(&ctx, &constraint));
    
    let obj2 = obj! { a: 10, b: 20 };
    let ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
    assert!(!validator.validate(&ctx2, &constraint));
}

#[test]
fn test_at_least_one_validator() {
    let validator = Validator::new();
    
    let fields_obj = Obj::new()
        .set("0", Value::String("a".to_string()))
        .set("1", Value::String("b".to_string()));
    
    let params = Obj::new()
        .set("fields", Value::Obj(fields_obj));
    
    let constraint = Constraint {
        name: "at_least_one".to_string(),
        target: "".to_string(),
        params,
    };
    
    let obj = obj! { a: 10 };
    let ctx = ValidationContext::new(ParserMode::Strict, obj);
    assert!(validator.validate(&ctx, &constraint));
    
    let obj2 = obj! { b: 20 };
    let ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
    assert!(validator.validate(&ctx2, &constraint));
    
    let obj3 = Obj::new();
    let ctx3 = ValidationContext::new(ParserMode::Strict, obj3);
    assert!(!validator.validate(&ctx3, &constraint));
}

#[test]
fn test_type_validator() {
    let validator = Validator::new();
    
    let obj = obj! { 
        name: "Alice",
        age: 25,
        active: true,
        data: obj! {}
    };
    let ctx = ValidationContext::new(ParserMode::Strict, obj);
    
    let params = Obj::new()
        .set("field", Value::String("name".to_string()))
        .set("expected", Value::String("string".to_string()));
    let constraint = Constraint {
        name: "type".to_string(),
        target: "".to_string(),
        params,
    };
    assert!(validator.validate(&ctx, &constraint));
    
    let params2 = Obj::new()
        .set("field", Value::String("age".to_string()))
        .set("expected", Value::String("number".to_string()));
    let constraint2 = Constraint {
        name: "type".to_string(),
        target: "".to_string(),
        params: params2,
    };
    assert!(validator.validate(&ctx, &constraint2));
    
    let params3 = Obj::new()
        .set("field", Value::String("data".to_string()))
        .set("expected", Value::String("object".to_string()));
    let constraint3 = Constraint {
        name: "type".to_string(),
        target: "".to_string(),
        params: params3,
    };
    assert!(validator.validate(&ctx, &constraint3));
}

#[test]
fn test_validate_all() {
    let validator = Validator::new();
    
    let obj = obj! { name: "Alice", age: 25 };
    let mut ctx = ValidationContext::new(ParserMode::Strict, obj);
    
    let constraints = vec![
        Constraint {
            name: "req".to_string(),
            target: "name".to_string(),
            params: Obj::new(),
        },
        Constraint {
            name: "req".to_string(),
            target: "age".to_string(),
            params: Obj::new(),
        },
        Constraint {
            name: "min".to_string(),
            target: "age".to_string(),
            params: Obj::new().set("value", Value::Number(18.0)),
        },
    ];
    
    let s = validator.validate_all(&mut ctx, &constraints);
    assert_eq!(s, 1.0);
    assert!(is_exportable(s));
    
    // Oggetto con solo name (age mancante)
    let obj2 = obj! { name: "Alice" };
    let mut ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
    
    let validator2 = Validator::new();
    let s2 = validator2.validate_all(&mut ctx2, &constraints);
    
    // Solo il constraint "req" su "name" è valido = 1/3
    let expected = 1.0 / 3.0;
    assert!((s2 - expected).abs() < 1e-6, "s2 = {}, expected = {}", s2, expected);
    assert!(!is_exportable(s2));
}


#[test]
fn test_calculate_s_score() {
    let s = calculate_s_score(8, 10, ParserMode::Strict);
    assert_eq!(s, 0.8);
    
    let s2 = calculate_s_score(8, 10, ParserMode::Generative);
    assert!((s2 - 0.5333333333333333).abs() < 1e-6);
    
    let s3 = calculate_s_score(10, 10, ParserMode::Strict);
    assert_eq!(s3, 1.0);
    
    let s4 = calculate_s_score(0, 10, ParserMode::Strict);
    assert_eq!(s4, 0.0);
    
    let s5 = calculate_s_score(0, 0, ParserMode::Strict);
    assert_eq!(s5, 1.0);
}

#[test]
fn test_is_exportable() {
    assert!(is_exportable(0.95));
    assert!(is_exportable(0.9));
    assert!(!is_exportable(0.89));
    assert!(!is_exportable(0.5));
}
