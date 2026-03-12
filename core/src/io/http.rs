//! Connettore per richieste HTTP

use super::{Connector, ConnectorResult, ConnectorError, ConnectorMetadata, ResourceKind};
use std::collections::HashMap;

pub struct HttpConnector {
base_url: String,
client: reqwest::blocking::Client,
headers: HashMap<String, String>,
}

impl HttpConnector {
pub fn new(base_url: impl Into<String>) -> Self {
Self {
base_url: base_url.into(),
client: reqwest::blocking::Client::new(),
headers: HashMap::new(),
}
}

pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
self.headers = headers;
self
}

pub fn add_header(&mut self, key: impl Into<String>, value: impl Into<String>) {
self.headers.insert(key.into(), value.into());
}

fn build_url(&self, path: &str) -> String {
let base = self.base_url.trim_end_matches('/');
let clean = path.trim_start_matches('/');
format!("{}/{}", base, clean)
}

fn build_request(&self, method: reqwest::Method, url: &str) -> reqwest::blocking::RequestBuilder {
let mut builder = self.client.request(method, url);
for (k, v) in &self.headers {
builder = builder.header(k, v);
}
builder
}
}

impl Connector for HttpConnector {
fn read(&self, path: &str) -> ConnectorResult<Vec<u8>> {
let url = self.build_url(path);
let response = self.build_request(reqwest::Method::GET, &url)
.send()
.map_err(|e| ConnectorError::with_source(format!("Errore HTTP GET: {}", url), e))?;

if !response.status().is_success() {
return Err(ConnectorError::new(format!("HTTP {}: {}", response.status(), url)));
}

response.bytes()
.map(|b| b.to_vec())
.map_err(|e| ConnectorError::with_source("Errore lettura body", e))
}

fn write(&mut self, path: &str, data: &[u8]) -> ConnectorResult<()> {
let url = self.build_url(path);
let response = self.build_request(reqwest::Method::POST, &url)
.body(data.to_vec())
.send()
.map_err(|e| ConnectorError::with_source(format!("Errore HTTP POST: {}", url), e))?;

if !response.status().is_success() {
return Err(ConnectorError::new(format!("HTTP {}: {}", response.status(), url)));
}

Ok(())
}

fn delete(&mut self, path: &str) -> ConnectorResult<()> {
let url = self.build_url(path);
let response = self.build_request(reqwest::Method::DELETE, &url)
.send()
.map_err(|e| ConnectorError::with_source(format!("Errore HTTP DELETE: {}", url), e))?;

if !response.status().is_success() {
return Err(ConnectorError::new(format!("HTTP {}: {}", response.status(), url)));
}

Ok(())
}

fn exists(&self, path: &str) -> bool {
let url = self.build_url(path);
match self.build_request(reqwest::Method::HEAD, &url).send() {
Ok(resp) => resp.status().is_success(),
Err(_) => false,
}
}

fn metadata(&self, path: &str) -> ConnectorResult<ConnectorMetadata> {
let url = self.build_url(path);
let response = self.build_request(reqwest::Method::HEAD, &url)
.send()
.map_err(|e| ConnectorError::with_source(format!("Errore HTTP HEAD: {}", url), e))?;

if !response.status().is_success() {
return Err(ConnectorError::new(format!("HTTP {}: {}", response.status(), url)));
}

let size = response.headers()
.get(reqwest::header::CONTENT_LENGTH)
.and_then(|v| v.to_str().ok())
.and_then(|s| s.parse::<u64>().ok());

Ok(ConnectorMetadata {
size,
kind: ResourceKind::Stream,
modified: None,
created: None,
})
}
}

