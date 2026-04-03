//! Generatore di struttura base senza riveter
//! Versione con albero gerarchico

use std::path::Path;
use anyhow::Result;
use walkdir::WalkDir;
use std::collections::BTreeMap;

// ==========================================
// 1. STRUTTURA DATI PER L'ALBERO
// ==========================================

#[derive(Debug)]
struct TreeNode {
    name: String,
    is_dir: bool,
    children: BTreeMap<String, TreeNode>,
}

impl TreeNode {
    fn new(name: String, is_dir: bool) -> Self {
        Self {
            name,
            is_dir,
            children: BTreeMap::new(),
        }
    }
    
    /// Aggiunge un percorso all'albero
    fn add_path(&mut self, path: &str, is_dir: bool) {
        let parts: Vec<&str> = path.split('/').collect();
        self.add_parts(&parts, is_dir);
    }
    
	fn add_parts(&mut self, parts: &[&str], is_dir: bool) {
    println!("add_parts: parts={:?}, is_dir={}, current node: {} has {} children", 
             parts, is_dir, self.name, self.children.len());
    
    if parts.is_empty() {
        return;
    }
    
    let current = parts[0].to_string();
    
    if parts.len() == 1 {
        println!("  last element '{}' - inserting as {}", current, if is_dir { "dir" } else { "file" });
        self.children
            .entry(current.clone())
            .or_insert_with(|| {
                println!("    creating new node: {}", current);
                TreeNode::new(current, is_dir)
            });
    } else {
        println!("  directory '{}' - recursing", current);
        let child = self.children
            .entry(current.clone())
            .or_insert_with(|| {
                println!("    creating new dir node: {}", current);
                TreeNode::new(current, true)
            });
        child.add_parts(&parts[1..], is_dir);
    }
    
    println!("  after add_parts, node {} has {} children", self.name, self.children.len());
}
    
    /// Genera rappresentazione testuale con indentazione
    /// Genera rappresentazione testuale con indentazione
	fn format(&self, depth: usize, include_src: bool) -> String {
	    let indent = "  ".repeat(depth);
	    let mut result = String::new();
	    
	    // Aggiungi questa directory/file
	    if depth > 0 {  // salta la root
	        if self.is_dir {
	            result.push_str(&format!("{}{}/\n", indent, self.name));
	        } else {
	            // Forza l'aggiunta dei file indipendentemente da include_src
	            if self.name.ends_with(".rs") {
	                result.push_str(&format!("{}{} (rust)\n", indent, self.name));
	            } else if self.name.ends_with(".toml") {
	                result.push_str(&format!("{}{} (toml)\n", indent, self.name));
	            } else if self.name.ends_with(".md") {
	                result.push_str(&format!("{}{} (markdown)\n", indent, self.name));
	            } else {
	                result.push_str(&format!("{}{}\n", indent, self.name));
	            }
	        }
	    }
	    
	    // Aggiungi figli in ordine
	    for (_, child) in &self.children {
	        result.push_str(&child.format(depth + 1, include_src));
	    }
	    
	    result
	}
}

// ==========================================
// 2. FUNZIONI PURE DI FILTRAGGIO
// ==========================================

/// Determina se un entry deve essere nascosta
fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.') || s == "target" || s == "node_modules")
        .unwrap_or(false)
}

/// Estrae il percorso relativo dal progetto
fn get_relative_path<'a>(project_path: &'a Path, entry: &'a walkdir::DirEntry) -> String {
    entry.path()
        .strip_prefix(project_path)
        .unwrap_or_else(|_| entry.path())
        .display()
        .to_string()
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
// 4. FUNZIONE PRINCIPALE
// ==========================================

/// Genera struttura base senza riveter
pub fn generate_basic_structure(project_path: &Path, include_src: bool) -> Result<String> {
    println!("generate_basic_structure: path={:?}", project_path);
    
    // Crea radice dell'albero
    let mut root = TreeNode::new("".to_string(), true);
    
    // Raccogli tutte le entry
    println!("Scanning directory...");
    for entry in WalkDir::new(project_path)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {
        let entry = entry?;
        let rel_path = get_relative_path(project_path, &entry);
        let is_dir = entry.file_type().is_dir();
        
        println!("  found: {} (dir={})", rel_path, is_dir);
        root.add_path(&rel_path, is_dir);
    }
    
    println!("Building output...");
    // Genera output
    let mut sson = String::new();
    sson.push_str(&generate_meta_section());
    sson.push_str(&generate_structure_header());
    sson.push_str(&root.format(0, include_src));
    
    println!("Output length: {} bytes", sson.len());
    
    Ok(sson)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    
    #[test]
    fn test_tree_node_add_path() {
        let mut root = TreeNode::new("".to_string(), true);
        root.add_path("src/main.rs", false);
        root.add_path("src/lib.rs", false);
        root.add_path("Cargo.toml", false);
        
        assert!(root.children.contains_key("src"));
        assert!(root.children.contains_key("Cargo.toml"));
        
        let src = root.children.get("src").unwrap();
        assert!(src.children.contains_key("main.rs"));
        assert!(src.children.contains_key("lib.rs"));
    }
    
    #[test]
    fn test_generate_basic_structure() {
        let dir = tempdir().unwrap();
        let test_path = dir.path();
        
        // Crea alcuni file di test
        fs::write(test_path.join("Cargo.toml"), "").unwrap();
        fs::create_dir_all(test_path.join("src")).unwrap();
        fs::write(test_path.join("src/main.rs"), "").unwrap();
        fs::write(test_path.join("src/lib.rs"), "").unwrap();
        
        let result = generate_basic_structure(test_path, true).unwrap();
        
        println!("{}", result);  // per debug
        
        assert!(result.contains("[_META]"));
        assert!(result.contains("[project.structure]"));
        assert!(result.contains("Cargo.toml"));
        assert!(result.contains("src/"));
        assert!(result.contains("  main.rs (rust)"));
        assert!(result.contains("  lib.rs (rust)"));
    }
}
