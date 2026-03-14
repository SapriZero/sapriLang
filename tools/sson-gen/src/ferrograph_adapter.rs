// tools/sson-gen/src/ferrograph_adapter.rs
use std::path::Path;
use std::process::Command;
use anyhow::{Result, Context};
use serde_json::Value;

pub struct FerrographData {
    pub functions: Vec<FunctionInfo>,
    pub structs: Vec<StructInfo>,
    pub enums: Vec<EnumInfo>,
    pub traits: Vec<TraitInfo>,
    pub calls: Vec<CallEdge>,
}

pub fn extract_with_ferrograph(project_path: &Path) -> Result<FerrographData> {
    // 1. Indexa con ferrograph
    let db_path = std::env::temp_dir().join(format!("ferrograph_{}.db", std::process::id()));
    
    Command::new("ferrograph")
        .args(&["index", project_path.to_str().unwrap(), "--output", db_path.to_str().unwrap()])
        .status()
        .context("Errore nell'esecuzione di ferrograph")?;

    // 2. Query funzioni
    let functions = query_nodes(&db_path, "function")?;
    let structs = query_nodes(&db_path, "struct")?;
    let enums = query_nodes(&db_path, "enum")?;
    let traits = query_nodes(&db_path, "trait")?;
    let calls = query_edges(&db_path)?;

    Ok(FerrographData { functions, structs, enums, traits, calls })
}

fn query_nodes(db_path: &Path, node_type: &str) -> Result<Vec<FunctionInfo>> {
    let query = format!(
        "?[id, name, file, line] := *nodes[id, type='{}', payload], payload: {{name: name, file: file, line: line}}",
        node_type
    );
    
    let output = Command::new("ferrograph")
        .args(&["query", "--db", db_path.to_str().unwrap(), &query])
        .output()?;
    
    // Parsa output JSON
    let json: Value = serde_json::from_slice(&output.stdout)?;
    // Converti in vettore di FunctionInfo...
    Ok(vec![])
}
