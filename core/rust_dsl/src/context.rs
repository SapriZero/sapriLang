//! Contesto per la risoluzione degli atomi con ereditarietà

use std::collections::HashMap;
use std::sync::Arc;
use sapri_base::Atom;
use crate::atom_value::AtomValue;

/// Contesto che contiene binding nome → Atom<AtomValue>
/// Supporta ereditarietà (contesto padre) e merge stile spread
#[derive(Clone)]
pub struct Context {
    local: HashMap<String, Atom<AtomValue>>,
    parent: Option<Arc<Context>>,
}

impl Context {
    /// Crea un nuovo contesto vuoto (senza padre)
    pub fn new() -> Self {
        Self {
            local: HashMap::new(),
            parent: None,
        }
    }

    /// Crea un nuovo contesto con un padre (eredita le definizioni)
    pub fn with_parent(parent: &Arc<Context>) -> Self {
        Self {
            local: HashMap::new(),
            parent: Some(parent.clone()),
        }
    }

    /// Imposta un valore nel contesto locale
    pub fn set(&mut self, name: &str, value: impl Into<Atom<AtomValue>>) {
        self.local.insert(name.to_string(), value.into());
    }
    
    /// Imposta un valore semplice (AtomValue) nel contesto locale
    /// Converte automaticamente in Atom::resolved()
    pub fn set_value(&mut self, name: &str, value: AtomValue) {
        self.local.insert(name.to_string(), ::sapri_base::Atom::resolved(value));
    }

    /// Ottiene un atomo dal contesto (cerca prima locale, poi padre)
    pub fn get(&self, name: &str) -> Option<Atom<AtomValue>> {
        if let Some(atom) = self.local.get(name) {
            return Some(atom.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.get(name);
        }
        None
    }

    /// Ottiene il valore risolto (chiama get() e poi get() sull'atomo)
	pub fn get_value(&self, name: &str) -> Option<AtomValue> {
        self.get(name).map(|atom| atom.get().clone())
    }

    /// Verifica se un nome è definito (locale o ereditato)
    pub fn contains(&self, name: &str) -> bool {
        self.local.contains_key(name) || self.parent.as_ref().map_or(false, |p| p.contains(name))
    }

    /// Crea un nuovo contesto che eredita da self e sovrascrive con other
    /// Equivalente allo spread operator: { ...self, ...other }
    pub fn merge(&self, other: &Context) -> Self {
        let mut merged = Context::with_parent(&Arc::new(self.clone()));
        for (k, v) in &other.local {
            merged.set(k, v.clone());
        }
        merged
    }

    /// Restituisce un riferimento al padre
    pub fn parent(&self) -> Option<&Arc<Context>> {
        self.parent.as_ref()
    }

    /// Restituisce il numero di definizioni locali
    pub fn len(&self) -> usize {
        self.local.len()
    }

    /// Restituisce true se non ci sono definizioni locali
    pub fn is_empty(&self) -> bool {
        self.local.is_empty()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "global-context")]
mod global {
    use super::*;
    use once_cell::sync::OnceCell;

    static GLOBAL_CONTEXT: OnceCell<Arc<Context>> = OnceCell::new();

    pub fn set_global_context(ctx: Arc<Context>) -> Result<(), Arc<Context>> {
        GLOBAL_CONTEXT.set(ctx)
    }

    pub fn get_global_context() -> Option<Arc<Context>> {
        GLOBAL_CONTEXT.get().cloned()
    }

    pub fn with_global_context<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&Context) -> R,
    {
        GLOBAL_CONTEXT.get().map(|ctx| f(ctx))
    }
}

#[cfg(feature = "global-context")]
pub use global::*;
