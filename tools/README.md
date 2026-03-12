# URCM Patch Tool

Tool per applicare patch mirate ai file del progetto URCM.

## Uso

```bash
node patch.js <file_patch>

--$FILE===: percorso/file.rs
contenuto completo del file...

--$CHANGE===: percorso/file.rs
--$LINESUB===15,20:
nuovo contenuto per le righe 15-20

--$LINEADD===25:
linea da aggiungere alla riga 25

--$LINEDEL===30,35
```

testi

--$FILE===: test/src/main.rs
fn main() {
    println!("Ciao mondo!");
    
    let x = 10;
    let y = 20;
    let risultato = x + y;
    
    println!("Risultato: {}", risultato);
}

--$CHANGE===: test/src/main.rs
--$LINESUB===5,6:
    let a = 15;
    let b = 25;

--$LINEADD===8:
    println!("Calcolo in corso...");

--$LINEDEL===10,11

--$LINEADD===12:
    println!("Fatto!");
