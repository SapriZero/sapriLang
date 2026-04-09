use sapri_rust_dsl::define;

#[test]
fn test_define_simple() {
    let ctx = define! {
        a = 10;
        b = 20;
        c = 30;
    };
    
    assert_eq!(ctx.get_value("a").unwrap().as_number(), Some(10.0));
    assert_eq!(ctx.get_value("b").unwrap().as_number(), Some(20.0));
    assert_eq!(ctx.get_value("c").unwrap().as_number(), Some(30.0));
}

#[test]
fn test_define_with_override() {
    let parent = define! {
        x = 100;
        y = 200;
    };
    
    let child = define! {
        using &parent,
        y = 250;
        z = 300;
    };
    
    assert_eq!(child.get_value("x").unwrap().as_number(), Some(100.0));
    assert_eq!(child.get_value("y").unwrap().as_number(), Some(250.0));
    assert_eq!(child.get_value("z").unwrap().as_number(), Some(300.0));
}

#[test]
fn test_define_string_values() {
    let ctx = define! {
        n = "Sapri";
        v = "0.1.0";
    };
    
    assert_eq!(ctx.get_value("n").unwrap().as_string(), Some("Sapri"));
    assert_eq!(ctx.get_value("v").unwrap().as_string(), Some("0.1.0"));
}

#[test]
fn test_define_mixed_types() {
    let ctx = define! {
        c = 42;
        a = true;
        l = "test";
    };
    
    assert_eq!(ctx.get_value("c").unwrap().as_number(), Some(42.0));
    assert_eq!(ctx.get_value("a").unwrap().as_bool(), Some(true));
    assert_eq!(ctx.get_value("l").unwrap().as_string(), Some("test"));
}
