//! Salvataggio e caricamento dati

use std::collections::HashMap;
use std::fs;
use std::io::{BufWriter, Write};

pub fn save_words(words: &HashMap<String, u32>, path: &str) -> Result<(), String> {
    let mut file = BufWriter::new(fs::File::create(path).map_err(|e| e.to_string())?);
    
    writeln!(file, "# Parole italiane estratte da Wikipedia").map_err(|e| e.to_string())?;
    writeln!(file, "# formato: parola peso").map_err(|e| e.to_string())?;
    writeln!(file).map_err(|e| e.to_string())?;
    
    let mut words_vec: Vec<(&String, &u32)> = words.iter().collect();
    words_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    for (word, count) in words_vec.iter().take(100000) {
        writeln!(file, "{} {}", word, count).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

pub fn save_verbs(verbs: &HashMap<String, u32>, path: &str) -> Result<(), String> {
    let mut file = BufWriter::new(fs::File::create(path).map_err(|e| e.to_string())?);
    
    writeln!(file, "# Verbi italiani estratti da Wikipedia").map_err(|e| e.to_string())?;
    writeln!(file, "# formato: verbo frequenza coniugazione").map_err(|e| e.to_string())?;
    writeln!(file).map_err(|e| e.to_string())?;
    
    let mut verbs_vec: Vec<(&String, &u32)> = verbs.iter().collect();
    verbs_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    for (verb, count) in verbs_vec.iter().take(10000) {
        let conj = get_conjugation(verb);
        writeln!(file, "{} {} {}", verb, count, conj).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

fn get_conjugation(verb: &str) -> &'static str {
    if verb.ends_with("are") { "are" }
    else if verb.ends_with("ere") { "ere" }
    else if verb.ends_with("ire") { "ire" }
    else { "irregolare" }
}

pub fn save_nouns(nouns: &HashMap<String, u32>, path: &str) -> Result<(), String> {
    let mut file = BufWriter::new(fs::File::create(path).map_err(|e| e.to_string())?);
    
    writeln!(file, "# Nomi italiani estratti da Wikipedia").map_err(|e| e.to_string())?;
    writeln!(file, "# formato: nome frequenza genere").map_err(|e| e.to_string())?;
    writeln!(file).map_err(|e| e.to_string())?;
    
    let mut nouns_vec: Vec<(&String, &u32)> = nouns.iter().collect();
    nouns_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    for (noun, count) in nouns_vec.iter().take(50000) {
        let gender = if noun.ends_with('a') { "f" }
                     else if noun.ends_with('o') { "m" }
                     else { "?" };
        writeln!(file, "{} {} {}", noun, count, gender).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

pub fn save_checkpoint(output_dir: &str, page_count: u64, byte_pos: u64, title: &str,
                       words: &HashMap<String, u32>,
                       verbs: &HashMap<String, u32>,
                       nouns: &HashMap<String, u32>) -> Result<(), String> {
    use crate::checkpoint::Checkpoint;
    
    let checkpoint = Checkpoint {
        page_count,
        byte_position: byte_pos,
        last_page_title: title.to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        word_counts: words.iter().map(|(k, v)| (k.clone(), *v)).collect(),
        verb_counts: verbs.iter().map(|(k, v)| (k.clone(), *v)).collect(),
        noun_counts: nouns.iter().map(|(k, v)| (k.clone(), *v)).collect(),
    };
    
    checkpoint.save(&format!("{}/checkpoint.json", output_dir))
}

pub fn load_checkpoint_from_dir(output_dir: &str) -> Result<crate::checkpoint::Checkpoint, String> {
    crate::checkpoint::Checkpoint::load(&format!("{}/checkpoint.json", output_dir))
}

pub fn load_checkpoint() -> Result<crate::checkpoint::Checkpoint, String> {
    crate::checkpoint::Checkpoint::load("checkpoint.json")
}
