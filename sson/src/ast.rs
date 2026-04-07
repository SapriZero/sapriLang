//! AST per il dizionario strutturale .sson
//! Dizionario piatto, vincoli espliciti, modalità strict/generative

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// ============================================================================
// TIPI MINIMALI (1-2 char normalizzati)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TypeCode {
    #[serde(rename = "s")] #[default] Str,
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
            Self::Str => "s",
            Self::Num => "n",
            Self::Bool => "b",
            Self::Date => "d",
            Self::Time => "t",
            Self::Path => "p",
            Self::Enum => "e",
            Self::Ref => "r",
            Self::Any => "*",
        }
    }
}

impl std::str::FromStr for TypeCode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_short(s).ok_or_else(|| format!("Unknown type code: '{}'", s))
    }
}

// ============================================================================
// MODALITÀ OPERATIVA
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SsonMode {
    #[serde(rename = "strict")] Strict,      // Zero tolleranza, errori bloccanti
    #[serde(rename = "generative")] Generative, // Fallback, warning, continua
}

impl Default for SsonMode {
    fn default() -> Self { Self::Generative }
}

// ============================================================================
// PROPRIETÀ CAMPO (evocatore `_: `)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldProperty {
    // Vincoli di valore
    Required,                                    // _:req
    Optional,                                    // _:opt (default)
    Default(serde_json::Value),                  // _:default=val
    Enum(Vec<String>),                           // _:[a,b,c] o _:enum=[a,b]
    Range { min: Option<f64>, max: Option<f64> }, // _:min=N, _:max=N
    Pattern(String),                             // _:pattern="regex"
    Length { min: Option<usize>, max: Option<usize> }, // _:len=N
    
    // Vincoli relazionali
    Mutex(Vec<String>),                          // _:mutex[a,b,c]
    Guard { path: String, expected: serde_json::Value }, // _:guard[path=val]
    Implies { if_field: String, then_field: String }, // _:implies[A,B]
    Sum(Vec<String>),                            // _:sum[a,b]=c
    
    // Riferimenti e tipi
    RefTarget(String),                           // _:name[] o _:name
    TypeOverride(TypeCode),                      // _:str, _:num, etc.
    
    // Metadati
    Description(String),                         // _:desc="testo"
    SeeAlso(Vec<String>),                        // _:see_also[path1,path2]
    
    // Stati e transizioni
    StateTransition { from: String, to: Vec<String> }, // _:state[A→B,C]
    
    // Placeholder per implementazione futura
    Null,                                        // _:NULL
}

impl FieldProperty {
    pub fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        
        if raw == "req" || raw == "required" {
            return Some(Self::Required);
        }
        if raw == "opt" || raw == "optional" {
            return Some(Self::Optional);
        }
        if raw == "null" || raw == "NULL" {
            return Some(Self::Null);
        }
        
        // _:default=val
        if let Some(val) = raw.strip_prefix("default=") {
            return Some(Self::Default(serde_json::Value::String(val.to_string())));
        }
        
        // _:min=N, _:max=N
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
        
        // _:len=N
        if let Some(n) = raw.strip_prefix("len=") {
            if let Ok(num) = n.parse::<usize>() {
                return Some(Self::Length { max: Some(num), min: None });
            }
        }
        
        // _:pattern="..."
        if let Some(p) = raw.strip_prefix("pattern=") {
            let p = p.trim_matches('"');
            return Some(Self::Pattern(p.to_string()));
        }
        
        // _:desc="..."
        if let Some(d) = raw.strip_prefix("desc=") {
            let d = d.trim_matches('"');
            return Some(Self::Description(d.to_string()));
        }
        
        // _:ref[name] o _:name[]
        if raw.ends_with("[]") {
            let name = &raw[..raw.len()-2];
            return Some(Self::RefTarget(name.to_string()));
        }
        if let Some(name) = raw.strip_prefix("ref=") {
            return Some(Self::RefTarget(name.to_string()));
        }
        
        // _:mutex[a,b,c]
        if raw.starts_with("mutex=[") && raw.ends_with(']') {
            let inner = &raw[7..raw.len()-1];
            let fields: Vec<String> = inner.split(',').map(|s| s.trim().to_string()).collect();
            return Some(Self::Mutex(fields));
        }
        
        // _:[a,b,c] (enum inline)
        if raw.starts_with('[') && raw.ends_with(']') {
            let inner = &raw[1..raw.len()-1];
            let values: Vec<String> = inner.split(',').map(|s| s.trim().to_string()).collect();
            return Some(Self::Enum(values));
        }
        
        // Type override: _:str, _:num, etc.
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
    
    /// Tipo normalizzato (default: Str)
    #[serde(default)]
    pub type_code: TypeCode,
    
    /// Campo obbligatorio?
    #[serde(default)]
    pub required: bool,
    
    /// Valore di default (opzionale)
    pub default: Option<serde_json::Value>,
    #[serde(default)]
    
    /// Vincoli applicati
    pub constraints: Vec<FieldProperty>,
    
    /// Riferimenti risolti (per _:ref[])
    #[serde(default)]
    pub resolved_refs: Vec<String>,
    
    /// Valori raccolti dalle righe dati
    #[serde(default)]
    pub values: Vec<serde_json::Value>,
    
    /// Punteggio di validazione locale (0.0–1.0)
    #[serde(default = "default_one")]
    pub validation_score: f64,
    
    /// Contesto di definizione (per debug)
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
// TABELLA DATI (per righe CSV)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataTable {
    /// Nome della tabella (path del contesto)
    pub name: String,
    
    /// Nomi delle colonne (campi)
    pub columns: Vec<String>,
    
    /// Righe di dati (allineate a columns)
    pub rows: Vec<Vec<String>>,
    
    /// Proprietà inline estratte dalle righe
    #[serde(default)]
    pub inline_props: HashMap<usize, Vec<FieldProperty>>,
}

// ============================================================================
// REGOLA DI VINCOLO (separata per cache locality)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintRule {
    /// ID univoco del vincolo
    pub id: String,
    
    /// Tipo di vincolo
    pub kind: ConstraintKind,
    
    /// Campi coinvolti (indici nel dizionario)
    pub target_fields: Vec<String>,
    
    /// Parametri specifici del vincolo
    pub params: serde_json::Value,
    
    /// Stato: attivo o spento (circuit breaker)
    #[serde(default)]
    pub active: bool,
    
    /// Messaggio di errore se violato
    pub error_msg: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintKind {
    /// _:mutex[a,b] — al massimo uno attivo
    Mutex,
    
    /// _:guard[path=val] — valido solo se condizione vera
    Guard { path: String, expected: serde_json::Value },
    
    /// _:sum[a,b]=c — vincolo aritmetico
    Sum { operands: Vec<String>, target: String },
    
    /// _:range[min,max] — vincolo numerico
    Range { min: Option<f64>, max: Option<f64> },
    
    /// _:pattern="..." — validazione regex
    Pattern(String),
    
    /// _:state[A→B,C] — transizioni di stato ammesse
    StateTransition { from: String, to: Vec<String> },
    
    /// _:implies[A,B] — se A attivo, B obbligatorio
    Implies { antecedent: String, consequent: String },
    
    /// _:ref[name] — riferimento a lista esterna
    Reference { target_list: String },
}

// ============================================================================
// DIZIONARIO PIATTO (output principale del parser)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlatDict {
    /// Mappa path → indice nel vettore nodes
    pub path_index: HashMap<String, usize>,
    
    /// Array contiguo di nodi (cache-friendly)
    pub nodes: Vec<FieldNode>,
    
    /// Tabelle dati (se presenti righe CSV)
    #[serde(default)]
    pub tables: Vec<DataTable>,
    
    /// Vincoli globali
    #[serde(default)]
    pub constraints: Vec<ConstraintRule>,
    
    /// Riferimenti da risolvere (pre-validazione)
    #[serde(default)]
    pub pending_refs: Vec<PendingRef>,
    
    /// Modalità operativa
    #[serde(default)]
    pub mode: SsonMode,
    
    /// Equilibrio calcolato (S globale)
    #[serde(default = "default_one")]
    pub s_global: f64,
    
    /// Fingerprint per drift detection
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
// DOCUMENTO .sson COMPLETO (AST intermedio)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SsonDocument {
    /// Metadati globali (se presenti)
    pub meta: HashMap<String, String>,
    
    /// Modalità operativa (override da [_META])
    #[serde(default)]
    pub mode: SsonMode,
    
    /// Dipendenze esterne (se definite)
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    
    /// Dizionario piatto costruito
    pub dictionary: FlatDict,
    
    /// Warning raccolti durante il parsing
    #[serde(default)]
    pub warnings: Vec<ParseWarning>,
    
    /// Errori fatali (in strict mode)
    #[serde(default)]
    pub errors: Vec<ParseError>,
    
    /// Statistiche di parsing
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
// REPORT DI VALIDAZIONE (per AI e CI/CD)
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
            path,
            type_code,
            required: false,
            default: None,
            constraints: Vec::new(),
            resolved_refs: Vec::new(),
            values: Vec::new(),
            validation_score: 1.0,
            defined_at: None,
        }
    }
    
    pub fn with_required(mut self, req: bool) -> Self {
        self.required = req;
        self
    }
    
    pub fn add_constraint(mut self, prop: FieldProperty) -> Self {
        self.constraints.push(prop);
        self
    }
}

impl ConstraintRule {
    pub fn new(id: String, kind: ConstraintKind, targets: Vec<String>) -> Self {
        Self {
            id,
            kind,
            target_fields: targets,
            params: serde_json::Value::Null,
            active: true,
            error_msg: None,
        }
    }
    
    pub fn with_params(mut self, params: serde_json::Value) -> Self {
        self.params = params;
        self
    }
    
    pub fn with_error(mut self, msg: String) -> Self {
        self.error_msg = Some(msg);
        self
    }
}
