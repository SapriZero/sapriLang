= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
GUIDA ALL'USO DI URCM-CORE
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

INDICE
1. Introduzione
2. Installazione e dipendenze
3. La struct UrcmCtx<T>
4. Definizione degli atomi
5. Valutazione delle espressioni
6. Macro urcm! e pipe!
7. Variabili reattive (maiuscole)
8. Esempi completi
9. Integrazione con l'interprete esistente

= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
1. INTRODUZIONE
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

urcm-core è una libreria Rust che implementa il paradigma URCM (Unità Relazionale
Contestuale di Movimento) direttamente in Rust. Permette di:

- Definire atomi (caratteri singoli) con valori numerici o funzioni
- Valutare espressioni come "t * u / 100" in modo efficiente
- Gestire variabili reattive (maiuscole) che si aggiornano automaticamente
- Usare macro per scrivere codice in stile URCM

La libreria è pensata per essere usata sia standalone che come motore per
l'interprete SAPRI Language.

= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
2. INSTALLAZIONE E DIPENDENZE
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

A. Come crate standalone
'''
[dependencies]
urcm-core = { path = "../core" }
'''

B. Per compilare come libreria dinamica (.so)
'''
[lib]
crate-type = ["cdylib", "rlib"]
'''

Poi compilare con:
'''
cd core
cargo build --release
'''

Il file .so si trova in target/release/

= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
3. LA STRUCT UrcmCtx<T>
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

UrcmCtx è il contesto che contiene tutti gli atomi e le loro relazioni.
È generico su T, che rappresenta i dati utente accessibili dalle funzioni.

'''
use urcm_core::UrcmCtx;

struct MyData {
    t: f64,
    u: f64,
}

let data = MyData { t: 25.0, u: 60.0 };
let mut ctx = UrcmCtx::new(data);
'''

= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
4. DEFINIZIONE DEGLI ATOMI
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

Esistono quattro tipi di atomi:

A. Atomo numerico (valore diretto)
'''
ctx.def_number('t', "temperatura", 25.0);
'''

B. Atomo campo (accede a un campo della struct)
'''
ctx.def_field('t', "temperatura", "t");
'''

C. Atomo funzione (chiama una fn(&T) -> f64)
'''
use std::collections::HashSet;

fn calcola_comfort(data: &MyData) -> f64 {
    data.t * data.u / 100.0
}

ctx.def_function('C', "comfort", calcola_comfort, 
                 HashSet::from(['t', 'u']));
'''

D. Atomo reattivo (come funzione, ma si aggiorna automaticamente)
'''
ctx.def_reactive('R', "reattivo", 
                  Box::new(|d| d.t * d.u), 
                  HashSet::from(['t', 'u']));
'''

= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
5. VALUTAZIONE DELLE ESPRESSIONI
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

A. Valutazione diretta con eval
'''
let comfort = ctx.eval("t * u / 100").unwrap();
println!("Comfort: {}", comfort);
'''

B. Ottenere il valore di un singolo atomo
'''
let t = ctx.get('t').unwrap();
'''

C. Aggiornare un atomo numerico
'''
if let Some(deps) = ctx.set('t', 30.0) {
    println!("Atomi da aggiornare: {:?}", deps);
}
'''

= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
6. MACRO urcm! E pipe!
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

A. urcm! eager - calcolo immediato
'''
let t = 25.0;
let u = 60.0;
let comfort = urcm!(t * u / 100);
println!("Comfort: {}", comfort);
'''

B. urcm! lazy - formula differita
'''
let formula = urcm!(|t, u| t * u / 100);
let comfort = formula(25.0, 60.0);
'''

C. urcm! con contesto
'''
let comfort = urcm!("t * u / 100", &ctx);
'''

D. urcm_c! - currying del contesto
'''
let urcm = urcm_c!(ctx);
let c1 = urcm("t * u");
let c2 = urcm("t + u");
'''

E. pipe! - pipeline di trasformazioni
'''
let risultato = pipe!(10, 
                      |x| x * 2, 
                      |x| x + 5, 
                      |x| x as f64 / 3.0);
'''

= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
7. VARIABILI REATTIVE (MAIUSCOLE)
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

Le variabili reattive (simboli maiuscoli) si aggiornano automaticamente quando
cambiano le loro dipendenze.

'''
use std::collections::HashSet;

let mut ctx = UrcmCtx::new(data);
ctx.def_number('t', "temperatura", 25.0)
   .def_number('u', "umidita", 60.0)
   .def_reactive('C', "comfort", 
                 Box::new(|d| d.t * d.u / 100.0),
                 HashSet::from(['t', 'u']));

// Legge il valore corrente
let c = ctx.get('C').unwrap();

// Aggiorna t -> C si aggiornerà automaticamente al prossimo get
ctx.set('t', 30.0);
let c_aggiornato = ctx.get('C').unwrap();
'''

Nota: l'aggiornamento è lazy (avviene quando si chiama get). 
Per aggiornamento eager bisogna chiamare manualmente update.

= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
8. ESEMPI COMPLETI
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

A. Esempio base con struct
'''
use urcm_core::{UrcmCtx, urcm, pipe};
use std::collections::HashSet;

struct MyData {
    t: f64,
    u: f64,
}

impl MyData {
    fn comfort(&self) -> f64 {
        self.t * self.u / 100.0
    }
}

fn main() {
    let data = MyData { t: 25.0, u: 60.0 };
    let mut ctx = UrcmCtx::new(data);
    
    ctx.def_field('t', "temperatura", "t")
       .def_field('u', "umidita", "u")
       .def_function('C', "comfort", MyData::comfort, 
                    HashSet::from(['t', 'u']));
    
    let comfort = ctx.eval("C").unwrap();
    println!("Comfort: {}", comfort);
    
    // Usando urcm! macro
    let c2 = urcm!("t * u / 100", &ctx).unwrap();
    println!("Comfort2: {}", c2);
}
'''

B. Esempio con pipe
'''
fn main() {
    let risultato = pipe!(10, 
        |x| x * 2,
        |x| x + 5,
        |x| x as f64 / 3.0,
        |x| format!("Risultato: {:.2}", x)
    );
    println!("{}", risultato); // "Risultato: 8.33"
}
'''

= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
9. INTEGRAZIONE CON L'INTERPRETE ESISTENTE
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

Per usare urcm-core dentro l'interprete SAPRI Language:

A. In interpreter/Cargo.toml aggiungi:
'''
[dependencies]
urcm-core = { path = "../core" }
'''

B. In interpreter/src/main.rs:
'''
use urcm_core::{UrcmCtx, urcm, pipe};

// Ora puoi usare il contesto URCM per valutare le espressioni
// lette dai file .urcm
'''

C. Esempio di integrazione con il parser esistente
'''
use urcm_core::UrcmCtx;
use std::collections::HashMap;

struct RuntimeData {
    vars: HashMap<String, f64>,
}

fn eval_urcm_line(line: &str, ctx: &UrcmCtx<RuntimeData>) -> Result<f64, String> {
    // line può essere "C = t * u" o "t * u"
    if line.contains('=') {
        let parts: Vec<&str> = line.split('=').collect();
        let expr = parts[1].trim();
        ctx.eval(expr)
    } else {
        ctx.eval(line)
    }
}
'''

= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
FUNZIONI DI UTILITÀ (fp module)
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

Il modulo fp fornisce funzioni funzionali di utilità:

'''
use urcm_core::{eval, mask, Either};

// eval - condizione funzionale
let valore = eval(condition, || 10, || 20);

// mask - converte bool in 0/1 (branchless)
let mask_value = mask(condition);  // 1 se true, 0 se false

// Either - due tipi possibili
let e: Either<i32, &str> = Either::Right("test");
if e.is_right() {
    println!("{}", e.unwrap_right());
}
'''

= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
NOTE SULLE PERFORMANCE
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

- Le espressioni vengono valutate senza allocazioni
- Le macro sono zero-cost (vengono espanse alla compilazione)
- Le variabili reattive usano lazy update per minimizzare i calcoli
- Pipe! crea una catena di chiamate inline

Per massimizzare le performance:
1. Usa atomi numerici invece di funzioni quando possibile
2. Definisci le dipendenze correttamente per le variabili reattive
3. Preferisci urcm! eager per calcoli semplici
