Ecco una bozza di README per `sapri_db`, pensata per essere letta da un AI (o da un umano) per capire **come funziona** il database, senza entrare nei dettagli di implementazione.

---

# sapri_db — Database a indirizzamento diretto O(1)

## Filosofia

Un database tradizionale cerca i dati. **Questo database sa già dove sono.**

Ogni dato è identificato da:
- **Header** (2-8 bit) → dice in quale tabella/categoria si trova
- **Indice** (8, 16 o 32 bit) → la posizione esatta nell'array

**Non devi cercare. Sai già dove andare.**

---

## Architettura a moduli

```
sapri_db/
├── error.rs         → Tipi di errore unificati
├── header.rs        → Header a bit variabili (es. "10", "110", "1110")
├── schema.rs        → Definizioni: FieldType, Constraint, FieldDef, TableDef
├── validator.rs     → Validatore di record contro uno schema
├── table.rs         → Tabella con array di record e indici O(1)
├── database.rs      → Collezione di tabelle (un mini-database)
├── loader.rs        → Caricamento da CSV (da implementare)
├── binary.rs        → Serializzazione binaria (da implementare)
├── join.rs          → Join tra tabelle (da implementare)
└── flat.rs          → Appiattimento tabelle (da implementare)
```

---

## Moduli principali

### 1. `error.rs` — Errori unificati

Un unico tipo `DbError` con varianti per ogni operazione:
- `Schema` → errore nella definizione dello schema
- `Loader` → errore nel caricamento dati
- `Validator` → errore di validazione
- `Binary` → errore in lettura/scrittura binaria
- `Table` → errore su tabella (indice fuori range, ecc.)
- `NotFound` → elemento non trovato
- `InvalidHeader` → header malformato

**Uso:** Tutte le funzioni restituiscono `Result<T, DbError>`.

---

### 2. `header.rs` — Header a bit variabili

Gli header sono stringhe di bit (es. `"10"`, `"110"`, `"1110"`) che identificano la tabella.

**Proprietà:**
- Lunghezza variabile (2-8 bit)
- Accesso O(1) per indice
- Pattern per debug

**Esempio:**
```rust
let h = Header::new("10");
assert_eq!(h.bits(), "10");
assert_eq!(h.len(), 2);
```

---

### 3. `schema.rs` — Definizioni dello schema

Definisce **come** sono fatte le tabelle e i campi.

| Tipo | Ruolo | Esempio |
|------|-------|---------|
| `FieldType` | Tipo di dato del campo | `String`, `Int`, `Float`, `Bool`, `Date`, `Ref`, `Enum` |
| `Constraint` | Vincolo sul valore | `Min`, `Max`, `MaxLength`, `Pattern` |
| `FieldDef` | Definizione di un campo | nome, tipo, vincoli, opzionale |
| `TableDef` | Definizione di una tabella | nome, header, indice, campi, indici |
| `IndexDef` | Indice secondario | nome, campi, tipo (hash/sorted) |

**Esempio di TableDef:**
```rust
let table_def = TableDef {
    name: "clienti".to_string(),
    header: "10".to_string(),
    index_bits: 16,
    fields: vec![
        FieldDef { name: "nome".to_string(), field_type: FieldType::String, ... },
        FieldDef { name: "eta".to_string(), field_type: FieldType::Int, ... },
    ],
    indexes: HashMap::new(),
};
```

---

### 4. `validator.rs` — Validatore di record

Prende uno `TableDef` e valida che un record (come `HashMap<String, Value>` o `Obj`) sia conforme.

**Cosa verifica:**
- Tutti i campi obbligatori sono presenti
- I tipi dei valori corrispondono allo schema
- I vincoli sono rispettati (min, max, maxLength, ecc.)

**Esempio:**
```rust
let validator = Validator::new(table_def);
validator.validate_obj(&obj)?;  // Ok() o Errore
```

---

### 5. `table.rs` — Tabella con array di record

La struttura più importante. Contiene:
- `header` → identificatore (es. `"10"`)
- `index_bits` → numero di bit per l'indice (8, 16 o 32)
- `records` → array lineare di record (Vec<Record>)
- `indexes` → indici secondari (mappa chiave → indice)

**Record:**
- `id` → posizione nell'array
- `values` → mappa campo → valore

**Operazioni:**
- `insert(record)` → aggiunge in fondo, restituisce indice O(1)
- `get(index)` → accesso diretto O(1)
- `update(index, record)` → sostituisce O(1)
- `delete(index)` → rimuove O(1) (shift degli elementi successivi)
- `add_index(name, field)` → crea indice secondario
- `search_index(name, value)` → cerca per indice secondario O(1)

**Accesso ai record:**
- `records()` → slice di tutti i record
- `records_mut()` → slice mutabile

---

### 6. `database.rs` — Collezione di tabelle

Un `Database` è una mappa che associa header → tabella e nome → schema.

**Operazioni del trait `DatabaseOps`:**
- `new()` → database vuoto
- `with_config(config)` → database con configurazione
- `add_table(header, table)` → aggiunge una tabella
- `add_schema(name, schema)` → aggiunge uno schema
- `get_table(header)` → ottiene tabella O(1)
- `get_table_mut(header)` → ottiene tabella mutabile
- `get_schema(name)` → ottiene schema
- `save_binary(path)` → salva in formato binario
- `load_binary(path)` → carica da binario

**Flusso tipico:**
```rust
let mut db = Database::new();
db.add_schema("clienti", clienti_schema);
db.add_table("10", clienti_table);
let table = db.get_table("10").unwrap();
let record = table.get(0).unwrap();
```

---

## Flusso di lavoro tipico

```
1. Definire lo schema (TableDef) → può arrivare da .sson via sapri_core
2. Validare lo schema (SchemaValidator)
3. Caricare i dati (CSV, JSON, o da .sson)
4. Creare una tabella (Table) con header e index_bits
5. Inserire record (Record)
6. Opzionale: creare indici secondari (add_index)
7. Opzionale: mettere la tabella in un Database
8. Validare i record contro lo schema (Validator)
```

---

## Esempio completo

```rust
use sapri_db::{
    Database, DatabaseOps,
    Table, Record,
    Validator, Validate,
    schema::{FieldDef, FieldType, TableDef},
};

// 1. Definisci lo schema
let schema = TableDef {
    name: "clienti".to_string(),
    header: "10".to_string(),
    index_bits: 16,
    fields: vec![
        FieldDef { name: "nome".to_string(), field_type: FieldType::String, .. },
        FieldDef { name: "eta".to_string(), field_type: FieldType::Int, .. },
    ],
    indexes: HashMap::new(),
};

// 2. Crea la tabella
let mut table = Table::new("10", 16);

// 3. Inserisci record
let mut record = Record::new(0);
record.set("nome", Value::from("Mario"));
record.set("eta", Value::from(30));
let idx = table.insert(record);

// 4. Valida il record
let validator = Validator::new(schema.clone());
validator.validate(table.get(idx).unwrap().values.as_ref())?;

// 5. Crea un database e aggiungi la tabella
let mut db = Database::new();
db.add_schema("clienti", schema);
db.add_table("10", table);

// 6. Recupera e usa
let t = db.get_table("10").unwrap();
let r = t.get(0).unwrap();
assert_eq!(r.get("nome").unwrap().as_str(), Some("Mario"));
```

---

## Moduli da implementare (TODO)

| Modulo | Funzionalità |
|--------|--------------|
| `loader` | Caricare dati da CSV (con separatore `|`) |
| `binary` | Serializzare/deserializzare in formato binario (numeri in binario, non testo) |
| `join` | Join tra tabelle (hash join, nested loop, merge join) |
| `flat` | Appiattimento tabelle (normalizzazione) |

---

## Perché è veloce

| Operazione | Complessità | Come |
|------------|-------------|------|
| Accesso per indice | O(1) | Array diretto |
| Ricerca per chiave primaria | O(1) | Header + indice |
| Ricerca per indice secondario | O(1) | HashMap collega valore → indice |
| Inserimento | O(1) | Append in fondo |
| Validazione record | O(numero_campi) | Scan lineare, ma pochi campi |

---

## Relazione con `sapri_core`

`sapri_db` è **puro**:
- **Non carica file** → riceve solo `Obj` già pronti
- **Non sa nulla di `.sson`** → la conversione la fa `sapri_core`
- **Non ha I/O** → salva/carica binario ma non sa da dove

**Chi fa cosa:**
| Responsabilità | Chi |
|----------------|-----|
| Caricare `.sson` → `Obj` | `sapri_core` |
| Convertire `Obj` → `TableDef` | `sapri_db::schema::TableDef::from_obj()` |
| Validare `TableDef` | `sapri_db::schema_impl::SchemaValidator` |
| Caricare dati CSV → `Table` | `sapri_core` (o future `loader`) |
| Validare record | `sapri_db::validator::Validator` |
| Operazioni CRUD | `sapri_db::table::Table` |