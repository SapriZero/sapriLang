//! Implementazioni trait per Atom.
//! Separato per chiarezza e per evitare cicli di dipendenza.

use crate::atom::Atom;
use crate::error::{BaseError, Result};

/// Stato di una promise/operazioni asincrone o lazy
pub trait PromiseState<T> {
    fn is_pending(&self) -> bool;
    fn is_resolved(&self) -> bool;
    fn resolve(self, value: T) -> Self;
    fn reject(self, reason: &str) -> Result<Self> where Self: Sized;
}

/// Sorgente esterna risolvibile (DB, HTTP, config, ecc.)
pub trait ExternalSource<T> {
    fn source(&self) -> Option<&str>;
    fn fetch_sync(&self) -> Result<T>;
}

// ============================================================================
// IMPLEMENTAZIONI
// ============================================================================

impl<T> PromiseState<T> for Atom<T> {
    fn is_pending(&self) -> bool { !self.is_resolved }
    fn is_resolved(&self) -> bool { self.is_resolved }
    
    fn resolve(mut self, value: T) -> Self {
        self.value = Some(value);
        self.is_resolved = true;
        self
    }
    
     fn reject(self, reason: &str) -> Result<Self> where Self: Sized {
        Err(BaseError::AtomError { msg: reason.into() })
    }
}

impl<T> ExternalSource<T> for Atom<T> 
where 
    T: Clone + Default 
{
    fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }
    
    fn fetch_sync(&self) -> Result<T> {
        if let Some(val) = &self.value {
            return Ok(val.clone());
        }
        // Placeholder: in extended verrà implementato il resolver reale
        Err(BaseError::Unsupported { 
            op: format!("fetch_sync for '{}'", self.source.as_deref().unwrap_or("unknown")) 
        })
    }
}
