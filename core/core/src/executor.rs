//! Esecutore di codice IRCM (sapri_rust_dsl)

use sapri_rust_dsl::{Context, AtomValue, scan};
use sapri_base::Atom;
use sapri_obj::Obj;

pub struct Executor {
    context: Context,
    history: Vec<String>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
            history: Vec::new(),
        }
    }
    
    pub fn with_initial(initial: Obj) -> Self {
        let mut ctx = Context::new();
        
        for key in initial.keys() {
            if let Some(value) = initial.get(&key) {
                if let Some(n) = value.as_number() {
                    let atom = Atom::resolved(AtomValue::Number(n));
                    ctx.set(&key, atom);
                } else {
                    let atom = Atom::resolved(AtomValue::String(value.to_string()));
                    ctx.set(&key, atom);
                }
            }
        }
        
        Self {
            context: ctx,
            history: Vec::new(),
        }
    }
    
    pub fn eval(&mut self, expr: &str) -> Result<AtomValue, String> {
        match scan!(expr, &self.context) {
            Ok(atom) => {
                let value = atom.get().clone();
                self.history.push(format!("eval: {} = {:?}", expr, value));
                Ok(value)
            }
            Err(e) => Err(format!("Eval error: {}", e)),
        }
    }
    
    pub fn define(&mut self, name: &str, value: AtomValue) {
        let atom = Atom::resolved(value.clone());
        self.context.set(name, atom);
        self.history.push(format!("define: {} = {:?}", name, value));
    }
    
    pub fn define_number(&mut self, name: &str, value: f64) {
        self.define(name, AtomValue::Number(value));
    }
    
    pub fn define_string(&mut self, name: &str, value: &str) {
        self.define(name, AtomValue::String(value.to_string()));
    }
    
    pub fn get(&self, name: &str) -> Option<AtomValue> {
        self.context.get_value(name)
    }
    
    /// Lista tutti gli atomi con i loro valori
    pub fn list_atoms(&self) -> Vec<(String, String)> {
        self.context.names()
            .iter()
            .filter_map(|name| {
                self.get(name).map(|value| (name.clone(), format!("{:?}", value)))
            })
            .collect()
    }
    
    pub fn history(&self) -> &[String] {
        &self.history
    }
    
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}
