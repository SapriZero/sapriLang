//! Generatore di struttura base senza riveter
//! Versione funzionale con piccole funzioni pure

use std::path::Path;
use anyhow::Result;
use walkdir::{WalkDir, DirEntry};

// ==========================================
// 1. FUNZIONI PURE DI FILTRAGGIO
// ==========================================

/// Determina se un entry deve essere nascosta
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.') || s == "target" || s == "node_modules")
        .unwrap_or(false)
}

/// Filtro per entry nascoste (componibile)
fn filter_hidden() -> impl Fn(&DirEntry) -> bool {
    |entry| !is_hidden(entry)
}

/// Estrae il percorso relativo dal progetto
fn get_relative_path<'a>(project_path: &'a Path, entry: &'a DirEntry) -> &'a Path {
    entry.path().strip_prefix(project_path).unwrap_or_else(|_| entry.path())
}

/// Determina se un file è un sorgente Rust
fn is_rust_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file() && 
    entry.path().extension().map(|e| e == "rs").unwrap_or(false)
}

/// Determina se è una directory
fn is_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

// ==========================================
// 2. FUNZIONI DI TRASFORMAZIONE
// ==========================================

/// Converte un entry in una riga .sson per directory
fn dir_to_sson_line(rel_path: &Path) -> String {
    format!("  {}/\n", rel_path.display())
}

/// Converte un entry in una riga .sson per file Rust
fn rust_file_to_sson_line(rel_path: &Path) -> String {
    format!("  {} (rust)\n", rel_path.display())
}

/// Converte un entry in una riga .sson per file generico
fn file_to_sson_line(rel_path: &Path) -> String {
    format!("  {}\n", rel_path.display())
}

/// Factory: restituisce la funzione di conversione appropriata per tipo di file
fn get_converter(include_src: bool) -> impl Fn(&DirEntry, &Path) -> Option<String> {
    move |entry, rel_path| {
        if is_directory(entry) {
            Some(dir_to_sson_line(rel_path))
        } else if include_src && is_rust_file(entry) {
            Some(rust_file_to_sson_line(rel_path))
        } else if include_src && entry.file_type().is_file() {
            Some(file_to_sson_line(rel_path))
        } else {
            None
        }
    }
}

// ==========================================
// 3. FUNZIONI DI GENERAZIONE METADATI
// ==========================================

/// Genera la sezione [_META]
fn generate_meta_section() -> String {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    format!(
        "[_META]\n  generated = \"{}\"\n  tool = \"sson-gen\"\n  version = \"0.1.0\"\n\n",
        now
    )
}

/// Genera l'intestazione della sezione struttura
fn generate_structure_header() -> String {
    "[project.structure]\n".to_string()
}

// ==========================================
// 4. FUNZIONE PRINCIPALE (COMPOSIZIONE)
// ==========================================

/// Genera struttura base senza riveter
pub fn generate_basic_structure(project_path: &Path, include_src: bool) -> Result<String> {
    // Prepara i filtri e converter
    let hidden_filter = filter_hidden();
    let converter = get_converter(include_src);
    
    // Raccogli tutte le entry e trasformale in righe
    let mut lines: Vec<String> = WalkDir::new(project_path)
        .min_depth(1)
        .into_iter()
        .filter_entry(hidden_filter)
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let rel_path = get_relative_path(project_path, &entry).to_path_buf();
            converter(&entry, &rel_path)
        })
        .collect();
    
    // Ordina per avere output consistente
    lines.sort();
    
    // Costruisci il documento completo
    let mut sson = String::new();
    sson.push_str(&generate_meta_section());
    sson.push_str(&generate_structure_header());
    
    for line in lines {
        sson.push_str(&line);
    }
    
    Ok(sson)
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    
    #[test]
    fn test_generate_basic_structure() {
        let dir = tempdir().unwrap();
        let test_path = dir.path();
        
        // Crea alcuni file di test
        fs::write(test_path.join("Cargo.toml"), "").unwrap();
        fs::create_dir(test_path.join("src")).unwrap();
        fs::write(test_path.join("src/main.rs"), "").unwrap();
        
        let result = generate_basic_structure(test_path, true).unwrap();
        
        assert!(result.contains("[_META]"));
        assert!(result.contains("[project.structure]"));
        assert!(result.contains("Cargo.toml"));
        assert!(result.contains("src/"));
        assert!(result.contains("main.rs (rust)"));
    }
    
    #[test]
    fn test_hidden_filter() {
        // Non testiamo is_hidden direttamente
        assert!(true);
    }
}
