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
        
        if let Some(header) = self.templates.get("file_header") {
            output.push_str(&self.render_template(header));
            output.push('\n');
        }
        
        // Aggiungi import per _impl
        output.push_str("use super::_impl;\n\n");
        
        let sections = self.parse_sections(content);
        
        output.push_str(&self.generate_structs(&sections)?);
        output.push_str(&self.generate_enums(&sections)?);
        output.push_str(&self.generate_impls(&sections)?);
        
        if let Some(footer) = self.templates.get("file_footer") {
            output.push_str(&self.render_template(footer));
        }
        
        Ok(output)
    }
    
    pub fn generate_from_content(&self, content: &str) -> Result<String, String> {
        self.generate(content)
    }
    
    pub fn generate_to_file(&self, input_path: &Path, output_dir: &Path) -> Result<(), String> {
        let stem = input_path.file_stem().unwrap().to_string_lossy();
        let output_path = output_dir.join(format!("{}.rs", stem));
        let impl_path = output_dir.join(format!("{}_impl.rs", stem));
        
        // Genera il file principale
        let content = std::fs::read_to_string(input_path)
            .map_err(|e| e.to_string())?;
        let rust_code = self.generate_from_content(&content)?;
        std::fs::write(&output_path, rust_code)
            .map_err(|e| e.to_string())?;
        
        // Genera il file _impl.rs se non esiste
        if !impl_path.exists() {
            let sections = self.parse_sections(&content);
            let functions = self.extract_functions(&sections);
            let impl_code = generate_impl_skeleton(&stem, functions);
            std::fs::write(&impl_path, impl_code)
                .map_err(|e| e.to_string())?;
            println!("  📝 Creato skeleton impl: {:?}", impl_path);
        }
        
        Ok(())
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
                if in_section && !current_name.is_empty() {
                    result.entry(current_type.clone())
                        .or_insert_with(Vec::new)
                        .push((current_name.clone(), current_fields.clone()));
                }
                
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
        
        if in_section && !current_name.is_empty() {
            result.entry(current_type)
                .or_insert_with(Vec::new)
                .push((current_name, current_fields));
        }
        
        result
    }
    
    fn extract_functions(&self, sections: &HashMap<String, Vec<(String, HashMap<String, String>)>>) -> Vec<(String, String, bool)> {
        let mut functions = Vec::new();
        
        if let Some(impls) = sections.get("impl") {
            let mut impl_data: HashMap<String, HashMap<String, String>> = HashMap::new();
            
            for (name, funcs) in impls {
                let entry = impl_data.entry(name.clone()).or_insert_with(HashMap::new);
                for (k, v) in funcs {
                    entry.insert(k.clone(), v.clone());
                }
            }
            
            for (_, data) in impl_data {
                if let Some(functions_value) = data.get("functions") {
                    for func_name in functions_value.split(',').map(|s| s.trim()) {
                        let func_name = func_name.trim_matches('"');
                        if func_name.is_empty() {
                            continue;
                        }
                        
                        let params_key = format!("{}.params", func_name);
                        let static_key = format!("{}.is_static", func_name);
                        
                        let params = data.get(&params_key)
                            .or_else(|| data.get("params"))
                            .map(|s| s.clone())
                            .unwrap_or_else(|| "".to_string());
                        
                        let is_static = data.get(&static_key)
                            .or_else(|| data.get("is_static"))
                            .map(|s| s == "true")
                            .unwrap_or(false);
                        
                        functions.push((func_name.to_string(), params, is_static));
                    }
                }
            }
        }
        
        functions
    }
    
    fn generate_structs(&self, sections: &HashMap<String, Vec<(String, HashMap<String, String>)>>) -> Result<String, String> {
        let mut output = String::new();
        let mut struct_data: HashMap<String, HashMap<String, String>> = HashMap::new();
        
        if let Some(structs) = sections.get("struct") {
            for (name, fields) in structs {
                let entry = struct_data.entry(name.clone()).or_insert_with(HashMap::new);
                for (k, v) in fields {
                    entry.insert(k.clone(), v.clone());
                }
            }
            
            for (name, data) in struct_data {
                let derive = data.get("derive").cloned().unwrap_or_else(|| "Debug, Clone".to_string());
                let template = self.rules.get("struct").cloned()
                    .unwrap_or_else(|| "#[derive({derive})]\npub struct {name} {{\n{fields}\n}}\n".to_string());
                
                let mut fields_str = String::new();
                let field_template = self.rules.get("struct.field").cloned()
                    .unwrap_or_else(|| "    pub {name}: {type_},\n".to_string());
                
                for (field_name, field_type) in data {
                    if field_name != "derive" {
                        let rust_type = self.map_type(&field_type);
                        let code = field_template
                            .replace("{name}", &field_name)
                            .replace("{type_}", &rust_type);
                        fields_str.push_str(&code);
                    }
                }
                
                let code = template
                    .replace("{derive}", &derive)
                    .replace("{name}", &name)
                    .replace("{fields}", &fields_str);
                output.push_str(&code);
                output.push('\n');
            }
        }
        
        Ok(output)
    }
    
    fn generate_enums(&self, sections: &HashMap<String, Vec<(String, HashMap<String, String>)>>) -> Result<String, String> {
        let mut output = String::new();
        let mut generated_names = std::collections::HashSet::new();
        
        if let Some(enums) = sections.get("enum") {
            for (name, variants) in enums {
                if generated_names.contains(name) {
                    continue;
                }
                generated_names.insert(name.clone());
                
                let derive = variants.get("derive").cloned().unwrap_or_else(|| "Debug, Clone".to_string());
                let template = self.rules.get("enum").cloned()
                    .unwrap_or_else(|| "#[derive({derive})]\npub enum {name} {{\n{variants}\n}}\n".to_string());
                
                let mut variants_str = String::new();
                let variant_template = self.rules.get("enum.variant").cloned()
                    .unwrap_or_else(|| "    {name},\n".to_string());
                
                if let Some(variants_value) = variants.get("variants") {
                    for line in variants_value.lines() {
                        let variant = line.trim();
                        if !variant.is_empty() {
                            let code = variant_template.replace("{name}", variant);
                            variants_str.push_str(&code);
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
	    let mut impl_data: HashMap<String, HashMap<String, String>> = HashMap::new();
	    
	    if let Some(impls) = sections.get("impl") {
	        for (name, funcs) in impls {
	            let entry = impl_data.entry(name.clone()).or_insert_with(HashMap::new);
	            for (k, v) in funcs {
	                entry.insert(k.clone(), v.clone());
	            }
	        }
	        
	        for (name, data) in impl_data {
	            output.push_str(&format!("impl {} {{\n", name));
	            
	            if let Some(functions_value) = data.get("functions") {
	                for func_name in functions_value.split(',').map(|s| s.trim()) {
	                    let func_name = func_name.trim_matches('"');
	                    if func_name.is_empty() {
	                        continue;
	                    }
	                    
	                    let params = data.get(&format!("{}.params", func_name))
	                        .or_else(|| data.get("params"))
	                        .map(|s| s.as_str())
	                        .unwrap_or("");
	                    
	                    let return_type = data.get(&format!("{}.return_type", func_name))
	                        .or_else(|| data.get("return_type"))
	                        .map(|s| s.as_str())
	                        .unwrap_or("()");
	                    
	                    let is_static = data.get(&format!("{}.is_static", func_name))
	                        .or_else(|| data.get("is_static"))
	                        .map(|s| s == "true")
	                        .unwrap_or(false);
	                    
	                    if is_static {
	                        // Funzione statica (new, with_max_size)
	                        output.push_str(&format!(
	                            "    pub fn {}({}) -> {} {{\n        _impl::{}({})\n    }}\n",
	                            func_name, params, return_type,
	                            func_name, extract_params_for_call(params)
	                        ));
	                    } else {
	                        // Metodo con self (remember, recall)
	                        let remaining = if params.contains(',') {
	                            params.splitn(2, ',').nth(1).unwrap_or("").trim()
	                        } else {
	                            ""
	                        };
	                        
	                        let self_type = if params.contains("&mut self") { "&mut self" } else { "&self" };
	                        
	                        let self_decl = if remaining.is_empty() {
	                            format!("{}", self_type)
	                        } else {
	                            format!("{}, {}", self_type, remaining)
	                        };
	                        
	                        output.push_str(&format!(
	                            "    pub fn {}({}) -> {} {{\n        _impl::{}(self, {})\n    }}\n",
	                            func_name,
	                            self_decl,
	                            return_type,
	                            func_name,
	                            extract_params_for_call(remaining)
	                        ));
	                    }
	                }
	            }
	            
	            output.push_str("}\n\n");
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

// Helper per estrarre i nomi dei parametri per la chiamata
fn extract_params_for_call(params: &str) -> String {
    if params.is_empty() {
        return String::new();
    }
    
    let parts: Vec<&str> = params.split(',').map(|p| p.trim()).collect();
    let call_params: Vec<String> = parts.iter()
        .map(|p| {
            // Estrae solo il nome del parametro (senza tipo)
            let name = p.split(':').next().unwrap_or(p).trim();
            name.to_string()
        })
        .collect();
    
    call_params.join(", ")
}

// Genera lo skeleton per il file _impl.rs
fn generate_impl_skeleton(stem: &str, functions: Vec<(String, String, bool)>) -> String {
    let struct_name = to_pascal_case(stem);
    
    let mut functions_code = String::new();
    
    for (func_name, params, is_static) in functions {
        if is_static {
            let return_type = if func_name == "new" { struct_name.clone() } else { "Self".to_string() };
            functions_code.push_str(&format!(
                "pub fn {}({}) -> {} {{\n    todo!(\"Implementare {}::{}\")\n}}\n\n",
                func_name, params, return_type, struct_name, func_name
            ));
        } else {
            // Metodo con self
            let self_decl = if params.is_empty() {
			    "&mut self".to_string()
			} else {
			    format!("&mut self, {}", params)
			};
            functions_code.push_str(&format!(
                "pub fn {}({}) -> () {{\n    todo!(\"Implementare {}::{}\")\n}}\n\n",
                func_name, self_decl, struct_name, func_name
            ));
        }
    }
    
    format!(
        r#"//! Implementazioni reali per {struct_name}
//! Modifica le implementazioni qui.

use super::{stem}::{struct_name};

// ============================================
// IMPLEMENTAZIONI
// ============================================

{}
"#,
        functions_code
    )
}

// Helper per convertire snake_case in PascalCase
fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}
