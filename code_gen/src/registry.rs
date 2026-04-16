//! Registry degli handler, popolato dai .sson

use std::collections::HashMap;
use std::sync::OnceLock;

pub type HandlerFn = Box<dyn Fn(&str, &str) -> String + Send + Sync>;

pub struct HandlerRegistry {
    handlers: HashMap<String, HandlerFn>,
}

impl HandlerRegistry {
    pub fn global() -> &'static HandlerRegistry {
        static INSTANCE: OnceLock<HandlerRegistry> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let mut registry = Self::new();
            registry.register_default_handlers();
            registry
        })
    }
    
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, name: &str, handler: HandlerFn) {
        self.handlers.insert(name.to_string(), handler);
    }
    
    pub fn get(&self, name: &str) -> Option<&HandlerFn> {
        self.handlers.get(name)
    }
    
    fn register_default_handlers(&mut self) {
        // Questi handler sono implementati in Rust
        // Ma possono essere sovrascritti da .sson se necessario
        self.register("handle_struct", Box::new(handle_struct));
        self.register("handle_struct_derive", Box::new(handle_struct_derive));
        self.register("handle_struct_fields", Box::new(handle_struct_fields));
        self.register("handle_enum", Box::new(handle_enum));
        self.register("handle_impl", Box::new(handle_impl));
        self.register("handle_impl_function", Box::new(handle_impl_function));
    }
}
