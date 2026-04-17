## `sapri_diz` 

Immagina di dover costruire una grande casa. Hai bisogno di:

1. **Un archivio** dove tenere tutti i materiali (legno, mattoni, chiodi)
2. **Un catalogo** che ti dice come si chiamano le stanze e gli oggetti
3. **Un regolatore** che controlla che tutto sia al posto giusto

`diz` è tutto questo messo insieme.

---

## Scenario 1: L'utente che scrive una frase

### Senza `diz`

L'utente scrive: *"Cosa sono i quark?"*

Il computer deve:
- Leggere ogni lettera: `C o s a s o n o i q u a r k`
- Confrontare con un enorme dizionario
- Capire se "quark" è una parola conosciuta

**Problema:** Ogni lettera occupa spazio. Una frase lunga occupa molta memoria.

### Con `diz` (charmap)

`diz` ha una **tabella magica** (charmap) che trasforma ogni lettera in un **numero piccolo**:

| Lettera | Numero |
|---------|--------|
| a | 0 |
| b | 1 |
| ... | ... |
| z | 25 |
| spazio | 52 |
| à | 53 |

Così la parola "casa" diventa: `[2, 0, 18, 0]` → **4 numeri invece di 4 lettere**

**Vantaggio:** Occupa meno memoria, è più veloce da confrontare.

---

## Scenario 2: Lo sviluppatore che scrive codice

### Senza `diz`

Lo sviluppatore scrive:

```rust
let filter_words = get_data("lang.it.filter_words.list");
let bits = get_data("lang.it.charmap.bits");
```

Se sbaglia a scrivere `"lang.it.stopword.list"` (manca una 's'), il programma va in errore **solo quando viene eseguito**.

### Con `diz` (paths costanti)

Lo sviluppatore scrive:

```rust
use sapri_diz::paths;

let filter_words = diz!(paths::LANG_IT_filter_words_LIST);
let bits = diz!(paths::LANG_IT_CHARMAP_BITS);
```

L'IDE **autocompleta** mentre scrive. Se sbaglia, l'errore appare **subito**, durante la scrittura.

**Vantaggio:** Errori trovati prima, codice più robusto.

---

## Scenario 3: Il team che lavora insieme

### Senza `diz`

Tre sviluppatori lavorano sullo stesso progetto. Uno chiama il campo "memory", un altro "memoria", un terzo "mem". Il codice diventa un caos.

### Con `diz` (vocabolario)

`diz` ha un **vocabolario ufficiale** di tutti i nomi ammessi:

```sson
[diz.vocabulary.fields]
    allowed_l: "memory", "knowledge", "runtime", "learner", "conversation"
```

Se uno sviluppatore prova a scrivere `"memoria"`, il compilatore **dà errore**:

```
Error: "memoria" not in vocabulary. Did you mean "memory"?
```

**Vantaggio:** Tutti usano gli stessi nomi. Zero confusione.

---

## Scenario 4: Cambiare lingua

### Senza `diz`

Per aggiungere l'inglese, devi:
- Creare nuovi file
- Modificare il codice per leggere i nuovi file
- Riscrivere parti del programma

### Con `diz`

Basta aggiungere i dati in `diz_data.json`:

```json
{
  "lang": {
    "it": { ... },
    "en": {
      "filter_words": { "list": ["a", "an", "the"] },
      "charmap": { "bits": 6 }
    }
  }
}
```

E nel codice:

```rust
let filter_words_it = diz!(lang::it::filter_words::LIST);
let filter_words_en = diz!(lang::en::filter_words::LIST);
```

L'IDE autocompleta `lang::en::` automaticamente.

**Vantaggio:** Aggiungere una lingua richiede solo dati, non modifiche al codice.

---

## Scenario 5: Refactoring (rinominare qualcosa)

### Senza `diz`

Decidi di rinominare `"filter_words"` in `"filtered_words"`. Devi:
- Cercare in tutto il progetto
- Sostituire a mano decine di occorrenze
- Sperare di non dimenticarne nessuna

### Con `diz`

Cambi il nome in `diz.sson`:

```sson
[diz.paths.lang.it.filtered_words.list]
    type_s: "array<string>"
```

Ricompili. Il compilatore ti dice **esattamente** dove ci sono errori.

**Vantaggio:** Refactoring sicuro e veloce.

---

## Riepilogo visivo

```
┌─────────────────────────────────────────────────────────────────────┐
│                         cosa fa diz                                 │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 1. COMPRIME IL TESTO (charmap)                              │   │
│  │    "casa" → [2,0,18,0] → meno memoria, più veloce          │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 2. GUIDA LO SVILUPPATORE (paths)                            │   │
│  │    paths::LANG_IT_filter_words_LIST → autocompletamento IDE    │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 3. CONTROLLA I NOMI (vocabulary)                            │   │
│  │    "memory" ✅ / "memoria" ❌ → errori a compile-time       │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 4. CENTRALIZZA I DATI (diz_data.json)                       │   │
│  │    filter_words, suffissi, charmap → un unico posto            │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 5. AUTOMATIZZA IL REFACTORING                               │   │
│  │    Cambi nome nel .sson → ricompila → errori guidati        │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

---

## In una frase

> **`diz` è il cervello che organizza nomi, dati e regole del sistema, così tu puoi concentrarti su cosa fare, non su come chiamare le cose.**
