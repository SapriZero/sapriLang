Ecco il `README.md` per `sapri_rust_dsl`:

```markdown
# sapri_rust_dsl

DSL embedded in Rust per scrivere espressioni relazionali in modo testuale.

## Filosofia

Questo crate implementa i principi di **URCM** (Unit Relational Contextual Movement):
- Ogni relazione è una **moltiplicazione** (proporzione)
- Le espressioni complesse si scompongono in passaggi semplici
- Identificatori a **un solo carattere** (atomi)
- Contesti con **ereditarietà** (spread)

## Installazione

```toml
[dependencies]
sapri_rust_dsl = { path = "core/rust_dsl" }
sapri_base = { path = "core/base" }
```

## Concetti base

### 1. Atomi (`Atom<AtomValue>`)
Un valore che può essere calcolato lazy e dipende da altri atomi.

### 2. Contesto (`Context`)
Contiene i binding nome → atomo, con supporto ereditarietà.

### 3. Scan (`scan!`)
Trasforma una stringa in un atomo. Solo moltiplicazione (implicita o esplicita).

### 4. Define (`define!`)
Crea un contesto con definizioni atomiche.

## Esempi

### Definizioni semplici

```rust
use sapri_rust_dsl::{define, scan};

let ctx = define! {
    a = 10;
    b = 20;
    c = 30;
};

assert_eq!(ctx.get_value("a").unwrap().as_number(), Some(10.0));
```

### Espressioni con scan

```rust
let ctx = define! {
    a = 10;
    b = 20;
};

// Moltiplicazione esplicita
let c = scan!("a * b", &ctx).unwrap();
assert_eq!(c.get().as_number(), Some(200.0));

// Moltiplicazione implicita (spazio)
let d = scan!("a b", &ctx).unwrap();
assert_eq!(d.get().as_number(), Some(200.0));

// Senza spazio (moltiplicazione implicita)
let e = scan!("ab", &ctx).unwrap();
assert_eq!(e.get().as_number(), Some(200.0));
```

### Numeri letterali

```rust
let ctx = define! {
    a = 10;
};

let result = scan!("a 5", &ctx).unwrap();
assert_eq!(result.get().as_number(), Some(50.0));
```

### Ereditarietà (spread)

```rust
let parent = define! {
    a = 10;
    b = 20;
};

let child = define! {
    using &parent,
    b = 30;  // override
    c = 40;  // nuovo
};

assert_eq!(child.get_value("a").unwrap().as_number(), Some(10.0));
assert_eq!(child.get_value("b").unwrap().as_number(), Some(30.0));
assert_eq!(child.get_value("c").unwrap().as_number(), Some(40.0));
```

### Contesto globale (feature flag)

```rust
#[cfg(feature = "global-context")]
{
    use sapri_rust_dsl::{set_global_context, scan};
    
    let ctx = define! { x = 10 };
    set_global_context(std::sync::Arc::new(ctx));
    
    let result = scan!("x 5").unwrap();
    assert_eq!(result.get().as_number(), Some(50.0));
}
```

## Tipi supportati

`AtomValue` può essere:

| Variante | Esempio |
|----------|---------|
| `Number(f64)` | `10`, `3.14` |
| `String(String)` | `"ciao"` |
| `Bool(bool)` | `true`, `false` |

```rust
let ctx = define! {
    count = 42;
    name = "Sapri";
    active = true;
};

assert_eq!(ctx.get_value("count").unwrap().as_number(), Some(42.0));
assert_eq!(ctx.get_value("name").unwrap().as_string(), Some("Sapri"));
assert_eq!(ctx.get_value("active").unwrap().as_bool(), Some(true));
```

## Lazy evaluation

Gli atomi sono valutati **lazy**: il calcolo avviene solo al primo `get()`.

```rust
let a = define! { a = 10 };
let b = define! { b = 20 };
let c = scan!("a b", &a.merge(&b)).unwrap();

// Il valore viene calcolato solo ora
assert_eq!(c.get().as_number(), Some(200.0));
// La seconda lettura è immediata (cached)
assert_eq!(c.get().as_number(), Some(200.0));
```

## Context API

| Metodo | Descrizione |
|--------|-------------|
| `new()` | Crea contesto vuoto |
| `with_parent(&parent)` | Crea contesto con ereditarietà |
| `set(name, value)` | Imposta un atomo |
| `set_value(name, atom_value)` | Imposta un valore semplice |
| `get(name)` | Ottiene un atomo |
| `get_value(name)` | Ottiene il valore (con risoluzione) |
| `contains(name)` | Verifica se esiste |
| `merge(&other)` | Unisce contesti (spread) |

## Scan syntax

| Espressione | Significato |
|-------------|-------------|
| `"a"` | Singolo atomo |
| `"ab"` | `a * b` (moltiplicazione implicita) |
| `"a b"` | `a * b` (spazio come moltiplicazione) |
| `"a * b"` | `a * b` (moltiplicazione esplicita) |
| `"a 5"` | `a * 5` (numero letterale) |
| `"a b c"` | `a * b * c` (moltiplicazione multipla) |

**Nota:** Sono supportate solo moltiplicazioni. Espressioni complesse (`+`, `-`, `/`) vanno calcolate in Rust prima di passarle a `define!`.

## Workflow tipico

```rust
use sapri_rust_dsl::{define, scan};

// 1. Calcoli complessi in Rust
let prezzo_iva = 100.0 * 1.22;

// 2. Definisci il contesto
let ctx = define! {
    prezzo = prezzo_iva;
    quantita = 3;
    sconto = 0.10;
};

// 3. Calcoli relazionali con scan
let subtotale = scan!("prezzo quantita", &ctx).unwrap();
let scontato = scan!("subtotale sconto", &ctx).unwrap();
```

## Integrazione con `sapri_obj`

```rust
use sapri_rust_dsl::{define, scan, AtomValue};
use sapri_obj::{obj, Obj};

let data = obj! {
    prezzo: 100,
    quantita: 3
};

let ctx = define! {
    p = data.get("prezzo").unwrap().as_number().unwrap();
    q = data.get("quantita").unwrap().as_number().unwrap();
};

let totale = scan!("p q", &ctx).unwrap();
assert_eq!(totale.get().as_number(), Some(300.0));
```

## Test

```bash
cargo test -p sapri_rust_dsl
```

## Dipendenze

- `sapri_base` (atomi, funzioni pure)
- `regex` (per scanner)
- `once_cell` (opzionale, per contesto globale)

## Feature flags

| Feature | Descrizione |
|---------|-------------|
| `global-context` | Abilita contesto globale implicito |

## Limitazioni

- Solo moltiplicazione (URCM puro)
- Identificatori a un solo carattere
- Nessuna espressione complessa nelle stringhe (usare Rust)

## Vedi anche

- [`sapri_base`](../base/README.md) - Fondamenti (Atom, fp, error)
- [`sapri_obj`](../obj/README.md) - Oggetti dinamici stile JavaScript
```
