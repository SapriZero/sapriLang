## Analisi dei README di `core/obj` e `core/base`

Ho letto i README. Ecco cosa contengono.

---

## 1. `core/base/README.md`

Questo crate fornisce i **fondamenti atomici** del sistema URCM.

### 1.1 Struttura

| Modulo | Contenuto |
|--------|-----------|
| `atom` | `Atom<T>` con `id`, `name`, `value`, `dependencies`, `compute` |
| `error` | `Error` e `Result` tipizzati |
| `fp` | Funzioni pure: `compose`, `pipe`, `curry`, `memoize`, `once` |

### 1.2 Concetti chiave

```rust
// Atomo base
pub struct Atom<T> {
    id: u64,
    name: String,
    value: Option<T>,
    dependencies: Vec<AtomId>,
    compute: Option<Box<dyn Fn() -> T>>,
}

// Pure functions
pub fn compose<A, B, C>(f: fn(B) -> C, g: fn(A) -> B) -> impl Fn(A) -> C
pub fn pipe<A, B, C>(f: fn(A) -> B, g: fn(B) -> C) -> impl Fn(A) -> C
pub fn curry<A, B, C>(f: fn(A, B) -> C) -> impl Fn(A) -> fn(B) -> C
pub fn memoize<A, B>(f: fn(A) -> B) -> impl Fn(A) -> B where A: Eq + Hash
pub fn once<F>(f: F) -> impl Fn() where F: Fn()
```

### 1.3 Cosa manca (per noi)

| Manca | Perché |
|-------|--------|
| **Contesto globale** | Non c'è un registry di atomi condiviso |
| **Reattività** | Gli atomi non propagano cambiamenti |
| **S = (v·i)/(t·k)** | Non implementato |
| **Moltiplicazione come unica operazione** | Non forzata |

**Questo crate è una base generica, non ancora URCM-specifica.**

---

## 2. `core/obj/README.md`

Questo crate fornisce **oggetti dinamici stile JavaScript** (tipo `serde_json::Value` ma più ricco).

### 2.1 Struttura

| Modulo | Contenuto |
|--------|-----------|
| `obj` | `Obj` struct con metodi `get`, `set`, `has`, `delete`, `keys` |
| `value` | `Value` enum (`Null`, `Bool`, `Number`, `String`, `Array`, `Object`) |
| `macros` | Macro `obj!` per costruzione dichiarativa |

### 2.2 Esempi

```rust
use sapri_obj::{obj, Obj};

let user = obj! {
    name: "Alice",
    age: 30,
    address: {
        city: "Wonderland"
    }
};

assert_eq!(user.get("name").as_string(), Some("Alice"));
assert_eq!(user.get("address.city").as_string(), Some("Wonderland"));
```

### 2.3 Funzionalità

| Funzione | Descrizione |
|----------|-------------|
| `get(path)` | Supporta path annidati (`"a.b.c"`) |
| `set(path, value)` | Crea i nodi intermedi se non esistono |
| `merge(other)` | Unisce due oggetti (spread) |
| `to_json()` | Converte in `serde_json::Value` |
| `from_json()` | Costruisce da JSON |

### 2.4 Cosa manca (per noi)

| Manca | Perché |
|-------|--------|
| **Validazione** | Non c'è schema, nessun vincolo |
| **Riferimenti** | `_:ref` non esiste |
| **Vincoli** | `_:req`, `_:guard`, `_:mutex` non esistono |
| **Stati** | `_:state` non esiste |

**Questo crate è un contenitore dati, non un validatore di vincoli.**

---

## 3. Come si integrano con `core/sson/`

La nostra architettura proposta:

```
core/sson/
├── validator.rs   → usa base::fp per composizione
├── resolver.rs    → usa obj::Obj per rappresentare il dizionario
├── constraint.rs  → usa base::fp per valutare condizioni
└── state_machine.rs → usa base::fp per transizioni
```

### 3.1 Relazione con `base`

| Funzione `base::fp` | Uso in `core/sson` |
|---------------------|-------------------|
| `compose` | Comporre validatori |
| `pipe` | Pipeline di validazione |
| `curry` | Validatori parametrizzati |
| `memoize` | Cache di risultati di validazione |
| `once` | Inizializzatori one-shot |

### 3.2 Relazione con `obj`

| Funzione `obj::Obj` | Uso in `core/sson` |
|---------------------|-------------------|
| `get(path)` | Risolvere `_:ref[path]` |
| `set(path, value)` | Applicare default |
| `merge` | Unire contesti (spread) |
| `to_json` | Esportare dizionario validato |

---

## 4. Cosa abbiamo già (riassunto)

| Crate | Funzione | Per `core/sson` serve |
|-------|----------|----------------------|
| `base/atom` | Atomi con dipendenze | Sì (DAG, risoluzione) |
| `base/fp` | Funzioni pure | Sì (validatori) |
| `base/error` | Gestione errori | Sì |
| `obj/obj` | Oggetti dinamici | Sì (dizionario) |
| `obj/value` | Valori tipizzati | Sì |
| `obj/macros` | `obj!` | Sì (test) |

**Manca ancora la semantica URCM:**
- `S = (v·i)/(t·k)`
- Moltiplicazione come unica operazione
- Reattività (push)
- Circuit breaker

---

## 5. Prossimo passo

Ora abbiamo un quadro chiaro:

1. **`base`** → atomi e funzioni pure (base tecnica)
2. **`obj`** → contenitore dati dinamico
3. **`core/sson`** → da scrivere (validatore, risolutore, stati)

**Manca ancora il motore URCM puro** (moltiplicazioni, reattività) che dovrebbe essere in `core/rust_dsl` (che abbiamo già visto).

---

**Vuoi che:**
1. **Scriviamo il `.sson` di specifica per `core/sson`** (descrivendo validatore, risolutore, stati)?
2. **Analizziamo `core/rust_dsl`** (che contiene il vero motore URCM)?
3. **Progettiamo l'integrazione tra `base`, `obj`, `rust_dsl` e `core/sson`**?
