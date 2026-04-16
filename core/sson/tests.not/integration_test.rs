// ============================================
// core/sson/tests/integration_test.rs
// Test end-to-end per il parser SSON
// ============================================

use sapri_sson::prelude::*;
use sapri_obj::{obj, Obj, Value};

// ============================================
// TEST UTILITY
// ============================================

fn create_test_config() -> ParserConfig {
    ParserConfig {
        mode: ParserMode::Strict,
        encoding: "utf-8".to_string(),
        max_depth: 100,
        version: SPEC_VERSION.to_string(),
        state: ParserState::Init,
    }
}

fn create_test_location() -> ErrorLocation {
    ErrorLocation {
        file: Some("test.sson".into()),
        line: 1,
        col: 1,
        path: "test".to_string(),
    }
}

// ============================================
// TEST 1: VALIDATORE BASE
// ============================================

mod test_validator {
    use super::*;

    #[test]
    fn test_req_validator() {
        let validator = BaseValidator::new();
        
        let obj = obj! { name: "Alice" };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let constraint = constraint!("req", "name");
        
        assert!(validator.validate(&ctx, &constraint));
        
        let empty_ctx = ValidationContext::new(ParserMode::Strict, Obj::new());
        assert!(!validator.validate(&empty_ctx, &constraint));
    }

    #[test]
    fn test_min_max_validator() {
        let validator = BaseValidator::new();
        
        let obj = obj! { age: 25 };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let min_constraint = constraint!("min", "age", value: 18.0);
        let max_constraint = constraint!("max", "age", value: 30.0);
        
        assert!(validator.validate(&ctx, &min_constraint));
        assert!(validator.validate(&ctx, &max_constraint));
        
        let min_constraint_fail = constraint!("min", "age", value: 30.0);
        assert!(!validator.validate(&ctx, &min_constraint_fail));
    }

    #[test]
    fn test_pattern_validator() {
        let validator = BaseValidator::new();
        
        let obj = obj! { email: "alice@example.com" };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let pattern = constraint!("pattern", "email", regex: r"^[^@]+@[^@]+\.[^@]+$");
        
        assert!(validator.validate(&ctx, &pattern));
        
        let obj2 = obj! { email: "invalid" };
        let ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
        assert!(!validator.validate(&ctx2, &pattern));
    }

    #[test]
    fn test_enum_validator() {
        let validator = BaseValidator::new();
        
        let obj = obj! { status: "active" };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let mut params = Obj::new();
        params.set("values", Value::Array(vec![
            Value::String("active".to_string()),
            Value::String("inactive".to_string()),
        ]));
        
        let enum_constraint = Constraint {
            name: "enum".to_string(),
            target: "status".to_string(),
            params,
        };
        
        assert!(validator.validate(&ctx, &enum_constraint));
        
        let obj2 = obj! { status: "deleted" };
        let ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
        assert!(!validator.validate(&ctx2, &enum_constraint));
    }

    #[test]
    fn test_mutex_validator() {
        let validator = BaseValidator::new();
        
        let obj = obj! { a: 10 };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let mut params = Obj::new();
        params.set("fields", Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]));
        
        let mutex_constraint = Constraint {
            name: "mutex".to_string(),
            target: "".to_string(),
            params,
        };
        
        assert!(validator.validate(&ctx, &mutex_constraint));
        
        let obj2 = obj! { a: 10, b: 20 };
        let ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
        assert!(!validator.validate(&ctx2, &mutex_constraint));
    }

    #[test]
    fn test_validate_all() {
        let mut validator = BaseValidator::new();
        
        let obj = obj! { name: "Alice", age: 25 };
        let mut ctx = ValidationContext::new(ParserMode::Strict, obj);
        
        let constraints = vec![
            constraint!("req", "name"),
            constraint!("req", "age"),
            constraint!("min", "age", value: 18.0),
        ];
        
        let s = validator.validate_all(&mut ctx, &constraints);
        assert_eq!(s, 1.0);
        
        let obj2 = obj! { name: "Alice" };
        let mut ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
        
        let s2 = validator.validate_all(&mut ctx2, &constraints);
        assert_eq!(s2, 2.0 / 3.0);
    }
}

// ============================================
// TEST 2: RESOLVER RIFERIMENTI
// ============================================

mod test_resolver {
    use super::*;

    #[test]
    fn test_path_normalizer() {
        let normalizer = PathNormalizer::new(10);
        
        assert_eq!(normalizer.normalize("user.name", "").unwrap(), "user.name");
        assert_eq!(normalizer.normalize(".age", "user").unwrap(), "user.age");
        assert_eq!(normalizer.normalize("..city", "user.address").unwrap(), "user.city");
    }

    #[test]
    fn test_basic_resolver() {
        let mut resolver = PathResolver::default();
        let mut ctx = ValidationContext::new(ParserMode::Strict, Obj::new());
        
        let mut base = Obj::new();
        base.set("version", Value::Number(1.0));
        resolver.set_base_context(base);
        
        let result = resolver.resolve(&mut ctx, "version");
        assert!(result.is_ok());
    }

    #[test]
    fn test_nested_resolver() {
        let mut resolver = PathResolver::default();
        let mut ctx = ValidationContext::new(ParserMode::Strict, Obj::new());
        
        let mut base = Obj::new();
        let mut user = Obj::new();
        user.set("name", Value::String("Alice".to_string()));
        user.set("age", Value::Number(30.0));
        base.set("user", Value::Object(user));
        resolver.set_base_context(base);
        
        let result = resolver.resolve(&mut ctx, "user.name");
        assert!(result.is_ok());
        
        let obj = result.unwrap();
        assert_eq!(obj.get("_value").as_string(), Some("Alice"));
    }

    #[test]
    fn test_resolver_cache() {
        let mut resolver = PathResolver::default();
        let mut ctx = ValidationContext::new(ParserMode::Strict, Obj::new());
        
        let mut base = Obj::new();
        base.set("value", Value::Number(42.0));
        resolver.set_base_context(base);
        
        let _ = resolver.resolve(&mut ctx, "value").unwrap();
        assert_eq!(resolver.stats().cache_misses, 1);
        assert_eq!(resolver.stats().cache_hits, 0);
        
        let _ = resolver.resolve(&mut ctx, "value").unwrap();
        assert_eq!(resolver.stats().cache_misses, 1);
        assert_eq!(resolver.stats().cache_hits, 1);
    }

    #[test]
    fn test_alias_resolver() {
        let mut alias_resolver = AliasResolver::new();
        alias_resolver.add_alias("u", "user");
        
        assert_eq!(alias_resolver.resolve_alias("u"), Some("user".to_string()));
        assert_eq!(alias_resolver.resolve_path("u.name"), "user.name");
    }
}

// ============================================
// TEST 3: FUNCTION REGISTRY
// ============================================

mod test_registry {
    use super::*;

    #[test]
    fn test_builtin_validators() {
        let registry = FunctionRegistryImpl::with_builtins();
        
        assert!(registry.contains("req"));
        assert!(registry.contains("min"));
        assert!(registry.contains("max"));
        assert!(registry.contains("pattern"));
        assert!(registry.contains("enum"));
    }

    #[test]
    fn test_custom_validator() {
        let mut registry = FunctionRegistryImpl::new();
        
        registry.register("is_even", Box::new(|ctx, constraint| {
            ctx.obj.get(&constraint.target).as_f64().map_or(false, |v| v % 2.0 == 0.0)
        }));
        
        let obj = obj! { value: 42 };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        let constraint = constraint!("is_even", "value");
        
        assert_eq!(registry.call("is_even", &ctx, &constraint), Some(true));
        
        let obj2 = obj! { value: 43 };
        let ctx2 = ValidationContext::new(ParserMode::Strict, obj2);
        assert_eq!(registry.call("is_even", &ctx2, &constraint), Some(false));
    }

    #[test]
    fn test_registry_cache() {
        let registry = FunctionRegistryImpl::with_builtins();
        
        let obj = obj! { name: "Alice" };
        let ctx = ValidationContext::new(ParserMode::Strict, obj);
        let constraint = constraint!("req", "name");
        
        let _ = registry.call("req", &ctx, &constraint);
        let stats = registry.get_stats();
        assert_eq!(stats.cache_misses, 1);
        
        let _ = registry.call("req", &ctx, &constraint);
        let stats2 = registry.get_stats();
        assert_eq!(stats2.cache_hits, 1);
    }
}

// ============================================
// TEST 4: OBJ ADAPTER
// ============================================

mod test_obj_adapter {
    use super::*;

    #[test]
    fn test_flat_to_nested() {
        let adapter = ObjAdapterImpl::new();
        
        let mut flat = HashMap::new();
        flat.insert("user.name".to_string(), Value::String("Alice".to_string()));
        flat.insert("user.age".to_string(), Value::Number(30.0));
        flat.insert("user.address.city".to_string(), Value::String("Wonderland".to_string()));
        
        let obj = adapter.from_flat_dict(&flat);
        
        assert_eq!(obj.get("user.name").as_string(), Some("Alice"));
        assert_eq!(obj.get("user.age").as_f64(), Some(30.0));
        assert_eq!(obj.get("user.address.city").as_string(), Some("Wonderland"));
    }

    #[test]
    fn test_nested_to_flat() {
        let adapter = ObjAdapterImpl::new();
        
        let obj = obj! {
            user: {
                name: "Alice",
                age: 30,
                address: {
                    city: "Wonderland"
                }
            }
        };
        
        let flat = adapter.to_flat_dict(&obj);
        
        assert_eq!(flat.get("user.name").unwrap().as_string(), Some("Alice"));
        assert_eq!(flat.get("user.age").unwrap().as_f64(), Some(30.0));
        assert_eq!(flat.get("user.address.city").unwrap().as_string(), Some("Wonderland"));
    }

    #[test]
    fn test_roundtrip() {
        let adapter = ObjAdapterImpl::new();
        
        let original = obj! {
            user: {
                name: "Alice",
                age: 30,
                tags: ["admin", "user"]
            }
        };
        
        let flat = adapter.to_flat_dict(&original);
        let reconstructed = adapter.from_flat_dict(&flat);
        
        assert_eq!(original.get("user.name").as_string(), reconstructed.get("user.name").as_string());
        assert_eq!(original.get("user.age").as_f64(), reconstructed.get("user.age").as_f64());
    }

    #[test]
    fn test_get_path() {
        let adapter = ObjAdapterImpl::new();
        
        let obj = obj! {
            user: {
                name: "Alice",
                address: {
                    city: "Wonderland"
                }
            }
        };
        
        assert_eq!(adapter.get_path(&obj, "user.name").unwrap().as_string(), Some("Alice"));
        assert_eq!(adapter.get_path(&obj, "user.address.city").unwrap().as_string(), Some("Wonderland"));
    }

    #[test]
    fn test_set_path() {
        let adapter = ObjAdapterImpl::new();
        let mut obj = Obj::new();
        
        adapter.set_path(&mut obj, "user.name", Value::String("Alice".to_string())).unwrap();
        adapter.set_path(&mut obj, "user.address.city", Value::String("Wonderland".to_string())).unwrap();
        
        assert_eq!(obj.get("user.name").as_string(), Some("Alice"));
        assert_eq!(obj.get("user.address.city").as_string(), Some("Wonderland"));
    }
}

// ============================================
// TEST 5: STATEMACHINE
// ============================================

mod test_statemachine {
    use super::*;

    #[test]
    fn test_simple_state_machine() {
        let mut sm = SimpleStateMachine::new();
        let config = create_test_config();
        
        let report = sm.run_pipeline("test", &config).unwrap();
        
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
}

// ============================================
// TEST 6: ERROR HANDLER
// ============================================

mod test_error_handler {
    use super::*;

    #[test]
    fn test_missing_required_error() {
        let err = missing_required_error("name", create_test_location());
        
        assert_eq!(err.code, error_codes::VALIDATE_REQUIRED_MISSING);
        assert_eq!(err.category, ErrorCategory::MissingRequired);
        assert_eq!(err.severity, Severity::Error);
        assert!(err.ai_hint.is_some());
    }

    #[test]
    fn test_min_value_error() {
        let err = min_value_error("age", 5.0, 18.0, create_test_location());
        
        assert_eq!(err.code, error_codes::VALIDATE_MIN_FAILED);
        assert!(err.ai_hint.is_some());
        
        if let Some(hint) = err.ai_hint {
            assert!(matches!(hint.action, AIAction::ChangeValue { .. }));
            assert_eq!(hint.target_path, "age");
        }
    }

    #[test]
    fn test_circular_error() {
        let stack = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let err = circular_error("c", stack, create_test_location());
        
        assert_eq!(err.code, error_codes::RESOLVE_CIRCULAR);
        assert_eq!(err.category, ErrorCategory::Circular);
    }

    #[test]
    fn test_error_handler_strict() {
        let mut handler = ErrorHandlerImpl::new(ParserMode::Strict);
        let err = missing_required_error("field", create_test_location());
        
        handler.add_error(err);
        assert!(handler.has_errors());
        assert_eq!(handler.errors().len(), 1);
        assert_eq!(handler.warnings().len(), 0);
    }

    #[test]
    fn test_error_handler_generative() {
        let mut handler = ErrorHandlerImpl::new(ParserMode::Generative);
        let err = missing_required_error("field", create_test_location());
        
        handler.add_error(err);
        assert!(!handler.has_errors());
        assert_eq!(handler.warnings().len(), 1);
        assert_eq!(handler.recovery_log().len(), 1);
    }

    #[test]
    fn test_is_exportable() {
        let mut handler = ErrorHandlerImpl::new(ParserMode::Strict);
        
        assert!(handler.is_exportable(0.95));
        
        handler.add_error(missing_required_error("field", create_test_location()));
        assert!(!handler.is_exportable(0.95));
        
        let mut handler2 = ErrorHandlerImpl::new(ParserMode::Strict);
        assert!(!handler2.is_exportable(0.5));
    }
}

// ============================================
// TEST 7: CALCOLO S (EQUILIBRIO)
// ============================================

mod test_equilibrium {
    use super::*;

    #[test]
    fn test_calculate_s_score() {
        // Strict mode: k = 1.0
        let s = calculate_s_score(8, 10, ParserMode::Strict);
        assert_eq!(s, 0.8);
        
        // Generative mode: k = 1.5
        let s2 = calculate_s_score(8, 10, ParserMode::Generative);
        assert!((s2 - 0.5333333333333333).abs() < 1e-6);
        
        // Tutti validi
        let s3 = calculate_s_score(10, 10, ParserMode::Strict);
        assert_eq!(s3, 1.0);
        
        // Nessuno valido
        let s4 = calculate_s_score(0, 10, ParserMode::Strict);
        assert_eq!(s4, 0.0);
    }

    #[test]
    fn test_is_exportable() {
        assert!(is_exportable(0.95, &[]));
        assert!(!is_exportable(0.85, &[]));
        
        let errors = vec![ParseError {
            code: "ERR_TEST".to_string(),
            message: "test".to_string(),
            location: create_test_location(),
            constraint: None,
        }];
        assert!(!is_exportable(0.95, &errors));
    }
}

// ============================================
// TEST 8: MACROS
// ============================================

mod test_macros {
    use super::*;

    #[test]
    fn test_constraint_macro() {
        let c = constraint!("req", "name");
        assert_eq!(c.name, "req");
        assert_eq!(c.target, "name");
        
        let c2 = constraint!("min", "age", value: 18.0);
        assert_eq!(c2.name, "min");
        assert_eq!(c2.target, "age");
        assert_eq!(c2.params.get("value").as_f64(), Some(18.0));
        
        let c3 = constraint!("range", "score", min: 0.0, max: 100.0);
        assert_eq!(c3.params.get("min").as_f64(), Some(0.0));
        assert_eq!(c3.params.get("max").as_f64(), Some(100.0));
    }

    #[test]
    fn test_error_location_macro() {
        let loc = error_location!();
        assert!(loc.file.is_some());
        assert!(loc.line > 0);
        
        let loc2 = error_location!("test.path");
        assert_eq!(loc2.path, "test.path");
    }
}

// ============================================
// TEST 9: INTEGRAZIONE COMPLETA
// ============================================

mod test_integration {
    use super::*;

    #[test]
    fn test_full_pipeline() {
        // 1. Crea configurazione
        let config = ParserConfig {
            mode: ParserMode::Strict,
            max_depth: 10,
            ..Default::default()
        };
        
        // 2. Crea un oggetto da validare
        let obj = obj! {
            user: {
                name: "Alice",
                age: 25,
                email: "alice@example.com",
                status: "active"
            }
        };
        
        // 3. Definisce constraints
        let constraints = vec![
            constraint!("req", "user.name"),
            constraint!("req", "user.age"),
            constraint!("req", "user.email"),
            constraint!("min", "user.age", value: 18.0),
            constraint!("max", "user.age", value: 100.0),
            constraint!("pattern", "user.email", regex: r"^[^@]+@[^@]+\.[^@]+$"),
            constraint!("enum", "user.status", values: vec!["active", "inactive"]),
        ];
        
        // 4. Esegue validazione
        let mut validator = BaseValidator::new();
        let mut ctx = ValidationContext::new(config.mode, obj);
        let s = validator.validate_all(&mut ctx, &constraints);
        
        // 5. Verifica
        assert_eq!(s, 1.0);
        assert!(is_exportable(s, &[]));
    }

    #[test]
    fn test_full_pipeline_with_errors() {
        let config = ParserConfig::default();
        
        // Oggetto con errori
        let obj = obj! {
            user: {
                name: "Alice",
                age: 15,  // Troppo giovane
                email: "invalid",  // Email non valida
                status: "deleted"  // Valore non nell'enum
            }
        };
        
        let constraints = vec![
            constraint!("req", "user.name"),
            constraint!("min", "user.age", value: 18.0),
            constraint!("pattern", "user.email", regex: r"^[^@]+@[^@]+\.[^@]+$"),
            constraint!("enum", "user.status", values: vec!["active", "inactive"]),
        ];
        
        let mut validator = BaseValidator::new();
        let mut ctx = ValidationContext::new(config.mode, obj);
        let s = validator.validate_all(&mut ctx, &constraints);
        
        // 3 errori su 4 constraints
        assert_eq!(s, 0.25);
        assert!(!is_exportable(s, &[]));
    }

    #[test]
    fn test_generative_mode_recovery() {
        let config = ParserConfig {
            mode: ParserMode::Generative,
            ..Default::default()
        };
        
        let obj = obj! {
            user: {
                name: "Alice",
                unknown_field: "something"  // Campo non previsto
            }
        };
        
        let constraints = vec![
            constraint!("req", "user.name"),
        ];
        
        let mut validator = BaseValidator::new();
        let mut ctx = ValidationContext::new(config.mode, obj);
        let s = validator.validate_all(&mut ctx, &constraints);
        
        // In modalità generative, il campo extra viene ignorato con warning
        assert_eq!(s, 1.0);
    }
}

// ============================================
// TEST 10: BENCHMARK (opzionale)
// ============================================

#[cfg(test)]
mod benchmark {
    use super::*;
    use std::time::Instant;

    #[test]
    fn bench_validation() {
        let validator = BaseValidator::new();
        let obj = obj! {
            name: "Alice",
            age: 25,
            email: "alice@example.com",
            status: "active",
            tags: ["admin", "user"],
            metadata: obj! { created: "2024-01-01", version: 2 }
        };
        
        let constraints = vec![
            constraint!("req", "name"),
            constraint!("req", "age"),
            constraint!("req", "email"),
            constraint!("req", "status"),
            constraint!("min", "age", value: 0.0),
            constraint!("max", "age", value: 150.0),
            constraint!("pattern", "email", regex: r"^[^@]+@[^@]+\.[^@]+$"),
            constraint!("enum", "status", values: vec!["active", "inactive"]),
        ];
        
        let start = Instant::now();
        for _ in 0..1000 {
            let ctx = ValidationContext::new(ParserMode::Strict, obj.clone());
            let _ = validator.validate_all(&mut ctx.clone(), &constraints);
        }
        let duration = start.elapsed();
        
        println!("1000 validations took {:?}", duration);
        println!("Average: {:?}", duration / 1000);
    }
}
