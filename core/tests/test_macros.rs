//! Test per le macro URCM

use urcm_core::*;

#[test]
fn test_obj_macro() {
    let obj = obj!({
        count: 42,
        name: "test",
        active: true
    });

    assert_eq!(obj.get(&["count"]), Some(&Value::Number(42.0)));
    assert_eq!(obj.get(&["name"]), Some(&Value::String("test".to_string())));
}

#[test]
fn test_path_macro() {
    let path = path!(a.b.c);
    assert_eq!(path, vec!["a", "b", "c"]);
}

#[test]
fn test_struct_with_keys() {
    struct_with_keys! {
        TestStruct {
            count: i32,
            name: String
        }
    }

    let obj = obj!({
        TestStruct.count: 100,
        TestStruct.name: "hello"
    });

    let s = TestStruct::from_obj(&obj).unwrap();
    assert_eq!(s.count, 100);
    assert_eq!(s.name, "hello");
}

#[test]
fn test_cascade() {
    let base = obj!({ count: 0, name: "base" });
    let user = obj!({ count: 42 });

    let merged = cascade!(base, user);
    assert_eq!(merged.get(&["count"]), Some(&Value::Number(42.0)));
    assert_eq!(merged.get(&["name"]), Some(&Value::String("base".to_string())));
}
