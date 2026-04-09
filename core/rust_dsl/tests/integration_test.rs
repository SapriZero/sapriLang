use sapri_rust_dsl::{define, scan, AtomValue};

#[test]
fn test_full_workflow() {
    let base = define! {
        t = 0.22;  // tax_rate
        d = 0.10;  // discount
    };
    
    let order = define! {
        using &base,
        p = 100.0;  // price
        q = 3;      // quantity
    };
    
    let mut ctx = order.clone();
    
    // s = subtotal = p * q
    let s_atom = scan!("p q", &ctx).unwrap();
    let s = s_atom.get().clone();
    assert_eq!(s.as_number(), Some(300.0));
    ctx.set_value("s", s);
    
    // a = discount_amt = s * d
    let a_atom = scan!("s d", &ctx).unwrap();
    let a = a_atom.get().clone();
    ctx.set_value("a", a);
    
    // b = after_discount = s * (1 - d) = s * 0.9
    let r = 0.9;  // 1 - discount
    ctx.set_value("r", AtomValue::Number(r));
    let b_atom = scan!("s r", &ctx).unwrap();
    let b = b_atom.get().clone();
    assert_eq!(b.as_number().unwrap(), 270.0);
    ctx.set_value("b", b);
    
    // total = b * (1 + t) = b * 1.22
    let f = 1.22;  // 1 + tax_rate
    ctx.set_value("f", AtomValue::Number(f));
    let total_atom = scan!("b f", &ctx).unwrap();
    let total = total_atom.get().clone();
    
    assert_eq!(total.as_number().unwrap(), 329.4);
}

#[test]
fn test_context_shadowing() {
    let outer = define! {
        x = 10;
        y = 20;
    };
    
    let inner = define! {
        using &outer,
        x = 100;
        z = 300;
    };
    
    assert_eq!(inner.get_value("x").unwrap().as_number(), Some(100.0));
    assert_eq!(inner.get_value("y").unwrap().as_number(), Some(20.0));
    assert_eq!(inner.get_value("z").unwrap().as_number(), Some(300.0));
    assert_eq!(outer.get_value("x").unwrap().as_number(), Some(10.0));
}
