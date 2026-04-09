# 📘 Sapri Core Base

Il fondamento modulare, ultra-leggero e type-safe per l'ecosistema Sapri.

## 🎯 Cos'è
`sapri-core-base` è il layer fondamentale dell'architettura Sapri. Progettato per essere **indipendente, veloce da compilare (< 2s) e privo di dipendenze esterne pesanti**, 
fornisce le primitive essenziali su cui costruire tutto il resto: dal motore funzionale, alla gestione dello stato, fino alle strutture dati ottimizzate.

## ✨ Filosofia di Design
- 🧩 **Modulare**: Ogni componente è isolato e testabile. Importa solo ciò che ti serve.
- ⚡ **Performante**: Zero overhead a runtime. Algoritmi puri, memoria prevedibile, compile-time ottimizzato.
- 🔒 **Type-Safe**: Controllo rigoroso a compile-time. Errori intercettati prima dell'esecuzione.
- 🚀 **Pronto per l'estensione**: Base solida e neutra su cui `sapri-core-extended`, `sapri-sson` e i generatori di codice possono costruire senza duplicazioni.

## 📦 Cosa contiene
| Modulo | Responsabilità |
|--------|----------------|
| `atom` & `atom_impl` | Primitiva di stato lazy/resolved. Trait `PromiseState` e `ExternalSource` per risoluzione differita. |
| `fp` & `macros` | Macro dichiarative pure: `eval!`, `mask!`, `curry!`, `lazy_if!`, `try_or!`. Zero proc-macro. |
| `bucket/array` & `sort` | Strutture dati ad accesso rapido e `counting_sort` ottimizzati per slice native (`u8`, `u16`). |
| `error` | Gerarchia di errori leggera e tipata, pronta per la propagazione ergonomica con `?`. |

> ⚠️ **Nota**: I moduli `context` ed `eval` (runtime, binding, serializzazione) sono stati spostati in `sapri-core-extended` per mantenere `base` pulito e veloce.



## 🚀 2. Installazione & Setup

### 📦 Aggiunta al progetto
Aggiungi `sapri-core-base` al tuo `Cargo.toml`:
```toml
[dependencies]
sapri-core-base = { path = "core/base", version = "0.1" }
```
*(Se pubblicato su crates.io in futuro, basterà `sapri-core-base = "0.1"`)*

### 🗂️ Configurazione Workspace (Consigliata)
Se il tuo progetto è un monorepo, assicurati che il `Cargo.toml` radice includa i crate strutturati correttamente:
```toml
[workspace]
members = [
    "core/base",
    "core/extended",
    "core-data",
    "editor-server",
    "sson",
    "tools/sson-gen"
]
resolver = "2"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### 🛠️ Comandi Essenziali
```bash
# Compila solo questo crate (ignora gli altri membri del workspace)
cargo build -p sapri-core-base

# Esegui la suite di test unitari
cargo test -p sapri-core-base

# Verifica senza generare binari (più veloce, ideale per CI)
cargo check -p sapri-core-base

# Analisi statica con clippy (best practices Rust)
cargo clippy -p sapri-core-base -- -D warnings
```

### ✅ Requisiti Minimi
- Rust `1.70+` (Edition 2021)
- Cargo workspace con `resolver = "2"` (obbligatorio per feature unification)
- **Zero dipendenze esterne pesanti**: compile-time target **< 2s** su hardware medio

---

## 🧱 3. Architettura Layered

Sapri è progettato come un'architettura a strati (**Layered Architecture**). Ogni livello dipende esclusivamente da quelli inferiori, garantendo compilazione rapida, isolamento dei test e massima flessibilità per i progetti che lo adottano.

### 📐 Diagramma di Dipendenza
```
[ Il Tuo Applicativo / Editor-Server ]
              │
              ▼
[ sapri-sson ]             ← Parser .sson, validazione S, codegen struct/flow
              │
              ▼
[ sapri-core-extended ]    ← Runtime context, eval avanzato, bucket complessi, macro generative
              │
              ▼
[ sapri-core-base ]        ← Primitive, FP puro, atom, counting sort, errori base (QUESTO CRATE)
              │
              ▼
          [ std ]          ← Libreria standard Rust
```

### 📦 Descrizione dei Layer
| Layer | Ruolo | Dipendenze Esterne | Compile-Time Target |
|-------|-------|-------------------|---------------------|
| `base` | Fondamento matematico/strutturale | `itertools`, `thiserror` (leggere) | **< 2s** |
| `extended` | Runtime, contesto, I/O leggero, macro pesanti | `serde`, `serde_json`, `base` | ~4-6s |
| `sson` | Linguaggio di configurazione, validazione, AST | `extended`, parser combinatori | ~5-8s |
| `Applicativo` | Logica di dominio, server, UI | `sson`, `extended`, framework scelti | Variabile |

### 📜 Regole Ferree di Dipendenza
1. 🔽 **Mai verso l'alto**: `base` non può importare `extended`, `sson` o l'applicativo.
2. 🔁 **Mai circolari**: Le dipendenze formano un DAG (Directed Acyclic Graph) puro.
3. 🧼 **Zero side-effect in base**: `base` contiene solo funzioni pure, struct dati e macro dichiarative. Niente I/O, async, o stato globale mutabile.
4. ⚖️ **Pay only for what you use**: I progetti minimali importano solo `base`. Quelli che usano `.sson` importano `sson` (che risolve automaticamente `extended` e `base`).

### 💡 Perché questa scelta?
- 🚀 **Compile-time prevedibile**: Modificare il parser `.sson` o il runtime non costringe a ricompilare le primitive di base.
- 🧪 **Testing isolato**: I test di `counting_sort` o `eval!` girano in millisecondi, senza mock complessi o setup di runtime.
- 🔄 **Evoluzione indipendente**: Puoi pubblicare `base` su crates.io subito, mentre `sson` o l'editor sono ancora in sviluppo attivo.
- 🎯 **IDE-Friendly**: `rust-analyzer` indicizza meglio i crate piccoli e focalizzati. L'autocomplete rimane reattivo anche in workspace con centinaia di file.


### 💡 Perché abbiamo diviso in crate? (Il "Perché" Architecturale)
La scelta di non creare un unico `core` monolitico nasce da limiti concreti del compiler Rust e dalla necessità di mantenere un ciclo di sviluppo veloce:

| Problema del Monolite | Soluzione Layered |
|----------------------|-------------------|
| 🐢 **Compile-time esplosivo**: modificare il parser `.sson` o aggiungere una `proc-macro` costringe a ricompilare anche algoritmi e primitive di base. | ⚡ **Compilazione incrementale**: `base` compila in <2s. Cambiare `extended` o `sson` non tocca il layer fondamentale. |
| 🔗 **Dipendenze nascoste**: `serde`, `tokio` o `syn` vengono tirati in anche da chi usa solo `counting_sort` o `eval!`. | 🧼 **Confini espliciti**: il compiler garantisce che `base` importi solo `std` e crate leggeri. Zero dipendenze transitive indesiderate. |
| 🧪 **Test lenti e fragili**: i test unitari devono inizializzare runtime, mockare I/O o parsare stringhe complesse. | 🎯 **Testing isolato**: i test di `base` sono puri, deterministici e girano in millisecondi. CI più veloce, feedback immediato. |
| 🔄 **Release bloccate**: non puoi pubblicare una fix al sort o agli errori finché il parser `.sson` non è stabile. | 📦 **Versioning indipendente**: `base`, `extended` e `sson` possono avere cicli di release separati e essere pubblicati su crates.io in momenti diversi. |
| 🖥️ **IDE pesante**: `rust-analyzer` fatica a indicizzare crate con 10k+ LOC, macro procedurali e dipendenze complesse. | 🚀 **Developer Experience**: crate piccoli e focalizzati → autocomplete istantaneo, refactoring sicuro, onboarding semplificato. |

> 📌 **Regola d'oro**: ogni crate deve avere un **dominio chiaro**, **dipendenze minime** e **zero responsabilità trasversali**. Se un modulo inizia a usare `serde` o I/O, esce da `base` e va in `extended`.

---

## 📖 4. API di Riferimento (Core)

Esempi minimali, pronti per il copy-paste. Tutti i simboli sono esposti direttamente alla radice del crate grazie a `#[macro_export]` e `pub use`.

### 🔹 Gestione Errori (`error`)
Alias tipato e varianti semantiche per le primitive di base.
```rust
use sapri_core_base::{BaseError, Result};

fn divide(a: i32, b: i32) -> Result<i32> {
    if b == 0 {
        Err(BaseError::InvalidArg { msg: "division by zero".into() })
    } else {
        Ok(a / b)
    }
}

// Utilizzo ergonomico con `?` o pattern matching
match divide(10, 0) {
    Ok(v) => println!("Risultato: {v}"),
    Err(BaseError::InvalidArg { msg }) => eprintln!("Errore valido: {msg}"),
    _ => {}
}
```

### 🔹 Stato Atomico (`atom` & `atom_impl`)
Valori lazy che risolvono il loro contenuto solo quando necessario. Ideale per configurazioni differite, cache o dati esterni.
```rust
use sapri_core_base::{Atom};
use sapri_core_base::atom_impl::PromiseState; // Trait necessario per .resolve()

let mut config = Atom::<String>::pending();
assert!(!config.is_ready());

// Risoluzione differita
config = config.resolve("db_host=127.0.0.1".into());
assert!(config.is_ready());
assert_eq!(config.get(), "db_host=127.0.0.1");

// Creazione da sorgente esterna
let external = Atom::<Vec<u8>>::external("s3://bucket/config.json");
assert_eq!(external.source(), Some("s3://bucket/config.json"));
```

### 🔹 Macro Funzionali (`fp` & `macros`)
Zero overhead a runtime. Espansione a compile-time, type-safe e composabili.
```rust
use sapri_core_base::{eval, mask, curry, try_or};

// eval! → Valutazione condizionale con short-circuit
let x = eval!(true, 10 + 5, panic!("Non eseguito"));
assert_eq!(x, 15);

// mask! → Trasformazione applicata solo se il predicato è vero
let value = mask!(42, |&v| v > 10, |v| v * 2);
assert_eq!(value, 84);

// curry! → Applica parzialmente una funzione a 2 argomenti
fn add(a: i32, b: i32) -> i32 { a + b }
let add_10 = curry!(add)(10);
assert_eq!(add_10(5), 15);

// try_or! → Fallback ergonomico su Result
let res: Result<i32, _> = Err(BaseError::NotFound { path: "key".into() });
let fallback = try_or!(res, -1);
assert_eq!(fallback, -1);
```

### 🔹 Ordinamento Ultra-Rapido (`bucket/sort`)
Counting sort ottimizzato per slice native. Complessità `O(n + k)`, zero allocazioni extra.
```rust
use sapri_core_base::bucket::{counting_sort_u8, counting_sort_u16};

// u8 (0-255)
let mut data_u8 = [5u8, 2, 8, 1, 9, 3, 7, 4, 6, 0];
counting_sort_u8(&mut data_u8);
assert_eq!(data_u8, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

// u16 (0-65535)
let mut data_u16 = [300u16, 100, 400, 50, 200];
counting_sort_u16(&mut data_u16);
assert_eq!(data_u16, [50, 100, 200, 300, 400]);

// ✅ Sicuro anche con slice vuote o già ordinate
let mut empty: [u8; 0] = [];
counting_sort_u8(&mut empty); // Panic-free, zero operazioni
```

### 🔗 Note sull'uso nei progetti reali
- Le macro `eval!`, `mask!`, ecc. **non richiedono `use` esplicito** se il crate è nella `Cargo.toml`, ma è buona pratica importarle per chiarezza: `use sapri_core_base::eval;`
- `Atom` è `Clone` e `Send + Sync` se `T` lo è: puoi passarlo tra thread o memorizzarlo in cache.
- `counting_sort` modifica la slice **in-place**. Per ordinamenti stabili con payload complessi, usa `sapri-core-extended::bucket::sort::counting_sort_u16_stable`.

---
```

### 🔜 Prossimo passo
Ora hai:
✅ **Sezione 1**: Intro + Filosofia + Tabella moduli  
✅ **Sezione 2**: Installazione + Workspace + Comandi  
✅ **Sezione 3**: Architettura Layered + Diagramma + **Perché la divisione**  
✅ **Sezione 4**: API Reference con esempi compilabili  

Vuoi che prepariamo:
🔹 **Sezione 5: 🧪 Pattern & Casi d'Uso Reali** → Come si integra `.sson` → struct generation, validazione `S`, flussi CRUD dichiarativi?  
🔹 **Sezione 6: 🛠️ Workflow di Sviluppo** → Test, clippy, benchmark, come aggiungere un nuovo modulo rispettando i confini?  

Dimmi quale numero vuoi chiudere per avere un README **pronto alla pubblicazione**. 🦀📖
