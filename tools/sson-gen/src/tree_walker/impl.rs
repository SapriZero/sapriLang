//! Implementazioni funzionali per tree_walker
//!
//! Funzioni piatte per operare sugli alberi

use super::{TreeNode, WalkStrategy, StrategyRegistry, WalkAction};
use crate::fp::{bind, fmap, tap, mask};

// ==========================================
// FUNZIONI BASE (PURE)
// ==========================================

/// Versione funzionale di fold (accumula su tutti i nodi)
pub fn fold_tree<T, A, F>(node: &TreeNode<T>, acc: A, f: &F) -> A
where
    F: Fn(A, &TreeNode<T>) -> A,
{
    let acc = f(acc, node);
    node.children.iter().fold(acc, |acc, child| fold_tree(child, acc, f))
}

/// Versione funzionale di map_tree (trasforma tutti i nodi)
pub fn map_tree<T, U, F>(node: &TreeNode<T>, f: &F) -> TreeNode<U>
where
    F: Fn(&TreeNode<T>) -> U + Copy,
{
    TreeNode {
        name: node.name.clone(),
        data: f(node),
        children: node.children.iter()
            .map(|child| map_tree(child, f))
            .collect(),
        path: node.path.clone(),
        qualified_name: node.qualified_name.clone(),
    }
}

/// Versione funzionale di filter_tree (restituisce nodi che soddisfano condizione)
pub fn filter_tree<T, F>(node: &TreeNode<T>, f: &F) -> Option<&TreeNode<T>>
where
    F: Fn(&TreeNode<T>) -> bool,
{
    if f(node) {
        return Some(node);
    }
    for child in &node.children {
        if let Some(found) = filter_tree(child, f) {
            return Some(found);
        }
    }
    None
}

/// Versione con bind (Option combinato)
pub fn find_by_qualified_name_bind<T>(node: &TreeNode<T>, name: &str) -> Option<&TreeNode<T>> {
    bind(Some(node), |n| {
        if n.qualified_name == name {
            Some(n)
        } else {
            n.children.iter()
                .find_map(|c| find_by_qualified_name_bind(c, name))
        }
    })
}

// ==========================================
// WALKER PRINCIPALE
// ==========================================

/// Walker con strategy (versione base)
pub fn walk_tree<T>(
    node: &TreeNode<T>,
    depth: usize,
    strategy: &dyn WalkStrategy<T>,
) -> WalkAction {
    // Entry
    match strategy.on_entry(node, depth) {
        WalkAction::Stop => return WalkAction::Stop,
        WalkAction::SkipChildren => return WalkAction::Continue,
        WalkAction::Continue => {}
    }

    // Children
    for child in &node.children {
        match walk_tree(child, depth + 1, strategy) {
            WalkAction::Stop => return WalkAction::Stop,
            _ => {}
        }
    }

    // Exit
    strategy.on_exit(node, depth)
}

/// Walker con registry (per strategy per nome)
pub fn walk_tree_registry<T>(
    node: &TreeNode<T>,
    depth: usize,
    registry: &StrategyRegistry<T>,
) -> WalkAction {
    let strategy = registry.get(&node.qualified_name);

    match strategy.on_entry(node, depth) {
        WalkAction::Stop => return WalkAction::Stop,
        WalkAction::SkipChildren => return WalkAction::Continue,
        WalkAction::Continue => {}
    }

    for child in &node.children {
        match walk_tree_registry(child, depth + 1, registry) {
            WalkAction::Stop => return WalkAction::Stop,
            _ => {}
        }
    }

    strategy.on_exit(node, depth)
}

// ==========================================
// FUNZIONI DI COLLEZIONE (CON FOLD)
// ==========================================

/// Raccoglie tutti i nomi qualificati
pub fn collect_qualified_names<T>(node: &TreeNode<T>) -> Vec<String> {
    fold_tree(node, Vec::new(), &|mut acc, n| {
        acc.push(n.qualified_name.clone());
        acc
    })
}

/// Raccoglie tutti i nodi di un certo tipo (usando un filtro)
pub fn collect_nodes_by<T, F>(node: &TreeNode<T>, filter: F) -> Vec<&TreeNode<T>>
where
    F: Fn(&TreeNode<T>) -> bool,
{
    fold_tree(node, Vec::new(), &|mut acc, n| {
        if filter(n) {
            acc.push(n);
        }
        acc
    })
}

/// Raccoglie tutti i dati dei nodi
pub fn collect_data<T: Clone>(node: &TreeNode<T>) -> Vec<T> {
    fold_tree(node, Vec::new(), &|mut acc, n| {
        acc.push(n.data.clone());
        acc
    })
}

// ==========================================
// FUNZIONI CON TAP (PER DEBUG/EFFETTI)
// ==========================================

/// Versione con tap per debug
pub fn walk_tree_with_tap<T>(
    node: &TreeNode<T>,
    depth: usize,
    strategy: &dyn WalkStrategy<T>,
    tap_fn: impl Fn(&TreeNode<T>, usize, &str),
) -> WalkAction {
    tap_fn(node, depth, "entry");
    let result = walk_tree(node, depth, strategy);
    tap_fn(node, depth, "exit");
    result
}

/// Stampa l'albero (debug)
pub fn print_tree<T: std::fmt::Debug>(node: &TreeNode<T>, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}{} ({:?})", indent, node.qualified_name, node.data);
    for child in &node.children {
        print_tree(child, depth + 1);
    }
}

// ==========================================
// FUNZIONI CON MASK (PER PERFORMANCE)
// ==========================================

/// Versione branchless di find (usa mask)
pub fn find_by_condition_branchless<T, F>(node: &TreeNode<T>, f: F) -> Option<&TreeNode<T>>
where
    F: Fn(&TreeNode<T>) -> bool,
{
    let matches = mask(f(node)) as usize;
    if matches > 0 {
        return Some(node);
    }

    for child in &node.children {
        if let Some(found) = find_by_condition_branchless(child, f) {
            return Some(found);
        }
    }
    None
}
