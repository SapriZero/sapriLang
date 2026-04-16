Ecco il `Cargo.toml` sistemato con le feature flags:


| Feature | Dipendenze attivate | Quando usarla |
|---------|---------------------|---------------|
| **`default`** | solo `json-unflattening` | Uso base |
| **`fast-json`** | `simd-json` | Parsing JSON velocissimo |
| **`validation`** | `jsonschema` + `genson-rs` | Validazione e inferenza |
| **`streaming`** | `pjson-rs` | Streaming JSON |
| **`arrow-backend`** | `arrow` + `datafusion` | Analisi dati pesanti |
| **`full`** | Tutte tranne arrow | Uso completo ma leggero |
| **`analytics`** | arrow + datafusion | Data science |

### 🎯 Esempi d'uso

```toml
# Chi usa solo flattening (default)
core-data = { path = "../core-data" }

# Chi vuole anche validazione
core-data = { path = "../core-data", features = ["validation"] }

# Chi fa analisi dati
core-data = { path = "../core-data", features = ["analytics"] }

# Chi vuole tutto
core-data = { path = "../core-data", features = ["full", "analytics"] }
```


## Sì, esattamente.

---

## Riepilogo

| Funzionalità | Dove va | Perché |
|--------------|---------|--------|
| **Inferenza schema da dati** | `.sson` (da implementare) | Generare `.sson` automaticamente da JSON |
| **Generazione JSON Schema** | `.sson` (da implementare) | Interoperabilità con altri sistemi |
| **Versionamento schema** | `.sson` (da implementare) | Tracciare evoluzione schemi |
| **Flatten/Expand** | `core-data/flatten` (usare così com'è) | Manipolazione flat ↔ nested |

---

## Architettura finale

```
┌─────────────────────────────────────────────────────────────────────┐
│                         .sson FORMAT                                │
│                                                                     │
│  - Definizione campi e tipi                                         │
│  - Validatori (req, min, max, pattern, enum, mutex, guard, sum)    │
│  - Stati macchina (_:state)                                         │
│  - Riferimenti (_:ref)                                              │
│  - Inferenza schema (da dati → .sson)          ← DA AGGIUNGERE      │
│  - Esportazione JSON Schema                   ← DA AGGIUNGERE      │
│  - Versionamento                              ← DA AGGIUNGERE      │
└─────────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────────┐
│                      sapri_sson (Rust)                              │
│                                                                     │
│  - Parser .sson                                                     │
│  - Validatore (12 validatori)                                       │
│  - Calcolo S = (v·i)/(t·k)                                          │
│  - Memoria olografica                                               │
│  - Macchina a stati                                                 │
│  - Function registry                                                │
└─────────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────────┐
│                      core-data/flatten                              │
│                                                                     │
│  - Appiattimento JSON → path piatti                                 │
│  - Espansione path piatti → JSON annidato                           │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Cosa serve da `core-data`

**Solo `flatten`**. Il resto (`validation`, `schema`, `stream`, `table`, `registry`) è **ridondante** o **non necessario**.

---

## Come integrare `flatten` in `sapri_sson`

Attualmente `sapri_sson` ha un `obj_adapter_impl` che fa flat ↔ nested con `sapri_obj`. Possiamo:

1. **Sostituirlo** con `core-data/flatten` (più completo)
2. **Oppure tenerlo** (già funzionante) e ignorare `flatten`

---

## La domanda

**Vuoi che:**

1. **Aggiungiamo a `.sson`** le funzionalità mancanti (inferenza, JSON Schema export, versionamento)?
2. **Integriamo `core-data/flatten`** in `sapri_sson`?
3. **Ignoriamo `core-data`** completamente (tranne forse `flatten` se serve)?
4. **Facciamo un piano** per l'implementazione delle nuove funzionalità di `.sson`?
