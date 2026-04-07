//! Parser .sson → AST + Dizionario Piatto

use crate::ast::{SsonDocument, FieldNode, SsonMode, TypeCode, FieldProperty, Table, DataTable};
use crate::error::Result;
use crate::token::{Lexer, Token};
use std::str::FromStr;

pub fn parse_sson(input: &str) -> Result<SsonDocument> {
    let mut lexer = Lexer::new(input);
    
    let mut doc = SsonDocument {
        meta: HashMap::new(),
        mode: SsonMode::Generative,
        dependencies: Vec::new(),
        dictionary: crate::ast::FlatDict::new(),
        warnings: Vec::new(),
        errors: Vec::new(),
        stats: Default::default(),
    };

    let mut current_path = String::new();
    let mut current_table: Option<Table> = None;

    loop {
        let token = lexer.next_token()?;
        match token {
            Token::Eof => break,
            Token::Separator => {
                if let Some(t) = current_table.take() {
                    doc.tables.push(t);
                }
                current_path.clear();
            }
            Token::Section(path) => {
                if let Some(t) = current_table.take() {
                    doc.tables.push(t);
                }
                current_path = path;
            }
            Token::Subsection(child) => {
                if let Some(t) = current_table.take() {
                    doc.tables.push(t);
                }
                if current_path.is_empty() {
                    current_path = child;
                } else {
                    current_path = format!("{}.{}", current_path, child);
                }
            }
            Token::KeyValue(key, val) => {
                if key == "mode" {
                    doc.mode = match val.to_lowercase().as_str() {
                        "strict" => SsonMode::Strict,
                        _ => SsonMode::Generative,
                    };
                }
            }
            Token::Comment(_, _) => {}
            Token::CsvLine(cells) => {
                // Separa proprietà _: dai dati
                let mut data_cells = Vec::new();
                let mut props = Vec::new();
                for cell in &cells {
                    if cell.starts_with("_:") {
                        props.push(cell[2..].to_string());
                    } else {
                        data_cells.push(cell.clone());
                    }
                }

                // Se è la prima riga e non ci sono dati, sono le intestazioni campi
                if let Some(ref mut table) = current_table {
                    if table.columns.is_empty() && data_cells.iter().any(|c| c.contains('_') || c.parse::<TypeCode>().is_ok()) {
                        table.columns = data_cells.clone();
                        continue;
                    }
                }

                if current_table.is_none() && !current_path.is_empty() {
                    current_table = Some(Table {
                        name: current_path.clone(),
                        columns: Vec::new(),
                        rows: Vec::new(),
                    });
                }

                if let Some(ref mut table) = current_table {
                    if table.columns.is_empty() && !data_cells.is_empty() {
                        table.columns = data_cells.clone();
                    } else {
                        table.rows.push(data_cells);
                    }
                }
            }
        }
    }

    if let Some(t) = current_table.take() {
        doc.tables.push(t);
    }

    // Build Flat Dictionary
    for table in &doc.tables {
        for (idx, col) in table.columns.iter().enumerate() {
            let (name, type_str) = parse_field_def(col);
            let type_code = TypeCode::from_str(&type_str).unwrap_or(TypeCode::Str);
            let required = col.contains(":req") || col.contains("_:req");

            let values: Vec<_> = table.rows.iter()
                .filter_map(|r| r.get(idx))
                .map(|v| {
                    if v == "null" || v.is_empty() { serde_json::Value::Null }
                    else if let Ok(n) = v.parse::<f64>() { serde_json::Value::Number(serde_json::Number::from_f64(n).unwrap_or(serde_json::Number::from(0))) }
                    else if v.to_lowercase() == "true" || v.to_lowercase() == "false" { serde_json::Value::Bool(v.to_lowercase() == "true") }
                    else { serde_json::Value::String(v.clone()) }
                })
                .collect();

            doc.fields.push(FieldNode {
                path: format!("{}.{}", table.name, name),
                type_code,
                required,
                constraints: extract_constraints(col),
                values,
                s_local: 1.0,
            });
        }
    }

    Ok(doc)
}

fn parse_field_def(raw: &str) -> (&str, &str) {
    if let Some(pos) = raw.find("_:") {
        (raw[..pos].trim(), raw[pos..].trim())
    } else if let Some(pos) = raw.find(':') {
        (raw[..pos].trim(), &raw[pos+1..])
    } else {
        (raw.trim(), "s")
    }
}

fn extract_constraints(raw: &str) -> Vec<String> {
    raw.split(",")
        .filter(|s| s.starts_with("_:") && !s.starts_with("_:req"))
        .map(|s| s.trim().trim_start_matches("_:").to_string())
        .collect()
}
