//! Handler per i pattern (hard-coded, pochi)

use sapri_obj::Obj;

pub struct HandlerRegistry {
    // Mappa nome_handler → funzione
}

impl HandlerRegistry {
    pub fn new() -> Self {
        let mut registry = Self { /* ... */ };
        registry.register_defaults();
        registry
    }
    
    fn register_defaults(&mut self) {
        // handle_struct, handle_enum, handle_impl, ...
    }
}
