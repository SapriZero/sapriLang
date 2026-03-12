//! Contesto URCM con gestione atomi e reattività
#![allow(unused_variables)]

use crate::core::atom::Atom;
use crate::core::atom_impl::{AtomImpl, PromiseState, ExternalSource};
use std::collections::{HashMap, HashSet};

pub struct UrcmCtx<T> {
data: T,
atoms: HashMap<char, Atom<T>>,
reactives: HashMap<char, Vec<char>>, // dipendenze inverse
epoch: usize, // contatore modifiche per lazy update
}

impl<T> UrcmCtx<T> {
pub fn new(data: T) -> Self {
Self {
data,
atoms: HashMap::new(),
reactives: HashMap::new(),
epoch: 0,
}
}

// Registra dipendenze
fn register_deps(&mut self, symbol: char, deps: HashSet<char>) {
for &dep in &deps {
self.reactives.entry(dep)
.or_insert_with(Vec::new)
.push(symbol);
}
}

// Definizione atomi
pub fn def_number(&mut self, symbol: char, label: impl Into<String>, value: f64) -> &mut Self {
self.atoms.insert(symbol, Atom::new_number(symbol, label, value));
self
}

pub fn def_computed(
&mut self,
symbol: char,
label: impl Into<String>,
func: impl Fn(&T) -> f64 + Send + Sync + 'static,
deps: HashSet<char>
) -> &mut Self {
self.register_deps(symbol, deps.clone());
self.atoms.insert(symbol, Atom::new_computed(symbol, label, func, deps));
self
}

pub fn def_promise(
&mut self,
symbol: char,
label: impl Into<String>,
future: impl PromiseState<T> + Send + Sync + 'static,
deps: HashSet<char>
) -> &mut Self {
self.register_deps(symbol, deps.clone());
self.atoms.insert(symbol, Atom::new_promise(symbol, label, future, deps));
self
}

pub fn def_external(
&mut self,
symbol: char,
label: impl Into<String>,
source: impl ExternalSource + Send + Sync + 'static,
initial: f64
) -> &mut Self {
self.atoms.insert(symbol, Atom::new_external(symbol, label, source, initial));
self
}

// Verifica se dipendenze sono cambiate
#[allow(unused_variables)]
fn dependencies_changed(&self, deps: &HashSet<char>) -> bool {
// TODO: implementare tracciamento versioni per atomo
true
}

// Ottiene valore di un atomo (con aggiornamento lazy)
pub fn get(&mut self, symbol: char) -> Option<f64> {
// Ottieni l'atomo
let atom_ptr = self.atoms.get(&symbol)? as *const Atom<T>;

// Determina il tipo e aggiorna se necessario
match unsafe { &(*atom_ptr).impl_type } {
AtomImpl::Number { value } => {
Some(*value)
}

AtomImpl::Computed { func, deps, cached, last_epoch } => {
// Rendi mutabili i puntatori
let cached_ptr = cached as *const Option<f64> as *mut Option<f64>;
let last_epoch_ptr = last_epoch as *const usize as *mut usize;

let cached_mut = unsafe { cached_ptr.as_mut().unwrap() };
let last_epoch_mut = unsafe { last_epoch_ptr.as_mut().unwrap() };

if self.dependencies_changed(deps) || *last_epoch_mut != self.epoch {
// Ricalcola
let new_val = func(&self.data);
*cached_mut = Some(new_val);
*last_epoch_mut = self.epoch;
}
*cached_mut
}

AtomImpl::Promise { future, value, .. } => {
    // Promise non ancora completamente implementate
    // Per ora restituiamo il valore corrente se presente
    *value
}

AtomImpl::External { value, source } => {
    // External non ancora completamente implementato
    // Per ora restituiamo il valore corrente
    Some(*value)
}
}
}

// Imposta valore di un atomo numerico (con propagazione)
pub fn set(&mut self, symbol: char, value: f64) -> Option<Vec<char>> {
if let Some(atom) = self.atoms.get_mut(&symbol) {
match &mut atom.impl_type {
AtomImpl::Number { value: v } => {
*v = value;
self.epoch += 1;
return self.reactives.get(&symbol).cloned();
}
_ => return None,
}
}
None
}

// Forza aggiornamento di un atomo reattivo
pub fn update(&mut self, symbol: char) -> Option<f64> {
self.epoch += 1;
self.get(symbol)
}

// Lista simboli
pub fn symbols(&self) -> Vec<char> {
self.atoms.keys().copied().collect()
}

// Debug
pub fn dump(&self) {
println!("n📊 Stato URCM:");
for (k, atom) in &self.atoms {
let val = match &atom.impl_type {
AtomImpl::Number { value } => format!("{}", value),
AtomImpl::Computed { cached, .. } => format!("{:?}", cached),
AtomImpl::Promise { value, .. } => format!("{:?}", value),
AtomImpl::External { value, .. } => format!("{}", value),
};
let react = if atom.is_react() { "⚡" } else { " " };
println!("  {}{} = {} ({})", react, k, val, atom.label);
}
}
}

