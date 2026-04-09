//! Primitiva di stato atomica.
//! Rappresenta un valore immutabile, lazy o risolto.

/// Wrapper atomico per valori URCM.
/// Supporta stati: Pending, Resolved, External.

#[derive(Debug, Clone, PartialEq)]
pub struct Atom<T> {
    pub value: Option<T>,
    pub is_resolved: bool,
    pub source: Option<String>,
}

impl<T> Atom<T> {
    /// Crea un atomo non risolto (pending)
    pub fn pending() -> Self {
        Self { value: None, is_resolved: false, source: None }
    }

    /// Crea un atomo già risolto
    pub fn resolved(value: T) -> Self {
        Self { value: Some(value), is_resolved: true, source: None }
    }

    /// Crea un atomo con sorgente esterna (es. DB, API, file)
    pub fn external(source: impl Into<String>) -> Self {
        Self { value: None, is_resolved: false, source: Some(source.into()) }
    }

    /// Ottieni il riferimento al valore (panics se pending)
    pub fn get(&self) -> &T {
        self.value.as_ref().expect("Atom is pending")
    }

    /// Verifica se l'atomo è pronto per l'uso
    pub fn is_ready(&self) -> bool {
        self.is_resolved || self.source.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atom_impl::{PromiseState, ExternalSource};

    #[test]
    fn test_atom_lifecycle() {
        let mut atom = Atom::<i32>::pending();
        assert!(!atom.is_ready());
        
         atom = atom.resolve(42);
        assert!(atom.is_ready());
        assert_eq!(*atom.get(), 42);
    }

    #[test]
    fn test_atom_external() {
        let atom = Atom::<String>::external("db:users");
        assert!(!atom.is_resolved);
        assert_eq!(atom.source(), Some("db:users"));
    }
}
