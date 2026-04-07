//! FieldDict: Dizionario Piatto, Validazione & Calcolo S

use serde::{Serialize, Deserialize};
use crate::ast::*;
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDict {
    pub nodes: Vec<FieldNode>,
    pub mode: SsonMode,
    pub s_global: f64,
    pub violations: usize,
    pub warnings: usize,
}

impl FieldDict {
    pub fn from_doc(doc: SsonDocument) -> Self {
        let mut dict = Self {
            nodes: doc.fields,
            mode: doc.mode,
            s_global: 1.0,
            violations: 0,
            warnings: 0,
        };
        dict.validate();
        dict
    }

    pub fn validate(&mut self) {
        let mut valid_sum = 0.0;
        let mut total_weight = 0.0;
        
        // Collect indices first to avoid borrow conflicts
        let node_indices: Vec<usize> = (0..self.nodes.len()).collect();
        
        for idx in node_indices {
            let (required, constraints) = {
                let node = &self.nodes[idx];
                (node.required, node.constraints.clone())
            };
            
            let weight = if required { 1.0 } else { 0.5 };
            total_weight += weight;

            let passed = Self::evaluate_node_static(&self.nodes[idx].values, &constraints);
            if passed {
                valid_sum += weight;
                self.nodes[idx].s_local = 1.0;
            } else {
                self.nodes[idx].s_local = 0.0;
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

    fn evaluate_node_static(values: &[serde_json::Value], constraints: &[String]) -> bool {
        if values.is_empty() { return false; }
        
        for constraint in constraints {
            if constraint.starts_with("enum=[") {
                let allowed = constraint.strip_prefix("enum=[").unwrap_or("").trim_end_matches(']');
                let allowed_vals: Vec<&str> = allowed.split(',').map(|s| s.trim()).collect();
                for val in values {
                    if let serde_json::Value::String(s) = val {
                        if !allowed_vals.contains(&s.as_str()) { return false; }
                    }
                }
            }
        }
        true
    }

    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
