//! Macro per stati reattivi
//!
//! reactive!({
//!     count: 0,
//!     on_count_change: |new| println!("{}", new)
//! })

#[macro_export]
macro_rules! reactive {
    ({ $($key:tt : $val:expr),* $(,)? }) => {
        {
            let __obj = $crate::obj!({ $($key : $val),* });
            $(
                if let Some(change_fn) = stringify!($key).strip_prefix("on_") {
                    let path = $crate::path!(change_fn);
                    __obj.on_change(&path, $val);
                }
            )*
            __obj
        }
    };
}
