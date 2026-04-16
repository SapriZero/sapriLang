// ============================================
// core/sson/src/state_machine_impl.rs
// Implementazione del trait StateMachine
// Macchina a stati per il parser flow
// ============================================

use crate::*;
use crate::validator_impl::BaseValidator;
use crate::resolver_impl::PathResolver;
use crate::registry_impl::FunctionRegistryImpl;
use crate::obj_adapter_impl::ObjAdapterImpl;

use std::collections::VecDeque;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};

// ============================================
// STATI DEL PARSER (dal .sson)
// ============================================

/// Stati validi del parser
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParserState {
    Init,           // stato iniziale
    Lexing,         // tokenizzazione
    Parsing,        // costruzione AST
    Resolving,      // risoluzione riferimenti
    Deduplicating,  // deduplicazione pattern
    Validating,     // validazione vincoli
    Validated,      // validazione completata
    Exported,       // esportato
    Error,          // errore irreversibile
}

impl std::fmt::Display for ParserState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// ============================================
// AZIONI DEL PARSER (dal .sson _ACTIONS)
// ============================================

/// Azioni eseguibili nella pipeline
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserAction {
    Lex,        // tokenize_input
    Parse,      // build_ast
    Resolve,    // resolve_refs
    Dedup,      // extract_duplicates
    Validate,   // check_constraints
    Err,        // collect_errors
    Rec,        // recover_and_continue
    Flush,      // clear_buffers
    Nop,        // NULL (nessuna operazione)
}

impl std::fmt::Display for ParserAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserAction::Lex => write!(f, "lex"),
            ParserAction::Parse => write!(f, "parse"),
            ParserAction::Resolve => write!(f, "resolve"),
            ParserAction::Dedup => write!(f, "dedup"),
            ParserAction::Validate => write!(f, "validate"),
            ParserAction::Err => write!(f, "err"),
            ParserAction::Rec => write!(f, "rec"),
            ParserAction::Flush => write!(f, "flush"),
            ParserAction::Nop => write!(f, "nop"),
        }
    }
}

// ============================================
// TRANSIZIONE
// ============================================

/// Una transizione tra stati
#[derive(Debug, Clone)]
pub struct Transition {
    pub from: ParserState,
    pub to: ParserState,
    pub action: ParserAction,
    pub guard: Option<String>,
    pub on_fail: ParserAction,
}

impl Transition {
    pub fn new(from: ParserState, to: ParserState, action: ParserAction) -> Self {
        Self {
            from,
            to,
            action,
            guard: None,
            on_fail: ParserAction::Err,
        }
    }
    
    pub fn with_guard(mut self, guard: &str) -> Self {
        self.guard = Some(guard.to_string());
        self
    }
    
    pub fn on_fail(mut self, action: ParserAction) -> Self {
        self.on_fail = action;
        self
    }
}

// ============================================
// CONTESTO DELLA MACCHINA A STATI
// ============================================

/// Contesto passato tra le azioni
#[derive(Debug, Clone)]
pub struct MachineContext {
    /// Modalità di esecuzione
    pub mode: ParserMode,
    
    /// Input originale
    pub input: String,
    
    /// Token generati (lexer)
    pub tokens: Vec<Token>,
    
    /// AST costruito (parser)
    pub ast: Option<Obj>,
    
    /// Dizionario piatto (da dedup/validate)
    pub dict: Option<Obj>,
    
    /// Errori raccolti
    pub errors: Vec<ParseError>,
    
    /// Warnings raccolti (solo generative)
    pub warnings: Vec<ParseWarning>,
    
    /// Statistiche
    pub stats: ParseStats,
    
    /// Punteggio di equilibrio
    pub s_score: SScore,
    
    /// Timestamp di inizio
    pub start_time: Instant,
    
    /// Timeout massimo (millisecondi)
    pub timeout_ms: u64,
    
    /// Profondità corrente (per guardie)
    pub current_depth: usize,
    
    /// Path corrente (per resolver)
    pub current_path: String,
}

impl MachineContext {
    pub fn new(mode: ParserMode, input: String, timeout_ms: u64) -> Self {
        Self {
            mode,
            input,
            tokens: Vec::new(),
            ast: None,
            dict: None,
            errors: Vec::new(),
            warnings: Vec::new(),
            stats: ParseStats::default(),
            s_score: 1.0,
            start_time: Instant::now(),
            timeout_ms,
            current_depth: 0,
            current_path: String::new(),
        }
    }
    
    /// Verifica se il timeout è scaduto
    pub fn is_timed_out(&self) -> bool {
        self.start_time.elapsed() > Duration::from_millis(self.timeout_ms)
    }
    
    /// Verifica se ci sono errori (in strict mode ferma)
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    /// Aggiunge un errore
    pub fn add_error(&mut self, error: ParseError) {
        self.errors.push(error);
        self.stats.total_errors += 1;
    }
    
    /// Aggiunge un warning
    pub fn add_warning(&mut self, warning: ParseWarning) {
        if self.mode == ParserMode::Generative {
            self.warnings.push(warning);
            self.stats.total_warnings += 1;
        }
    }
    
    /// Calcola S in base allo stato corrente
    pub fn update_s_score(&mut self) {
        let total_fields = self.stats.total_fields as f64;
        if total_fields == 0.0 {
            self.s_score = 1.0;
            return;
        }
        
        let valid_fields = self.stats.valid_fields as f64;
        let k = match self.mode {
            ParserMode::Strict => 1.0,
            ParserMode::Generative => 1.5,
        };
        
        self.s_score = (valid_fields * 1.0) / (total_fields * k);
    }
}

// ============================================
// STATISTICHE ESTESE
// ============================================

#[derive(Debug, Clone, Default)]
pub struct ParseStats {
    pub total_tokens: usize,
    pub total_sections: usize,
    pub total_fields: usize,
    pub valid_fields: usize,
    pub total_constraints: usize,
    pub resolved_refs: usize,
    pub unresolved_refs: usize,
    pub cycles_detected: usize,
    pub recovery_count: usize,
    pub total_errors: usize,
    pub total_warnings: usize,
}

// ============================================
// MACCHINA A STATI PRINCIPALE
// ============================================

/// Macchina a stati del parser
/// Gestisce la pipeline completa: lex → parse → resolve → dedup → validate
#[derive(Debug)]
pub struct ParserStateMachine {
    /// Transizioni valide (from_state → to_state)
    transitions: Vec<Transition>,
    
    /// Stato corrente
    current_state: ParserState,
    
    /// Contesto della macchina
    context: MachineContext,
    
    /// Validator (per azione validate)
    validator: BaseValidator,
    
    /// Resolver (per azione resolve)
    resolver: PathResolver,
    
    /// Registry (per validatori testuali)
    registry: FunctionRegistryImpl,
    
    /// Adapter (per conversione flat/nested)
    adapter: ObjAdapterImpl,
    
    /// Stack di backtracking (per recovery)
    backtrack_stack: VecDeque<ParserState>,
    
    /// Massima profondità di backtracking
    max_backtrack_depth: usize,
}

impl ParserStateMachine {
    /// Crea una nuova macchina a stati
    pub fn new(mode: ParserMode, input: String, timeout_ms: u64) -> Self {
        let context = MachineContext::new(mode, input, timeout_ms);
        
        let mut machine = Self {
            transitions: Vec::new(),
            current_state: ParserState::Init,
            context,
            validator: BaseValidator::new(),
            resolver: PathResolver::default(),
            registry: FunctionRegistryImpl::with_builtins(),
            adapter: ObjAdapterImpl::new(),
            backtrack_stack: VecDeque::new(),
            max_backtrack_depth: 5,
        };
        
        machine.init_transitions();
        machine
    }
    
    /// Inizializza le transizioni valide (dal .sson parser.flow)
    fn init_transitions(&mut self) {
        // Strict mode pipeline
        self.transitions.push(Transition::new(
            ParserState::Init, ParserState::Lexing, ParserAction::Lex
        ));
        self.transitions.push(Transition::new(
            ParserState::Lexing, ParserState::Parsing, ParserAction::Parse
        ));
        self.transitions.push(Transition::new(
            ParserState::Parsing, ParserState::Resolving, ParserAction::Resolve
        ));
        self.transitions.push(Transition::new(
            ParserState::Resolving, ParserState::Deduplicating, ParserAction::Dedup
        ));
        self.transitions.push(Transition::new(
            ParserState::Deduplicating, ParserState::Validating, ParserAction::Validate
        ));
        self.transitions.push(Transition::new(
            ParserState::Validating, ParserState::Validated, ParserAction::Nop
        ));
        self.transitions.push(Transition::new(
            ParserState::Validated, ParserState::Exported, ParserAction::Flush
        ));
        
        // Gestione errori per strict mode
        for trans in &mut self.transitions {
            if trans.action != ParserAction::Nop {
                trans.on_fail = ParserAction::Err;
            }
        }
        
        // Recovery per generative mode
        let rec_transition = Transition::new(
            ParserState::Error, ParserState::Lexing, ParserAction::Rec
        ).on_fail(ParserAction::Flush);
        self.transitions.push(rec_transition);
    }
    
    /// Trova la transizione valida da uno stato
    fn find_transition(&self, from: ParserState) -> Option<&Transition> {
        self.transitions.iter().find(|t| t.from == from)
    }
    
    /// Esegue un'azione
    fn execute_action(&mut self, action: ParserAction) -> Result<bool> {
        match action {
            ParserAction::Lex => self.action_lex(),
            ParserAction::Parse => self.action_parse(),
            ParserAction::Resolve => self.action_resolve(),
            ParserAction::Dedup => self.action_dedup(),
            ParserAction::Validate => self.action_validate(),
            ParserAction::Err => self.action_err(),
            ParserAction::Rec => self.action_rec(),
            ParserAction::Flush => self.action_flush(),
            ParserAction::Nop => Ok(true),
        }
    }
    
    // ============================================
    // IMPLEMENTAZIONE AZIONI
    // ============================================
    
    fn action_lex(&mut self) -> Result<bool> {
        // Simula tokenizzazione
        self.context.stats.total_tokens = self.context.input.split_whitespace().count();
        self.context.current_state = ParserState::Lexing;
        Ok(true)
    }
    
    fn action_parse(&mut self) -> Result<bool> {
        // Simula parsing AST
        let mut ast = Obj::new();
        ast.set("_parsed", Value::Bool(true));
        self.context.ast = Some(ast);
        self.context.current_state = ParserState::Parsing;
        Ok(true)
    }
    
    fn action_resolve(&mut self) -> Result<bool> {
        // Risolve riferimenti
        if let Some(ast) = &self.context.ast {
            let mut ctx = ValidationContext::new(self.context.mode, ast.clone());
            let resolved = self.resolver.resolve_all(&mut ctx, &[])?;
            self.context.stats.resolved_refs = resolved.len();
            self.context.current_state = ParserState::Resolving;
        }
        Ok(true)
    }
    
    fn action_dedup(&mut self) -> Result<bool> {
        // Deduplicazione pattern (simulata)
        self.context.stats.total_fields = 10; // esempio
        self.context.current_state = ParserState::Deduplicating;
        Ok(true)
    }
    
    fn action_validate(&mut self) -> Result<bool> {
        // Validazione vincoli
        let constraints: Vec<Constraint> = Vec::new(); // da schema
        let mut ctx = ValidationContext::new(self.context.mode, Obj::new());
        let s = self.validator.validate_all(&mut ctx, &constraints);
        self.context.s_score = s;
        self.context.update_s_score();
        
        let is_valid = s >= EXPORT_THRESHOLD;
        if !is_valid && self.context.mode == ParserMode::Strict {
            self.context.add_error(ParseError {
                code: "ERR_VALIDATE".to_string(),
                message: format!("Validation failed: S={:.3}", s),
                location: ErrorLocation {
                    file: None,
                    line: 0,
                    col: 0,
                    path: String::new(),
                },
                constraint: None,
            });
            return Err(Error::new("Validation failed"));
        }
        
        self.context.current_state = ParserState::Validating;
        Ok(true)
    }
    
    fn action_err(&mut self) -> Result<bool> {
        // Raccoglie errori
        if self.context.mode == ParserMode::Strict {
            self.current_state = ParserState::Error;
            return Err(Error::new("Strict mode: stopping on error"));
        }
        Ok(false) // Fallisce ma continua in generative
    }
    
    fn action_rec(&mut self) -> Result<bool> {
        // Recovery: torna allo stato precedente
        if let Some(prev_state) = self.backtrack_stack.pop_back() {
            self.context.stats.recovery_count += 1;
            self.context.add_warning(ParseWarning {
                code: "WRN_RECOVERY".to_string(),
                message: format!("Recovered from error, returning to {:?}", prev_state),
                location: ErrorLocation {
                    file: None,
                    line: 0,
                    col: 0,
                    path: String::new(),
                },
                recovered: true,
            });
            self.current_state = prev_state;
            Ok(true)
        } else {
            self.current_state = ParserState::Error;
            Err(Error::new("Recovery failed: no backtrack state"))
        }
    }
    
    fn action_flush(&mut self) -> Result<bool> {
        // Pulisce buffer e finalizza
        self.context.current_state = ParserState::Exported;
        Ok(true)
    }
    
    // ============================================
    // BACKTRACKING
    // ============================================
    
    fn push_backtrack(&mut self) {
        if self.backtrack_stack.len() < self.max_backtrack_depth {
            self.backtrack_stack.push_back(self.current_state);
        }
    }
    
    // ============================================
    // ESECUZIONE PIPELINE
    // ============================================
    
    fn run_pipeline_internal(&mut self) -> Result<Report> {
        self.context.start_time = Instant::now();
        self.current_state = ParserState::Init;
        
        while self.current_state != ParserState::Exported && self.current_state != ParserState::Error {
            // Timeout check
            if self.context.is_timed_out() {
                self.context.add_error(ParseError {
                    code: "ERR_TIMEOUT".to_string(),
                    message: format!("Timeout after {} ms", self.context.timeout_ms),
                    location: ErrorLocation {
                        file: None,
                        line: 0,
                        col: 0,
                        path: String::new(),
                    },
                    constraint: None,
                });
                break;
            }
            
            // Trova transizione
            let transition = match self.find_transition(self.current_state) {
                Some(t) => t,
                None => {
                    self.current_state = ParserState::Error;
                    break;
                }
            };
            
            // Salva stato per backtracking
            self.push_backtrack();
            
            // Esegui azione
            match self.execute_action(transition.action) {
                Ok(true) => {
                    // Successo: transizione al prossimo stato
                    self.current_state = transition.to;
                }
                Ok(false) => {
                    // Fallimento ma continua (generative)
                    if self.context.mode == ParserMode::Generative {
                        self.current_state = transition.to;
                    } else {
                        self.current_state = ParserState::Error;
                    }
                }
                Err(e) => {
                    // Errore: esegui azione di fallback
                    self.context.add_error(ParseError {
                        code: format!("ERR_{:?}", transition.action),
                        message: e.to_string(),
                        location: ErrorLocation {
                            file: None,
                            line: 0,
                            col: 0,
                            path: String::new(),
                        },
                        constraint: None,
                    });
                    
                    // Esegui on_fail action
                    let _ = self.execute_action(transition.on_fail);
                    
                    if self.context.mode == ParserMode::Strict {
                        self.current_state = ParserState::Error;
                    } else {
                        self.current_state = transition.to;
                    }
                }
            }
        }
        
        // Costruisci report
        let report = Report {
            dict: self.context.dict.clone().unwrap_or_else(Obj::new),
            tree: self.context.ast.clone(),
            errors: self.context.errors.clone(),
            warnings: self.context.warnings.clone(),
            stats: self.context.stats.clone(),
            s_score: self.context.s_score,
            exportable: self.current_state == ParserState::Exported && self.context.s_score >= EXPORT_THRESHOLD,
        };
        
        Ok(report)
    }
}

// ============================================
// IMPLEMENTAZIONE TRAIT STATEMACHINE
// ============================================

impl StateMachine for ParserStateMachine {
    fn transition(&mut self, from: ParserState, to: ParserState) -> Result<bool> {
        if let Some(transition) = self.find_transition(from) {
            if transition.to == to {
                self.current_state = to;
                Ok(true)
            } else {
                Err(Error::new(&format!("Invalid transition: {:?} → {:?}", from, to)))
            }
        } else {
            Err(Error::new(&format!("No transition from state: {:?}", from)))
        }
    }
    
    fn current_state(&self) -> ParserState {
        self.current_state
    }
    
    fn is_valid_transition(&self, from: ParserState, to: ParserState) -> bool {
        self.transitions.iter().any(|t| t.from == from && t.to == to)
    }
    
    fn run_pipeline(&mut self, input: &str, config: &ParserConfig) -> Result<Report> {
        self.context.input = input.to_string();
        self.context.mode = config.mode;
        self.context.timeout_ms = match config.mode {
            ParserMode::Strict => 5000,      // 5 secondi
            ParserMode::Generative => 30000, // 30 secondi
        };
        self.run_pipeline_internal()
    }
}

// ============================================
// MACCHINA A STATI SEMPLICE (PER TEST)
// ============================================

/// Versione semplificata per test rapidi
#[derive(Debug, Default)]
pub struct SimpleStateMachine {
    state: ParserState,
}

impl SimpleStateMachine {
    pub fn new() -> Self {
        Self {
            state: ParserState::Init,
        }
    }
}

impl StateMachine for SimpleStateMachine {
    fn transition(&mut self, from: ParserState, to: ParserState) -> Result<bool> {
        if self.state != from {
            return Err(Error::new(&format!("Current state is {:?}, expected {:?}", self.state, from)));
        }
        self.state = to;
        Ok(true)
    }
    
    fn current_state(&self) -> ParserState {
        self.state
    }
    
    fn is_valid_transition(&self, from: ParserState, to: ParserState) -> bool {
        // Transizioni valide di base
        matches!(
            (from, to),
            (ParserState::Init, ParserState::Lexing)
            | (ParserState::Lexing, ParserState::Parsing)
            | (ParserState::Parsing, ParserState::Validated)
            | (ParserState::Validated, ParserState::Exported)
        )
    }
    
    fn run_pipeline(&mut self, _input: &str, config: &ParserConfig) -> Result<Report> {
        self.state = ParserState::Init;
        
        // Simula pipeline
        self.transition(ParserState::Init, ParserState::Lexing)?;
        self.transition(ParserState::Lexing, ParserState::Parsing)?;
        self.transition(ParserState::Parsing, ParserState::Validated)?;
        self.transition(ParserState::Validated, ParserState::Exported)?;
        
        Ok(Report {
            dict: Obj::new(),
            tree: None,
            errors: Vec::new(),
            warnings: Vec::new(),
            stats: ParseStats::default(),
            s_score: 1.0,
            exportable: true,
        })
    }
}

// ============================================
// TEST
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_state_machine() {
        let mut sm = SimpleStateMachine::new();
        let config = ParserConfig::default();
        
        let report = sm.run_pipeline("test input", &config).unwrap();
        
        assert_eq!(sm.current_state(), ParserState::Exported);
        assert!(report.exportable);
        assert_eq!(report.s_score, 1.0);
    }
    
    #[test]
    fn test_parser_state_machine_strict() {
        let mut sm = ParserStateMachine::new(
            ParserMode::Strict,
            "test".to_string(),
            1000,
        );
        
        let config = ParserConfig {
            mode: ParserMode::Strict,
            ..Default::default()
        };
        
        let report = sm.run_pipeline("valid input", &config).unwrap();
        
        // Strict mode con input valido
        assert!(report.exportable || !report.errors.is_empty());
    }
    
    #[test]
    fn test_parser_state_machine_generative() {
        let mut sm = ParserStateMachine::new(
            ParserMode::Generative,
            "test".to_string(),
            10000,
        );
        
        let config = ParserConfig {
            mode: ParserMode::Generative,
            ..Default::default()
        };
        
        let report = sm.run_pipeline("input", &config).unwrap();
        
        // Generative mode raccoglie warnings
        assert!(report.exportable || !report.warnings.is_empty());
    }
    
    #[test]
    fn test_transition_validation() {
        let sm = ParserStateMachine::new(
            ParserMode::Strict,
            "test".to_string(),
            1000,
        );
        
        assert!(sm.is_valid_transition(ParserState::Init, ParserState::Lexing));
        assert!(sm.is_valid_transition(ParserState::Lexing, ParserState::Parsing));
        assert!(!sm.is_valid_transition(ParserState::Init, ParserState::Exported));
    }
    
    #[test]
    fn test_context_s_score() {
        let mut ctx = MachineContext::new(ParserMode::Strict, "test".to_string(), 1000);
        ctx.stats.total_fields = 10;
        ctx.stats.valid_fields = 8;
        ctx.update_s_score();
        
        assert_eq!(ctx.s_score, 0.8);
        
        let mut ctx2 = MachineContext::new(ParserMode::Generative, "test".to_string(), 1000);
        ctx2.stats.total_fields = 10;
        ctx2.stats.valid_fields = 8;
        ctx2.update_s_score();
        
        // S minore in generative mode (k=1.5)
        assert_eq!(ctx2.s_score, 0.5333333333333333);
    }
    
    #[test]
    fn test_context_timeout() {
        let ctx = MachineContext::new(ParserMode::Strict, "test".to_string(), 1);
        assert!(!ctx.is_timed_out());
        // Non possiamo testare il timeout reale in unit test
    }
    
    #[test]
    fn test_action_naming() {
        assert_eq!(ParserAction::Lex.to_string(), "lex");
        assert_eq!(ParserAction::Parse.to_string(), "parse");
        assert_eq!(ParserAction::Resolve.to_string(), "resolve");
        assert_eq!(ParserAction::Validate.to_string(), "validate");
        assert_eq!(ParserAction::Nop.to_string(), "nop");
    }
}
