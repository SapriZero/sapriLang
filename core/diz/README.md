# sapri_diz - Dizionario Strutturato Globale

Dizionario centralizzato per la gestione di **path costanti**, **vocabolario** e **dati di configurazione** con autocompletamento IDE e validazione a compile-time.

---

## Filosofia

Tutti i nomi (paths, tipi, campi, verbi) sono definiti in `.sson` e diventano **costanti Rust** a compile-time.  
Niente stringhe hard-coded. Niente errori a runtime. Autocompletamento IDE completo.

---

## Struttura

```
core/diz/
├── Cargo.toml
├── build.rs              # Genera codice Rust da diz.sson
├── diz.sson              # Definizione di paths e vocabolario
├── diz_data.json         # Dati reali (filter_words, suffissi, charmap)
├── src/
│   └── lib.rs            # Libreria (include codice generato)
└── README.md
```

---


## Spiegazione dei componenti di `diz_data.json`

### `filter_words`

Le **filter_words** sono parole che vengono **filtrate/ignorate** durante l'elaborazione del testo perché:

- Non portano significato semantico rilevante
- Sono estremamente frequenti
- Appesantiscono l'analisi senza aggiungere valore

**Esempio:** In italiano, parole come `"il"`, `"lo"`, `"la"`, `"e"`, `"a"` non ci dicono molto sul contenuto di una frase.

```json
"filter_words": {
  "list": ["a", "e", "i", "o", "il", "lo", "la", "con", "per", "non", "che", "è", "sono"]
}
```

## Hai ragione! Sono due cose diverse. Facciamo chiarezza.

---

## 1. `charmap` - Per il **testo dell'utente** (input/output)

Serve per **comprimere le parole che l'utente scrive** (es. domande, frasi).

```
Utente scrive: "casa"
       ↓
charmap (6-bit)
       ↓
[2, 0, 18, 0]  (array di byte)
       ↓
Ricerca nel dizionario compresso
```

**Scopo:** Risparmiare memoria e velocizzare la ricerca.

---

## 2. `vocabulary` - Per i **nomi del programma** (codice)

Serve per **validare che i nomi nel codice siano ammessi** (es. struct, field, verbo).

```rust
// Questi nomi sono definiti in .sson e validati a compile-time
validate_name!("Brain", structs);        // OK
validate_name!("Runtime", fields);       // OK
validate_name!("BrainX", structs);       // ERRORE!
```

**Scopo:** Evitare errori di battitura, centralizzare il naming, IDE autocompletamento.

---

## La differenza sostanziale

| Aspetto | `charmap` | `vocabulary` |
|---------|-----------|--------------|
| **Usato per** | Testo utente (domande, risposte) | Nomi nel codice Rust |
| **Quando** | Runtime (elaborazione testo) | Compile-time (validazione codice) |
| **Contenuto** | Caratteri (a-z, à, è, spazio, ...) | Nomi di struct, field, verbi |
| **Scopo** | Compressione, ricerca O(1) | Controllo naming, autocompletamento |
| **Esempio** | `"casa"` → `[2,0,18,0]` | `"Brain"` → controllo se esiste nel `.sson` |

---

## Schema riassuntivo

```
┌─────────────────────────────────────────────────────────────────────┐
│                          SAPRI SYSTEM                               │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    INPUT UTENTE (testo)                      │   │
│  │                                                              │   │
│  │  "casa" → [charmap] → [2,0,18,0] → ricerca nel dizionario   │   │
│  │                                                              │   │
│  │  Usa: charmap (6-bit, escape 63)                            │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    CODICE RUST (nomi)                        │   │
│  │                                                              │   │
│  │  struct Brain { ... }                                        │   │
│  │         ↑                                                    │   │
│  │  validate_name!("Brain", structs) ← controllo vocabolario   │   │
│  │                                                              │   │
│  │  Usa: vocabulary (tipi, structs, fields, verbs)             │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    ACCESSO AI DATI (paths)                   │   │
│  │                                                              │   │
│  │  let data = diz!(lang.it.filter_words.list);                    │   │
│  │                   ↑                                          │   │
│  │              path costante (autocompletato)                  │   │
│  │                                                              │   │
│  │  Usa: paths (costanti per accedere a diz_data.json)         │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    DATI REALI (configurazione)               │   │
│  │                                                              │   │
│  │  diz_data.json: { "lang": { "it": { "filter_words": {...} } }   │   │
│  │                                                              │   │
│  │  Usa: load_diz() → struct tipizzate (autocompletate)        │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Esempio concreto nel codice

```rust
use sapri_diz::{diz, paths, vocabulary, load_diz, validate_name};

fn main() {
    // ============================================
    // 1. VALIDAZIONE NOMI DEL CODICE (compile-time)
    // ============================================
    validate_name!("Brain", structs);        // OK - Brain è nel vocabolario
    validate_name!("memory", fields);        // OK - memory è un field valido
    validate_name!("new", verbs);            // OK - new è un verbo valido
    
    // ============================================
    // 2. ACCESSO AI DATI (paths costanti)
    // ============================================
    // Usando path costante
    let filter_words: Vec<String> = diz!(paths::LANG_IT_filter_words_LIST -> Vec<String>).unwrap();
    
    // Usando struct annidata (autocompletamento IDE)
    let filter_words2: Vec<String> = diz!(lang::it::filter_words::LIST -> Vec<String>).unwrap();
    
    // ============================================
    // 3. DATI TIPIZZATI (struct)
    // ============================================
    let diz_data = load_diz();
    let bits = diz_data.it.charmap.bits;           // 6
    let escape = diz_data.it.charmap.escape_code;  // 63
    
    // ============================================
    // 4. COMPRESSIONE TESTO UTENTE (charmap)
    // ============================================
    // Qui useremmo charmap per codificare "casa" in [2,0,18,0]
    // (implementato in un altro crate, non in diz)
}
```

---

## Perché servono entrambi?

| Domanda | Risposta |
|---------|----------|
| **Perché non usare solo `charmap` per tutto?** | `charmap` codifica **caratteri**, non **nomi di struct**. Non ha senso codificare "Brain" come array di byte. |
| **Perché non usare solo `vocabulary` per tutto?** | `vocabulary` valida **nomi del codice**, non può comprimere testo utente variabile. |
| **Perché servono i `paths`?** | Per non scrivere stringhe hard-coded `"lang.it.filter_words.list"` nel codice. |
| **Perché servono le struct `load_diz()`?** | Per avere autocompletamento sui dati reali (es. `diz_data.it.charmap.bits`). |

---

**In sintesi:** `charmap` è per i **caratteri del testo**. `vocabulary`/`paths`/`load_diz` sono per i **nomi e la struttura del programma**. Sono livelli diversi.



**Uso nel codice:**
```rust
if filter_words.contains(&word) {
    // Salta questa parola, non viene indicizzata
    continue;
}
```

---

### `charmap` (Mappa caratteri)

La **mappa caratteri** converte caratteri Unicode in **codici numerici compatti** (da 0 a 63) per:

1. **Ridurre lo spazio di memoria** (6 bit invece di 8/16/32)
2. **Velocizzare la ricerca** (confronto numerico invece che stringhe)
3. **Normalizzare varianti** (es. 'à' e 'a' diventano stesso codice)

#### `bits: 6`

Significa che ogni carattere viene codificato in **6 bit** (valori da 0 a 63).

| Numero bit | Valori possibili | Risparmio rispetto a 8-bit |
|------------|------------------|---------------------------|
| 8 bit | 0-255 | 0% (base) |
| 7 bit | 0-127 | 12.5% |
| **6 bit** | **0-63** | **25%** |
| 5 bit | 0-31 | 37.5% |

**25% di risparmio** su ogni carattere. Per una parola di 10 caratteri → 60 bit invece di 80.

#### `escape_code: 63`

Il codice **63** è riservato come **"escape"** per caratteri non nella mappa.

Quando incontra un carattere non previsto (es. `'€'`, `'@'`, caratteri cinesi), il sistema:
1. Scrive `63` (escape)
2. Poi scrive il codice completo del carattere (16 o 32 bit)

```
[escape_code][codice_completo]
```

**Esempio:**
```
La parola "casa€" viene codificata come:
c → 2
a → 0
s → 18
a → 0
€ → 63 + 0x20AC (codice Unicode di €)
```

---

### Tabella di codifica dei caratteri italiani

| Intervallo | Caratteri | Codici | Descrizione |
|------------|-----------|--------|-------------|
| 0-25 | a, b, c, ..., z | 0-25 | Lettere minuscole |
| 26-51 | A, B, C, ..., Z | 26-51 | Lettere maiuscole |
| 52 | spazio | 52 | Separatore parole |
| 53-58 | à, è, é, ì, ò, ù | 53-58 | Vocali accentate |
| 59-62 | ., ,, -, _ | 59-62 | Punteggiatura comune |
| 63 | ESCAPE | 63 | Carattere non mappato |

---

### Perché 6 bit?

| Fattore | Spiegazione |
|---------|-------------|
| **Dimensione** | 6 bit = 64 caratteri → copre a-z, A-Z, spazio, accentati, punteggiatura |
| **Allineamento** | 6 bit è un multiplo di 8 bit (byte = 8 bit), ma richiede attenzione nell'impacchettamento |
| **Storico** | ASCII originale usava 7 bit (128 caratteri). 6 bit era usato nei primi computer per compattazione |
| **Italiano** | L'alfabeto italiano ha 21 lettere + 5 accentate + maiuscole → 52 caratteri + spazio + punteggiatura → entra in 6 bit |

---

### Esempio di codifica

```rust
let charmap = load_diz().it.charmap;

// Codifica
let code = charmap.char_to_code('c');  // 2
let code = charmap.char_to_code('à');  // 53
let code = charmap.char_to_code('€');  // 63 (escape)

// Decodifica
let c = charmap.code_to_char(2);   // Some('c')
let c = charmap.code_to_char(53);  // Some('à')
let c = charmap.code_to_char(63);  // None (escape, serve lettura successiva)
```

---

### Vantaggi nel dizionario compresso

| Aspetto | Prima | Con charmap 6-bit |
|---------|-------|-------------------|
| **Memoria per parola** | (byte per carattere) | 25% meno |
| **Ricerca** | Confronto stringhe | Confronto array di byte |
| **Cache CPU** | Più cache miss | Più cache hit |
| **Normalizzazione** | Manuale | Automatica (à→53, a→0, diversi) |

**Esempio pratico:**
```
Parola "casa"
- UTF-8: 4 byte (32 bit)
- 6-bit: 4 × 6 = 24 bit → 8 bit risparmiati
```

---
## Componenti

### 1. Path Costanti (`paths`)

```rust
use sapri_diz::paths;

// Costanti per accesso ai dati
let filter_words_path = paths::LANG_IT_filter_words_LIST;  // "lang.it.filter_words.list"
let bits_path = paths::LANG_IT_CHARMAP_BITS;          // "lang.it.charmap.bits"
```

### 2. Struct Annidate (`lang`, `core`)

```rust
use sapri_diz::{lang, core};

// Autocompletamento IDE funziona!
let filter_words_path = lang::it::filter_words::LIST;
let runtime_path = core::brain::fields::RUNTIME;
```

### 3. Vocabolario (`vocabulary`)

```rust
use sapri_diz::vocabulary;

// Verifica se un nome è ammesso
if vocabulary::fields::ALLOWED.contains(&"runtime") {
    println!("'runtime' è un campo valido");
}
```

### 4. Macro `diz!`

```rust
use sapri_diz::diz;

// Ottieni valore grezzo
let filter_words = diz!(lang.it.filter_words.list);

// Ottieni con tipo
let filter_words: Vec<String> = diz!(lang.it.filter_words.list -> Vec<String>).unwrap();
let bits: u8 = diz!(lang.it.charmap.bits -> u8).unwrap();
```

### 5. Macro `validate_name!` (compile-time)

```rust
use sapri_diz::validate_name;

validate_name!("Brain", structs);   // ✅ OK
validate_name!("BrainX", structs);  // ❌ ERRORE a compile-time!
```

### 6. Dati strutturati (`load_diz`)

```rust
use sapri_diz::load_diz;

let diz = load_diz();
println!("filter_words: {:?}", diz.it.filter_words.list);
println!("Charmap bits: {}", diz.it.charmap.bits);
```

---

## File di configurazione

### `diz.sson` - Definizione

```sson
[diz]
version_s: "1.0"

[diz.paths.lang.it.filter_words.list]
    type_s: "array<string>"
    source_s: "lang/it/filter_words.sson"

[diz.vocabulary.structs]
    allowed_l: "Brain", "HolographicMemory", "KnowledgeBase"
```

### `diz_data.json` - Dati reali

```json
{
  "lang": {
    "it": {
      "filter_words": { "list": ["a", "e", "i", "o", "il", "lo", "la"] },
      "charmap": { "bits": 6, "escape_code": 63 }
    }
  }
}
```

---

## Uso tipico

```rust
use sapri_diz::{diz, lang, paths, vocabulary, load_diz, validate_name};

fn main() {
    // 1. Accesso con macro
    let filter_words: Vec<String> = diz!(lang::it::filter_words::LIST -> Vec<String>).unwrap();
    
    // 2. Accesso con struct tipizzate
    let diz_data = load_diz();
    let bits = diz_data.it.charmap.bits;
    
    // 3. Validazione compile-time
    validate_name!("HolographicMemory", structs);
    
    // 4. Verifica vocabolario
    if vocabulary::fields::ALLOWED.contains(&"entries") {
        println!("'entries' è un campo valido");
    }
}
```

---

## Vantaggi

| Aspetto | Senza `diz` | Con `diz` |
|---------|-------------|-----------|
| **Path** | Stringhe hard-coded `"lang.it.filter_words.list"` | Costanti `lang::it::filter_words::LIST` |
| **IDE** | Nessun autocompletamento | Autocompletamento completo |
| **Errori** | Runtime (path sbagliato) | Compile-time (costante non trovata) |
| **Vocabolario** | Sparso nel codice | Centralizzato in `.sson` |
| **Refactoring** | Cerca/sostituisci manuale | Rinomina nel `.sson` → ricompila |
| **Lingue multiple** | Codice separato | Aggiungi dati in `.sson` |

---

## Aggiungere una nuova lingua

1. Aggiungi dati in `diz_data.json`:
```json
"en": {
  "filter_words": { "list": ["a", "an", "the"] },
  "charmap": { "bits": 6 }
}
```

2. Aggiungi path in `diz.sson`:
```sson
[diz.paths.lang.en.filter_words.list]
    type_s: "array<string>"
    source_s: "lang/en/filter_words.sson"
```

3. Ricompila → nuove costanti disponibili:
```rust
let en_filter_words: Vec<String> = diz!(lang::en::filter_words::LIST -> Vec<String>).unwrap();
```

---

## Dipendenze

- `serde` / `serde_json` - Parsing JSON
- `once_cell` - Lazy initialization

---

## Build

```bash
cargo build
cargo test
```

---

## Licenza

MIT / Apache-2.0
