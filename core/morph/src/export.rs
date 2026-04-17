//! Esportazione in formato .sson

use crate::{VerbInfo, NounInfo, AdjectiveInfo};
use std::fs;

pub fn export_verbs(verbs: &[VerbInfo], output_path: &str) {
    let mut content = String::new();
    content.push_str("# Verbi italiani\n\n");
    
    for verb in verbs {
        content.push_str(&format!("[verb.{}]\n", verb.infinitive));
        content.push_str(&format!("conjugation: {:?}\n", verb.conjugation));
        content.push_str("forms:\n");
        
        for form in &verb.forms {
            content.push_str(&format!("  - tense: {:?}, form: \"{}\"\n", form.tense, form.form));
            if let Some(p) = &form.person {
                content.push_str(&format!("    person: {:?}\n", p));
            }
            if let Some(n) = &form.number {
                content.push_str(&format!("    number: {:?}\n", n));
            }
        }
        content.push('\n');
    }
    
    fs::write(output_path, content).unwrap();
}

pub fn export_nouns(nouns: &[NounInfo], output_path: &str) {
    let mut content = String::new();
    content.push_str("# Nomi italiani\n\n");
    
    for noun in nouns {
        content.push_str(&format!("[noun.{}]\n", noun.word));
        if let Some(g) = &noun.gender {
            content.push_str(&format!("gender: {}\n", g));
        }
        if let Some(n) = &noun.number {
            content.push_str(&format!("number: {}\n", n));
        }
        content.push('\n');
    }
    
    fs::write(output_path, content).unwrap();
}

pub fn export_adjectives(adjectives: &[AdjectiveInfo], output_path: &str) {
    let mut content = String::new();
    content.push_str("# Aggettivi italiani\n\n");
    
    for adj in adjectives {
        content.push_str(&format!("[adj.{}]\n", adj.word));
        if let Some(g) = &adj.gender {
            content.push_str(&format!("gender: {}\n", g));
        }
        if let Some(n) = &adj.number {
            content.push_str(&format!("number: {}\n", n));
        }
        content.push('\n');
    }
    
    fs::write(output_path, content).unwrap();
}
