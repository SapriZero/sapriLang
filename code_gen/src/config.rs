//! Caricamento configurazione (semplificato)

use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct GenConfig {
    pub grammar: GrammarConfig,
    pub type_mapping: HashMap<String, String>,
    pub rules: HashMap<String, String>,
    pub templates: HashMap<String, String>,
    pub files_to_process: Vec<PathBuf>,
    pub output_dir: PathBuf,
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub struct GrammarConfig {
    pub sections: Vec<GrammarSection>,
}

#[derive(Debug, Clone)]
pub struct GrammarSection {
    pub name: String,
    pub pattern: String,
    pub handler: String,
}

impl GenConfig {
    /// Carica la configurazione del generatore
    /// - `generator_config_dir`: directory contenente i file .sson del generatore (grammar, type_mapping, rules, templates)
    /// - `project_dir`: directory contenente il loader.sson del progetto
    pub fn load(generator_config_dir: &str, project_dir: &str) -> Result<Self, String> {
        println!("📖 Caricamento configurazione generatore da: {}", generator_config_dir);
        
        // Prova a caricare i file, se non esistono usa hard-coded
        let grammar = match load_grammar(&format!("{}/grammar.sson", generator_config_dir)) {
            Ok(g) => g,
            Err(_) => {
                println!("⚠️ grammar.sson non trovato, uso default");
                get_default_grammar()
            }
        };
        
        let type_mapping = match load_type_mapping(&format!("{}/type_mapping.sson", generator_config_dir)) {
            Ok(t) => t,
            Err(_) => {
                println!("⚠️ type_mapping.sson non trovato, uso default");
                get_default_type_mapping()
            }
        };
        
        let rules = match load_rules(&format!("{}/rules.sson", generator_config_dir)) {
            Ok(r) => r,
            Err(_) => {
                println!("⚠️ rules.sson non trovato, uso default");
                get_default_rules()
            }
        };
        
        let templates = match load_templates(&format!("{}/templates.sson", generator_config_dir)) {
            Ok(t) => t,
            Err(_) => {
                println!("⚠️ templates.sson non trovato, uso default");
                get_default_templates()
            }
        };
        
        println!("📖 Caricamento loader progetto da: {}", project_dir);
        let loader_path = format!("{}/loader.sson", project_dir);
        let (files, output_dir, verbose) = if std::path::Path::new(&loader_path).exists() {
            load_loader(&loader_path)?
        } else {
            println!("⚠️ loader.sson non trovato, uso default: input=sson/, output=src/generated/");
            let input_dir = format!("{}/sson", project_dir);
            scan_directory(&input_dir, &PathBuf::from("src/generated"), false)?
        };
        
        Ok(Self {
            grammar,
            type_mapping,
            rules,
            templates,
            files_to_process: files,
            output_dir,
            verbose,
        })
    }
}

fn load_grammar(path: &str) -> Result<GrammarConfig, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut sections = Vec::new();
    let mut current_name = String::new();
    let mut current_pattern = String::new();
    let mut current_handler = String::new();
    let mut in_section = false;
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        
        if line.starts_with("[grammar.sections.") && line.ends_with(']') {
            if in_section {
                sections.push(GrammarSection {
                    name: current_name.clone(),
                    pattern: current_pattern.clone(),
                    handler: current_handler.clone(),
                });
            }
            current_name = line[19..line.len()-1].to_string();
            current_pattern.clear();
            current_handler.clear();
            in_section = true;
        } else if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');
            match key {
                "pattern_s" => current_pattern = value.to_string(),
                "handler_s" => current_handler = value.to_string(),
                _ => {}
            }
        }
    }
    if in_section {
        sections.push(GrammarSection {
            name: current_name,
            pattern: current_pattern,
            handler: current_handler,
        });
    }
    
    Ok(GrammarConfig { sections })
}

fn load_type_mapping(path: &str) -> Result<HashMap<String, String>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut map = HashMap::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || !line.contains(':') { continue; }
        
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');
            map.insert(key.to_string(), value.to_string());
        }
    }
    
    Ok(map)
}

fn load_rules(path: &str) -> Result<HashMap<String, String>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut map = HashMap::new();
    let mut current_key = String::new();
    let mut current_value = String::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        
        if line.starts_with('[') && line.ends_with(']') {
            if !current_key.is_empty() {
                map.insert(current_key.clone(), current_value.trim().to_string());
                current_key.clear();
                current_value.clear();
            }
            current_key = line[1..line.len()-1].to_string();
        } else if line.starts_with("template_s:") {
            let template = line[12..].trim().trim_matches('"');
            current_value = template.to_string();
        }
    }
    if !current_key.is_empty() {
        map.insert(current_key, current_value.trim().to_string());
    }
    
    Ok(map)
}

fn load_templates(path: &str) -> Result<HashMap<String, String>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut map = HashMap::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');
            map.insert(key.to_string(), value.to_string());
        }
    }
    
    Ok(map)
}

fn load_loader(path: &str) -> Result<(Vec<PathBuf>, PathBuf, bool), String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut input_dir = ".".to_string();
    let mut output_dir = PathBuf::from("src/generated");
    let mut verbose = false;
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');
            match key {
                "input_dir_s" => input_dir = value.to_string(),
                "output_dir_s" => output_dir = PathBuf::from(value),
                "verbose_b" => verbose = value == "true",
                _ => {}
            }
        }
    }
    
    let mut files = Vec::new();
    let input_path = PathBuf::from(&input_dir);
    
    for entry in WalkDir::new(&input_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "sson"))
    {
        let path = entry.path().to_path_buf();
        let name = path.file_stem().unwrap().to_string_lossy();
        // Escludi i file di configurazione del generatore (che non devono essere processati)
        if !["config", "grammar", "type_mapping", "rules", "templates", "loader"]
            .contains(&name.as_ref()) {
            files.push(path);
        }
    }
    
    Ok((files, output_dir, verbose))
}

// ============================================
// DEFAULT HARD-CODED (usati se i file non esistono)
// ============================================

fn get_default_grammar() -> GrammarConfig {
    let mut sections = Vec::new();
    
    sections.push(GrammarSection {
        name: "struct".to_string(),
        pattern: r"^\[struct\.([a-zA-Z][a-zA-Z0-9_]*)\]$".to_string(),
        handler: "handle_struct".to_string(),
    });
    sections.push(GrammarSection {
        name: "struct.derive".to_string(),
        pattern: r"^\[struct\.([a-zA-Z][a-zA-Z0-9_]*)\.derive\]$".to_string(),
        handler: "handle_struct_derive".to_string(),
    });
    sections.push(GrammarSection {
        name: "struct.fields".to_string(),
        pattern: r"^\[struct\.([a-zA-Z][a-zA-Z0-9_]*)\.fields\]$".to_string(),
        handler: "handle_struct_fields".to_string(),
    });
    sections.push(GrammarSection {
        name: "impl".to_string(),
        pattern: r"^\[impl\.([a-zA-Z][a-zA-Z0-9_]*)\]$".to_string(),
        handler: "handle_impl".to_string(),
    });
    sections.push(GrammarSection {
        name: "impl.function".to_string(),
        pattern: r"^\[impl\.([a-zA-Z][a-zA-Z0-9_]*)\.([a-zA-Z][a-zA-Z0-9_]*)\]$".to_string(),
        handler: "handle_impl_function".to_string(),
    });
    
    GrammarConfig { sections }
}


fn get_default_type_mapping() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("s".to_string(), "String".to_string());
    map.insert("n".to_string(), "f64".to_string());
    map.insert("b".to_string(), "bool".to_string());
    map.insert("usize".to_string(), "usize".to_string());
    map.insert("u64".to_string(), "u64".to_string());
    map.insert("i32".to_string(), "i32".to_string());
    map
}

fn get_default_rules() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("struct".to_string(), "#[derive({derive})]\npub struct {name} {{\n{fields}\n}}\n".to_string());
    map.insert("struct.field".to_string(), "    pub {name}: {type_},\n".to_string());
    map.insert("enum".to_string(), "#[derive({derive})]\npub enum {name} {{\n{variants}\n}}\n".to_string());
    map.insert("impl".to_string(), "impl {name} {{\n{functions}\n}}\n".to_string());
    map.insert("impl.function".to_string(), "    pub fn {name}({params}) -> {return_type} {{\n        todo!()\n    }}\n".to_string());
    map
}

fn get_default_templates() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("file_header".to_string(), "// Auto-generated by sapri-code-gen on {timestamp}\n// Do not edit manually\n\n".to_string());
    map.insert("file_footer".to_string(), "\n// End of auto-generated code\n".to_string());
    map
}

fn scan_directory(input_dir: &str, output_dir: &PathBuf, verbose: bool) -> Result<(Vec<PathBuf>, PathBuf, bool), String> {
    let mut files = Vec::new();
    
    if !std::path::Path::new(input_dir).exists() {
        return Ok((files, output_dir.clone(), verbose));
    }
    
    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "sson"))
    {
        let path = entry.path().to_path_buf();
        let name = path.file_stem().unwrap().to_string_lossy();
        if name != "loader" {
            files.push(path);
        }
    }
    
    Ok((files, output_dir.clone(), verbose))
}
