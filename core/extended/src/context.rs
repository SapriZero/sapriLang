//! Contesto di esecuzione URCM.
//! Gestisce binding, scope e modalità di valutazione.

use std::collections::HashMap;
use crate::atom::Atom;

/// Modalità di esecuzione
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    /// Zero tolleranza: errore al primo vincolo violato
    Strict,
    /// Fallback automatico, warning, continua
    Generative,
}

impl Default for ExecutionMode {
    fn default() -> Self { Self::Generative }
}

/// Contesto URCM: ambiente di valutazione, binding e scope.
/// Ottimizzato per lookup O(1) e clonazione leggera.
#[derive(Debug, Clone)]
pub struct UrcmCtx {
    pub bindings: HashMap<String, Atom<serde_json::Value>>,
    pub mode: ExecutionMode,
    pub parent: Option<Box<UrcmCtx>>,
    pub metadata: HashMap<String, String>,
}

impl UrcmCtx {
    pub fn new(mode: ExecutionMode) -> Self {
        Self {
            bindings: HashMap::new(),
            mode,
            parent: None,
            metadata: HashMap::new(),
        }
    }

    /// Crea un child context (eredita bindings e mode)
    pub fn child(&self) -> Self {
        Self {
            bindings: HashMap::new(),
            mode: self.mode,
            parent: Some(Box::new(self.clone())),
            metadata: self.metadata.clone(),
        }
    }

    /// Cerca un binding nel contesto corrente o nel parent
    pub fn resolve_binding(&self, key: &str) -> Option<&Atom<serde_json::Value>> {
        self.bindings.get(key)
            .or_else(|| self.parent.as_ref().and_then(|p| p.resolve_binding(key)))
    }

    /// Imposta un binding nel contesto corrente
    pub fn bind(&mut self, key: impl Into<String>, atom: Atom<serde_json::Value>) {
        self.bindings.insert(key.into(), atom);
    }
}
