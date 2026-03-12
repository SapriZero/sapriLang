# Motore Rust per URCM / SapriLanguage

https://github.com/SapriZero/sapriLang.git

Questo progetto contiene due componenti:

## /interpreter
Motore principale che:
- Legge file .urcm (DSL)
- Compila in bytecode
- Esegue con hot-reload su modifiche

## /runtime
Esempio di runtime embedded per integrare URCM in altre app

## Compilazione
```bash
cd interpreter
cargo run -- esempio.urcm

cd ../runtime


#PATH: README.md
# Motore Rust per URCM / SapriLanguage - Versione Corretta

Questo progetto contiene due componenti:

## /interpreter
Motore principale che:
- Legge file .urcm (DSL)
- Compila in bytecode
- Esegue con hot-reload su modifiche

## /runtime
Esempio di runtime embedded per integrare URCM in altre app

## Compilazione
```bash
cd interpreter
cargo run -- esempio.urcm

cd ../runtime
cargo run






## 🚀 COME USARE

1. Salva il testo sopra come `urcm_rust_bundle.txt`
2. Salva anche lo script splitter come `urcm_splitter.sh`
3. Esegui:
```bash
chmod +x urcm_splitter.sh
./urcm_splitter.sh urcm_rust_bundle.txt   # chiede conferma
# oppure
./urcm_splitter.sh urcm_rust_bundle.txt yes   # sovrascrive senza chiedere

    Entra nella cartella e compila:

bash

cd urcm_rust_motor
cd interpreter
cargo run -- examples/test.urcm

Il sistema:

    Legge file .urcm

    Compila in bytecode interno

    Esegue con hot-reload (modifica test.urcm e vedrai il live update)
# sapriLang
