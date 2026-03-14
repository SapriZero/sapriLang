//! sson-gen - Generatore di file .sson dalla struttura di progetti Rust
//!
//! Punto di ingresso principale.

use clap::{Arg, ArgAction, Command as ClapCommand};
use colored::*;
use anyhow::{Result, Context};
use std::path::Path;
use std::fs;

mod engine;
mod xml_parser;
mod basic_generator;

use engine::generate_with_riveter;
use basic_generator::generate_basic_structure;

fn main() -> Result<()> {
    let matches = ClapCommand::new("sson-gen")
        .version("0.1.0")
        .about("Genera file .sson dalla struttura di progetti Rust")
        .arg(
            Arg::new("project")
                .short('p')
                .long("project")
                .value_name("PATH")
                .help("Percorso del progetto da analizzare")
                .default_value(".")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("File .sson di output")
                .default_value("project.ssd")
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Formato output: ssd, ssf, ssi")
                .default_value("ssd")
        )
        .arg(
            Arg::new("include-src")
                .long("include-src")
                .help("Includi anche i file sorgente nel .sson")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("no-riveter")
                .long("no-riveter")
                .help("Non usare riveter (solo struttura base)")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    let project_path = matches.get_one::<String>("project").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();
    let format = matches.get_one::<String>("format").unwrap();
    let include_src = matches.get_flag("include-src");
    let use_riveter = !matches.get_flag("no-riveter");

    println!("{}", "🔍 sson-gen - Generatore .sson".bright_green());
    println!("  Progetto: {}", project_path.bright_cyan());
    println!("  Output:   {}", output_file.bright_cyan());
    println!("  Formato:  {}", format.bright_cyan());
    println!();

    // Verifica che il progetto esista
    let project_path = Path::new(project_path);
    if !project_path.exists() {
        anyhow::bail!("Il percorso {} non esiste", project_path.display());
    }

    // Genera il .sson
    let sson_content = if use_riveter {
        generate_with_riveter(project_path, include_src)?
    } else {
        generate_basic_structure(project_path, include_src)?
    };

    // Salva il file
    fs::write(output_file, sson_content)
        .context(format!("Impossibile scrivere {}", output_file))?;

    println!("\n{}", "✅ Generazione completata!".bright_green());
    println!("  File creato: {}", output_file.bright_cyan());

    Ok(())
}
