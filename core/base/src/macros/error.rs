//! Macro ergonomiche per gestione errori nel layer base.

/// Tenta un'operazione, se fallisce ritorna un valore di fallback.
#[macro_export]
macro_rules! try_or {
    ($expr:expr, $fallback:expr) => {
        match $expr {
            Ok(v) => v,
            Err(_) => $fallback,
        }
    };
}

/// Variante di `unwrap_or_else` per macro, evita borrow checker su closure complesse.
#[macro_export]
macro_rules! unwrap_or_else {
    ($opt:expr, $default:expr) => {
        match $opt {
            Some(v) => v,
            None => $default,
        }
    };
}
