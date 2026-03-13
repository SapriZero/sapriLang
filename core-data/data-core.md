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