//! Generatore di codice Rust

use std::collections::HashMap;
use crate::config::GenConfig;
use regex::Regex;
use std::path::Path;

pub struct CodeGenerator {
    type_mapping: HashMap<String, String>,
    rules: HashMap<String, String>,
    templates: HashMap<String, String>,
    grammar: Vec<(Regex, String)>,
}

impl CodeGenerator {
    pub fn new(config: &GenConfig) -> Result<Self, String> {
        let mut grammar = Vec::new();
        for section in &config.grammar.sections {
            let regex = Regex::new(&section.pattern)
                .map_err(|e| format!("Regex invalido: {} - {}", section.pattern, e))?;
            grammar.push((regex, section.handler.clone()));
        }
        
        Ok(Self {
            type_mapping: config.type_mapping.clone(),
            rules: config.rules.clone(),
            templates: config.templates.clone(),
            grammar,
        })
    }
    
    pub fn generate(&self, content: &str) -> Result<String, String> {
        let mut output = String::new();
        
        // Header
        if let Some(header) = self.templates.get("file_header") {
            output.push_str(&self.render_template(header));
            output.push('\n');
        }
        
        // Parsa le sezioni
        let sections = self.parse_sections(content);
        
        // Genera struct
        output.push_str(&self.generate_structs(&sections)?);
        
        // Genera enum
        output.push_str(&self.generate_enums(&sections)?);
        
        // Genera impl
        output.push_str(&self.generate_impls(&sections)?);
        
        // Footer
        if let Some(footer) = self.templates.get("file_footer") {
            output.push_str(&self.render_template(footer));
        }
        
        Ok(output)
    }
    
	    /// Genera codice Rust dal contenuto di un file .sson (versione senza path)
	pub fn generate_from_content(&self, content: &str) -> Result<String, String> {
	    let mut output = String::new();
	    
	    // Header
	    if let Some(header) = self.templates.get("file_header") {
	        output.push_str(&self.render_template(header));
	        output.push('\n');
	    }
	    
	    // Parsa le sezioni
	    let sections = self.parse_sections(content);
	    
	    // Genera struct
	    output.push_str(&self.generate_structs(&sections)?);
	    
	    // Genera enum
	    output.push_str(&self.generate_enums(&sections)?);
	    
	    // Genera impl
	    output.push_str(&self.generate_impls(&sections)?);
	    
	    // Footer
	    if let Some(footer) = self.templates.get("file_footer") {
	        output.push_str(&self.render_template(footer));
	    }
	    
	    Ok(output)
	}
	
	/// Genera codice Rust da un file .sson e lo salva su disco
	/// Non sovrascrive i file _impl.rs
	pub fn generate_to_file(&self, input_path: &Path, output_dir: &Path) -> Result<(), String> {
	    let stem = input_path.file_stem().unwrap().to_string_lossy();
	    let output_path = output_dir.join(format!("{}.rs", stem));
	    
	    // Non sovrascrivere i file _impl.rs
	    if output_path.exists() && stem.to_string().ends_with("_impl") {
	        println!("  ⏭️ Skipping existing impl file: {:?}", output_path);
	        return Ok(());
	    }
	    
	    let content = std::fs::read_to_string(input_path)
	        .map_err(|e| e.to_string())?;
	    
	    let rust_code = self.generate_from_content(&content)?;
	    
	    std::fs::write(&output_path, rust_code)
	        .map_err(|e| e.to_string())
	}	
    
    
    fn parse_sections(&self, content: &str) -> HashMap<String, Vec<(String, HashMap<String, String>)>> {
        let mut result: HashMap<String, Vec<(String, HashMap<String, String>)>> = HashMap::new();
        let mut current_type = String::new();
        let mut current_name = String::new();
        let mut current_fields = HashMap::new();
        let mut in_section = false;
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            
            if line.starts_with('[') && line.ends_with(']') {
                // Salva sezione precedente
                if in_section && !current_name.is_empty() {
                    result.entry(current_type.clone())
                        .or_insert_with(Vec::new)
                        .push((current_name.clone(), current_fields.clone()));
                }
                
                // Nuova sezione
                let section = &line[1..line.len()-1];
                in_section = true;
                current_fields.clear();
                
                let parts: Vec<&str> = section.split('.').collect();
                if parts.len() >= 2 {
                    current_type = parts[0].to_string();
                    current_name = parts[1].to_string();
                } else {
                    current_type = section.to_string();
                    current_name = String::new();
                }
            } else if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                current_fields.insert(key.to_string(), value.to_string());
            }
        }
        
        // Ultima sezione
        if in_section && !current_name.is_empty() {
            result.entry(current_type)
                .or_insert_with(Vec::new)
                .push((current_name, current_fields));
        }
        
        result
    }
    
    fn generate_structs(&self, sections: &HashMap<String, Vec<(String, HashMap<String, String>)>>) -> Result<String, String> {
        let mut output = String::new();
        
        if let Some(structs) = sections.get("struct") {
            for (name, fields) in structs {
                let derive = fields.get("derive").cloned().unwrap_or_else(|| "Debug, Clone".to_string());
                let template = self.rules.get("struct").cloned()
                    .unwrap_or_else(|| "#[derive({derive})]\npub struct {name} {{\n{fields}\n}}\n".to_string());
                
                let mut fields_str = String::new();
                let field_template = self.rules.get("struct.field").cloned()
                    .unwrap_or_else(|| "    pub {name}: {type_},\n".to_string());
                
                // Cerca i campi
                for (key, value) in fields {
                    if key == "fields" {
                        for line in value.lines() {
                            if let Some((fname, ftype)) = line.split_once(':') {
                                let fname = fname.trim();
                                let ftype = ftype.trim();
                                let rust_type = self.map_type(ftype);
                                let code = field_template
                                    .replace("{name}", fname)
                                    .replace("{type_}", &rust_type);
                                fields_str.push_str(&code);
                            }
                        }
                    }
                }
                
                let code = template
                    .replace("{derive}", &derive)
                    .replace("{name}", name)
                    .replace("{fields}", &fields_str);
                output.push_str(&code);
                output.push('\n');
            }
        }
        
        Ok(output)
    }
    
    fn generate_enums(&self, sections: &HashMap<String, Vec<(String, HashMap<String, String>)>>) -> Result<String, String> {
        let mut output = String::new();
        
        if let Some(enums) = sections.get("enum") {
            for (name, variants) in enums {
                let derive = variants.get("derive").cloned().unwrap_or_else(|| "Debug, Clone".to_string());
                let template = self.rules.get("enum").cloned()
                    .unwrap_or_else(|| "#[derive({derive})]\npub enum {name} {{\n{variants}\n}}\n".to_string());
                
                let mut variants_str = String::new();
                let variant_template = self.rules.get("enum.variant").cloned()
                    .unwrap_or_else(|| "    {name},\n".to_string());
                
                for (key, value) in variants {
                    if key == "variants" {
                        for line in value.lines() {
                            let variant = line.trim();
                            if !variant.is_empty() {
                                let code = variant_template.replace("{name}", variant);
                                variants_str.push_str(&code);
                            }
                        }
                    }
                }
                
                let code = template
                    .replace("{derive}", &derive)
                    .replace("{name}", name)
                    .replace("{variants}", &variants_str);
                output.push_str(&code);
                output.push('\n');
            }
        }
        
        Ok(output)
    }
    
    fn generate_impls(&self, sections: &HashMap<String, Vec<(String, HashMap<String, String>)>>) -> Result<String, String> {
        let mut output = String::new();
        
        if let Some(impls) = sections.get("impl") {
            for (name, funcs) in impls {
                let template = self.rules.get("impl").cloned()
                    .unwrap_or_else(|| "impl {name} {{\n{functions}\n}}\n".to_string());
                let func_template = self.rules.get("impl.function").cloned()
                    .unwrap_or_else(|| "    pub fn {name}({params}) -> {return_type} {{\n        todo!()\n    }}\n".to_string());
                
                let mut functions_str = String::new();
                
                for (key, value) in funcs {
                    if key == "functions" {
                        for line in value.lines() {
                            if let Some((func_name, rest)) = line.split_once('(') {
                                let func_name = func_name.trim();
                                if let Some((params, rest2)) = rest.split_once(')') {
                                    let return_type = if let Some(ret) = rest2.split_once("->") {
                                        ret.1.trim()
                                    } else {
                                        "()"
                                    };
                                    let code = func_template
                                        .replace("{name}", func_name)
                                        .replace("{params}", params)
                                        .replace("{return_type}", return_type);
                                    functions_str.push_str(&code);
                                }
                            }
                        }
                    }
                }
                
                let code = template
                    .replace("{name}", name)
                    .replace("{functions}", &functions_str);
                output.push_str(&code);
                output.push('\n');
            }
        }
        
        Ok(output)
    }
    
    fn map_type(&self, sson_type: &str) -> String {
        self.type_mapping.get(sson_type)
            .cloned()
            .unwrap_or_else(|| "String".to_string())
    }
    
    fn render_template(&self, template: &str) -> String {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        template.replace("{timestamp}", &timestamp)
    }
}
