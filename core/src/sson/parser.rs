//! Parser principale: da token a AST + FlatDict

use crate::sson::ast::*;
use crate::sson::lexer::Lexer;
use crate::sson::token::Token;
use crate::sson::error::{Result};
use std::collections::HashMap;

pub type ParseResult = Result<SsonDocument>;

pub fn parse_sson(input: &str) -> ParseResult {
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let parser = Parser::new(tokens);
    parser.parse()
}

struct Parser<'a> {
    tokens: std::vec::IntoIter<Token>,
    current: Option<Token>,
    current_path: Vec<String>,
    current_table: Option<DataTable>,
    doc: SsonDocument,
    _phantom: std::marker::PhantomData<&'a str>,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token>) -> Self {
        let mut iter = tokens.into_iter();
        let current = iter.next();
        Self {
            tokens: iter,
            current,
            current_path: Vec::new(),
            current_table: None,
            doc: SsonDocument::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.current.take();
        self.current = self.tokens.next();
        token
    }
	
	#[allow(dead_code)] 
    fn peek(&self) -> Option<&Token> {
        self.current.as_ref()
    }

    fn resolve_path(&mut self, path: &str) -> Vec<String> {
        if path.starts_with('.') {
            let mut resolved = self.current_path.clone();
            for part in path[1..].split('.').filter(|s| !s.is_empty()) {
                resolved.push(part.to_string());
            }
            resolved
        } else {
            let parts: Vec<String> = path.split('.').map(|s| s.to_string()).collect();
            self.current_path = parts.clone();
            parts
        }
    }

    fn parse_field_def(raw: &str) -> (String, TypeCode, Vec<FieldProperty>) {
        let raw_trimmed = raw.trim();
        let (name_part, rest_part) = if let Some(pos) = raw_trimmed.find("_:") {
            (&raw_trimmed[..pos], &raw_trimmed[pos..])
        } else if let Some(pos) = raw_trimmed.find(':') {
            (&raw_trimmed[..pos], &raw_trimmed[pos + 1..])
        } else {
            (raw_trimmed, "")
        };
        
        let name = name_part.trim().to_string();
        let rest = rest_part.trim();
        
        let type_code = if rest.is_empty() {
            TypeCode::default()
        } else if let Some(tc) = TypeCode::from_short(rest.trim_start_matches('_').trim_start_matches(':')) {
            tc
        } else {
            TypeCode::default()
        };
        
        let props: Vec<FieldProperty> = rest
            .split(',')
            .map(|s: &str| s.trim())
            .filter(|s: &&str| s.starts_with("_:"))
            .filter_map(|s: &str| FieldProperty::parse(&s[2..]))
            .collect();
        
        (name, type_code, props)
    }

    fn infer_type(value: &str) -> serde_json::Value {
        if value == "null" || value.is_empty() {
            return serde_json::Value::Null;
        }
        if let Ok(n) = value.parse::<f64>() {
            return serde_json::Value::Number(
                serde_json::Number::from_f64(n).unwrap_or(serde_json::Number::from(0))
            );
        }
        if matches!(value.to_lowercase().as_str(), "true" | "false") {
            return serde_json::Value::Bool(value.to_lowercase() == "true");
        }
        serde_json::Value::String(value.to_string())
    }

    fn extract_props_from_cells(cells: &[String]) -> (Vec<String>, Vec<FieldProperty>) {
        let mut data = Vec::new();
        let mut props = Vec::new();
        for cell in cells {
            if cell.starts_with("_:") {
                if let Some(prop) = FieldProperty::parse(&cell[2..]) {
                    props.push(prop);
                }
            } else {
                data.push(cell.clone());
            }
        }
        (data, props)
    }

    fn parse(mut self) -> ParseResult {
        while let Some(token) = self.advance() {
            match token {
                Token::Separator => {
                    if let Some(t) = self.current_table.take() {
                        self.doc.dictionary.tables.push(t);
                    }
                    self.current_path.clear();
                }
                Token::Section(path) => {
                    if let Some(t) = self.current_table.take() {
                        self.doc.dictionary.tables.push(t);
                    }
                    if path.starts_with('_') {
                        self.parse_meta_section(&path)?;
                    } else {
                        let resolved = self.resolve_path(&path);
                        let table_name = resolved.join(".");
                        self.current_table = Some(DataTable {
                            name: table_name.clone(),
                            columns: Vec::new(),
                            rows: Vec::new(),
                            inline_props: HashMap::new(),
                        });
                    }
                }
                Token::Subsection(child) => {
                    if let Some(t) = self.current_table.take() {
                        self.doc.dictionary.tables.push(t);
                    }
                    let resolved = self.resolve_path(&format!(".{}", child));
                    let table_name = resolved.join(".");
                    self.current_table = Some(DataTable {
                        name: table_name,
                        columns: Vec::new(),
                        rows: Vec::new(),
                        inline_props: HashMap::new(),
                    });
                }
                Token::KeyValue(key, value) => {
                    if key == "mode" {
                        self.doc.mode = if value.to_lowercase() == "strict" {
                            SsonMode::Strict
                        } else {
                            SsonMode::Generative
                        };
                    } else {
                        self.doc.meta.insert(key, value);
                    }
                }
                Token::CsvLine(cells) => {
                    if cells.iter().any(|c| c.contains('_') || TypeCode::from_short(c).is_some()) {
                        if let Some(ref mut table) = self.current_table {
                            if table.columns.is_empty() {
                                let mut new_fields = Vec::new();
                                for cell in &cells {
                                    let (name, tc, props) = Self::parse_field_def(cell);
                                    let mut node = FieldNode::new(
                                        format!("{}.{}", table.name, name),
                                        tc
                                    );
                                    for prop in &props {
                                        match prop {
                                            FieldProperty::Required => node.required = true,
                                            FieldProperty::Default(v) => node.default = Some(v.clone()),
                                            _ => node.constraints.push(prop.clone()),
                                        }
                                    }
                                    new_fields.push(node);
                                }
                                table.columns = cells.iter()
                                    .map(|c| Self::parse_field_def(c).0)
                                    .collect();
                                
                                for node in new_fields {
                                    self.doc.dictionary.add_field(node);
                                }
                                continue;
                            }
                        }
                    }
                    if let Some(ref mut table) = self.current_table {
                        if table.columns.is_empty() && !cells.is_empty() {
                            table.columns = cells.iter()
                                .map(|c| Self::parse_field_def(c).0)
                                .collect();
                        } else {
                            let (data_cells, _inline_props) = Self::extract_props_from_cells(&cells);
                            table.rows.push(data_cells);
                        }
                    }
                }
                Token::Comment(_, _) | Token::Eof => {}
            }
        }
        if let Some(t) = self.current_table.take() {
            self.doc.dictionary.tables.push(t);
        }
        self.build_flat_dict_from_tables()?;
        Ok(self.doc)
    }

    fn parse_meta_section(&mut self, path: &str) -> Result<()> {
        match path {
            "_META" | "_DEPS" | "_RULES" => Ok(()),
            _ => Ok(()),
        }
    }

    fn build_flat_dict_from_tables(&mut self) -> Result<()> {
        let tables_to_process: Vec<DataTable> = self.doc.dictionary.tables.clone();
        
        for table in &tables_to_process {
            for (col_idx, col_name) in table.columns.iter().enumerate() {
                let (name, tc, props) = Self::parse_field_def(col_name);
                let mut node = FieldNode::new(
                    format!("{}.{}", table.name, name),
                    tc
                );
                for prop in &props {
                    match prop {
                        FieldProperty::Required => node.required = true,
                        FieldProperty::Default(v) => node.default = Some(v.clone()),
                        _ => node.constraints.push(prop.clone()),
                    }
                }
                node.values = table.rows.iter()
                    .filter_map(|r: &Vec<String>| r.get(col_idx).cloned())
                    .map(|v: String| Self::infer_type(&v))
                    .collect();
                self.doc.dictionary.add_field(node);
            }
        }
        Ok(())
    }
}

impl Default for SsonDocument {
    fn default() -> Self {
        Self {
            meta: HashMap::new(),
            mode: SsonMode::default(),
            dependencies: Vec::new(),
            dictionary: FlatDict::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
            stats: ParseStats::default(),
        }
    }
}
