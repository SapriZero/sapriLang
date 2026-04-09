//! Engine di valutazione pura.
//! Qui vivono le firme e la logica base. 
//! Il parsing pesante e le macro vanno in extended/fp.

use crate::context::UrcmCtx;
use crate::atom::Atom;
use crate::error::Result;

/// Valuta un'espressione stringa nel contesto dato.
/// Restituisce un Atom risolto o un errore.
pub fn evaluate_expr(ctx: &UrcmCtx, expr: &str) -> Result<Atom<serde_json::Value>> {
    // Placeholder: la logica reale di parsing/masking/lazy va in sapri-extended
    if expr.trim().is_empty() {
        return Ok(Atom::pending());
    }
    
    match ctx.mode {
        ExecutionMode::Strict => Err(crate::error::BaseError::Unsupported { 
            op: format!("strict eval of '{}'", expr) 
        }),
        ExecutionMode::Generative => Ok(Atom::resolved(serde_json::Value::Null)),
    }
}

/// Applica un mask/trasformazione a un valore esistente
pub fn apply_mask<T, F>(atom: Atom<T>, f: F) -> Result<Atom<T>>
where
    F: FnOnce(T) -> T,
{
    if atom.is_resolved {
        Ok(Atom::resolved(f(atom.get().clone())))
    } else {
        Ok(atom) // Pass-through se pending
    }
}
