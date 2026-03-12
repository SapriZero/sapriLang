use crate::io::{Connector, ConnectorResult, ConnectorError, ConnectorMetadata, JsonOperations};
use std::collections::HashMap;

pub struct Crud {
connectors: HashMap<String, Box<dyn Connector + Send + Sync>>,
}

impl Crud {
pub fn new() -> Self {
Self {
connectors: HashMap::new(),
}
}

pub fn register<C: Connector + Send + Sync + 'static>(
&mut self,
name: impl Into<String>,
connector: C
) -> &mut Self {
self.connectors.insert(name.into(), Box::new(connector));
self
}

pub fn get(&self, name: &str) -> Option<&dyn Connector> {
self.connectors.get(name).map(|b| b.as_ref() as &dyn Connector)
}

pub fn get_mut(&mut self, name: &str) -> Option<&mut dyn Connector> {
self.connectors.get_mut(name).map(|b| b.as_mut() as &mut dyn Connector)
}

pub fn get_json<T: Connector + JsonOperations>(&self, _name: &str) -> Option<&T> {
// Questo metodo richiederebbe downcasting, non lo implementiamo
None
}

fn parse_uri(uri: &str) -> ConnectorResult<(String, String)> {
if let Some(pos) = uri.find("://") {
let connector = uri[..pos].to_string();
let path = uri[pos+3..].to_string();
Ok((connector, path))
} else {
Err(ConnectorError::new(format!("URI non valido (manca ://): {}", uri)))
}
}

pub fn read(&self, uri: &str) -> ConnectorResult<Vec<u8>> {
let (conn, path) = Self::parse_uri(uri)?;
let connector = self.get(&conn)
.ok_or_else(|| ConnectorError::new(format!("Connettore non trovato: {}", conn)))?;
connector.read(&path)
}

pub fn read_string(&self, uri: &str) -> ConnectorResult<String> {
let (conn, path) = Self::parse_uri(uri)?;
let connector = self.get(&conn)
.ok_or_else(|| ConnectorError::new(format!("Connettore non trovato: {}", conn)))?;
connector.read_string(&path)
}

// I metodi JSON richiedono l'accesso diretto al connettore
// L'utente deve fare: crud.get("nome").unwrap().read_json(...)

pub fn write(&mut self, uri: &str, data: &[u8]) -> ConnectorResult<()> {
let (conn, path) = Self::parse_uri(uri)?;
let connector = self.get_mut(&conn)
.ok_or_else(|| ConnectorError::new(format!("Connettore non trovato: {}", conn)))?;
connector.write(&path, data)
}

pub fn write_string(&mut self, uri: &str, data: &str) -> ConnectorResult<()> {
let (conn, path) = Self::parse_uri(uri)?;
let connector = self.get_mut(&conn)
.ok_or_else(|| ConnectorError::new(format!("Connettore non trovato: {}", conn)))?;
connector.write_string(&path, data)
}

pub fn delete(&mut self, uri: &str) -> ConnectorResult<()> {
let (conn, path) = Self::parse_uri(uri)?;
let connector = self.get_mut(&conn)
.ok_or_else(|| ConnectorError::new(format!("Connettore non trovato: {}", conn)))?;
connector.delete(&path)
}

pub fn append(&mut self, uri: &str, data: &[u8]) -> ConnectorResult<()> {
let (conn, path) = Self::parse_uri(uri)?;
let connector = self.get_mut(&conn)
.ok_or_else(|| ConnectorError::new(format!("Connettore non trovato: {}", conn)))?;
connector.append(&path, data)
}

pub fn exists(&self, uri: &str) -> bool {
if let Ok((conn, path)) = Self::parse_uri(uri) {
if let Some(connector) = self.get(&conn) {
return connector.exists(&path);
}
}
false
}

pub fn metadata(&self, uri: &str) -> ConnectorResult<ConnectorMetadata> {
let (conn, path) = Self::parse_uri(uri)?;
let connector = self.get(&conn)
.ok_or_else(|| ConnectorError::new(format!("Connettore non trovato: {}", conn)))?;
connector.metadata(&path)
}
}

impl Default for Crud {
fn default() -> Self {
Self::new()
}
}
