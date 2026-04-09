//! Atomo URCM con metadati e label

use std::collections::HashSet;
use super::atom_impl::{AtomImpl, PromiseState, ExternalSource};

/// Atomo URCM completo
pub struct Atom<T> {
pub symbol: char,
pub label: String,
pub impl_type: AtomImpl<T>,
}

impl<T> Atom<T> {
pub fn new(symbol: char, label: impl Into<String>, impl_type: AtomImpl<T>) -> Self {
Self {
symbol,
label: label.into(),
impl_type,
}
}

pub fn new_number(symbol: char, label: impl Into<String>, value: f64) -> Self {
Self::new(symbol, label, AtomImpl::number(value))
}

pub fn new_computed(
symbol: char,
label: impl Into<String>,
func: impl Fn(&T) -> f64 + Send + Sync + 'static,
deps: HashSet<char>
) -> Self {
Self::new(symbol, label, AtomImpl::computed(func, deps))
}

pub fn new_promise(
symbol: char,
label: impl Into<String>,
future: impl PromiseState<T> + Send + Sync + 'static,
deps: HashSet<char>
) -> Self {
Self::new(symbol, label, AtomImpl::promise(future, deps))
}

pub fn new_external(
symbol: char,
label: impl Into<String>,
source: impl ExternalSource + Send + Sync + 'static,
initial: f64
) -> Self {
Self::new(symbol, label, AtomImpl::external(source, initial))
}

pub fn deps(&self) -> HashSet<char> {
match &self.impl_type {
AtomImpl::Number { .. } => HashSet::new(),
AtomImpl::Computed { deps, .. } => deps.clone(),
AtomImpl::Promise { deps, .. } => deps.clone(),
AtomImpl::External { .. } => HashSet::new(),
}
}

pub fn is_react(&self) -> bool {
matches!(&self.impl_type, AtomImpl::Computed { .. } | AtomImpl::Promise { .. })
}
}

