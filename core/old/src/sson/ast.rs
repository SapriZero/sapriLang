//! AST per il dizionario strutturale .sson

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// ============================================================================
// TIPI MINIMALI (1-2 char normalizzati)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeCode {
    #[serde(rename = "s")] Str,
    #[serde(rename = "n")] Num,
    #[serde(rename = "b")] Bool,
    #[serde(rename = "d")] Date,
    #[serde(rename = "t")] Time,
    #[serde(rename = "p")] Path,
    #[serde(rename = "e")] Enum,
    #[serde(rename = "r")] Ref,
    #[serde(rename = "*")] Any,
}

impl TypeCode {
    pub fn from_short(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "s" | "str" | "string" => Some(Self::Str),
            "n" | "num" | "number" => Some(Self::Num),
            "b" | "bool" | "boolean" => Some(Self::Bool),
            "d" | "date" => Some(Self::Date),
            "t" | "time" | "timestamp" => Some(Self::Time),
            "p" | "path" => Some(Self::Path),
            "e" | "enum" => Some(Self::Enum),
            "r" | "ref" | "reference" => Some(Self::Ref),
            "*" | "any" => Some(Self::Any),
            _ => None,
        }
    }
    pub fn to_short(self) -> &'static str {
        match self {
            Self::Str => "s", Self::Num => "n", Self::Bool => "b",
            Self::Date => "d", Self::Time => "t", Self::Path => "p",
            Self::Enum => "e", Self::Ref => "r", Self::Any => "*",
        }
    }
}

impl std::str::FromStr for TypeCode {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::from_short(s).ok_or_else(|| format!("Unknown type code: '{}'", s))
    }
}

impl Default for TypeCode {
    fn default() -> Self { Self::Str }
}

// ============================================================================
// MODALITÀ OPERATIVA
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SsonMode {
    #[serde(rename = "strict")] Strict,
    #[serde(rename = "generative")] Generative,
}

impl Default for SsonMode {
    fn default() -> Self { Self::Generative }
}

// ============================================================================
// PROPRIETÀ CAMPO (evocatore `_: `)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldProperty {
    Required,
    Optional,
    Default(serde_json::Value),
    Enum(Vec<String>),
    Range { min: Option<f64>, max: Option<f64> },
    Pattern(String),
    Length { min: Option<usize>, max: Option<usize> },
    Mutex(Vec<String>),
    Guard { path: String, expected: serde_json::Value },
    Implies { if_field: String, then_field: String },
    Sum(Vec<String>),
    RefTarget(String),
    TypeOverride(TypeCode),
    Description(String),
    SeeAlso(Vec<String>),
    StateTransition { from: String, to: Vec<String> },
    Null,
}

impl FieldProperty {
    pub fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        if raw == "req" || raw == "required" { return Some(Self::Required); }
        if raw == "opt" || raw == "optional" { return Some(Self::Optional); }
        if raw == "null" || raw == "NULL" { return Some(Self::Null); }

        if let Some(val) = raw.strip_prefix("default=") {
            return Some(Self::Default(serde_json::Value::String(val.to_string())));
        }
        if let Some(n) = raw.strip_prefix("min=") {
            if let Ok(num) = n.parse::<f64>() {
                return Some(Self::Range { min: Some(num), max: None });
            }
        }
        if let Some(n) = raw.strip_prefix("max=") {
            if let Ok(num) = n.parse::<f64>() {
                return Some(Self::Range { max: Some(num), min: None });
            }
        }
        if let Some(n) = raw.strip_prefix("len=") {
            if let Ok(num) = n.parse::<usize>() {
                return Some(Self::Length { max: Some(num), min: None });
            }
        }
        if let Some(p) = raw.strip_prefix("pattern=") {
            return Some(Self::Pattern(p.trim_matches('"').to_string()));
        }
        if let Some(d) = raw.strip_prefix("desc=") {
            return Some(Self::Description(d.trim_matches('"').to_string()));
        }
        if raw.ends_with("[]") {
            return Some(Self::RefTarget(raw[..raw.len()-2].to_string()));
        }
        if let Some(name) = raw.strip_prefix("ref=") {
            return Some(Self::RefTarget(name.to_string()));
        }
        if raw.starts_with("mutex=[") && raw.ends_with(']') {
            let inner = &raw[7..raw.len()-1];
            let fields: Vec<String> = inner.split(',').map(|s| s.trim().to_string()).collect();
            return Some(Self::Mutex(fields));
        }
        if raw.starts_with('[') && raw.ends_with(']') {
            let inner = &raw[1..raw.len()-1];
            let values: Vec<String> = inner.split(',').map(|s| s.trim().to_string()).collect();
            return Some(Self::Enum(values));
        }
        if let Some(tc) = TypeCode::from_short(raw) {
            return Some(Self::TypeOverride(tc));
        }
        None
    }
}

// ============================================================================
// NODO CAMPO (dizionario piatto)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldNode {
    pub path: String,
    #[serde(default)]
    pub type_code: TypeCode,
    #[serde(default)]
    pub required: bool,
    pub default: Option<serde_json::Value>,
    #[serde(default)]
    pub constraints: Vec<FieldProperty>,
    #[serde(default)]
    pub resolved_refs: Vec<String>,
    #[serde(default)]
    pub values: Vec<serde_json::Value>,
    #[serde(default = "default_one")]
    pub validation_score: f64,
    pub defined_at: Option<Location>,
}

fn default_one() -> f64 { 1.0 }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

// ============================================================================
// TABELLA DATI
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataTable {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    #[serde(default)]
    pub inline_props: HashMap<usize, Vec<FieldProperty>>,
}

// ============================================================================
// REGOLA DI VINCOLO
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintRule {
    pub id: String,
    pub kind: ConstraintKind,
    pub target_fields: Vec<String>,
    pub params: serde_json::Value,
    #[serde(default)]
    pub active: bool,
    pub error_msg: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintKind {
    Mutex,
    Guard { path: String, expected: serde_json::Value },
    Sum { operands: Vec<String>, target: String },
    Range { min: Option<f64>, max: Option<f64> },
    Pattern(String),
    StateTransition { from: String, to: Vec<String> },
    Implies { antecedent: String, consequent: String },
    Reference { target_list: String },
}

// ============================================================================
// DIZIONARIO PIATTO
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlatDict {
    pub path_index: HashMap<String, usize>,
    pub nodes: Vec<FieldNode>,
    #[serde(default)]
    pub tables: Vec<DataTable>,
    #[serde(default)]
    pub constraints: Vec<ConstraintRule>,
    #[serde(default)]
    pub pending_refs: Vec<PendingRef>,
    #[serde(default)]
    pub mode: SsonMode,
    #[serde(default = "default_one")]
    pub s_global: f64,
    pub validation_hash: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingRef {
    pub field_path: String,
    pub ref_target: String,
    pub resolved: bool,
}

impl FlatDict {
    pub fn new() -> Self {
        Self {
            path_index: HashMap::new(),
            nodes: Vec::new(),
            tables: Vec::new(),
            constraints: Vec::new(),
            pending_refs: Vec::new(),
            mode: SsonMode::default(),
            s_global: 1.0,
            validation_hash: 0,
        }
    }
    pub fn add_field(&mut self, node: FieldNode) {
        let idx = self.nodes.len();
        self.path_index.insert(node.path.clone(), idx);
        self.nodes.push(node);
    }
    pub fn get_by_path(&self, path: &str) -> Option<&FieldNode> {
        self.path_index.get(path).and_then(|&idx| self.nodes.get(idx))
    }
    pub fn get_mut_by_path(&mut self, path: &str) -> Option<&mut FieldNode> {
        let idx = *self.path_index.get(path)?;
        self.nodes.get_mut(idx)
    }
}

// ============================================================================
// DOCUMENTO .sson COMPLETO
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SsonDocument {
    pub meta: HashMap<String, String>,
    #[serde(default)]
    pub mode: SsonMode,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    pub dictionary: FlatDict,
    #[serde(default)]
    pub warnings: Vec<ParseWarning>,
    #[serde(default)]
    pub errors: Vec<ParseError>,
    #[serde(default)]
    pub stats: ParseStats,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseWarning {
    pub code: String,
    pub message: String,
    pub location: Option<Location>,
    pub recoverable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseError {
    pub code: String,
    pub message: String,
    pub location: Option<Location>,
    pub fatal: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ParseStats {
    pub total_lines: usize,
    pub sections_parsed: usize,
    pub fields_defined: usize,
    pub rows_parsed: usize,
    pub constraints_added: usize,
    pub refs_resolved: usize,
    pub parse_time_ms: f64,
}

// ============================================================================
// REPORT DI VALIDAZIONE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub meta: ReportMeta,
    pub summary: ReportSummary,
    pub violations: Vec<Violation>,
    pub warnings: Vec<Warning>,
    pub state_machines: Vec<StateMachineReport>,
    pub ai_patches: Vec<AiPatch>,
    pub constraint_graph: ConstraintGraphReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMeta {
    pub parser_version: String,
    pub mode: SsonMode,
    pub timestamp: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total_nodes: usize,
    pub valid_nodes: usize,
    pub violations_count: usize,
    pub warnings_count: usize,
    pub s_equilibrium: f64,
    pub exportable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: ViolationType,
    pub severity: Severity,
    pub location: ErrorLocation,
    pub constraint: String,
    pub message: String,
    pub ai_hint: Option<AiHint>,
    pub s_local: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationType {
    #[serde(rename = "mutex_violation")] MutexViolation,
    #[serde(rename = "guard_fail")] GuardFail,
    #[serde(rename = "circular_ref")] CircularRef,
    #[serde(rename = "missing_req")] MissingRequired,
    #[serde(rename = "type_mismatch")] TypeMismatch,
    #[serde(rename = "state_impossible")] StateImpossible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    #[serde(rename = "error")] Error,
    #[serde(rename = "warning")] Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLocation {
    pub path: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiHint {
    pub action: PatchAction,
    pub target_path: String,
    pub reason: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatchAction {
    #[serde(rename = "add_field")] AddField,
    #[serde(rename = "remove_field")] RemoveField,
    #[serde(rename = "change_type")] ChangeType,
    #[serde(rename = "update_constraint")] UpdateConstraint,
    #[serde(rename = "inject_default")] InjectDefault,
    #[serde(rename = "break_cycle")] BreakCycle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub message: String,
    pub location: Option<ErrorLocation>,
    pub s_local: f64,
    pub recovery_applied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachineReport {
    pub name: String,
    pub current_state: String,
    pub valid_transitions: Vec<String>,
    pub dead_ends: Vec<String>,
    pub cycles_detected: bool,
    pub ttl_remaining: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiPatch {
    pub patch_id: String,
    pub target_path: String,
    pub operation: PatchAction,
    pub new_value: Option<serde_json::Value>,
    pub constraint_ref: Option<String>,
    pub confidence: f64,
    pub dry_run_result: Option<DryRunResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DryRunResult {
    pub s_global_after: f64,
    pub violations_after: usize,
    pub warnings_after: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintGraphReport {
    pub total_constraints: usize,
    pub active: usize,
    pub broken: usize,
    pub circuit_breakers: usize,
    pub s_map: HashMap<String, f64>,
}

// ============================================================================
// UTILITÀ
// ============================================================================

impl FieldNode {
    pub fn new(path: String, type_code: TypeCode) -> Self {
        Self {
            path, type_code, required: false, default: None,
            constraints: Vec::new(), resolved_refs: Vec::new(),
            values: Vec::new(), validation_score: 1.0, defined_at: None,
        }
    }
    pub fn with_required(mut self, req: bool) -> Self { self.required = req; self }
    pub fn add_constraint(mut self, prop: FieldProperty) -> Self {
        self.constraints.push(prop); self
    }
}

impl ConstraintRule {
    pub fn new(id: String, kind: ConstraintKind, targets: Vec<String>) -> Self {
        Self { id, kind, target_fields: targets, params: serde_json::Value::Null, active: true, error_msg: None }
    }
    pub fn with_params(mut self, params: serde_json::Value) -> Self { self.params = params; self }
    pub fn with_error(mut self, msg: String) -> Self { self.error_msg = Some(msg); self }
}
