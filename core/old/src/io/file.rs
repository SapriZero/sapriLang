use super::{Connector, ConnectorResult, ConnectorError, ConnectorMetadata, ResourceKind};
use std::fs;
use std::path::{Path, PathBuf};

pub struct FileConnector {
base_path: PathBuf,
}

impl FileConnector {
pub fn new(base_path: impl AsRef<Path>) -> Self {
Self {
base_path: base_path.as_ref().to_path_buf(),
}
}

fn resolve_path(&self, path: &str) -> PathBuf {
let clean = path.trim_start_matches('/');
self.base_path.join(clean)
}
}

impl Connector for FileConnector {
fn read(&self, path: &str) -> ConnectorResult<Vec<u8>> {
let full = self.resolve_path(path);
fs::read(&full).map_err(|e| ConnectorError::with_source(
format!("Errore lettura file: {}", full.display()), e
))
}

fn write(&mut self, path: &str, data: &[u8]) -> ConnectorResult<()> {
let full = self.resolve_path(path);

if let Some(parent) = full.parent() {
fs::create_dir_all(parent).map_err(|e| ConnectorError::with_source(
format!("Errore creazione directory: {}", parent.display()), e
))?;
}

fs::write(&full, data).map_err(|e| ConnectorError::with_source(
format!("Errore scrittura file: {}", full.display()), e
))
}

fn delete(&mut self, path: &str) -> ConnectorResult<()> {
let full = self.resolve_path(path);

if full.is_dir() {
fs::remove_dir_all(&full).map_err(|e| ConnectorError::with_source(
format!("Errore cancellazione directory: {}", full.display()), e
))
} else {
fs::remove_file(&full).map_err(|e| ConnectorError::with_source(
format!("Errore cancellazione file: {}", full.display()), e
))
}
}

fn append(&mut self, path: &str, data: &[u8]) -> ConnectorResult<()> {
use std::io::Write;

let full = self.resolve_path(path);
let mut file = fs::OpenOptions::new()
.create(true)
.append(true)
.open(&full)
.map_err(|e| ConnectorError::with_source(
format!("Errore apertura file in append: {}", full.display()), e
))?;

file.write_all(data).map_err(|e| ConnectorError::with_source(
format!("Errore scrittura in append: {}", full.display()), e
))
}

fn exists(&self, path: &str) -> bool {
self.resolve_path(path).exists()
}

fn metadata(&self, path: &str) -> ConnectorResult<ConnectorMetadata> {
let full = self.resolve_path(path);
let meta = fs::metadata(&full).map_err(|e| ConnectorError::with_source(
format!("Errore lettura metadata: {}", full.display()), e
))?;

let kind = if meta.is_dir() {
ResourceKind::Directory
} else {
ResourceKind::File
};

Ok(ConnectorMetadata {
size: Some(meta.len()),
kind,
modified: meta.modified().ok(),
created: meta.created().ok(),
})
}
}

