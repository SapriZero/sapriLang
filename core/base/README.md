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








	
Diagramma testuale base → extended → sson → app, regole di dipendenza
	
Spiega il "perché" della suddivisione in crate
4. 📖 API di Riferimento (Core)
	
Esempi minimi per atom, fp/macros, bucket, error
	
Documentazione viva, copy-paste ready
5. 🧪 Esempi Pratici & Pattern
	
Casi d'uso reali: stato lazy, valutazione condizionale, ordinamento bucket
	
Mostra il valore concreto della libreria
6. 🔗 Integrazione con Extended & SSON
	
Come base si collega al parser .sson e alla generazione struct
	
Ponte verso il resto del progetto
7. 🛠️ Sviluppo & Workflow
	
Test, clippy, benchmark, regole di commit, come aggiungere moduli
	
Manutenzione a lungo termine
8. 📜 Licenza & Crediti
	
MIT, autori, link a repo correlati
	
Chiusura professionale
