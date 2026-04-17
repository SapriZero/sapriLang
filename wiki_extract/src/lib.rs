//! Processore Wikipedia con checkpoint e ripresa

pub mod checkpoint;
pub mod extractor;
pub mod storage;

pub use checkpoint::Checkpoint;
pub use extractor::extract_from_page;
pub use storage::{save_words, save_verbs, save_nouns, save_checkpoint, load_checkpoint};
