//! FieldDict: validazione, calcolo S, deduplicazione

use crate::sson::ast::*;
use crate::sson::error::{Result, SsonError};
use std::collections::{HashMap};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDict {
    pub nodes: Vec<FieldNode>,
    pub path_index: HashMap<String, usize>,
    pub mode: SsonMode,
    pub s_global: f64,
    pub violations: usize,
    pub warnings: usize,
}

impl FieldDict {
    pub fn from_document(doc: SsonDocument, mode: SsonMode) -> Self {
        let mut dict = Self {
            nodes: doc.dictionary.nodes,
            path_index: doc.dictionary.path_index,
            mode,
            s_global: 1.0,
            violations: 0,
            warnings: 0,
        };
        dict.resolve_references();
        dict
    }

    pub fn validate(&mut self) {
        let mut valid_sum = 0.0;
        let mut total_weight = 0.0;
        let node_indices: Vec<usize> = (0..self.nodes.len()).collect();

        for idx in node_indices {
            let (required, constraints, values) = {
                let node = &self.nodes[idx];
                (node.required, node.constraints.clone(), node.values.clone())
            };
            let weight = if required { 1.0 } else { 0.5 };
            total_weight += weight;
            let passed = Self::evaluate_node_static(&values, &constraints, self.mode);
            if passed {
                valid_sum += weight;
                self.nodes[idx].validation_score = 1.0;
            } else {
                self.nodes[idx].validation_score = 0.0;
                self.violations += 1;
                if self.mode == SsonMode::Strict {
                    self.s_global = 0.0;
                    return;
                } else {
                    self.warnings += 1;
                }
            }
        }
        if total_weight > 0.0 {
            let k = match self.mode { SsonMode::Strict => 1.0, SsonMode::Generative => 1.5 };
            self.s_global = (valid_sum / total_weight) / k;
        }
    }

    fn evaluate_node_static(values: &[serde_json::Value], constraints: &[FieldProperty], mode: SsonMode) -> bool {
        if values.is_empty() { return false; }
        for constraint in constraints {
            match constraint {
                FieldProperty::Enum(allowed) => {
                    for val in values {
                        if let serde_json::Value::String(s) = val {
                            if !allowed.contains(s) {
                                if mode == SsonMode::Strict { return false; }
                            }
                        }
                    }
                }
                FieldProperty::Range { min, max } => {
                    for val in values {
                        if let serde_json::Value::Number(n) = val {
                            if let Some(num) = n.as_f64() {
                                if let Some(mn) = min { if num < *mn { if mode == SsonMode::Strict { return false; } } }
                                if let Some(mx) = max { if num > *mx { if mode == SsonMode::Strict { return false; } } }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        true
    }

    fn resolve_references(&mut self) {
        // Placeholder: risolve _:ref[] cercando nel dizionario
        for node in &mut self.nodes {
            for constraint in &node.constraints {
                if let FieldProperty::RefTarget(target) = constraint {
                    if self.path_index.contains_key(target) {
                        node.resolved_refs.push(target.clone());
                    }
                }
            }
        }
    }

    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        // Placeholder: DFS per rilevare cicli nei riferimenti
        Vec::new()
    }

    pub fn export_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| SsonError::Other(e.to_string()))
    }
}
