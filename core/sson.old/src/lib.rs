// ============================================
// core/sson/src/lib.rs
// SSON Parser - Complete Library
// 
// Questo crate implementa il parser per il formato .sson
// con supporto per:
// - Validazione vincoli (_:req, _:min, _:max, _:pattern, _:enum, _:mutex, etc.)
// - Risoluzione riferimenti (_:ref[path])
// - Deduplicazione pattern ripetuti
// - Macchina a stati con modalità strict/generative
// - Errori dettagliati con AI hint
// - Adapter per core/obj
// ============================================

#![allow(unused_imports)]
#![allow(dead_code)]

// ============================================
// DEPENDENZE ESTERNE
// ============================================

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

// Core dependencies
use sapri_base::atom::Atom;
use sapri_base::error::{Error, Result};
use sapri_base::fp::{compose, curry, memoize, once, pipe};
use sapri_obj::{obj, Obj, Value};

// ============================================
// MODULE DECLARATIONS
// ============================================

/// Tipi base (type aliases, enums)
pub mod types;

/// Implementazione validatore constraint
pub mod validator_impl;

/// Implementazione resolver riferimenti
pub mod resolver_impl;

/// Function registry per validatori testuali
pub mod registry_impl;

/// Adapter per core/obj (flat ↔ nested)
pub mod obj_adapter_impl;

/// Macchina a stati per parser flow
pub mod state_machine_impl;

/// Errori dettagliati e recovery
pub mod error_impl;

// ============================================
// RE-EXPORT PRINCIPALI
// ============================================

// Types
pub use types::*;

// Validator
pub use validator_impl::BaseValidator;
pub use validator_impl::CompositeValidator;
pub use validator_impl::MemoizingValidator;
pub use validator_impl::validate_object;

// Resolver
pub use resolver_impl::PathResolver;
pub use resolver_impl::AliasResolver;
pub use resolver_impl::CompositeResolver;
pub use resolver_impl::PathNormalizer;
pub use resolver_impl::create_default_resolver;

// Registry
pub use registry_impl::FunctionRegistryImpl;
pub use registry_impl::ValidatorFn;
pub use registry_impl::TransformFn;
pub use registry_impl::global_registry;

// Obj Adapter
pub use obj_adapter_impl::ObjAdapterImpl;
pub use obj_adapter_impl::CachedObjAdapter;
pub use obj_adapter_impl::obj_to_json;
pub use obj_adapter_impl::json_to_obj;

// State Machine
pub use state_machine_impl::ParserStateMachine;
pub use state_machine_impl::SimpleStateMachine;
pub use state_machine_impl::ParserState;
pub use state_machine_impl::ParserAction;
pub use state_machine_impl::Transition;
pub use state_machine_impl::MachineContext;
pub use state_machine_impl::ParseStats;

// Error
pub use error_impl::ErrorHandlerImpl;
pub use error_impl::DetailedError;
pub use error_impl::ErrorCategory;
pub use error_impl::Severity;
pub use error_impl::AIHint;
pub use error_impl::AIAction;
pub use error_impl::RecoveryEntry;
pub use error_impl::DetailedResult;
pub use error_impl::IntoDetailedError;
pub use error_impl::error_code;
pub use error_impl::warning_code;
pub use error_impl::error_codes;
pub use error_impl::warning_codes;
pub use error_impl::missing_required_error;
pub use error_impl::min_value_error;
pub use error_impl::pattern_error;
pub use error_impl::circular_error;
pub use error_impl::path_not_found_error;
pub use error_impl::default_used_warning;
pub use error_impl::field_ignored_warning;
pub use error_impl::type_coerced_warning;

// ============================================
// TYPE ALIASES
// ============================================

/// Punteggio di equilibrio S = (v·i)/(t·k)
/// Range: 0.0 = totalmente squilibrato, 1.0 = equilibrio, 2.0 = tensione massima
pub type SScore = f64;

/// Path a un campo nel dizionario piatto (es. "user.address.city")
pub type FieldPath = String;

/// Nome di un constraint (es. "req", "min", "pattern")
pub type ConstraintName = String;

/// Profondità massima di annidamento
pub type MaxDepth = usize;

// ============================================
// ENUMS PRINCIPALI
// ============================================

/// Modalità di esecuzione del parser
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParserMode {
    /// Modalità stretta: primo errore → stop, exportable=false
    #[default]
    Strict,
    /// Modalità generativa: raccoglie warning, continua, tenta recovery
    Generative,
}

/// Token types dal lexer
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Sep,            // separatore
    Section,        // [sezione]
    Subsection,     // [.sottosezione]
    KeyVal,         // chiave: valore
    List,           // lista
    Comment,        // commento
    Eof,            // fine file
    PropEvoker,     // _: (property evoker)
    String,         // valore stringa
    Number,         // valore numerico
    Bool,           // valore booleano
    Null,           // valore nullo
}

/// Tipo di campo (allineato con core/obj::Value)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FieldType {
    #[default]
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
}

/// Allineamento righe CSV
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    #[default]
    Strict,
    PadNull,
    TrimExtra,
    InlinePropExtract,
}

// ============================================
// STRUCT PRINCIPALI
// ============================================

/// Configurazione globale del parser
#[derive(Debug, Clone)]
pub struct ParserConfig {
    pub mode: ParserMode,
    pub encoding: String,
    pub max_depth: MaxDepth,
    pub version: String,
    pub state: ParserState,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            mode: ParserMode::Strict,
            encoding: "utf-8".to_string(),
            max_depth: 100,
            version: "2.0".to_string(),
            state: ParserState::Init,
        }
    }
}

/// Contesto di validazione (passato ai validator)
#[derive(Debug, Clone)]
pub struct ValidationContext {
    pub mode: ParserMode,
    pub max_depth: MaxDepth,
    pub current_path: FieldPath,
    pub obj: Obj,
    pub s_score: SScore,
    pub state: ValidationState,
}

impl ValidationContext {
    pub fn new(mode: ParserMode, obj: Obj) -> Self {
        Self {
            mode,
            max_depth: 100,
            current_path: String::new(),
            obj,
            s_score: 1.0,
            state: ValidationState::Pending,
        }
    }
}

/// Stato di validazione
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidationState {
    #[default]
    Pending,
    Valid,
    Invalid,
}

/// Un constraint (es. _:req, _:min=0)
#[derive(Debug, Clone)]
pub struct Constraint {
    pub name: ConstraintName,
    pub target: FieldPath,
    pub params: Obj,
}

/// Posizione dell'errore
#[derive(Debug, Clone)]
pub struct ErrorLocation {
    pub file: Option<PathBuf>,
    pub line: usize,
    pub col: usize,
    pub path: FieldPath,
}

/// Errore di parsing/validazione (formato standard)
#[derive(Debug, Clone)]
pub struct ParseError {
    pub code: String,
    pub message: String,
    pub location: ErrorLocation,
    pub constraint: Option<Constraint>,
}

/// Warning di parsing (formato standard)
#[derive(Debug, Clone)]
pub struct ParseWarning {
    pub code: String,
    pub message: String,
    pub location: ErrorLocation,
    pub recovered: bool,
}

/// Token del lexer
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub col: usize,
}

/// Report finale del parser
#[derive(Debug, Clone)]
pub struct Report {
    pub dict: Obj,
    pub tree: Option<Obj>,
    pub errors: Vec<ParseError>,
    pub warnings: Vec<ParseWarning>,
    pub stats: ParseStats,
    pub s_score: SScore,
    pub exportable: bool,
}

// ============================================
// TRAITS PRINCIPALI
// ============================================

/// Validatore di constraint
pub trait Validator: Send + Sync {
    fn validate(&self, ctx: &ValidationContext, constraint: &Constraint) -> bool;
    
    fn validate_all(&mut self, ctx: &mut ValidationContext, constraints: &[Constraint]) -> SScore {
        let mut valid_count = 0;
        for constraint in constraints {
            if self.validate(ctx, constraint) {
                valid_count += 1;
            }
        }
        let total = constraints.len() as f64;
        if total == 0.0 { return 1.0; }
        valid_count as f64 / total
    }
}

/// Resolver di riferimenti (_:ref[path])
pub trait Resolver: Send + Sync {
    fn resolve(&mut self, ctx: &mut ValidationContext, path: &str) -> Result<Obj>;
    
    fn resolve_all(&mut self, ctx: &mut ValidationContext, paths: &[String]) -> Result<Vec<Obj>> {
        paths.iter().map(|p| self.resolve(ctx, p)).collect()
    }
    
    fn clear_cache(&mut self);
}

/// Macchina a stati per il parser
pub trait StateMachine: Send + Sync {
    fn transition(&mut self, from: ParserState, to: ParserState) -> Result<bool>;
    fn current_state(&self) -> ParserState;
    fn is_valid_transition(&self, from: ParserState, to: ParserState) -> bool;
    fn run_pipeline(&mut self, input: &str, config: &ParserConfig) -> Result<Report>;
}

/// Registry di funzioni validatrici (chiamate per nome dal .sson)
pub trait FunctionRegistry: Send + Sync {
    fn register(&mut self, name: &str, f: Box<dyn Fn(&ValidationContext, &Constraint) -> bool>);
    fn call(&self, name: &str, ctx: &ValidationContext, constraint: &Constraint) -> Option<bool>;
    fn contains(&self, name: &str) -> bool;
}

/// Adapter per core/obj (conversione flat ↔ nested)
pub trait ObjAdapter: Send + Sync {
    fn from_flat_dict(&self, flat: &HashMap<FieldPath, Value>) -> Obj;
    fn to_flat_dict(&self, obj: &Obj) -> HashMap<FieldPath, Value>;
    fn get_path(&self, obj: &Obj, path: &str) -> Result<Value>;
    fn set_path(&self, obj: &mut Obj, path: &str, value: Value) -> Result<()>;
}

/// Gestore errori con recovery
pub trait ErrorHandler: Send + Sync {
    fn add_error(&mut self, error: ParseError);
    fn add_warning(&mut self, warning: ParseWarning);
    fn recover(&mut self, error: &ParseError) -> Option<ParseWarning>;
    fn has_errors(&self) -> bool;
    fn is_exportable(&self, s_score: SScore) -> bool;
}

// ============================================
// COSTANTI GLOBALI
// ============================================

/// Soglia minima per considerare un report esportabile (S ≥ 0.9)
pub const EXPORT_THRESHOLD: SScore = 0.9;

/// Massima profondità di default
pub const DEFAULT_MAX_DEPTH: MaxDepth = 100;

/// Versione della specifica
pub const SPEC_VERSION: &str = "2.0";

/// Nomi dei validatori predefiniti
pub const BUILTIN_VALIDATORS: &[&str] = &[
    "req", "min", "max", "pattern", "enum", "range", 
    "mutex", "at_least_one", "exactly", "guard", "sum", 
    "compare", "type", "length",
];

// ============================================
// FUNZIONI HELPER
// ============================================

/// Calcola S = (v·i)/(t·k)
/// - v: valori validi (0..t)
/// - i: peso (1.0 per default)
/// - t: totale campi
/// - k: tolleranza (1.0 strict, 1.5 generative)
pub fn calculate_s_score(valid_count: usize, total_count: usize, mode: ParserMode) -> SScore {
    let v = valid_count as f64;
    let i = 1.0;
    let t = total_count as f64;
    let k = match mode {
        ParserMode::Strict => 1.0,
        ParserMode::Generative => 1.5,
    };
    
    if t == 0.0 || k == 0.0 {
        return 1.0;
    }
    
    (v * i) / (t * k)
}

/// Verifica se un report è esportabile
pub fn is_exportable(s_score: SScore, errors: &[ParseError]) -> bool {
    s_score >= EXPORT_THRESHOLD && errors.is_empty()
}

/// Normalizza un path (rimuove spazi, converte in lowercase)
pub fn normalize_path(path: &str) -> String {
    path.trim().to_lowercase()
}

/// Verifica se un path è valido
pub fn is_valid_path(path: &str) -> bool {
    !path.is_empty() && path.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '_')
}

// ============================================
// MACRO
// ============================================

/// Macro per registrare validatori
#[macro_export]
macro_rules! register_validators {
    ($registry:expr, $($name:ident => $func:expr),* $(,)?) => {
        $(
            $registry.register(stringify!($name), Box::new($func));
        )*
    };
}

/// Macro per creare un constraint
#[macro_export]
macro_rules! constraint {
    ($name:expr, $target:expr) => {
        Constraint {
            name: $name.to_string(),
            target: $target.to_string(),
            params: Obj::new(),
        }
    };
    ($name:expr, $target:expr, $($key:ident : $value:expr),* $(,)?) => {{
        let mut params = Obj::new();
        $(params.set(stringify!($key), $value.into());)*
        Constraint {
            name: $name.to_string(),
            target: $target.to_string(),
            params,
        }
    }};
}

/// Macro per creare una posizione di errore
#[macro_export]
macro_rules! error_location {
    () => {
        ErrorLocation {
            file: Some(std::path::PathBuf::from(file!())),
            line: line!(),
            col: column!(),
            path: String::new(),
        }
    };
    ($path:expr) => {
        ErrorLocation {
            file: Some(std::path::PathBuf::from(file!())),
            line: line!(),
            col: column!(),
            path: $path.to_string(),
        }
    };
}

// ============================================
// PRELUDE (import comune)
// ============================================

/// Prelude per importare i tipi più comuni
pub mod prelude {
    pub use crate::{
        // Types
        ParserMode, ParserConfig, ParserState, ValidationContext, ValidationState,
        FieldType, Alignment, TokenType,
        Constraint, ParseError, ParseWarning, Report, ParseStats,
        SScore, FieldPath, ConstraintName, MaxDepth,
        // Traits
        Validator, Resolver, StateMachine, FunctionRegistry, ObjAdapter, ErrorHandler,
        // Implementations
        BaseValidator, CompositeValidator, MemoizingValidator,
        PathResolver, AliasResolver, CompositeResolver,
        FunctionRegistryImpl, ObjAdapterImpl, CachedObjAdapter,
        ParserStateMachine, SimpleStateMachine,
        ErrorHandlerImpl,
        // Helpers
        calculate_s_score, is_exportable, normalize_path, is_valid_path,
        EXPORT_THRESHOLD, DEFAULT_MAX_DEPTH, SPEC_VERSION, BUILTIN_VALIDATORS,
        // Macros
        constraint, error_location, register_validators,
        // Error utilities
        error_codes, warning_codes,
        missing_required_error, min_value_error, pattern_error,
        circular_error, path_not_found_error,
    };
    
    // Re-export core/obj per comodità
    pub use sapri_obj::{obj, Obj, Value};
}
