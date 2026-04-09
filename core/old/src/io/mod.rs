//! Connettori I/O per URCM Core

use std::error::Error;
use std::fmt;

pub mod file;
pub mod console;
pub mod memory;

#[cfg(feature = "http")]
pub mod http;

#[derive(Debug)]
pub struct ConnectorError {
pub message: String,
pub source: Option<Box<dyn Error + Send + Sync>>,
}

impl ConnectorError {
pub fn new(msg: impl Into<String>) -> Self {
Self {
message: msg.into(),
source: None,
}
}

pub fn with_source(msg: impl Into<String>, err: impl Error + Send + Sync + 'static) -> Self {
Self {
message: msg.into(),
source: Some(Box::new(err)),
}
}
}

impl fmt::Display for ConnectorError {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
write!(f, "{}", self.message)
}
}

impl Error for ConnectorError {
fn source(&self) -> Option<&(dyn Error + 'static)> {
self.source.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
}
}

pub type ConnectorResult<T> = Result<T, ConnectorError>;

#[derive(Debug, Clone)]
pub struct ConnectorMetadata {
pub size: Option<u64>,
pub kind: ResourceKind,
pub modified: Option<std::time::SystemTime>,
pub created: Option<std::time::SystemTime>,
}

impl ConnectorMetadata {
pub fn file(size: u64) -> Self {
Self {
size: Some(size),
kind: ResourceKind::File,
modified: None,
created: None,
}
}

pub fn dir() -> Self {
Self {
size: None,
kind: ResourceKind::Directory,
modified: None,
created: None,
}
}

pub fn stream() -> Self {
Self {
size: None,
kind: ResourceKind::Stream,
modified: None,
created: None,
}
}
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceKind {
File,
Directory,
Stream,
Unknown,
}

pub trait Connector {
fn read(&self, path: &str) -> ConnectorResult<Vec<u8>>;

fn read_string(&self, path: &str) -> ConnectorResult<String> {
let data = self.read(path)?;
String::from_utf8(data).map_err(|e| ConnectorError::with_source("Errore encoding UTF-8", e))
}

fn write(&mut self, path: &str, data: &[u8]) -> ConnectorResult<()>;

fn write_string(&mut self, path: &str, data: &str) -> ConnectorResult<()> {
self.write(path, data.as_bytes())
}

fn delete(&mut self, path: &str) -> ConnectorResult<()>;

fn append(&mut self, _path: &str, _data: &[u8]) -> ConnectorResult<()> {
    Err(ConnectorError::new("Append non supportato da questo connettore"))
}

fn exists(&self, path: &str) -> bool;

fn metadata(&self, path: &str) -> ConnectorResult<ConnectorMetadata>;
}

// Metodi JSON in un trait separato per mantenere Connector dyn-compatible
pub trait JsonOperations: Connector {
fn read_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> ConnectorResult<T> {
let s = self.read_string(path)?;
serde_json::from_str(&s).map_err(|e| ConnectorError::with_source("Errore parsing JSON", e))
}

fn write_json<T: serde::Serialize>(&mut self, path: &str, data: &T) -> ConnectorResult<()> {
let json = serde_json::to_string(data)
.map_err(|e| ConnectorError::with_source("Errore serializzazione JSON", e))?;
self.write_string(path, &json)
}
}

impl<T: Connector> JsonOperations for T {}

pub use file::FileConnector;
pub use console::ConsoleConnector;
pub use memory::MemoryConnector;

#[cfg(feature = "http")]
pub use http::HttpConnector;

