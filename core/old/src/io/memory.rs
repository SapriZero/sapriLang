//! Connettore in memoria (per test e caching)

use super::{Connector, ConnectorResult, ConnectorError, ConnectorMetadata, ResourceKind};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone)]
struct MemoryEntry {
data: Vec<u8>,
created: SystemTime,
modified: SystemTime,
}

pub struct MemoryConnector {
storage: HashMap<String, MemoryEntry>,
}

impl MemoryConnector {
pub fn new() -> Self {
Self {
storage: HashMap::new(),
}
}

pub fn clear(&mut self) {
self.storage.clear();
}

pub fn keys(&self) -> Vec<String> {
self.storage.keys().cloned().collect()
}

pub fn len(&self) -> usize {
self.storage.len()
}

pub fn is_empty(&self) -> bool {
self.storage.is_empty()
}

fn now() -> SystemTime {
SystemTime::now()
}
}

impl Connector for MemoryConnector {
fn read(&self, path: &str) -> ConnectorResult<Vec<u8>> {
self.storage.get(path)
.map(|e| e.data.clone())
.ok_or_else(|| ConnectorError::new(format!("Risorsa non trovata: {}", path)))
}

fn write(&mut self, path: &str, data: &[u8]) -> ConnectorResult<()> {
let now = Self::now();
let entry = MemoryEntry {
data: data.to_vec(),
created: self.storage.get(path).map(|e| e.created).unwrap_or(now),
modified: now,
};
self.storage.insert(path.to_string(), entry);
Ok(())
}

fn delete(&mut self, path: &str) -> ConnectorResult<()> {
self.storage.remove(path)
.ok_or_else(|| ConnectorError::new(format!("Risorsa non trovata: {}", path)))?;
Ok(())
}

fn append(&mut self, path: &str, data: &[u8]) -> ConnectorResult<()> {
let now = Self::now();
let entry = self.storage.entry(path.to_string()).or_insert_with(|| MemoryEntry {
data: Vec::new(),
created: now,
modified: now,
});
entry.data.extend_from_slice(data);
entry.modified = now;
Ok(())
}

fn exists(&self, path: &str) -> bool {
self.storage.contains_key(path)
}

fn metadata(&self, path: &str) -> ConnectorResult<ConnectorMetadata> {
self.storage.get(path)
.map(|e| ConnectorMetadata {
size: Some(e.data.len() as u64),
kind: ResourceKind::File,
modified: Some(e.modified),
created: Some(e.created),
})
.ok_or_else(|| ConnectorError::new(format!("Risorsa non trovata: {}", path)))
}
}

impl Default for MemoryConnector {
fn default() -> Self {
Self::new()
}
}

