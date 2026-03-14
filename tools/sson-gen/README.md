# sson-gen - Generatore .sson per progetti Rust

Genera file `.sson` (Sapri Structure Definition) dalla struttura di progetti Rust, utilizzando `riveter` per analisi approfondita.

## Installazione

```bash
# Clona il repository
cd tools/sson-gen
cargo build --release
cp target/release/sson-gen ~/.local/bin/  # o in qualsiasi PATH
