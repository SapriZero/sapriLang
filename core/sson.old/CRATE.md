## `core/sson/Cargo.toml` - Configurazione del Crate

---

## Riepilogo del file `Cargo.toml`

| Sezione | Contenuto | Descrizione |
|---------|-----------|-------------|
| `[package]` | name, version, edition, authors, description, license | Metadata del crate |
| `[features]` | default, std, serde, full | Feature flags per funzionalità opzionali |
| `[dependencies]` | sapri_base, sapri_obj, regex, once_cell | Dipendenze obbligatorie |
| `[dependencies]` | serde, serde_json (opzionali) | Serializzazione JSON |
| `[dev-dependencies]` | pretty_assertions | Test utilities |
| `[lib]` | name, path, test, doctest | Configurazione libreria |
| `[[test]]` | name, path, required-features | Configurazione test |
| `[profile.*]` | opt-level, debug, lto | Ottimizzazioni compilazione |

---

## Dipendenze spiegate

| Dipendenza | Versione | Scopo | Obbligatoria |
|------------|----------|-------|--------------|
| `sapri_base` | path | Atomi, funzioni pure, errori | ✅ Sì |
| `sapri_obj` | path | Oggetti dinamici, Value, Obj | ✅ Sì |
| `regex` | 1.10 | Validazione pattern (`_:pattern`) | ✅ Sì |
| `once_cell` | 1.19 | Lazy singleton per registry globale | ✅ Sì |
| `serde` | 1.0 (opzionale) | Serializzazione per JSON | ❌ No |
| `serde_json` | 1.0 (opzionale) | Parsing JSON | ❌ No |

---

## Feature flags

| Feature | Dipendenze attivate | Funzionalità |
|---------|---------------------|--------------|
| `default` | `std` | Funzionalità base |
| `std` | - | Supporto libreria standard |
| `serde` | `serde`, `serde_json` | Serializzazione/deserializzazione |
| `full` | `std`, `serde` | Tutte le funzionalità |

---

## Come usare il crate

### In `Cargo.toml` del progetto principale

```toml
[dependencies]
sapri_sson = { path = "core/sson" }
```

### Con feature opzionali

```toml
[dependencies]
sapri_sson = { path = "core/sson", features = ["full"] }
```

### Esempio di utilizzo in Rust

```rust
use sapri_sson::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Crea configurazione
    let config = ParserConfig {
        mode: ParserMode::Strict,
        ..Default::default()
    };
    
    // Crea validatore
    let validator = BaseValidator::new();
    
    // Crea oggetto da validare
    let obj = obj! {
        name: "Alice",
        age: 25
    };
    
    // Crea constraints
    let constraints = vec![
        constraint!("req", "name"),
        constraint!("min", "age", value: 18.0),
    ];
    
    // Valida
    let mut ctx = ValidationContext::new(config.mode, obj);
    let s = validator.validate_all(&mut ctx, &constraints);
    
    println!("S = {:.3}", s);
    println!("Exportable: {}", is_exportable(s, &[]));
    
    Ok(())
}
```
---

## Struttura finale del crate

```
core/sson/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── validator_impl.rs
│   ├── resolver_impl.rs
│   ├── registry_impl.rs
│   ├── obj_adapter_impl.rs
│   ├── state_machine_impl.rs
│   └── error_impl.rs
└── tests/
    └── integration_test.rs
```
