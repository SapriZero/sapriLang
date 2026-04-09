//! Macro e funzione per scan di espressioni stringa

use crate::context::Context;
use crate::scanner::{tokenize, parse, compile};
use crate::atom_value::AtomValue;
use sapri_base::Atom;

/// Scansiona una stringa e la trasforma in un Atom<AtomValue>
///
/// Supporta:
/// - Moltiplicazione implicita: "ab" → a * b
/// - Moltiplicazione esplicita: "a * b" → a * b
/// - Numeri: "a * 2" → a * 2
pub fn scan(expr: &str, ctx: &Context) -> Result<Atom<AtomValue>, String> {
    let tokens = tokenize(expr)?;
    let ast = parse(&tokens)?;
    compile(&ast, ctx)
}

/// Versione macro di scan che supporta contesto globale implicito
#[macro_export]
macro_rules! scan {
    // Con contesto esplicito
    ($expr:expr, $ctx:expr) => {{
        $crate::scan::scan($expr, $ctx)
    }};

    // Con contesto globale implicito (richiede feature global-context)
    ($expr:expr) => {{
        #[cfg(feature = "global-context")]
        {
            let ctx = $crate::context::get_global_context()
                .ok_or_else(|| "Nessun contesto globale impostato".to_string())?;
            $crate::scan::scan($expr, &ctx)
        }
        #[cfg(not(feature = "global-context"))]
        {
            compile_error!("Il contesto globale richiede la feature 'global-context'");
        }
    }};
}
