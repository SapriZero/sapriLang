use sapri_rust_dsl::{define, scan, Context};

#[test]
fn test_scan_simple_product() {
    let ctx = define! {
        a = 10;
        b = 20;
    };
    
    let result = scan!("a b", &ctx).unwrap();
    assert_eq!(result.get().as_number(), Some(200.0));
}

#[test]
fn test_scan_implicit_product() {
    let ctx = define! {
        a = 10;
        b = 20;
    };
    
    let result = scan!("ab", &ctx).unwrap();
    assert_eq!(result.get().as_number(), Some(200.0));
}

#[test]
fn test_scan_with_number() {
    let ctx = define! {
        a = 10;
    };
    
    let result = scan!("a 5", &ctx).unwrap();
    assert_eq!(result.get().as_number(), Some(50.0));
}

#[test]
fn test_scan_three_factors() {
    let ctx = define! {
        a = 2;
        b = 3;
        c = 4;
    };
    
    let result = scan!("a b c", &ctx).unwrap();
    assert_eq!(result.get().as_number(), Some(24.0));
}

#[test]
fn test_scan_single_ident() {
    let ctx = define! {
        x = 42;
    };
    
    let result = scan!("x", &ctx).unwrap();
    assert_eq!(result.get().as_number(), Some(42.0));
}

#[test]
fn test_scan_not_found() {
    let ctx = Context::new();
    let result = scan!("x", &ctx);
    assert!(result.is_err());
}

#[test]
fn test_scan_with_parent_context() {
    let parent = define! {
        a = 10;
    };
    
    let child = define! {
        using &parent,
        b = 5;
    };
    
    let result = scan!("a b", &child).unwrap();
    assert_eq!(result.get().as_number(), Some(50.0));
}
