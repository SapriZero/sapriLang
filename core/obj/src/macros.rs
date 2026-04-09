//! Macro per creare oggetti con sintassi stile JavaScript

/// Crea un oggetto Obj con sintassi stile JavaScript
///
/// # Esempi
///
/// ```
/// use sapri_obj::obj;
///
/// // Oggetto semplice
/// let person = obj! {
///     name: "Alice",
///     age: 30,
///     active: true
/// };
///
/// // Spread con using (vecchia sintassi)
/// let base = obj! { a: 10, b: 20 };
/// let extended = obj! {
///     using base,
///     b: 30,
///     c: 40
/// };
///
/// // Spread con .. (nuova sintassi)
/// let extended2 = obj! {
///     ..base;
///     b: 30,
///     c: 40
/// };
///
/// // Spread multipli
/// let obj1 = obj! { a: 1, b: 2 };
/// let obj2 = obj! { c: 3, d: 4 };
/// let merged = obj! {
///     ..obj1,
///     ..obj2;
///     e: 5
/// };
/// ```
#[macro_export]
macro_rules! obj {
    // Oggetto vuoto
    () => {
        $crate::Obj::new()
    };

    // Vecchia sintassi: using base, campi
    (using $parent:expr, $($key:ident : $value:expr),* $(,)?) => {{
        let mut _obj = $parent.clone();
        $(
            _obj = _obj.set(stringify!($key), $value);
        )*
        _obj
    }};

    // Solo spread con .. (senza campi)
    ($(..$spread:expr),* $(,)?) => {{
        let mut _obj = $crate::Obj::new();
        $(
            _obj = _obj.merge($spread);
        )*
        _obj
    }};

    // Spread con .. + campi (separati da ;)
    ($(..$spread:expr),* ; $($key:ident : $value:expr),* $(,)?) => {{
        let mut _obj = $crate::Obj::new();
        $(
            _obj = _obj.merge($spread);
        )*
        $(
            _obj = _obj.set(stringify!($key), $value);
        )*
        _obj
    }};

    // Solo campi (senza spread)
    ($($key:ident : $value:expr),* $(,)?) => {{
        let mut _obj = $crate::Obj::new();
        $(
            _obj = _obj.set(stringify!($key), $value);
        )*
        _obj
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_obj_empty() {
        let obj = obj! {};
        assert!(obj.is_empty());
    }

    #[test]
    fn test_obj_simple() {
        let obj = obj! {
            a: 10,
            b: 20.5,
            c: "hello",
            d: true
        };

        assert_eq!(obj.get("a").unwrap().as_number(), Some(10.0));
        assert_eq!(obj.get("b").unwrap().as_number(), Some(20.5));
        assert_eq!(obj.get("c").unwrap().as_str(), Some("hello"));
        assert_eq!(obj.get("d").unwrap().as_bool(), Some(true));
    }

    #[test]
    fn test_obj_nested() {
        let obj = obj! {
            a: 10,
            b: obj! {
                c: 20,
                d: obj! {
                    e: 30
                }
            }
        };

        assert_eq!(obj.get("a").unwrap().as_number(), Some(10.0));

        let b = obj.get("b").unwrap().as_obj().unwrap();
        assert_eq!(b.get("c").unwrap().as_number(), Some(20.0));

        let d = b.get("d").unwrap().as_obj().unwrap();
        assert_eq!(d.get("e").unwrap().as_number(), Some(30.0));
    }

    #[test]
    fn test_obj_path_access() {
        let obj = obj! {
            a: 10,
            b: obj! {
                c: 20,
                d: obj! {
                    e: 30
                }
            }
        };

        assert_eq!(obj.get_path(&["b", "c"]).unwrap().as_number(), Some(20.0));
        assert_eq!(obj.get_path(&["b", "d", "e"]).unwrap().as_number(), Some(30.0));
        assert_eq!(obj.get_dot("b.c").unwrap().as_number(), Some(20.0));
        assert_eq!(obj.get_dot("b.d.e").unwrap().as_number(), Some(30.0));
    }

    #[test]
    fn test_obj_merge() {
        let base = obj! {
            a: 10,
            b: 20
        };

        let extended = obj! {
            using base,
            b: 30,
            c: 40
        };

        assert_eq!(extended.get("a").unwrap().as_number(), Some(10.0));
        assert_eq!(extended.get("b").unwrap().as_number(), Some(30.0));
        assert_eq!(extended.get("c").unwrap().as_number(), Some(40.0));
    }

    #[test]
    fn test_obj_remove() {
        let obj = obj! { a: 10, b: 20 };
        let obj = obj.remove("a");
        assert!(!obj.contains("a"));
        assert!(obj.contains("b"));
    }

    #[test]
    fn test_obj_keys() {
        let obj = obj! { a: 1, b: 2, c: 3 };
        let mut keys: Vec<_> = obj.keys().into_iter().map(|s| s.to_string()).collect();
        keys.sort();
        assert_eq!(keys, vec!["a", "b", "c"]);
    }
}

#[test]
fn test_obj_using() {
    let base = obj! { a: 10, b: 20 };
    let extended = obj! { using base, b: 30, c: 40 };
    
    assert_eq!(extended.get("a").unwrap().as_number(), Some(10.0));
    assert_eq!(extended.get("b").unwrap().as_number(), Some(30.0));
    assert_eq!(extended.get("c").unwrap().as_number(), Some(40.0));
}

#[test]
fn test_obj_spread() {
    let base = obj! { a: 10, b: 20 };
    let extended = obj! { ..base; b: 30, c: 40 };
    
    assert_eq!(extended.get("a").unwrap().as_number(), Some(10.0));
    assert_eq!(extended.get("b").unwrap().as_number(), Some(30.0));
    assert_eq!(extended.get("c").unwrap().as_number(), Some(40.0));
}

#[test]
fn test_obj_multiple_spread() {
    let obj1 = obj! { a: 1, b: 2 };
    let obj2 = obj! { c: 3, d: 4 };
    let merged = obj! { ..obj1, ..obj2; e: 5 };
    
    assert_eq!(merged.get("a").unwrap().as_number(), Some(1.0));
    assert_eq!(merged.get("b").unwrap().as_number(), Some(2.0));
    assert_eq!(merged.get("c").unwrap().as_number(), Some(3.0));
    assert_eq!(merged.get("d").unwrap().as_number(), Some(4.0));
    assert_eq!(merged.get("e").unwrap().as_number(), Some(5.0));
}

#[test]
fn test_obj_spread_only() {
    let base = obj! { a: 10, b: 20 };
    let copy = obj! { ..base };
    
    assert_eq!(copy.get("a").unwrap().as_number(), Some(10.0));
    assert_eq!(copy.get("b").unwrap().as_number(), Some(20.0));
}
