// ============================================
// core/sson/src/error_impl.rs
// Implementazione di errori, warning e recovery
// ============================================

use crate::*;
use std::fmt;
use std::backtrace::Backtrace;

// ============================================
// TIPI DI ERRORE (dal .sson)
// ============================================

/// Categoria dell'errore
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Errore di parsing sintattico
    Parse,
    /// Errore di risoluzione riferimenti
    Resolve,
    /// Errore di validazione vincoli
    Validate,
    /// Riferimento circolare
    Circular,
    /// Timeout esecuzione
    Timeout,
    /// Profondità massima superata
    MaxDepth,
    /// Campo obbligatorio mancante
    MissingRequired,
    /// Tipo incompatibile
    TypeMismatch,
    /// Stato non valido
    InvalidState,
    /// Recovery fallita
    RecoveryFailed,
    /// Errore interno
    Internal,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCategory::Parse => write!(f, "PARSE"),
            ErrorCategory::Resolve => write!(f, "RESOLVE"),
            ErrorCategory::Validate => write!(f, "VALIDATE"),
            ErrorCategory::Circular => write!(f, "CIRCULAR"),
            ErrorCategory::Timeout => write!(f, "TIMEOUT"),
            ErrorCategory::MaxDepth => write!(f, "MAX_DEPTH"),
            ErrorCategory::MissingRequired => write!(f, "MISSING_REQUIRED"),
            ErrorCategory::TypeMismatch => write!(f, "TYPE_MISMATCH"),
            ErrorCategory::InvalidState => write!(f, "INVALID_STATE"),
            ErrorCategory::RecoveryFailed => write!(f, "RECOVERY_FAILED"),
            ErrorCategory::Internal => write!(f, "INTERNAL"),
        }
    }
}

// ============================================
// CODICI ERRORE STANDARDIZZATI
// ============================================

/// Genera codice errore standardizzato
pub fn error_code(category: ErrorCategory, code: u16) -> String {
    format!("ERR_{}_{:03}", category, code)
}

/// Codici errore predefiniti
pub mod error_codes {
    use super::*;
    
    // Parse errors (PARSE_XXX)
    pub const PARSE_UNEXPECTED_TOKEN: &str = "ERR_PARSE_001";
    pub const PARSE_UNEXPECTED_EOF: &str = "ERR_PARSE_002";
    pub const PARSE_INVALID_SECTION: &str = "ERR_PARSE_003";
    pub const PARSE_INVALID_KEYVAL: &str = "ERR_PARSE_004";
    
    // Resolve errors (RESOLVE_XXX)
    pub const RESOLVE_PATH_NOT_FOUND: &str = "ERR_RESOLVE_001";
    pub const RESOLVE_INVALID_REF: &str = "ERR_RESOLVE_002";
    pub const RESOLVE_CIRCULAR: &str = "ERR_RESOLVE_003";
    pub const RESOLVE_MAX_DEPTH: &str = "ERR_RESOLVE_004";
    
    // Validate errors (VALIDATE_XXX)
    pub const VALIDATE_REQUIRED_MISSING: &str = "ERR_VALIDATE_001";
    pub const VALIDATE_MIN_FAILED: &str = "ERR_VALIDATE_002";
    pub const VALIDATE_MAX_FAILED: &str = "ERR_VALIDATE_003";
    pub const VALIDATE_PATTERN_FAILED: &str = "ERR_VALIDATE_004";
    pub const VALIDATE_ENUM_FAILED: &str = "ERR_VALIDATE_005";
    pub const VALIDATE_MUTEX_FAILED: &str = "ERR_VALIDATE_006";
    pub const VALIDATE_GUARD_FAILED: &str = "ERR_VALIDATE_007";
    pub const VALIDATE_SUM_FAILED: &str = "ERR_VALIDATE_008";
    pub const VALIDATE_TYPE_MISMATCH: &str = "ERR_VALIDATE_009";
    
    // State errors (STATE_XXX)
    pub const STATE_INVALID_TRANSITION: &str = "ERR_STATE_001";
    pub const STATE_DEADLOCK: &str = "ERR_STATE_002";
    
    // System errors (SYS_XXX)
    pub const SYS_TIMEOUT: &str = "ERR_SYS_001";
    pub const SYS_INTERNAL: &str = "ERR_SYS_002";
}

// ============================================
// CODICI WARNING
// ============================================

/// Genera codice warning standardizzato
pub fn warning_code(category: ErrorCategory, code: u16) -> String {
    format!("WRN_{}_{:03}", category, code)
}

/// Codici warning predefiniti
pub mod warning_codes {
    use super::*;
    
    pub const RECOVERY_APPLIED: &str = "WRN_RECOVERY_001";
    pub const DEFAULT_USED: &str = "WRN_DEFAULT_001";
    pub const FIELD_IGNORED: &str = "WRN_FIELD_001";
    pub const TYPE_COERCED: &str = "WRN_TYPE_001";
    pub const DEPRECATED_FIELD: &str = "WRN_DEPRECATED_001";
}

// ============================================
// STRUTTURA ERRORE AVANZATA
// ============================================

/// Errore di parsing/validazione con metadati
#[derive(Debug, Clone)]
pub struct DetailedError {
    /// Codice errore standardizzato
    pub code: String,
    /// Messaggio descrittivo
    pub message: String,
    /// Categoria dell'errore
    pub category: ErrorCategory,
    /// Severità (error, warning)
    pub severity: Severity,
    /// Posizione dell'errore
    pub location: ErrorLocation,
    /// Vincolo che ha causato l'errore (opzionale)
    pub constraint: Option<Constraint>,
    /// Valore che ha causato l'errore (opzionale)
    pub value: Option<Value>,
    /// Valore atteso (opzionale)
    pub expected: Option<Value>,
    /// Stack trace (solo debug)
    pub backtrace: Option<String>,
    /// Hint per AI su come risolvere
    pub ai_hint: Option<AIHint>,
    /// Errore sorgente (causa)
    pub source: Option<Box<DetailedError>>,
}

/// Severità dell'errore
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Severity {
    /// Errore bloccante (exportable=false)
    Error,
    /// Avviso (non bloccante)
    Warning,
    /// Solo informazione
    Info,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => write!(f, "ERROR"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Info => write!(f, "INFO"),
        }
    }
}

// ============================================
// AI HINT PER AUTOCORREZIONE
// ============================================

/// Hint per AI su come risolvere l'errore
#[derive(Debug, Clone)]
pub struct AIHint {
    /// Azione suggerita
    pub action: AIAction,
    /// Path del campo da modificare
    pub target_path: String,
    /// Valore suggerito
    pub suggested_value: Option<Value>,
    /// Ragione del suggerimento
    pub reason: String,
    /// Confidenza del suggerimento (0.0-1.0)
    pub confidence: f64,
}

/// Azione suggerita all'AI
#[derive(Debug, Clone)]
pub enum AIAction {
    /// Aggiungi il campo mancante
    AddField { default_value: Option<Value> },
    /// Rimuovi il campo
    RemoveField,
    /// Modifica il valore
    ChangeValue { new_value: Value },
    /// Coerci il tipo
    CoerceType { target_type: String },
    /// Applica default
    ApplyDefault,
    /// Ignora il campo
    IgnoreField,
    /// Crea nuovo osservatore per questo pattern
    CreateObserver { name: String },
    /// Chiedi conferma all'utente
    AskUser { question: String },
}

impl fmt::Display for AIAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AIAction::AddField { default_value } => {
                if let Some(v) = default_value {
                    write!(f, "Add field with default: {}", v)
                } else {
                    write!(f, "Add field")
                }
            }
            AIAction::RemoveField => write!(f, "Remove field"),
            AIAction::ChangeValue { new_value } => write!(f, "Change value to {}", new_value),
            AIAction::CoerceType { target_type } => write!(f, "Coerce type to {}", target_type),
            AIAction::ApplyDefault => write!(f, "Apply default value"),
            AIAction::IgnoreField => write!(f, "Ignore field"),
            AIAction::CreateObserver { name } => write!(f, "Create observer '{}'", name),
            AIAction::AskUser { question } => write!(f, "Ask user: {}", question),
        }
    }
}

// ============================================
// ERRORI STANDARD
// ============================================

/// Crea un errore "required field missing"
pub fn missing_required_error(field: &str, location: ErrorLocation) -> DetailedError {
    DetailedError {
        code: error_codes::VALIDATE_REQUIRED_MISSING.to_string(),
        message: format!("Required field '{}' is missing", field),
        category: ErrorCategory::MissingRequired,
        severity: Severity::Error,
        location,
        constraint: None,
        value: None,
        expected: None,
        backtrace: None,
        ai_hint: Some(AIHint {
            action: AIAction::AddField { default_value: None },
            target_path: field.to_string(),
            suggested_value: None,
            reason: format!("Field '{}' is marked as required (_:req)", field),
            confidence: 0.95,
        }),
        source: None,
    }
}

/// Crea un errore "min value failed"
pub fn min_value_error(field: &str, value: f64, min: f64, location: ErrorLocation) -> DetailedError {
    DetailedError {
        code: error_codes::VALIDATE_MIN_FAILED.to_string(),
        message: format!("Value {} is less than minimum {}", value, min),
        category: ErrorCategory::Validate,
        severity: Severity::Error,
        location,
        constraint: None,
        value: Some(Value::Number(value)),
        expected: Some(Value::Number(min)),
        backtrace: None,
        ai_hint: Some(AIHint {
            action: AIAction::ChangeValue { new_value: Value::Number(min) },
            target_path: field.to_string(),
            suggested_value: Some(Value::Number(min)),
            reason: format!("Value must be at least {}", min),
            confidence: 0.9,
        }),
        source: None,
    }
}

/// Crea un errore "pattern mismatch"
pub fn pattern_error(field: &str, value: &str, pattern: &str, location: ErrorLocation) -> DetailedError {
    DetailedError {
        code: error_codes::VALIDATE_PATTERN_FAILED.to_string(),
        message: format!("Value '{}' does not match pattern '{}'", value, pattern),
        category: ErrorCategory::Validate,
        severity: Severity::Error,
        location,
        constraint: None,
        value: Some(Value::String(value.to_string())),
        expected: Some(Value::String(format!("pattern: {}", pattern))),
        backtrace: None,
        ai_hint: Some(AIHint {
            action: AIAction::ChangeValue { new_value: Value::Null },
            target_path: field.to_string(),
            suggested_value: None,
            reason: format!("Value must match regex: {}", pattern),
            confidence: 0.7,
        }),
        source: None,
    }
}

/// Crea un errore "circular reference"
pub fn circular_error(path: &str, stack: Vec<String>, location: ErrorLocation) -> DetailedError {
    DetailedError {
        code: error_codes::RESOLVE_CIRCULAR.to_string(),
        message: format!("Circular reference detected at '{}': {:?}", path, stack),
        category: ErrorCategory::Circular,
        severity: Severity::Error,
        location,
        constraint: None,
        value: None,
        expected: None,
        backtrace: None,
        ai_hint: Some(AIHint {
            action: AIAction::RemoveField,
            target_path: path.to_string(),
            suggested_value: None,
            reason: "Circular references are not allowed".to_string(),
            confidence: 0.85,
        }),
        source: None,
    }
}

/// Crea un errore "path not found"
pub fn path_not_found_error(path: &str, location: ErrorLocation) -> DetailedError {
    DetailedError {
        code: error_codes::RESOLVE_PATH_NOT_FOUND.to_string(),
        message: format!("Reference path '{}' not found", path),
        category: ErrorCategory::Resolve,
        severity: Severity::Error,
        location,
        constraint: None,
        value: None,
        expected: None,
        backtrace: None,
        ai_hint: Some(AIHint {
            action: AIAction::AddField { default_value: None },
            target_path: path.to_string(),
            suggested_value: None,
            reason: format!("The referenced path '{}' does not exist", path),
            confidence: 0.8,
        }),
        source: None,
    }
}

// ============================================
// WARNING CON HINT
// ============================================

/// Crea un warning "default used"
pub fn default_used_warning(field: &str, default_value: Value, location: ErrorLocation) -> DetailedError {
    DetailedError {
        code: warning_codes::DEFAULT_USED.to_string(),
        message: format!("Using default value {} for field '{}'", default_value, field),
        category: ErrorCategory::Validate,
        severity: Severity::Warning,
        location,
        constraint: None,
        value: None,
        expected: Some(default_value.clone()),
        backtrace: None,
        ai_hint: Some(AIHint {
            action: AIAction::ApplyDefault,
            target_path: field.to_string(),
            suggested_value: Some(default_value),
            reason: format!("Field '{}' is optional, using default", field),
            confidence: 0.95,
        }),
        source: None,
    }
}

/// Crea un warning "field ignored" (generative mode)
pub fn field_ignored_warning(field: &str, location: ErrorLocation) -> DetailedError {
    DetailedError {
        code: warning_codes::FIELD_IGNORED.to_string(),
        message: format!("Unknown field '{}' ignored (generative mode)", field),
        category: ErrorCategory::Parse,
        severity: Severity::Warning,
        location,
        constraint: None,
        value: None,
        expected: None,
        backtrace: None,
        ai_hint: Some(AIHint {
            action: AIAction::IgnoreField,
            target_path: field.to_string(),
            suggested_value: None,
            reason: "Field not defined in schema".to_string(),
            confidence: 0.9,
        }),
        source: None,
    }
}

/// Crea un warning "type coerced"
pub fn type_coerced_warning(field: &str, from_type: &str, to_type: &str, location: ErrorLocation) -> DetailedError {
    DetailedError {
        code: warning_codes::TYPE_COERCED.to_string(),
        message: format!("Type coerced from {} to {} for field '{}'", from_type, to_type, field),
        category: ErrorCategory::Validate,
        severity: Severity::Warning,
        location,
        constraint: None,
        value: None,
        expected: None,
        backtrace: None,
        ai_hint: Some(AIHint {
            action: AIAction::CoerceType { target_type: to_type.to_string() },
            target_path: field.to_string(),
            suggested_value: None,
            reason: format!("Expected type {}, found {}", to_type, from_type),
            confidence: 0.7,
        }),
        source: None,
    }
}

// ============================================
// ERROR HANDLER (TRAIT IMPL)
// ============================================

/// Handler errori con supporto recovery
#[derive(Debug, Default)]
pub struct ErrorHandlerImpl {
    errors: Vec<DetailedError>,
    warnings: Vec<DetailedError>,
    recovery_log: Vec<RecoveryEntry>,
    mode: ParserMode,
}

/// Entry nel log di recovery
#[derive(Debug, Clone)]
pub struct RecoveryEntry {
    pub timestamp: std::time::Instant,
    pub original_error: String,
    pub recovery_action: AIAction,
    pub success: bool,
}

impl ErrorHandlerImpl {
    pub fn new(mode: ParserMode) -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            recovery_log: Vec::new(),
            mode,
        }
    }
    
    /// Registra un errore (se strict mode, ferma)
    pub fn add_error(&mut self, error: DetailedError) {
        if self.mode == ParserMode::Strict {
            self.errors.push(error);
        } else {
            // In generative mode, prova recovery
            self.try_recovery(error);
        }
    }
    
    /// Registra un warning
    pub fn add_warning(&mut self, warning: DetailedError) {
        self.warnings.push(warning);
    }
    
    /// Tenta recovery da un errore
    fn try_recovery(&mut self, error: DetailedError) {
        let start = std::time::Instant::now();
        let mut success = false;
        let mut recovery_action = AIAction::IgnoreField;
        
        if let Some(hint) = &error.ai_hint {
            recovery_action = hint.action.clone();
            success = match &hint.action {
                AIAction::AddField { default_value } => default_value.is_some(),
                AIAction::RemoveField => true,
                AIAction::ChangeValue { new_value: _ } => true,
                AIAction::CoerceType { target_type: _ } => true,
                AIAction::ApplyDefault => true,
                AIAction::IgnoreField => true,
                AIAction::CreateObserver { name: _ } => false, // Richiede AI
                AIAction::AskUser { question: _ } => false,    // Richiede utente
            };
        }
        
        self.recovery_log.push(RecoveryEntry {
            timestamp: start,
            original_error: error.message.clone(),
            recovery_action,
            success,
        });
        
        if success {
            // Converti errore in warning
            let mut warning = error;
            warning.severity = Severity::Warning;
            warning.code = warning_codes::RECOVERY_APPLIED.to_string();
            warning.message = format!("[RECOVERED] {}", warning.message);
            self.warnings.push(warning);
        } else {
            // Recovery fallita, mantieni errore
            self.errors.push(error);
        }
    }
    
    /// Verifica se ci sono errori bloccanti
    pub fn has_blocking_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    /// Ottieni tutti gli errori
    pub fn errors(&self) -> &[DetailedError] {
        &self.errors
    }
    
    /// Ottieni tutti i warning
    pub fn warnings(&self) -> &[DetailedError] {
        &self.warnings
    }
    
    /// Ottieni log recovery
    pub fn recovery_log(&self) -> &[RecoveryEntry] {
        &self.recovery_log
    }
    
    /// Pulisce errori e warning
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.recovery_log.clear();
    }
    
    /// Converte in ParseError (formato standard)
    pub fn to_parse_errors(&self) -> Vec<ParseError> {
        self.errors.iter().map(|e| ParseError {
            code: e.code.clone(),
            message: e.message.clone(),
            location: e.location.clone(),
            constraint: e.constraint.clone(),
        }).collect()
    }
    
    /// Converte in ParseWarning (formato standard)
    pub fn to_parse_warnings(&self) -> Vec<ParseWarning> {
        self.warnings.iter().map(|w| ParseWarning {
            code: w.code.clone(),
            message: w.message.clone(),
            location: w.location.clone(),
            recovered: true,
        }).collect()
    }
}

impl ErrorHandler for ErrorHandlerImpl {
    fn add_error(&mut self, error: ParseError) {
        let detailed = DetailedError {
            code: error.code,
            message: error.message,
            category: ErrorCategory::Parse,
            severity: Severity::Error,
            location: error.location,
            constraint: error.constraint,
            value: None,
            expected: None,
            backtrace: None,
            ai_hint: None,
            source: None,
        };
        self.add_error(detailed);
    }
    
    fn add_warning(&mut self, warning: ParseWarning) {
        let detailed = DetailedError {
            code: warning.code,
            message: warning.message,
            category: ErrorCategory::Parse,
            severity: Severity::Warning,
            location: warning.location,
            constraint: None,
            value: None,
            expected: None,
            backtrace: None,
            ai_hint: None,
            source: None,
        };
        self.warnings.push(detailed);
    }
    
    fn recover(&mut self, error: &ParseError) -> Option<ParseWarning> {
        if self.mode != ParserMode::Generative {
            return None;
        }
        
        // Simula recovery
        Some(ParseWarning {
            code: warning_codes::RECOVERY_APPLIED.to_string(),
            message: format!("Recovered from error: {}", error.message),
            location: error.location.clone(),
            recovered: true,
        })
    }
    
    fn has_errors(&self) -> bool {
        self.has_blocking_errors()
    }
    
    fn is_exportable(&self, s_score: SScore) -> bool {
        !self.has_blocking_errors() && s_score >= EXPORT_THRESHOLD
    }
}

// ============================================
// RISULTATO CON ERRORI DETTAGLIATI
// ============================================

/// Risultato con supporto errori dettagliati
pub type DetailedResult<T> = std::result::Result<T, DetailedError>;

/// Estensione per Result con conversione a DetailedError
pub trait IntoDetailedError<T> {
    fn detailed(self, location: ErrorLocation) -> DetailedResult<T>;
}

impl<T, E: fmt::Display> IntoDetailedError<T> for std::result::Result<T, E> {
    fn detailed(self, location: ErrorLocation) -> DetailedResult<T> {
        self.map_err(|e| DetailedError {
            code: error_codes::SYS_INTERNAL.to_string(),
            message: e.to_string(),
            category: ErrorCategory::Internal,
            severity: Severity::Error,
            location,
            constraint: None,
            value: None,
            expected: None,
            backtrace: Some(format!("{:?}", Backtrace::capture())),
            ai_hint: None,
            source: None,
        })
    }
}

// ============================================
// TEST
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    
    fn test_location() -> ErrorLocation {
        ErrorLocation {
            file: Some("test.sson".into()),
            line: 10,
            col: 5,
            path: "test.field".to_string(),
        }
    }
    
    #[test]
    fn test_missing_required_error() {
        let err = missing_required_error("name", test_location());
        assert_eq!(err.code, error_codes::VALIDATE_REQUIRED_MISSING);
        assert!(err.message.contains("name"));
        assert_eq!(err.category, ErrorCategory::MissingRequired);
        assert_eq!(err.severity, Severity::Error);
        assert!(err.ai_hint.is_some());
    }
    
    #[test]
    fn test_min_value_error() {
        let err = min_value_error("age", 5.0, 18.0, test_location());
        assert_eq!(err.code, error_codes::VALIDATE_MIN_FAILED);
        assert!(err.message.contains("5"));
        assert!(err.message.contains("18"));
        assert!(err.ai_hint.is_some());
        
        if let Some(hint) = &err.ai_hint {
            assert!(matches!(hint.action, AIAction::ChangeValue { .. }));
            assert_eq!(hint.target_path, "age");
        }
    }
    
    #[test]
    fn test_circular_error() {
        let stack = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let err = circular_error("c", stack.clone(), test_location());
        assert_eq!(err.code, error_codes::RESOLVE_CIRCULAR);
        assert!(err.message.contains("c"));
        assert_eq!(err.category, ErrorCategory::Circular);
    }
    
    #[test]
    fn test_default_used_warning() {
        let warn = default_used_warning("optional", Value::String("default".to_string()), test_location());
        assert_eq!(warn.code, warning_codes::DEFAULT_USED);
        assert_eq!(warn.severity, Severity::Warning);
        assert!(warn.ai_hint.is_some());
    }
    
    #[test]
    fn test_error_handler_strict() {
        let mut handler = ErrorHandlerImpl::new(ParserMode::Strict);
        let err = missing_required_error("field", test_location());
        
        handler.add_error(err);
        assert!(handler.has_blocking_errors());
        assert_eq!(handler.errors().len(), 1);
        assert_eq!(handler.warnings().len(), 0);
    }
    
    #[test]
    fn test_error_handler_generative_with_recovery() {
        let mut handler = ErrorHandlerImpl::new(ParserMode::Generative);
        let err = missing_required_error("field", test_location());
        
        handler.add_error(err);
        // L'errore dovrebbe essere convertito in warning
        assert!(!handler.has_blocking_errors());
        assert_eq!(handler.warnings().len(), 1);
        assert_eq!(handler.recovery_log().len(), 1);
    }
    
    #[test]
    fn test_is_exportable() {
        let mut handler = ErrorHandlerImpl::new(ParserMode::Strict);
        
        // Senza errori, S alto
        assert!(handler.is_exportable(0.95));
        
        // Con errori
        handler.add_error(missing_required_error("field", test_location()));
        assert!(!handler.is_exportable(0.95));
        
        // S basso
        let mut handler2 = ErrorHandlerImpl::new(ParserMode::Strict);
        assert!(!handler2.is_exportable(0.5));
    }
    
    #[test]
    fn test_error_code_format() {
        assert_eq!(error_code(ErrorCategory::Parse, 1), "ERR_PARSE_001");
        assert_eq!(error_code(ErrorCategory::Validate, 99), "ERR_VALIDATE_099");
        assert_eq!(warning_code(ErrorCategory::Parse, 1), "WRN_PARSE_001");
    }
    
    #[test]
    fn test_detailed_result() {
        let location = test_location();
        let result: std::result::Result<i32, &str> = Err("test error");
        let detailed = result.detailed(location);
        
        assert!(detailed.is_err());
        let err = detailed.unwrap_err();
        assert_eq!(err.code, error_codes::SYS_INTERNAL);
        assert!(err.backtrace.is_some());
    }
    
    #[test]
    fn test_error_conversion() {
        let mut handler = ErrorHandlerImpl::new(ParserMode::Strict);
        
        let parse_error = ParseError {
            code: "ERR_TEST".to_string(),
            message: "test".to_string(),
            location: test_location(),
            constraint: None,
        };
        
        handler.add_error(parse_error);
        let converted = handler.to_parse_errors();
        assert_eq!(converted.len(), 1);
        assert_eq!(converted[0].code, "ERR_TEST");
    }
}
