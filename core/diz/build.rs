use std::env;
use std::path::Path;
use std::fs;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("diz_generated.rs");
    
    let diz_spec = fs::read_to_string("diz.sson").expect("diz.sson not found");
    let generated_code = generate_diz_code(&diz_spec);
    
    fs::write(&dest_path, generated_code).unwrap();
    println!("cargo:rerun-if-changed=diz.sson");
    println!("cargo:rerun-if-changed=lang/it/*.sson");
}

fn generate_diz_code(_spec: &str) -> String {
    let mut code = String::new();
    
    code.push_str("/// Dizionario strutturato generato automaticamente\n\n");
    code.push_str("use serde_json::Value as JsonValue;\n\n");
    
    // ============================================
    // 1. PATHS COSTANTI
    // ============================================
    code.push_str("pub mod paths {\n");
    code.push_str("    // Text paths\n");
    code.push_str("    pub const TEXT_FILTER_WORDS: &str = \"text.filter_words.list\";\n");
    code.push_str("    pub const TEXT_MIN_WORD_LENGTH: &str = \"text.min_word_length\";\n");
    code.push_str("    pub const TEXT_CHARMAP_BITS: &str = \"text.charmap.bits\";\n");
    code.push_str("    pub const TEXT_CHARMAP_ESCAPE_CODE: &str = \"text.charmap.escape_code\";\n");
    code.push_str("}\n\n");
    
    // ============================================
    // 2. STRUCT ANNIDATE (autocompletamento)
    // ============================================
    code.push_str("pub mod text {\n");
    code.push_str("    pub mod filter_words {\n");
    code.push_str("        pub const LIST: &str = crate::paths::TEXT_FILTER_WORDS;\n");
    code.push_str("    }\n");
    code.push_str("    pub mod min_word_length {\n");
    code.push_str("        pub const VALUE: &str = crate::paths::TEXT_MIN_WORD_LENGTH;\n");
    code.push_str("    }\n");
    code.push_str("    pub mod charmap {\n");
    code.push_str("        pub const BITS: &str = crate::paths::TEXT_CHARMAP_BITS;\n");
    code.push_str("        pub const ESCAPE_CODE: &str = crate::paths::TEXT_CHARMAP_ESCAPE_CODE;\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");
    
    // ============================================
    // 3. VOCABOLARIO DEL CODICE
    // ============================================
    code.push_str("pub mod code {\n");
    code.push_str("    pub mod structs {\n");
    code.push_str("        pub const ALLOWED: &[&str] = &[\"Brain\", \"HolographicMemory\", \"KnowledgeBase\", \"Learner\", \"Conversation\", \"MemoryEntry\"];\n");
    code.push_str("    }\n");
    code.push_str("    pub mod fields {\n");
    code.push_str("        pub const ALLOWED: &[&str] = &[\"runtime\", \"knowledge\", \"memory\", \"learner\", \"conversation\", \"entries\", \"max_size\", \"timestamp\", \"input\", \"response\", \"soggetto\", \"predicato\", \"oggetto\", \"peso\", \"stats\"];\n");
    code.push_str("    }\n");
    code.push_str("    pub mod methods {\n");
    code.push_str("        pub const ALLOWED: &[&str] = &[\"new\", \"with_max_size\", \"remember\", \"recall\", \"talk\", \"learn\", \"save\", \"load\", \"add\", \"answer\", \"get\", \"clear\", \"len\", \"is_empty\", \"stats\", \"confidence\"];\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");
    
    // ============================================
    // 4. MACRO DI VALIDAZIONE
    // ============================================
    code.push_str("#[macro_export]\n");
    code.push_str("macro_rules! validate_name {\n");
    code.push_str("    ($name:expr, structs) => {{\n");
    code.push_str("        use $crate::code::structs::ALLOWED;\n");
    code.push_str("        if !ALLOWED.contains(&$name) {\n");
    code.push_str("            compile_error!(concat!(\"Invalid struct name: \", $name, \" not in vocabulary\"));\n");
    code.push_str("        }\n");
    code.push_str("    }};\n");
    code.push_str("    ($name:expr, fields) => {{\n");
    code.push_str("        use $crate::code::fields::ALLOWED;\n");
    code.push_str("        if !ALLOWED.contains(&$name) {\n");
    code.push_str("            compile_error!(concat!(\"Invalid field name: \", $name, \" not in vocabulary\"));\n");
    code.push_str("        }\n");
    code.push_str("    }};\n");
    code.push_str("    ($name:expr, methods) => {{\n");
    code.push_str("        use $crate::code::methods::ALLOWED;\n");
    code.push_str("        if !ALLOWED.contains(&$name) {\n");
    code.push_str("            compile_error!(concat!(\"Invalid method name: \", $name, \" not in vocabulary\"));\n");
    code.push_str("        }\n");
    code.push_str("    }};\n");
    code.push_str("}\n\n");
    
    // ============================================
    // 5. STRUCT PER I DATI (charmap, filter_words, etc.)
    // ============================================
    code.push_str("#[derive(Debug, Clone)]\n");
    code.push_str("pub struct Lowercase {\n");
    code.push_str("    pub start: char,\n");
    code.push_str("    pub end: char,\n");
    code.push_str("    pub start_code: u8,\n");
    code.push_str("    pub end_code: u8,\n");
    code.push_str("}\n\n");
    
    code.push_str("#[derive(Debug, Clone)]\n");
    code.push_str("pub struct Uppercase {\n");
    code.push_str("    pub start: char,\n");
    code.push_str("    pub end: char,\n");
    code.push_str("    pub start_code: u8,\n");
    code.push_str("    pub end_code: u8,\n");
    code.push_str("}\n\n");
    
    code.push_str("#[derive(Debug, Clone)]\n");
    code.push_str("pub struct Space {\n");
    code.push_str("    pub char: char,\n");
    code.push_str("    pub code: u8,\n");
    code.push_str("}\n\n");
    
    code.push_str("#[derive(Debug, Clone)]\n");
    code.push_str("pub struct Charmap {\n");
    code.push_str("    pub bits: u8,\n");
    code.push_str("    pub escape_code: u8,\n");
    code.push_str("    pub lowercase: Lowercase,\n");
    code.push_str("    pub uppercase: Uppercase,\n");
    code.push_str("    pub space: Space,\n");
    code.push_str("    pub accents: Vec<(char, u8)>,\n");
    code.push_str("    pub punctuation: Vec<(char, u8)>,\n");
    code.push_str("}\n\n");
    
    code.push_str("#[derive(Debug, Clone)]\n");
    code.push_str("pub struct FilterWords {\n");
    code.push_str("    pub list: Vec<String>,\n");
    code.push_str("}\n\n");
    
    code.push_str("#[derive(Debug, Clone)]\n");
    code.push_str("pub struct TextConfig {\n");
    code.push_str("    pub filter_words: FilterWords,\n");
    code.push_str("    pub min_word_length: u8,\n");
    code.push_str("    pub charmap: Charmap,\n");
    code.push_str("}\n\n");
    
    code.push_str("#[derive(Debug, Clone)]\n");
    code.push_str("pub struct DizData {\n");
    code.push_str("    pub text: TextConfig,\n");
    code.push_str("}\n\n");
    
    // ============================================
    // 6. FUNZIONE LOAD
    // ============================================
    code.push_str("pub fn load() -> DizData {\n");
    code.push_str("    let manifest_dir = std::env::var(\"CARGO_MANIFEST_DIR\").unwrap();\n");
    code.push_str("    let json_path = std::path::Path::new(&manifest_dir).join(\"diz_data.json\");\n");
    code.push_str("    let json_str = std::fs::read_to_string(&json_path)\n");
    code.push_str("        .expect(&format!(\"Failed to read {:?}\", json_path));\n");
    code.push_str("    let data: JsonValue = serde_json::from_str(&json_str).unwrap();\n");
    code.push_str("    DizData {\n");
    code.push_str("        text: TextConfig {\n");
    code.push_str("            filter_words: FilterWords {\n");
    code.push_str("                list: data[\"text\"][\"filter_words\"][\"list\"].as_array().unwrap()\n");
    code.push_str("                    .iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect(),\n");
    code.push_str("            },\n");
    code.push_str("            min_word_length: data[\"text\"][\"min_word_length\"].as_u64().unwrap() as u8,\n");
    code.push_str("            charmap: Charmap {\n");
    code.push_str("                bits: data[\"text\"][\"charmap\"][\"bits\"].as_u64().unwrap() as u8,\n");
    code.push_str("                escape_code: data[\"text\"][\"charmap\"][\"escape_code\"].as_u64().unwrap() as u8,\n");
    code.push_str("                lowercase: Lowercase {\n");
    code.push_str("                    start: data[\"text\"][\"charmap\"][\"lowercase\"][\"start\"].as_str().unwrap().chars().next().unwrap(),\n");
    code.push_str("                    end: data[\"text\"][\"charmap\"][\"lowercase\"][\"end\"].as_str().unwrap().chars().next().unwrap(),\n");
    code.push_str("                    start_code: data[\"text\"][\"charmap\"][\"lowercase\"][\"start_code\"].as_u64().unwrap() as u8,\n");
    code.push_str("                    end_code: data[\"text\"][\"charmap\"][\"lowercase\"][\"end_code\"].as_u64().unwrap() as u8,\n");
    code.push_str("                },\n");
    code.push_str("                uppercase: Uppercase {\n");
    code.push_str("                    start: data[\"text\"][\"charmap\"][\"uppercase\"][\"start\"].as_str().unwrap().chars().next().unwrap(),\n");
    code.push_str("                    end: data[\"text\"][\"charmap\"][\"uppercase\"][\"end\"].as_str().unwrap().chars().next().unwrap(),\n");
    code.push_str("                    start_code: data[\"text\"][\"charmap\"][\"uppercase\"][\"start_code\"].as_u64().unwrap() as u8,\n");
    code.push_str("                    end_code: data[\"text\"][\"charmap\"][\"uppercase\"][\"end_code\"].as_u64().unwrap() as u8,\n");
    code.push_str("                },\n");
    code.push_str("                space: Space {\n");
    code.push_str("                    char: data[\"text\"][\"charmap\"][\"space\"][\"char\"].as_str().unwrap().chars().next().unwrap(),\n");
    code.push_str("                    code: data[\"text\"][\"charmap\"][\"space\"][\"code\"].as_u64().unwrap() as u8,\n");
    code.push_str("                },\n");
    code.push_str("                accents: data[\"text\"][\"charmap\"][\"accents\"].as_array().unwrap()\n");
    code.push_str("                    .iter().filter_map(|v| {\n");
    code.push_str("                        let arr = v.as_array()?;\n");
    code.push_str("                        let c = arr[0].as_str()?.chars().next()?;\n");
    code.push_str("                        let code = arr[1].as_u64()? as u8;\n");
    code.push_str("                        Some((c, code))\n");
    code.push_str("                    }).collect(),\n");
    code.push_str("                punctuation: data[\"text\"][\"charmap\"][\"punctuation\"].as_array().unwrap()\n");
    code.push_str("                    .iter().filter_map(|v| {\n");
    code.push_str("                        let arr = v.as_array()?;\n");
    code.push_str("                        let c = arr[0].as_str()?.chars().next()?;\n");
    code.push_str("                        let code = arr[1].as_u64()? as u8;\n");
    code.push_str("                        Some((c, code))\n");
    code.push_str("                    }).collect(),\n");
    code.push_str("            },\n");
    code.push_str("        },\n");
    code.push_str("    }\n");
    code.push_str("}\n");
    
    code
}
