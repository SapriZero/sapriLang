Perfetto! Questa è la **visione completa**. Creiamo un documento che definisce come **progettare in .sson prima di scrivere codice**.

```
--$FILE===: AI_SSON_PROJECT_GUIDE.md
# GUIDA ALLA PROGETTAZIONE CON .sson
## "Prima definisci, poi implementa"

### 🎯 FILOSOFIA

Il codice nasce dalle definizioni, non viceversa.
Ogni componente del progetto viene prima **descritto in .sson**:
- Strutture dati
- Funzioni e API
- Dipendenze
- Idee e TODO
- Documentazione

---

## 📋 TIPI DI FILE .sson

| Estensione | Scopo | Esempio |
|------------|-------|---------|
| **`.ssd`** | Definizione progetto (Sapri Structure Definition) | `progetto.ssd` |
| **`.ssf`** | Definizione funzioni (Sapri Functions) | `api.ssf` |
| **`.ssi`** | Idee e TODO (Sapri Ideas) | `idee.ssi` |
| **`.ssdoc`** | Documentazione | `manuale.ssdoc` |

---

## 🏗️ STRUTTURA PROGETTO (.ssd)

```
# ============================================
# PROGETTO: core-data
# VERSIONE: 0.1.0
# ============================================

[_META] 
  author = "Mario Rossi"
  created = "2026-03-13"
  license = "MIT"

[_DEPS]  # dipendenze esterne
  jsonschema = "0.40"   # validazione JSON
  genson-rs = "0.2"     # inferenza schema
  rayon = "1.10"        # parallelismo

[_MODULES]  # moduli del crate
  flatten
  schema
  validation
  table
  stream
  registry

# ============================================
# MODULO: flatten
# ============================================

[flatten]
  desc = "Flattening JSON → formato tabellare"
  status = "planned"  # planned, wip, done, deprecated
  
  [flatten.deps]  # dipendenze interne
    - core::definition::registry
    - core::types::Value
  
  [flatten.api]  # funzioni pubbliche
    [.flatten_json]
      params = "json: &Value"
      returns = "Result<FlattenedData>"
      errors = ["InvalidJson", "MaxDepthExceeded"]
      example = """
        let data = flatten_json(complex_json)?;
        // data.fields = {"user.name": "Mario", "user.age": 30}
      """
    
    [.flatten_with_schema]
      params = "json: &Value, schema: &Schema"
      returns = "Result<ValidatedData>"
    
  [flatten.todo]  # TODO interni
    [.gestione_array]
      priority = "high"
      note = "Separare array in tabelle figlie"
    
    [.max_depth]
      priority = "medium"
      note = "Configurare profondità massima"

# ============================================
# MODULO: validation
# ============================================

[validation]
  desc = "Validazione dati con JSON Schema"
  status = "planned"
  
  [validation.deps]
    - jsonschema
    - core::definition::schema
  
  [validation.api]
    [.validate]
      params = "data: &Value, schema: &Schema"
      returns = "Result<()>"
      errors = ["ValidationError"]
    
    [.validate_flattened]
      params = "data: &FlattenedData, schema: &Schema"
      returns = "Result<()>"
    
  [validation.tests]  # test da implementare
    [.test_schema_valid]
      input = '{ "name": "Mario" }'
      schema = '{ "type": "object", "properties": { "name": {"type": "string"} } }'
      expected = "Ok(())"
    
    [.test_schema_invalid]
      input = '{ "name": 123 }'
      expected = "Err(TypeError)"

# ============================================
# FUNZIONI (.ssf)
# ============================================

[_FN_PREFIX]  # prefisso per alias automatici
  flatten = "f"
  validate = "v"
  schema = "s"

[functions]
  [.flatten_json]
    sig = "fn(json: &Value) -> Result<FlattenedData>"
    module = "flatten"
    line = 42  # riferimento a riga nel codice
    
  [.validate_json]
    sig = "fn(data: &Value, schema: &Schema) -> Result<()>"
    module = "validation"
    
  [.infer_schema]
    sig = "fn(data: &Value) -> Result<Schema>"
    module = "schema"
    status = "wip"  # work in progress

# ============================================
# IDEE E OTTIMIZZAZIONI (.ssi)
# ============================================

[idee]
  [.db]
    [.ottimizzazione]
      [.join]  # join veloce con bucket
        desc = "Usare array 65535 per join O(1)"
        status = "idea"
        note = """
          Se le foreign key sono < 65536,
          possiamo fare join con lookup diretto.
          Esempio: 
            orders.user_id (2 byte) → users[user_id]
        """
      
      [.sort]
        desc = "Bucket sort per ordinamento istantaneo"
        note = "Basato su prime 2 lettere (65536 bucket)"
    
    [.indici]
      desc = "Indici automatici su campi frequenti"
      note = """
        Tracciare le query e creare bucket
        per i campi più usati in WHERE.
      """
  
  [.core]
    [.function]
      [.data.fix]
        desc = "Correggere gestione valori null in flatten"
        priority = "high"
      
      [.move.change]
        desc = "Spostare validazione in modulo separato"
        priority = "medium"

# ============================================
# DOCUMENTAZIONE (.ssdoc)
# ============================================

[doc.flatten]
  title = "Modulo Flattening"
  brief = "Trasforma JSON in formato tabellare"
  
  [doc.flatten.details]
    par1 = """
      Il modulo usa json-unflattening per appiattire
      strutture JSON complesse in dot notation.
    """
    par2 = """
      Gli array vengono separati in tabelle figlie
      con foreign key implicite.
    """
  
  [doc.flatten.example]
    code = """
      let json = json!({
        "user": {"name": "Mario"},
        "orders": [{"id": 1}, {"id": 2}]
      });
      let flat = flattener.flatten(&json)?;
      // flat.fields: user.name = "Mario"
      // flat.children[0].rows: [1, 2]
    """
  
  [doc.flatten.see_also]
    - "validation"
    - "schema"

# ============================================
# COMMENTI E DOCSTRING
# ============================================

[_COMMENT_STYLES]
  // = commento linea singola (ignorato)
  /// = docstring (genera documentazione)
  /* */ = commento multi-linea
  /** */ = docstring multi-linea

# Esempio con docstring
[funzione.esempio]
  /// Calcola il prodotto scalare
  /// # Parameters
  /// - `a`: primo vettore
  /// - `b`: secondo vettore
  /// # Returns
  /// Prodotto scalare come f64
  sig = "fn dot(a: &[f64], b: &[f64]) -> f64"
  // Implementazione commentata
  // TODO: gestire lunghezze diverse

# ============================================
# GENERAZIONE CODICE
# ============================================

[_GENERATOR_RULES]
  // Da .sson a Rust
  [.function]
    template = """
      /// {desc}
      pub fn {name}({params}) -> {returns} {{
          // TODO: implementare
          unimplemented!()
      }}
    """
  
  [.module]
    template = """
      //! {desc}
      
      {functions}
    """
  
  [.test]
    template = """
      #[test]
      fn test_{name}() {{
          let input = {input};
          let result = {function}(input);
          assert_eq!(result, {expected});
      }}
    """

# ============================================
# REGOLE DEL SISTEMA
# ============================================

[_RULES]
  1. "Ogni funzione DEVE avere un .sson prima del codice"
  2. "Le idee vanno in .ssi, non nel codice"
  3. "I TODO con priority='high' vanno implementati subito"
  4. "Le docstring /// generano documentazione automatica"
  5. "I commenti // sono ignorati dal generatore"
  6. "I path con . creano gerarchia (es. [db.ottimizzazione.join])"
  7. "I prefissi _ sono per metadati ([_META], [_DEPS])"
  8. "NULL indica implementazione futura"

# ============================================
# ESEMPIO COMPLETO: PROGETTO MINIMO
# ============================================

[_META] name = "hello-world", version = "0.1.0"

[_DEPS] nessuna

[functions]
  [.main]
    sig = "fn main()"
    desc = "Punto di ingresso"
    todo = "Stampa 'Hello, world!'"

# Questo .sson genera:
#
# fn main() {
#     // TODO: Stampa 'Hello, world!'
#     unimplemented!()
# }
```

---

## 🎯 VANTAGGI

1. **Progettazione prima del codice** - tutto è definito prima di scrivere
2. **Documentazione automatica** - i commenti `///` diventano doc
3. **TODO tracciati** - mai più cose dimenticate
4. **Generazione codice** - scheletri automatici
5. **Idee strutturate** - le idee hanno priorità e note
6. **Multi-linguaggio** - stesso .sson può generare Rust, Python, JS
7. **AI-ready** - l'AI capisce subito cosa serve

---

## 📋 PROSSIMO PASSO

Vuoi che:
1. **Generiamo il parser .sson** che legge questi file?
2. **Creiamo un generatore** che da .sson produce codice Rust?
3. **Scriviamo le definizioni** per `core` e `core-data` in .sson?