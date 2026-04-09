Ecco il `README.md` per `sapri_obj`:

```markdown
# sapri_obj

Struttura dinamica a oggetti stile JavaScript per Rust.

## Caratteristiche

- ✅ Sintassi JavaScript-like con macro `obj!`
- ✅ Spread operator (`using base` e `..base`)
- ✅ Accesso per path (`get_path`, `get_dot`)
- ✅ Merge superficiale e profondo
- ✅ Conversione tipata con `get_string`, `get_u32`, ecc.
- ✅ Zero dipendenze (solo std)

## Installazione

Aggiungi al tuo `Cargo.toml`:

```toml
[dependencies]
sapri_obj = { path = "core/obj" }
```

## Uso base

```rust
use sapri_obj::{obj, Obj};

let persona = obj! {
    nome: "Mario",
    eta: 30,
    attivo: true
};

assert_eq!(persona.get("nome").unwrap().as_str(), Some("Mario"));
assert_eq!(persona.get("eta").unwrap().as_number(), Some(30.0));
assert_eq!(persona.get("attivo").unwrap().as_bool(), Some(true));
```

## Spread operator (ereditarietà)

### Sintassi `using` (stile JavaScript)

```rust
let base = obj! { a: 10, b: 20 };
let esteso = obj! {
    using base,
    b: 30,
    c: 40
};

// Risultato: { a: 10, b: 30, c: 40 }
```

### Sintassi `..` (stile Rust)

```rust
let base = obj! { a: 10, b: 20 };
let esteso = obj! { ..base; b: 30, c: 40 };
```

### Spread multipli

```rust
let obj1 = obj! { a: 1, b: 2 };
let obj2 = obj! { c: 3, d: 4 };
let merged = obj! { ..obj1, ..obj2; e: 5 };
// Risultato: { a: 1, b: 2, c: 3, d: 4, e: 5 }
```

## Oggetti annidati

```rust
let obj = obj! {
    a: 10,
    b: obj! {
        c: 20,
        d: obj! {
            e: 30
        }
    }
};

assert_eq!(obj.get_dot("b.c").unwrap().as_number(), Some(20.0));
assert_eq!(obj.get_dot("b.d.e").unwrap().as_number(), Some(30.0));
```

## Accesso per path

```rust
// Path array
let value = obj.get_path(&["b", "c"]);

// Dot notation
let value = obj.get_dot("b.c");

// Set per path
let obj = obj.set_path(&["a", "b", "c"], 42);
let obj = obj.set_dot("a.b.c", 42);
```

## Merge

```rust
let obj1 = obj! { a: 10, b: 20 };
let obj2 = obj! { b: 30, c: 40 };

// Merge superficiale (solo primo livello)
let merged = obj1.merge(obj2);
// { a: 10, b: 30, c: 40 }

// Merge profondo (ricorsivo su oggetti annidati)
let deep1 = obj! { a: obj! { x: 1 } };
let deep2 = obj! { a: obj! { y: 2 } };
let merged_deep = deep1.merge_deep(deep2);
// { a: { x: 1, y: 2 } }
```

## Conversione tipata

```rust
let obj = obj! {
    nome: "Mario",
    eta: 30,
    prezzo: 19.99,
    attivo: true,
    indirizzo: obj! {
        via: "Roma",
        numero: 10
    }
};

let nome: String = obj.get_string("nome")?;
let eta: u32 = obj.get_u32("eta")?;
let prezzo: f64 = obj.get_f64("prezzo")?;
let attivo: bool = obj.get_bool("attivo")?;
let indirizzo: Obj = obj.get_obj("indirizzo")?;

// Gestione errori
match obj.get_i32("campo_inesistente") {
    Ok(val) => println!("{}", val),
    Err(e) => eprintln!("Errore: {}", e),
}
```

## Da Obj a Struct

```rust
use sapri_obj::{obj, Obj};

struct Persona {
    nome: String,
    eta: u32,
    attivo: bool,
}

impl Persona {
    fn from_obj(obj: &Obj) -> Result<Self, String> {
        Ok(Persona {
            nome: obj.get_string("nome")?,
            eta: obj.get_u32("eta")?,
            attivo: obj.get_bool("attivo")?,
        })
    }
}

let obj = obj! { nome: "Mario", eta: 30, attivo: true };
let persona = Persona::from_obj(&obj).unwrap();
```

## API Riferimento

### Costruttori

| Metodo | Descrizione |
|--------|-------------|
| `Obj::new()` | Crea un oggetto vuoto |
| `obj! { ... }` | Macro per creare oggetti con sintassi JS |

### Metodi base

| Metodo | Descrizione |
|--------|-------------|
| `set(key, value)` | Imposta un campo |
| `get(key)` | Ottiene un campo come `Option<&Value>` |
| `contains(key)` | Verifica se esiste un campo |
| `remove(key)` | Rimuove un campo |
| `keys()` | Restituisce tutte le chiavi |
| `values()` | Restituisce tutti i valori |
| `len()` | Numero di campi |
| `is_empty()` | Verifica se vuoto |

### Path access

| Metodo | Descrizione |
|--------|-------------|
| `set_path(&[key, ...], value)` | Imposta per path |
| `get_path(&[key, ...])` | Ottiene per path |
| `set_dot("a.b.c", value)` | Imposta per dot notation |
| `get_dot("a.b.c")` | Ottiene per dot notation |
| `remove_path(&[key, ...])` | Rimuove per path |

### Merge

| Metodo | Descrizione |
|--------|-------------|
| `merge(other)` | Unione superficiale |
| `merge_deep(other)` | Unione profonda (ricorsiva) |

### Conversione tipata (Result)

| Metodo | Restituisce |
|--------|-------------|
| `get_string(key)` | `Result<String, String>` |
| `get_i32(key)` | `Result<i32, String>` |
| `get_u32(key)` | `Result<u32, String>` |
| `get_f64(key)` | `Result<f64, String>` |
| `get_bool(key)` | `Result<bool, String>` |
| `get_obj(key)` | `Result<Obj, String>` |

## Value enum

```rust
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Obj(Obj),
}
```

### Metodi di Value

| Metodo | Descrizione |
|--------|-------------|
| `as_number()` | `Option<f64>` |
| `as_bool()` | `Option<bool>` |
| `as_str()` | `Option<&str>` |
| `as_obj()` | `Option<&Obj>` |
| `into_obj()` | `Option<Obj>` (con consumo) |
| `is_null()` | `bool` |
| `is_obj()` | `bool` |

## Test

```bash
cargo test
```

## Licenza

MIT
```

Salva come `core/obj/README.md`.
