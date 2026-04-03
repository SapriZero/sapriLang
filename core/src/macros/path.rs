//! Macro per convertire path dot notation in array
//!
//! path!(a.b.c) → ["a", "b", "c"]

#[macro_export]
macro_rules! path {
    // Singolo segmento
    ($seg:ident) => { vec![stringify!($seg)] };

    // Path con punti
    ($first:ident.$($rest:ident).*) => {
        {
            let mut __path = vec![stringify!($first)];
            $(
                __path.push(stringify!($rest));
            )*
            __path
        }
    };

    // Stringa
    ($s:expr) => { vec![$s] };
}
