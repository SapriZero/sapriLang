//! Walker generico per alberi con strategy pattern
//!
//! Definisce le strutture e i trait (poche definizioni)

use crate::fp::{eval, get_or_default};

// ==========================================
// NODO ALBERO (STRUTTURA BASE)
// ==========================================

/// Nodo generico dell'albero con naming qualificato
#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    /// Nome semplice del nodo
    pub name: String,
    /// Dato associato
    pub data: T,
    /// Figli
    pub children: Vec<TreeNode<T>>,
    /// Path completo (con /)
    pub path: String,
    /// Nome qualificato (con .)
    pub qualified_name: String,
}

impl<T> TreeNode<T> {
    pub fn new(name: String, data: T) -> Self {
        Self {
            name: name.clone(),
            data,
            children: Vec::new(),
            path: name.clone(),
            qualified_name: name,
        }
    }

    /// Versione funzionale di with_child (restituisce nuovo nodo)
    pub fn with_child(mut self, child: TreeNode<T>) -> Self {
        let child_path = eval!(self.path.is_empty(), child.name.clone(), {
            format!("{}/{}", self.path, child.name)
        });

        let child_qualified = eval!(self.qualified_name.is_empty(), child.name.clone(), {
            format!("{}.{}", self.qualified_name, child.name)
        });

        let mut child = child;
        child.path = child_path;
        child.qualified_name = child_qualified;
        self.children.push(child);
        self
    }

    /// Versione funzionale di map
    pub fn map<U, F>(self, f: F) -> TreeNode<U>
    where
        F: Fn(T) -> U + Copy,
    {
        TreeNode {
            name: self.name,
            data: f(self.data),
            children: self.children.into_iter()
                .map(|child| child.map(f))
                .collect(),
            path: self.path,
            qualified_name: self.qualified_name,
        }
    }

    /// Cerca per nome qualificato
    pub fn find_by_qualified_name(&self, name: &str) -> Option<&TreeNode<T>> {
        if self.qualified_name == name {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find_by_qualified_name(name) {
                return Some(found);
            }
        }
        None
    }
}

// ==========================================
// STRATEGY PATTERN
// ==========================================

/// Azioni del walker
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WalkAction {
    Continue,
    SkipChildren,
    Stop,
}

/// Strategy per elaborare i nodi
pub trait WalkStrategy<T> {
    fn on_entry(&self, node: &TreeNode<T>, depth: usize) -> WalkAction;
    fn on_exit(&self, node: &TreeNode<T>, depth: usize) -> WalkAction;
    fn name(&self) -> &'static str;
}

/// Strategy vuota (default)
pub struct EmptyStrategy;
impl<T> WalkStrategy<T> for EmptyStrategy {
    fn on_entry(&self, _node: &TreeNode<T>, _depth: usize) -> WalkAction { WalkAction::Continue }
    fn on_exit(&self, _node: &TreeNode<T>, _depth: usize) -> WalkAction { WalkAction::Continue }
    fn name(&self) -> &'static str { "empty" }
}

// ==========================================
// REGISTRY DELLE STRATEGY
// ==========================================

/// Registry che mappa nomi qualificati a strategy
pub struct StrategyRegistry<T> {
    strategies: std::collections::HashMap<String, Box<dyn WalkStrategy<T>>>,
    default: Box<dyn WalkStrategy<T>>,
}

impl<T> StrategyRegistry<T> {
    pub fn new() -> Self {
        Self {
            strategies: std::collections::HashMap::new(),
            default: Box::new(EmptyStrategy),
        }
    }

    pub fn register<S: WalkStrategy<T> + 'static>(&mut self, name: &str, strategy: S) {
        self.strategies.insert(name.to_string(), Box::new(strategy));
    }

    pub fn get(&self, name: &str) -> &dyn WalkStrategy<T> {
        self.strategies.get(name)
            .map(|b| b.as_ref())
            .unwrap_or(self.default.as_ref())
    }
}
