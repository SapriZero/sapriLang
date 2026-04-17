//! Parser per file .aff di Hunspell

use std::collections::HashMap;
use std::fs;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct AffData {
    pub encoding: String,
    pub try_chars: String,
    pub map_rules: Vec<MapRule>,
    pub suffixes: Vec<SuffixRule>,
    pub prefixes: Vec<PrefixRule>,
    pub flag_to_suffixes: HashMap<String, Vec<AffixRule>>,
    pub flag_to_prefixes: HashMap<String, Vec<AffixRule>>,
}

#[derive(Debug, Clone)]
pub struct MapRule {
    pub from: char,
    pub to: Vec<char>,
}

#[derive(Debug, Clone)]
pub struct SuffixRule {
    pub flag: String,
    pub cross_product: bool,
    pub count: usize,
    pub rules: Vec<AffixRule>,
}

#[derive(Debug, Clone)]
pub struct PrefixRule {
    pub flag: String,
    pub cross_product: bool,
    pub count: usize,
    pub rules: Vec<AffixRule>,
}

#[derive(Debug, Clone)]
pub struct AffixRule {
    pub strip: String,
    pub add: String,
    pub condition: String,
    pub flags: Option<String>,
}

impl AffData {
    pub fn from_file(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::parse(&content)
    }
    
    pub fn parse(content: &str) -> Result<Self, String> {
        let mut encoding = "ISO8859-15".to_string();
        let mut try_chars = String::new();
        let mut map_rules = Vec::new();
        let mut suffixes: Vec<SuffixRule> = Vec::new();
        let mut prefixes: Vec<PrefixRule> = Vec::new();
        let mut flag_to_suffixes: HashMap<String, Vec<AffixRule>> = HashMap::new();
        let mut flag_to_prefixes: HashMap<String, Vec<AffixRule>> = HashMap::new();
        
        let sfx_header_re = Regex::new(r"^SFX\s+(\S+)\s+([YN])\s+(\d+)").unwrap();
        let pfx_header_re = Regex::new(r"^PFX\s+(\S+)\s+([YN])\s+(\d+)").unwrap();
        let sfx_rule_re = Regex::new(r"^SFX\s+\S+\s+(\S+)\s+(\S+)\s+(.*)$").unwrap();
        let pfx_rule_re = Regex::new(r"^PFX\s+\S+\s+(\S+)\s+(\S+)\s+(.*)$").unwrap();
        
        let mut current_suffix_flag: Option<String> = None;
        let mut current_prefix_flag: Option<String> = None;
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            
            if line.starts_with("SET") {
                encoding = line[4..].trim().to_string();
            } else if line.starts_with("TRY") {
                try_chars = line[4..].trim().to_string();
            } else if line.starts_with("MAP") {
                let parts: Vec<&str> = line[4..].trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    let from = parts[0].chars().next().unwrap();
                    let to: Vec<char> = parts[1].chars().collect();
                    map_rules.push(MapRule { from, to });
                }
            } else if let Some(caps) = sfx_header_re.captures(line) {
                let flag = caps[1].to_string();
                let cross_product = caps[2] == "Y";
                let count = caps[3].parse().unwrap_or(0);
                current_suffix_flag = Some(flag.clone());
                suffixes.push(SuffixRule { 
                    flag, 
                    cross_product, 
                    count, 
                    rules: Vec::new() 
                });
            } else if let Some(caps) = pfx_header_re.captures(line) {
                let flag = caps[1].to_string();
                let cross_product = caps[2] == "Y";
                let count = caps[3].parse().unwrap_or(0);
                current_prefix_flag = Some(flag.clone());
                prefixes.push(PrefixRule { 
                    flag, 
                    cross_product, 
                    count, 
                    rules: Vec::new() 
                });
            } else if let Some(caps) = sfx_rule_re.captures(line) {
                let strip = caps[1].to_string();
                let add = caps[2].to_string();
                let rest = caps[3].to_string();
                
                let (condition, flags) = if let Some(idx) = rest.find('/') {
                    (rest[..idx].to_string(), Some(rest[idx+1..].to_string()))
                } else {
                    (rest, None)
                };
                
                let rule = AffixRule { strip, add, condition, flags };
                
                if let Some(flag) = &current_suffix_flag {
                    if let Some(last) = suffixes.last_mut() {
                        last.rules.push(rule.clone());
                    }
                    flag_to_suffixes.entry(flag.clone()).or_insert_with(Vec::new).push(rule);
                }
            } else if let Some(caps) = pfx_rule_re.captures(line) {
                let strip = caps[1].to_string();
                let add = caps[2].to_string();
                let rest = caps[3].to_string();
                
                let (condition, flags) = if let Some(idx) = rest.find('/') {
                    (rest[..idx].to_string(), Some(rest[idx+1..].to_string()))
                } else {
                    (rest, None)
                };
                
                let rule = AffixRule { strip, add, condition, flags };
                
                if let Some(flag) = &current_prefix_flag {
                    if let Some(last) = prefixes.last_mut() {
                        last.rules.push(rule.clone());
                    }
                    flag_to_prefixes.entry(flag.clone()).or_insert_with(Vec::new).push(rule);
                }
            }
        }
        
        Ok(Self {
            encoding,
            try_chars,
            map_rules,
            suffixes,
            prefixes,
            flag_to_suffixes,
            flag_to_prefixes,
        })
    }
    
    /// Ottiene le regole di suffisso per un flag
    pub fn get_suffix_rules(&self, flag: &str) -> Vec<AffixRule> {
        self.flag_to_suffixes.get(flag).cloned().unwrap_or_default()
    }
    
    /// Ottiene le regole di prefisso per un flag
    pub fn get_prefix_rules(&self, flag: &str) -> Vec<AffixRule> {
        self.flag_to_prefixes.get(flag).cloned().unwrap_or_default()
    }
}
