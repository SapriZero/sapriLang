//! Macro per creare oggetti con path e valori
//!
//! Sintassi:
//!   obj!({ key: value, ... })                    // oggetto semplice
//!   obj!({ key.subkey: value, ... })              // con path
//!   obj!(default_obj => { key: value, ... })      // con default
//!   obj!(ctx, { key: ctx.field, ... })            // con contesto
//!   obj!([key1, key2] => value)                   // path come array (currying)

#[macro_export]
macro_rules! obj {
    // Versione base con path dot
    ({ $($key:tt : $val:expr),* $(,)? }) => {
        {
            let mut __obj = $crate::Obj::new();
            $(
                __obj = __obj.set(
                    &$crate::path!($key).as_slice(),
                    $val
                );
            )*
            __obj
        }
    };

    // Con default
    ($default:expr => { $($key:tt : $val:expr),* $(,)? }) => {
        {
            let mut __obj = $default.clone();
            $(
                __obj = __obj.set(
                    &$crate::path!($key).as_slice(),
                    $val
                );
            )*
            __obj
        }
    };

    // Con contesto
    ($ctx:expr, { $($key:tt : $val:expr),* $(,)? }) => {
        {
            let mut __obj = $crate::Obj::new();
            $(
                __obj = __obj.set(
                    &$crate::path!($key).as_slice(),
                    $val
                );
            )*
            __obj
        }
    };

    // Path come array (currying)
    ([$($key:expr),*] => $val:expr) => {
        move |obj: $crate::Obj| obj.set(&[$($key),*], $val)
    };
}
