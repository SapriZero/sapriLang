//! Estrazione verbi e coniugazioni

use crate::{DicEntry, AffData, AffixRule};

#[derive(Debug, Clone, PartialEq)]
pub enum ConjugationType {
    First,   // -are
    Second,  // -ere
    Third,   // -ire
    Irregular,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tense {
    Infinitive,
    Present,
    Imperfect,
    Future,
    PastParticiple,
    Gerund,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Debug, Clone)]
pub struct VerbForm {
    pub tense: Tense,
    pub person: Option<Person>,
    pub number: Option<Number>,
    pub form: String,
}

#[derive(Debug, Clone)]
pub struct VerbInfo {
    pub infinitive: String,
    pub conjugation: ConjugationType,
    pub forms: Vec<VerbForm>,
}

impl VerbInfo {
    pub fn from_entry(entry: &DicEntry, aff: &AffData) -> Option<Self> {
        // Verifica che sia un verbo
        let is_verb = entry.flags.contains(&"V".to_string());
        if !is_verb {
            return None;
        }
        
        let infinitive = entry.word.clone();
        let conjugation = detect_conjugation(&infinitive);
        let forms = generate_forms(&infinitive, conjugation, aff);
        
        Some(VerbInfo {
            infinitive,
            conjugation,
            forms,
        })
    }
}

fn detect_conjugation(infinitive: &str) -> ConjugationType {
    if infinitive.ends_with("are") {
        ConjugationType::First
    } else if infinitive.ends_with("ere") {
        ConjugationType::Second
    } else if infinitive.ends_with("ire") {
        ConjugationType::Third
    } else {
        ConjugationType::Irregular
    }
}

fn generate_forms(infinitive: &str, conj: ConjugationType, _aff: &AffData) -> Vec<VerbForm> {
    let mut forms = Vec::new();
    let stem = &infinitive[..infinitive.len() - 3];
    
    match conj {
        ConjugationType::First => {
            // Presente
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::First), number: Some(Number::Singular), form: format!("{}o", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Second), number: Some(Number::Singular), form: format!("{}i", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Third), number: Some(Number::Singular), form: format!("{}a", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::First), number: Some(Number::Plural), form: format!("{}iamo", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Second), number: Some(Number::Plural), form: format!("{}ate", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Third), number: Some(Number::Plural), form: format!("{}ano", stem) });
            
            // Participio passato
            forms.push(VerbForm { tense: Tense::PastParticiple, person: None, number: None, form: format!("{}ato", stem) });
        }
        ConjugationType::Second => {
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::First), number: Some(Number::Singular), form: format!("{}o", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Second), number: Some(Number::Singular), form: format!("{}i", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Third), number: Some(Number::Singular), form: format!("{}e", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::First), number: Some(Number::Plural), form: format!("{}iamo", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Second), number: Some(Number::Plural), form: format!("{}ete", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Third), number: Some(Number::Plural), form: format!("{}ono", stem) });
            
            forms.push(VerbForm { tense: Tense::PastParticiple, person: None, number: None, form: format!("{}uto", stem) });
        }
        ConjugationType::Third => {
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::First), number: Some(Number::Singular), form: format!("{}o", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Second), number: Some(Number::Singular), form: format!("{}i", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Third), number: Some(Number::Singular), form: format!("{}e", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::First), number: Some(Number::Plural), form: format!("{}iamo", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Second), number: Some(Number::Plural), form: format!("{}ite", stem) });
            forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Third), number: Some(Number::Plural), form: format!("{}ono", stem) });
            
            forms.push(VerbForm { tense: Tense::PastParticiple, person: None, number: None, form: format!("{}ito", stem) });
        }
        ConjugationType::Irregular => {
            forms.push(VerbForm { tense: Tense::Infinitive, person: None, number: None, form: infinitive.to_string() });
        }
    }
    
    forms
}

pub fn extract(entries: &[DicEntry], aff: &AffData) -> Vec<VerbInfo> {
    entries.iter()
        .filter_map(|e| VerbInfo::from_entry(e, aff))
        .collect()
}
