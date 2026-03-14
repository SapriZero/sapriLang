//! Generatore di struttura base senza riveter

use std::path::Path;
use anyhow::Result;
use walkdir::WalkDir;

/// Genera struttura base senza riveter
pub fn generate_basic_structure(project_path: &Path, include_src: bool) -> Result<String> {
    let mut sson = String::new();

    // Metadati
    sson.push_str("[_META]\n");
    sson.push_str(&format!("  generated = \"{}\"\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    sson.push_str(&format!("  tool = \"sson-gen\"\n"));
    sson.push_str(&format!("  version = \"0.1.0\"\n\n"));

    // Struttura directory
    sson.push_str("[project.structure]\n");
    for entry in WalkDir::new(project_path)
        .max_depth(3)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(project_path).unwrap_or(path);

        if entry.file_type().is_dir() {
            sson.push_str(&format!("  dir: {}\n", relative.display()));
        } else if include_src && entry.file_type().is_file() {
            if let Some(ext) = path.extension() {
                if ext == "rs" || ext == "toml" || ext == "md" {
                    sson.push_str(&format!("  file: {} (rust)\n", relative.display()));
                }
            }
        }
    }

    // File Rust
    if include_src {
        sson.push_str("\n[project.sources]\n");
        for entry in WalkDir::new(project_path)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
        {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let relative = path.strip_prefix(project_path).unwrap_or(path);
                sson.push_str(&format!("  {}\n", relative.display()));
            }
        }
    }

    Ok(sson)
}

/// Verifica se un entry è nascosta (inizia con . o è in .gitignore)
fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.') || s == "target" || s == "node_modules")
        .unwrap_or(false)
}
