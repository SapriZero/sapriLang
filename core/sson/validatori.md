
## Riepilogo del file

| Sezione | Contenuto | Righe |
|---------|-----------|-------|
| **Type aliases** | `ValidatorFn`, `TransformFn` | 15-20 |
| **FunctionRegistryImpl** | Struttura principale con cache e stats | 25-90 |
| **register_builtins** | Registrazione req, min, max, pattern, enum, range, mutex, guard, sum, compare, type, length | 45-200 |
| **Cache e stats** | `params_hash`, `update_stats`, `get_stats`, `clear_cache` | 220-260 |
| **Trait impl** | `register`, `call`, `contains` | 270-310 |
| **Trasformazioni** | `register_transform`, `call_transform` | 320-350 |
| **Built-in transforms** | default, coerce, trim, lowercase, uppercase | 360-400 |
| **Lazy singleton** | `GLOBAL_REGISTRY` | 410-420 |
| **Tests** | req, min, pattern, mutex, cache, contains, custom, sum, type | 430-550 |

---

## Come si integra

```
.sson specifica
    │
    ↓ (genera)
validator_impl.rs
    │
    ↓ (usa)
registry_impl.rs
    │
    ├── register("req", fn)
    ├── register("min", fn)
    ├── register("pattern", fn)
    └── ...
    │
    ↓ (chiamato da)
BaseValidator.validate()
    │
    ↓ (usa)
registry.call("req", ctx, constraint) → bool
```

---

## Validatori built-in disponibili

| Nome | Descrizione | Parametri |
|------|-------------|-----------|
| `req` | Campo obbligatorio | - |
| `min` | Valore minimo | `value` o `min` |
| `max` | Valore massimo | `value` o `max` |
| `pattern` | Regex | `regex` o `pattern` |
| `enum` | Valori consentiti | `values` o `enum` |
| `range` | Intervallo | `min`, `max` |
| `mutex` | Esattamente uno attivo | `fields` |
| `at_least_one` | Almeno uno attivo | `fields` |
| `exactly` | Conteggio esatto | `fields`, `count` |
| `guard` | Condizione booleana | `field`, `value` |
| `sum` | Somma di campi | `fields`, `target` |
| `compare` | Confronto tra campi | `field1`, `field2`, `op` |
| `type` | Verifica tipo | `field`, `expected` |
| `length` | Lunghezza stringa/array | `field`, `min`, `max`, `exactly` |

---

## Prossimo passo

Ora abbiamo:

1. ✅ `lib.rs` (definizioni)
2. ✅ `validator_impl.rs` (BaseValidator)
3. ✅ `resolver_impl.rs` (PathResolver)
4. ✅ `registry_impl.rs` (FunctionRegistry)

**Vuoi che procediamo con:**

5. **`obj_adapter_impl.rs`** (adapter per core/obj - get_path, set_path, flat ↔ nested)
6. **`state_machine_impl.rs`** (macchina a stati per parser.flow)
7. **`error_impl.rs`** (errori e recovery)
