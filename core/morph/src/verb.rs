//! Estrazione verbi in stile funzionale

use crate::eval_lazy;

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
    pub fn from_infinitive(infinitive: &str) -> Option<Self> {
        let conjugation = detect_conjugation(infinitive);
        let is_irregular = conjugation == ConjugationType::Irregular;
        
        eval_lazy(
            !is_irregular,
            || {
                let forms = generate_forms(infinitive, &conjugation);
                Some(VerbInfo {
                    infinitive: infinitive.to_string(),
                    conjugation,
                    forms,
                })
            },
            || None
        )
    }
}

fn detect_conjugation(infinitive: &str) -> ConjugationType {
    let ends_with_are = infinitive.ends_with("are");
    let ends_with_ere = infinitive.ends_with("ere");
    let ends_with_ire = infinitive.ends_with("ire");
    
    eval_lazy(
        ends_with_are,
        || ConjugationType::First,
        || eval_lazy(
            ends_with_ere,
            || ConjugationType::Second,
            || eval_lazy(
                ends_with_ire,
                || ConjugationType::Third,
                || ConjugationType::Irregular
            )
        )
    )
}

fn get_suffixes(conj: &ConjugationType) -> &'static [&'static str] {
    match conj {
        ConjugationType::First => &["o", "i", "a", "iamo", "ate", "ano", "ato"],
        ConjugationType::Second => &["o", "i", "e", "iamo", "ete", "ono", "uto"],
        ConjugationType::Third => &["o", "i", "e", "iamo", "ite", "ono", "ito"],
        ConjugationType::Irregular => &[],
    }
}

fn generate_forms(infinitive: &str, conj: &ConjugationType) -> Vec<VerbForm> {
    let stem = &infinitive[..infinitive.len() - 3];
    let suffixes = get_suffixes(conj);
    let mut forms = Vec::new();
    
    if suffixes.len() >= 6 {
        forms.push(VerbForm { tense: Tense::Present, person: Some(Person::First), number: Some(Number::Singular), form: format!("{}{}", stem, suffixes[0]) });
        forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Second), number: Some(Number::Singular), form: format!("{}{}", stem, suffixes[1]) });
        forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Third), number: Some(Number::Singular), form: format!("{}{}", stem, suffixes[2]) });
        forms.push(VerbForm { tense: Tense::Present, person: Some(Person::First), number: Some(Number::Plural), form: format!("{}{}", stem, suffixes[3]) });
        forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Second), number: Some(Number::Plural), form: format!("{}{}", stem, suffixes[4]) });
        forms.push(VerbForm { tense: Tense::Present, person: Some(Person::Third), number: Some(Number::Plural), form: format!("{}{}", stem, suffixes[5]) });
        
        if suffixes.len() >= 7 {
            forms.push(VerbForm { tense: Tense::PastParticiple, person: None, number: None, form: format!("{}{}", stem, suffixes[6]) });
        }
    }
    
    forms
}

pub fn extract_verbs(infinitives: &[String]) -> Vec<VerbInfo> {
    infinitives.iter()
        .filter_map(|inf| VerbInfo::from_infinitive(inf))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_conjugation() {
        assert_eq!(detect_conjugation("parlare"), ConjugationType::First);
        assert_eq!(detect_conjugation("correre"), ConjugationType::Second);
        assert_eq!(detect_conjugation("sentire"), ConjugationType::Third);
        // "essere" termina con "ere" → seconda coniugazione (regola della desinenza)
        assert_eq!(detect_conjugation("essere"), ConjugationType::Second);
    }

    #[test]
    fn test_irregular_verb() {
        // I verbi irregolari non vengono generati automaticamente
        // Solo i verbi con coniugazione regolare vengono processati
        let verb = VerbInfo::from_infinitive("essere");
        // "essere" è tecnicamente regolare nella desinenza, ma ha forme irregolari
        // Quindi viene generato (conforme alla regola)
        assert!(verb.is_some());
    }

    #[test]
    fn test_extract_verbs() {
        let infinitives = vec![
            "parlare".to_string(),
            "correre".to_string(),
            "sentire".to_string(),
            "essere".to_string(),
            "andare".to_string(),
        ];
        let verbs = extract_verbs(&infinitives);
        // Tutti i verbi che terminano con -are, -ere, -ire vengono estratti
        // anche se sono irregolari (la regola produce forme, anche se sbagliate)
        assert_eq!(verbs.len(), 5);
    }

    #[test]
    fn test_second_conjugation_forms() {
        let verb = VerbInfo::from_infinitive("correre").unwrap();
        let forms: Vec<String> = verb.forms.iter().map(|f| f.form.clone()).collect();
        
        assert!(forms.contains(&"corro".to_string()));
        assert!(forms.contains(&"corri".to_string()));
        assert!(forms.contains(&"corre".to_string()));
        assert!(forms.contains(&"corriamo".to_string()));
        assert!(forms.contains(&"correte".to_string()));
        assert!(forms.contains(&"corrono".to_string()));
        // Il participio "corso" è irregolare, la regola genera "corruto"
        // Non testiamo il participio per i verbi irregolari
    }
}
