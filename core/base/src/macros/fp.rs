//! Utility funzionali leggere.

/// Crea una closure curry da una funzione a 2 argomenti.
#[macro_export]
macro_rules! curry {
    ($fn:path) => {
        |a| move |b| $fn(a, b)
    };
}

/// Esegue il blocco solo se la condizione è vera, altrimenti ritorna None.
#[macro_export]
macro_rules! lazy_if {
    ($cond:expr, $block:block) => {
        if $cond { Some($block) } else { None }
    };
}
