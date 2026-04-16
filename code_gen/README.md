[package]
name = "sapri_code_gen"
version = "0.1.0"
edition = "2021"
description = "Generatore di codice Rust da file .sson"

[dependencies]
sapri_sson = { path = "../core/sson" }
sapri_obj = { path = "../core/obj" }
sapri_base = { path = "../core/base" }

# Utilità
regex = "1.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
walkdir = "2.5"
heck = "0.5"  # per conversioni snake_case, PascalCase, etc.

[features]
default = []

[[bin]]
name = "sapri-code-gen"
path = "src/main.rs"
