// wiki_extract/src/dic_parser.rs
use crate::flag_map::get_flag_map; 
use std::fs;

pub struct DicEntry {
    pub word: String,
    pub flags: Vec<char>,
    pub pos: Option<String>,
    pub gender: Option<String>,
    pub number: Option<String>,
    pub verb_conjugation: Option<String>,
}

pub fn parse_dic(path: &str) -> Result<Vec<DicEntry>, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let flag_map = get_flag_map();
    let mut entries = Vec::new();
    let mut lines = content.lines();
    
    // Prima riga: numero di parole (ignora)
    lines.next();
    
    for line in lines {
        let line = line.trim();
        if line.is_empty() { continue; }
        
        let (word, flags_str) = if let Some(idx) = line.find('/') {
            (line[..idx].to_string(), line[idx+1..].to_string())
        } else {
            (line.to_string(), String::new())
        };
        
        let flags: Vec<char> = flags_str.chars().collect();
        let mut pos = None;
        let mut gender = None;
        let mut number = None;
        let mut verb_conjugation = None;
        
        for &flag in &flags {
            match flag {
                'A' => { pos = Some("verb".to_string()); verb_conjugation = Some("are".to_string()); }
                'B' => { pos = Some("verb".to_string()); verb_conjugation = Some("ere".to_string()); }
                'C' => { pos = Some("verb".to_string()); verb_conjugation = Some("ire".to_string()); }
                'N' => pos = Some("noun".to_string()),
                'G' | 'H' | 'O' | 'R' | 'W' => pos = Some("adjective".to_string()),
                'I' | 'Y' => pos = Some("adverb".to_string()),
                'Q' => { pos = Some("noun".to_string()); gender = Some("feminine".to_string()); }
                'S' => { pos = Some("noun".to_string()); gender = Some("masculine".to_string()); }
                'T' | 'q' => number = Some("singular".to_string()),
                'V' | 'p' => number = Some("plural".to_string()),
                'U' | 's' => gender = Some("feminine".to_string()),
                'W' | 'r' => gender = Some("masculine".to_string()),
                _ => {}
            }
        }
        
        entries.push(DicEntry {
            word,
            flags,
            pos,
            gender,
            number,
            verb_conjugation,
        });
    }
    
    Ok(entries)
}
