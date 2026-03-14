//! Motore principale per la generazione con riveter

use std::path::Path;
use std::process::Command;
use anyhow::{Result, Context, anyhow};
use std::fs;
use colored::*;

use crate::xml_parser::{
    extract_xml_tag, extract_tree, extract_all_files, count_files, decode_html_entities
};

/// Verifica che riveter sia installato
fn check_riveter_installed() -> Result<()> {
    let output = Command::new("riveter")
        .arg("--version")
        .output();

    match output {
        Ok(o) if o.status.success() => Ok(()),
        _ => {
            println!("{}", "  ⚠ riveter non trovato. Installalo con:".bright_yellow());
            println!("    cargo install riveter");
            anyhow::bail!("riveter non installato");
        }
    }
}

/// Genera .sson usando riveter per analisi approfondita
pub fn generate_with_riveter(project_path: &Path, include_src: bool) -> Result<String> {
    println!("{}", "📦 Analisi con riveter...".bright_yellow());

    // Verifica che riveter sia installato
    check_riveter_installed()?;

    // Crea directory temporanea con nome univoco
    let temp_dir = std::env::temp_dir();
    let xml_output = temp_dir.join(format!("riveter_{}.xml", std::process::id()));

    println!("  Output XML: {}", xml_output.display());

    // Esegui riveter scrivendo direttamente su file
    let status = Command::new("riveter")
        .args(&[
            "-d", project_path.to_str().unwrap(),
            "-f", "xml"
        ])
        .stdout(fs::File::create(&xml_output)?)
        .status()
        .context("Errore nell'esecuzione di riveter")?;

    if !status.success() {
        anyhow::bail!("riveter ha fallito con codice {}", status);
    }

    // Verifica che il file esista e non sia vuoto
    let metadata = fs::metadata(&xml_output)?;
    println!("  File XML creato: {} bytes", metadata.len());

    if metadata.len() == 0 {
        anyhow::bail!("Il file XML è vuoto");
    }

    println!("  Parsing output XML...");

    // Leggi il file
    let xml_content = fs::read_to_string(&xml_output)?;
    println!("  XML letto: {} bytes", xml_content.len());

    // Pulisci XML: rimuovi caratteri di controllo non validi (mantieni \n \r \t)
    let clean_xml: String = xml_content
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\r' || *c == '\t')
        .collect();

    println!("  XML pulito: {} bytes", clean_xml.len());

    // Decodifica entità HTML
    let decoded_xml = decode_html_entities(&clean_xml);
    println!("  XML decodificato: {} bytes", decoded_xml.len());

    // Estrai manualmente i dati invece di usare serde
    println!("  Estrazione dati da XML...");

    // Estrai rootPath
    let root_path = extract_xml_tag(&decoded_xml, "rootPath").unwrap_or_else(|| "unknown".to_string());

    // Costruisci .sson manualmente
    let mut sson = String::new();
    sson.push_str("[_META]\n");
    sson.push_str(&format!("  root = \"{}\"\n", root_path));
    sson.push_str("  generated_by = \"riveter + sson-gen\"\n\n");

    sson.push_str("[project.tree]\n");
    extract_tree(&decoded_xml, &mut sson, 0);

    if include_src {
        sson.push_str("\n[project.files]\n");
        extract_all_files(&decoded_xml, &mut sson);
    }

    // Metriche
    sson.push_str("\n[project.metrics]\n");
    let file_count = count_files(&decoded_xml);
    sson.push_str(&format!("  total_files = {}\n", file_count));

    // Pulisci il file temporaneo
    let _ = fs::remove_file(xml_output);

    Ok(sson)
}
