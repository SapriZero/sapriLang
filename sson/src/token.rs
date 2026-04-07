//! Lexer minimale ed elastico per .sson
//! Riconosce sezioni, proprietà _: , righe CSV e commenti.

use crate::error::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Section(String),          // [path]
    Subsection(String),       // [.child]
    KeyValue(String, String), // key = value
    CsvLine(Vec<String>),     // val1, val2, "val,3"
    Comment(bool, String),    // bool = true se docstring ///
    Separator,
    Eof,
}

pub struct Lexer<'a> {
    lines: std::vec::IntoIter<&'a str>,
    line_num: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let lines: Vec<&'a str> = input.lines().collect();
        Self { lines: lines.into_iter(), line_num: 0 }
    }

    fn parse_value(raw: &str) -> String {
        let trimmed = raw.trim();
        if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
            trimmed[1..trimmed.len() - 1].to_string()
        } else {
            trimmed.to_string()
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        loop {
            self.line_num += 1;
            let line = match self.lines.next() {
                Some(l) => l.trim_end_matches('\r'),
                None => return Ok(Token::Eof),
            };
            let line = line.trim();

            if line.is_empty() || line.starts_with("///") {
                continue; // ignora righe vuote e docstring nel flusso base
            }
            if line.starts_with("//") {
                continue;
            }
            if line.starts_with('=') {
                return Ok(Token::Separator);
            }
            if line.starts_with('[') {
                let inner = line.trim_matches(|c: char| c == '[' || c == ']' || c.is_whitespace());
                return if inner.starts_with('.') {
                    Ok(Token::Subsection(inner[1..].to_string()))
                } else {
                    Ok(Token::Section(inner.to_string()))
                };
            }
            // Riga CSV o KeyValue
            if line.contains('=') && !line.contains(',') {
                let mut parts = line.splitn(2, '=');
                let k = parts.next().unwrap_or("").trim().to_string();
                let v = parts.next().unwrap_or("").trim().to_string();
                return Ok(Token::KeyValue(k, v));
            }
            // CSV splitting rispettando virgolette
            let mut cells = Vec::new();
            let mut current = String::new();
            let mut in_quotes = false;
            for c in line.chars() {
                match c {
                    '"' if !in_quotes => in_quotes = true,
                    '"' if in_quotes => in_quotes = false,
                    ',' if !in_quotes => {
                        cells.push(Self::parse_value(&current));
                        current.clear();
                    }
                    _ => current.push(c),
                }
            }
            cells.push(Self::parse_value(&current));
            return Ok(Token::CsvLine(cells));
        }
    }
}
