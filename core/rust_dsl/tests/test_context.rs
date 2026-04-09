use sapri_rust_dsl::{Context, AtomValue};

#[test]
fn test_context_new() {
    let ctx = Context::new();
    assert!(ctx.is_empty());
    assert_eq!(ctx.len(), 0);
}

#[test]
fn test_context_set_get() {
    let mut ctx = Context::new();
    ctx.set_value("a", AtomValue::Number(10.0));
    
    assert!(ctx.contains("a"));
    let val = ctx.get_value("a").unwrap();
    assert_eq!(val.as_number(), Some(10.0));
}

#[test]
fn test_context_inheritance() {
    let mut parent = Context::new();
    parent.set_value("a", AtomValue::Number(10.0));
    parent.set_value("b", AtomValue::Number(20.0));
    
    use std::sync::Arc;
    let parent_arc = Arc::new(parent);
    let mut child = Context::with_parent(&parent_arc);
    child.set_value("b", AtomValue::Number(30.0));
    child.set_value("c", AtomValue::Number(40.0));
    
    assert_eq!(child.get_value("a").unwrap().as_number(), Some(10.0));
    assert_eq!(child.get_value("b").unwrap().as_number(), Some(30.0));
    assert_eq!(child.get_value("c").unwrap().as_number(), Some(40.0));
}

#[test]
fn test_context_merge() {
    let mut ctx1 = Context::new();
    ctx1.set_value("a", AtomValue::Number(10.0));
    ctx1.set_value("b", AtomValue::Number(20.0));
    
    let mut ctx2 = Context::new();
    ctx2.set_value("b", AtomValue::Number(30.0));
    ctx2.set_value("c", AtomValue::Number(40.0));
    
    let merged = ctx1.merge(&ctx2);
    
    assert_eq!(merged.get_value("a").unwrap().as_number(), Some(10.0));
    assert_eq!(merged.get_value("b").unwrap().as_number(), Some(30.0));
    assert_eq!(merged.get_value("c").unwrap().as_number(), Some(40.0));
}
