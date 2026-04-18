# sapri_morph - Analisi Morfologica Funzionale

Analisi morfologica di parole italiane: verbi, nomi, aggettivi.  
**Totalmente funzionale** (nessun if/else annidato), integrato con `sapri_diz` per la configurazione.

---

## Filosofia

Invece di scrivere:

```rust
if parola.ends_with("are") {
    // 50 righe di codice
} else if parola.ends_with("ere") {
    // altre 50 righe
} else {
    // gestione irregolari
}
```

Usiamo `eval_lazy()` per mantenere il codice **piatto e dichiarativo**:

```rust
let conjugation = eval_lazy(
    ends_with_are, || ConjugationType::First,
    || eval_lazy(ends_with_ere, || ConjugationType::Second,
        || eval_lazy(ends_with_ire, || ConjugationType::Third, || ConjugationType::Irregular
    ))
);
```

**Vantaggi:**
- Nessun nesting profondo
- Ogni branch è una closure (valutata solo se necessario)
- Leggibile e manutenibile

---

## Cosa fa

### 1. Verbi

Coniuga verbi regolari italiani (1a, 2a, 3a coniugazione):

| Infinito | Presente | Participio |
|----------|----------|------------|
| parlare | parlo, parli, parla, parliamo, parlate, parlano | parlato |
| correre | corro, corri, corre, corriamo, correte, corrono | corso* |
| sentire | sento, senti, sente, sentiamo, sentite, sentono | sentito |

*I verbi irregolari (come "correre" → "corso") non sono gestiti automaticamente*

### 2. Nomi

Riconosce genere e numero in base alla desinenza:

| Parola | Genere | Numero |
|--------|--------|--------|
| gatto | maschile | singolare |
| gatti | ? | plurale |
| casa | femminile | singolare |
| case | ? | plurale |

### 3. Aggettivi

Analogamente ai nomi, riconosce desinenze:

| Parola | Genere | Numero |
|--------|--------|--------|
| bello | maschile | singolare |
| bella | femminile | singolare |
| belli | ? | plurale |

---

## Integrazione con `sapri_diz`

Tutti i dati di configurazione vengono da `sapri_diz`:

```rust
use sapri_diz::load_diz;

let diz = load_diz();
let filter_words = &diz.text.filter_words.list;  // parole da ignorare
let min_len = diz.text.min_word_length;           // 3
```

**Niente file JSON locali.** I dati sono centralizzati in `sapri_diz`.

---

## API

### Verbi

```rust
use sapri_morph::verb::{VerbInfo, ConjugationType, extract_verbs};

let verb = VerbInfo::from_infinitive("parlare").unwrap();
println!("Coniugazione: {:?}", verb.conjugation);  // First
println!("Forme: {:?}", verb.forms);

let verbi = extract_verbs(&["parlare", "correre", "sentire"]);
```

### Nomi

```rust
use sapri_morph::noun::{NounInfo, extract_nouns};

let nome = NounInfo::from_word("gatto").unwrap();
println!("Genere: {:?}", nome.gender);   // Some("masculine")
println!("Numero: {:?}", nome.number);   // Some("singular")

let nomi = extract_nouns(&["casa", "gatto", "cani"]);
```

### Aggettivi

```rust
use sapri_morph::adj::{AdjectiveInfo, extract_adjectives};

let adj = AdjectiveInfo::from_word("bello").unwrap();
println!("Genere: {:?}", adj.gender);    // Some("masculine")

let aggettivi = extract_adjectives(&["bello", "bella", "belli"]);
```

### Utility funzionale

```rust
use sapri_morph::eval_lazy;

let result = eval_lazy(
    x > 0,
    || x * 2,
    || 0
);
```

---

## Limitazioni (e perché)

| Limitazione | Motivo |
|-------------|--------|
| **Verbi irregolari non gestiti** | Richiedono un dizionario delle eccezioni (in arrivo) |
| **Genere non rilevato per plurali** | "gatti" non finisce con 'o' o 'a' → genere sconosciuto |
| **Falsi positivi** | "correre" finisce con 'e' → viene considerato nome |

**Perché?** L'obiettivo non è la perfezione linguistica, ma un **sistema funzionale e componibile** che può essere esteso con dati (non codice). 
Le eccezioni verranno caricate da `.sson` tramite `sapri_diz`.

---

## Test

```bash
cargo test
```

Output atteso:

```text
running 14 tests
test adj::tests::test_extract_adjectives ... ok
test adj::tests::test_feminine_adj ... ok
test adj::tests::test_masculine_adj ... ok
test adj::tests::test_plural_adj ... ok
test noun::tests::test_feminine_noun ... ok
test noun::tests::test_masculine_noun ... ok
test noun::tests::test_extract_nouns ... ok
test noun::tests::test_plural_noun ... ok
test tests::test_eval_lazy ... ok
test tests::test_eval_lazy_with_closure ... ok
test verb::tests::test_detect_conjugation ... ok
test verb::tests::test_extract_verbs ... ok
test verb::tests::test_irregular_verb ... ok
test verb::tests::test_second_conjugation_forms ... ok

test result: ok. 14 passed; 0 failed
```

---

## Prossimi passi

1. **Caricare eccezioni da `.sson`** (verbi irregolari, nomi con genere irregolare)
2. **Integrare con `sapri_diz`** per i dati linguistici
3. **Parsing dei numeri in lettere** (in un modulo separato)
4. **Analisi delle frasi** (soggetto, verbo, oggetto)

---

## Dipendenze

- `sapri_core` - Runtime e utilità
- `sapri_diz` - Dizionario globale e configurazione

---

## Licenza

MIT / Apache-2.0
