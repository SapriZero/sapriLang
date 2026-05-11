# URCM Semantic Compression Specification v1.2

## Struttura dei codici

| Codice | Tipo | Bit | Capacità |
|--------|------|-----|----------|
| `10 0` | Corto 1-2 char | 11 | 256 |
| `10 1` | Corto 3 char | 19 | 65536 |
| `11` | Frequente 1 | 18-21 | 65536 |
| `010 00` | Frequente 2 | 22-24 | 65536 |
| ... | ... | ... | ... |

## Flag e desinenze

- **2 bit di desinenza**: o/a/i/e (genere/numero)
- **4 bit di persona**: io/tu/lui/noi/voi/loro
- **1 bit di coniugabilità**: se la parola è una radice

## Hot-reload

I file `.sson` in `core/sson/spec/` sono monitorati.  
Al loro cambiamento, `sapri_core` ricarica le tabelle senza riavviare.

## Uso

```rust
use sapri_core::{UrcmEncoder, UrcmDecoder};

let encoder = UrcmEncoder::new();
let encoded = encoder.encode("gatto");  // → 010 10 01 + 12345 + 00
let decoded = decoder.decode(encoded);  // → "gatto"
```

## 3. Cosa manca ancora (checklist finale)

| Cosa | Stato | Note |
|------|-------|------|
| Tabella codici v1.2 | ✅ Definito | Nel `.sson` sopra |
| Desinenze (2 bit) | ✅ Definito | o/a/i/e |
| Persone (4 bit) | ✅ Definito | io/tu/lui/noi/voi/loro |
| Flag coniugabilità (1 bit) | ✅ Definito | radice vs parola piena |
| Categorie semantiche | ✅ Definito | SP/TM/DR/QT |
| Numeri (variabile) | ✅ Definito | Con bit di stop `1111` |
| Parole sconosciute | ✅ Definito | Codifica a caratteri con stop |
| Spazio riservato | ✅ Definito | `010 01` e `100 11` |
| Hot-reload | ✅ In `sapri_core` | Con `notify` + `crossbeam` |
| `morph` integrato? | ❓ Da verificare | Forse è in `sapri_diz` o separato |
