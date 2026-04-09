//! Connettore per console (stdin/stdout)

use super::{Connector, ConnectorResult, ConnectorError, ConnectorMetadata};
use std::io::{self, Write};

pub struct ConsoleConnector {
buffer: Vec<String>,
}

impl ConsoleConnector {
pub fn new() -> Self {
Self {
buffer: Vec::new(),
}
}

pub fn read_line(&self) -> ConnectorResult<String> {
let mut input = String::new();
io::stdin().read_line(&mut input)
.map_err(|e| ConnectorError::with_source("Errore lettura stdin", e))?;
Ok(input.trim().to_string())
}

pub fn flush(&mut self) -> ConnectorResult<()> {
io::stdout().flush()
.map_err(|e| ConnectorError::with_source("Errore flush stdout", e))?;
Ok(())
}

pub fn get_buffer(&self) -> &Vec<String> {
&self.buffer
}

pub fn clear_buffer(&mut self) {
self.buffer.clear();
}
}

impl Connector for ConsoleConnector {
fn read(&self, path: &str) -> ConnectorResult<Vec<u8>> {
match path {
"stdin" | "in" | "console" => {
let mut input = String::new();
io::stdin().read_line(&mut input)
.map_err(|e| ConnectorError::with_source("Errore lettura stdin", e))?;
Ok(input.into_bytes())
}
_ => Err(ConnectorError::new(format!("Path non supportato: {}", path))),
}
}

fn write(&mut self, path: &str, data: &[u8]) -> ConnectorResult<()> {
match path {
"stdout" | "out" | "console" => {
let s = String::from_utf8_lossy(data);
print!("{}", s);
self.buffer.push(s.to_string());
io::stdout().flush()
.map_err(|e| ConnectorError::with_source("Errore flush stdout", e))?;
Ok(())
}
"stderr" | "err" => {
let s = String::from_utf8_lossy(data);
eprint!("{}", s);
self.buffer.push(s.to_string());
io::stderr().flush()
.map_err(|e| ConnectorError::with_source("Errore flush stderr", e))?;
Ok(())
}
_ => Err(ConnectorError::new(format!("Path non supportato: {}", path))),
}
}

fn delete(&mut self, path: &str) -> ConnectorResult<()> {
match path {
"clear" | "cls" => {
print!("x1B[2Jx1B[1;1H");
self.flush()?;
Ok(())
}
"buffer" => {
self.clear_buffer();
Ok(())
}
_ => Err(ConnectorError::new(format!("Delete non supportato per: {}", path))),
}
}

fn exists(&self, path: &str) -> bool {
matches!(path, "stdin" | "stdout" | "stderr" | "in" | "out" | "err" | "console")
}

fn metadata(&self, path: &str) -> ConnectorResult<ConnectorMetadata> {
match path {
"stdin" | "in" | "stdout" | "out" | "stderr" | "err" | "console" => {
Ok(ConnectorMetadata::stream())
}
_ => Err(ConnectorError::new("Risorsa non trovata")),
}
}
}

impl Default for ConsoleConnector {
fn default() -> Self {
Self::new()
}
}

