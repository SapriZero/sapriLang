## `core/morph/README.md`

```markdown
# sapri_morph - Analisi Morfologica Funzionale

Analisi morfologica di parole italiane: verbi, nomi, aggettivi, numeri.  
**Totalmente funzionale** (uso di `eval_lazy` invece di if/else annidati), integrato con `sapri_diz` per la configurazione.

---

## Differenza tra `sapri_diz` e `sapri_morph`

| Crate | Ruolo | Contiene | Esempio |
|-------|-------|----------|---------|
| **`sapri_diz`** | Dizionario globale | Path costanti, vocabolario, charmap, filter_words, configurazione lingua | `diz!(text.filter_words.list)` → stopwords |
| **`sapri_morph`** | Analisi morfologica | Parsing di `.aff` e `.dic`, coniugazione verbi, genere/nome numeri, parsing numeri in lettere | `parse_number_word("ventuno")` → 21 |

**Regola pratica:**
- Se è configurazione statica (paths, vocabolario, mappe) → va in `sapri_diz`
- Se è elaborazione dinamica (coniugare un verbo, parsare un numero) → va in `sapri_morph`

---

## Cosa fa

### 1. Parsing di file Hunspell (`.aff` e `.dic`)

Legge i dizionari morfologici italiani:

| File | Contenuto | Esempio |
|------|-----------|---------|
| `.aff` | Regole di suffissi/prefissi | `SFX A Y 81` (prima coniugazione) |
| `.dic` | Parole con flag morfologici | `casa/Nfs` (nome femminile singolare) |

### 2. Verbi

Coniuga verbi regolari italiani (1a, 2a, 3a coniugazione):

| Infinito | Presente | Participio |
|----------|----------|------------|
| parlare | parlo, parli, parla, parliamo, parlate, parlano | parlato |
| correre | corro, corri, corre, corriamo, correte, corrono | corso* |
| sentire | sento, senti, sente, sentiamo, sentite, sentono | sentito |

*I verbi irregolari sono gestiti con eccezioni (non regole)

### 3. Nomi

Riconosce genere e numero in base alla desinenza:

| Parola | Genere | Numero |
|--------|--------|--------|
| gatto | maschile | singolare |
| gatti | maschile | plurale |
| casa | femminile | singolare |
| case | femminile | plurale |

### 4. Aggettivi

Analogamente ai nomi, riconosce desinenze e concordanze.

### 5. Numeri in lettere

Converte parole numeriche in valori numerici:

| Input | Output |
|-------|--------|
| "uno" | 1 |
| "ventuno" | 21 |
| "cento" | 100 |
| "mille" | 1000 |
| "un milione trecentocinque" | 1.000.305 |

---

## Integrazione con `sapri_diz`

```rust
use sapri_diz::load_diz;
use sapri_morph::parse_number_word;

let diz = load_diz();
let min_len = diz.text.min_word_length;  // configurazione da diz

let numero = parse_number_word("ventuno");  // elaborazione in morph
```

**Niente file JSON locali.** I dati di configurazione sono centralizzati in `sapri_diz`.

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

### Numeri

```rust
use sapri_morph::number::{parse_number_word, number_to_words, parse_numbers_in_text};

assert_eq!(parse_number_word("ventuno"), Some(21));
assert_eq!(number_to_words(21), "ventuno");

let text = "ho ventuno anni e mio fratello ne ha trentatré";
let numeri = parse_numbers_in_text(text);
assert_eq!(numeri[0].1, 21);
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

## Parsing di `.aff` e `.dic`

### Struttura del file `.aff`

```text
SFX A Y 81           # Flag A, 81 regole per prima coniugazione
SFX A are o are      # regola: infinito → presente 1a pers sing
SFX A are i are      # regola: infinito → presente 2a pers sing
...
```

### Struttura del file `.dic`

```text
12345                # numero di parole
casa/Nfs             # nome femminile singolare
gatto/Nms            # nome maschile singolare
parlare/V            # verbo
bello/Ams            # aggettivo maschile singolare
```

### Flag morfologici

| Flag | Significato |
|------|-------------|
| `N` | Nome |
| `V` | Verbo |
| `A` | Aggettivo |
| `m` / `f` | Maschile / Femminile |
| `s` / `p` | Singolare / Plurale |

---

## Test

```bash
cargo test
```

Output atteso:

```text
running 14 tests
test adj::tests::test_extract_adjectives ... ok
test noun::tests::test_extract_nouns ... ok
test verb::tests::test_extract_verbs ... ok
test number::tests::test_parse_number_word ... ok
...
test result: ok. 14 passed
```

---

## Prossimi passi

1. **Caricare eccezioni da `.sson`** (verbi irregolari, nomi con genere irregolare)
2. **Integrare con `sapri_diz`** per i dati linguistici
3. **Analisi delle frasi** (soggetto, verbo, oggetto) usando i dati morfologici

---

## Dipendenze

- `sapri_core` - Runtime e utilità
- `sapri_diz` - Dizionario globale e configurazione

---

## Licenza

MIT / Apache-2.0
```
