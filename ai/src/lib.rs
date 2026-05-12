//! SAPRI AI - Intelligenza Artificiale basata su URCM
//!
//! Moduli:
//! - brain: cervello principale
//! - conversation: gestione conversazioni
//! - memory: memoria olografica
//! - learning: apprendimento
//! - knowledge: base di conoscenza

pub mod brain;
pub mod conversation;
pub mod memory;
pub mod learning;
pub mod knowledge;

pub use brain::Brain;
pub use conversation::Conversation;
pub use memory::HolographicMemory;
pub use learning::Learner;
pub use knowledge::KnowledgeBase;
