use sapri_rust_dsl::AtomValue;

#[test]
fn test_atom_value_number() {
    let v = AtomValue::Number(42.0);
    assert_eq!(v.as_number(), Some(42.0));
    assert_eq!(v.as_string(), None);
    assert_eq!(v.as_bool(), None);
}

#[test]
fn test_atom_value_string() {
    let v = AtomValue::String("hello".to_string());
    assert_eq!(v.as_number(), None);
    assert_eq!(v.as_string(), Some("hello"));
    assert_eq!(v.as_bool(), None);
}

#[test]
fn test_atom_value_bool() {
    let v = AtomValue::Bool(true);
    assert_eq!(v.as_number(), None);
    assert_eq!(v.as_string(), None);
    assert_eq!(v.as_bool(), Some(true));
}

#[test]
fn test_atom_value_from() {
    let v1 = AtomValue::from(42);
    assert_eq!(v1.as_number(), Some(42.0));
    
    let v2 = AtomValue::from(3.14);
    assert_eq!(v2.as_number(), Some(3.14));
    
    let v3 = AtomValue::from("test".to_string());
    assert_eq!(v3.as_string(), Some("test"));
    
    let v4 = AtomValue::from("hello");
    assert_eq!(v4.as_string(), Some("hello"));
    
    let v5 = AtomValue::from(true);
    assert_eq!(v5.as_bool(), Some(true));
}

#[test]
fn test_atom_value_mul() {
    let a = AtomValue::Number(10.0);
    let b = AtomValue::Number(20.0);
    let c = a * b;
    assert_eq!(c.as_number(), Some(200.0));
}
