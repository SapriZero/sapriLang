//! Estrattore veloce di Wikipedia

pub mod checkpoint;
pub mod dic_parser;
pub mod extractor;
pub mod flag_map;
pub mod models;
pub mod namespace_filter;
pub mod storage;

pub use checkpoint::Checkpoint;
pub use extractor::extract_from_page;
pub use storage::{save_words, save_verbs, save_nouns, save_checkpoint, load_checkpoint, ProgressBar};
