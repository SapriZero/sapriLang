//! SAPRI AI - Motore di ragionamento strutturato
//!
//! Questo modulo implementa l'AI basata su:
//! - sapri_morph per analisi grammaticale
//! - sapri_db per conoscenza O(1)
//! - IRCM per proiezione multidimensionale
//! - MSAS per rilevamento struttura

// Moduli esistenti (da tenere)
pub mod conversation;

// Nuovi moduli
pub mod knowledge;
pub mod memory;
pub mod brain;
pub mod reader;
pub mod analyzer;
pub mod reasoner;
pub mod utils;

// Re-export principali
pub use brain::Brain;
pub use knowledge::KnowledgeBase;
pub use memory::HolographicMemory;
pub use conversation::Conversation;
