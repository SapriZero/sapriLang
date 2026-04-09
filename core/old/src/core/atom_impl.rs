//! Implementazioni concrete dei diversi tipi di atomi
//! Supporta: Number, Computed (lazy), Promise, External (push)

use std::collections::HashSet;
use std::sync::Arc;

/// Stato di una promise (async)
pub trait PromiseState<T>: Send + Sync {
fn poll(&mut self, data: &T) -> Option<f64>;
fn is_ready(&self) -> bool;
fn reset(&mut self);
}

/// Fonte esterna push (websocket, sensori)
pub trait ExternalSource: Send + Sync {
fn update(&mut self) -> Option<f64>;
fn subscribe(&mut self, callback: Box<dyn Fn(f64) + Send + Sync>);
}

/// Implementazione concreta di un atomo
pub enum AtomImpl<T> {
/// Valore numerico fisso o modificabile
Number {
value: f64,
},

/// Calcolato lazy (solo su richiesta)
Computed {
func: Arc<dyn Fn(&T) -> f64 + Send + Sync>,
deps: HashSet<char>,
cached: Option<f64>,
last_epoch: usize,
},

/// Promise (async, si aggiorna quando risolta)
Promise {
future: Box<dyn PromiseState<T> + Send + Sync>,
deps: HashSet<char>,
value: Option<f64>,
},

/// Fonte esterna (push da websocket, sensori)
External {
value: f64,
source: Box<dyn ExternalSource + Send + Sync>,
},
}

impl<T> AtomImpl<T> {
pub fn number(value: f64) -> Self {
AtomImpl::Number { value }
}

pub fn computed(
func: impl Fn(&T) -> f64 + Send + Sync + 'static,
deps: HashSet<char>
) -> Self {
AtomImpl::Computed {
func: Arc::new(func),
deps,
cached: None,
last_epoch: 0,
}
}

pub fn promise(
future: impl PromiseState<T> + Send + Sync + 'static,
deps: HashSet<char>
) -> Self {
AtomImpl::Promise {
future: Box::new(future),
deps,
value: None,
}
}

pub fn external(
source: impl ExternalSource + Send + Sync + 'static,
initial: f64
) -> Self {
AtomImpl::External {
value: initial,
source: Box::new(source),
}
}

pub fn is_lazy(&self) -> bool {
matches!(self, AtomImpl::Computed { .. })
}

pub fn is_eager(&self) -> bool {
matches!(self, AtomImpl::Promise { .. } | AtomImpl::External { .. })
}
}

impl<T> std::fmt::Debug for AtomImpl<T> {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match self {
AtomImpl::Number { value } => write!(f, "Number({})", value),
AtomImpl::Computed { deps, cached, .. } => {
write!(f, "Computed(deps={:?}, cached={:?})", deps, cached)
}
AtomImpl::Promise { deps, value, .. } => {
write!(f, "Promise(deps={:?}, value={:?})", deps, value)
}
AtomImpl::External { value, .. } => {
write!(f, "External({})", value)
}
}
}
}

