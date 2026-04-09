//! Obj - Struttura dinamica a oggetti con path
//!
//! Permette accesso via path array, merge, e reattività

use std::collections::HashMap;
use std::any::Any;

type Key = String;

#[derive(Debug, Clone, Default)]
pub struct Obj {
    data: HashMap<Key, Value>,
    children: HashMap<Key, Obj>,
    listeners: HashMap<Key, Vec<Box<dyn Fn(&Value)>>>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Action(Box<dyn Fn()>),
    Any(Box<dyn Any + Send + Sync>),
}

impl Obj {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(mut self, path: &[Key], value: impl Into<Value>) -> Self {
        if path.is_empty() {
            return self;
        }

        if path.len() == 1 {
            let key = path[0].clone();
            self.data.insert(key, value.into());
            self.trigger_change(path);
        } else {
            let key = path[0].clone();
            let child = self.children
                .remove(&key)
                .unwrap_or_default()
                .set(&path[1..], value);
            self.children.insert(key, child);
        }

        self
    }

    pub fn get(&self, path: &[Key]) -> Option<&Value> {
        if path.is_empty() {
            return None;
        }

        if path.len() == 1 {
            self.data.get(&path[0])
        } else {
            self.children.get(&path[0])
                .and_then(|child| child.get(&path[1..]))
        }
    }

    pub fn merge(self, other: Obj) -> Self {
        let mut result = self;
        for (k, v) in other.data {
            result.data.insert(k, v);
        }
        for (k, child) in other.children {
            let merged = result.children
                .remove(&k)
                .unwrap_or_default()
                .merge(child);
            result.children.insert(k, merged);
        }
        result
    }

    pub fn on_change(&self, path: &[Key], f: impl Fn(&Value) + 'static) {
        // Implementazione listener
    }

    fn trigger_change(&self, path: &[Key]) {
        if let Some(listeners) = self.listeners.get(&path.join(".")) {
            if let Some(value) = self.get(path) {
                for listener in listeners {
                    listener(value);
                }
            }
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Value::Number(n as f64)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl<F: Fn() + 'static> From<F> for Value {
    fn from(f: F) -> Self {
        Value::Action(Box::new(f))
    }
}
