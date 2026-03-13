//! Streaming JSON processing con priorità
//!
//! Usa pjson-rs per streaming prioritario (6.3x più veloce)

use serde_json::Value;
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub enum Priority {
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone)]
pub struct StreamConfig {
    pub buffer_size: usize,
    pub high_priority_paths: Vec<String>,
    pub low_priority_paths: Vec<String>,
    pub parallel: bool,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            buffer_size: 8192,
            high_priority_paths: Vec::new(),
            low_priority_paths: Vec::new(),
            parallel: true,
        }
    }
}

pub struct StreamProcessor<R: Read, W: Write> {
    reader: R,
    writer: W,
    config: StreamConfig,
    flattener: Flattener,
}

impl<R: Read, W: Write> StreamProcessor<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader,
            writer,
            config: StreamConfig::default(),
            flattener: Flattener::new(),
        }
    }

    pub fn with_config(mut self, config: StreamConfig) -> Self {
        self.config = config;
        self
    }

    pub fn process(&mut self) -> Result<(), StreamError> {
        // TODO: implementare streaming con pjson-rs
        // Priorità: campi high priority processati prima
        Ok(())
    }

    pub fn process_parallel(&mut self) -> Result<(), StreamError> {
        // TODO: implementare processing parallelo con rayon
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
