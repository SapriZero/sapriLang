//! Validatore e report generatore

use crate::sson::ast::*;
use crate::sson::dict::FieldDict;
use std::collections::HashMap;
use crate::sson::error::Result;
pub use crate::sson::ast::ValidationReport;

pub struct Validator {
	#[allow(dead_code)] 
    rules: Vec<ValidationRule>,
}

#[derive(Debug, Clone)]
pub enum ValidationRule {
    RequiredField { section: String, field: String },
    ValidType,
    UniqueNames,
    NoCircularDeps,
}

impl Validator {
    pub fn new() -> Self {
        Self { rules: vec![
            ValidationRule::RequiredField { section: "module".into(), field: "desc".into() },
            ValidationRule::ValidType,
            ValidationRule::UniqueNames,
        ]}
    }

     pub fn validate(&self, dict: &FieldDict) -> Result<ValidationReport> {
        let mut report = ValidationReport {
            meta: ReportMeta {
                parser_version: crate::sson::VERSION.into(),
                mode: dict.mode,
                timestamp: chrono::Utc::now().to_rfc3339(),
                source: "unknown".into(),
            },
            summary: ReportSummary {
                total_nodes: dict.nodes.len(),
                valid_nodes: dict.nodes.iter().filter(|n| n.validation_score > 0.5).count(),
                violations_count: dict.violations,
                warnings_count: dict.warnings,
                s_equilibrium: dict.s_global,
                exportable: dict.s_global >= 0.9 && dict.violations == 0,
            },
            violations: Vec::new(),
            warnings: Vec::new(),
            state_machines: Vec::new(),
            ai_patches: Vec::new(),
            constraint_graph: ConstraintGraphReport {
                total_constraints: dict.nodes.iter().map(|n| n.constraints.len()).sum(),
                active: 0, broken: dict.violations, circuit_breakers: 0,
                s_map: HashMap::new(),
            },
        };

        // Popola violazioni e warning
        for (idx, node) in dict.nodes.iter().enumerate() {
            if node.validation_score < 0.5 {
                report.violations.push(Violation {
                    id: format!("ERR_FIELD_{:03}", idx),
                    kind: ViolationType::MissingRequired,
                    severity: Severity::Error,
                    location: ErrorLocation { path: node.path.clone(), line: None, column: None },
                    constraint: "required".into(),
                    message: format!("Campo '{}' non soddisfa i vincoli", node.path),
                    ai_hint: Some(AiHint {
                        action: PatchAction::InjectDefault,
                        target_path: node.path.clone(),
                        reason: "Campo richiesto mancante".into(),
                        confidence: 0.95,
                    }),
                    s_local: node.validation_score,
                });
            }
        }

        Ok(report)
    }
}
