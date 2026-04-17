//! Estrazione aggettivi in stile funzionale

use crate::eval_lazy;

#[derive(Debug, Clone)]
pub struct AdjectiveInfo {
    pub word: String,
    pub gender: Option<String>,
    pub number: Option<String>,
}

impl AdjectiveInfo {
    pub fn from_word(word: &str) -> Option<Self> {
        let word_lower = word.to_lowercase();
        
        let ends_with_o = word_lower.ends_with('o');
        let ends_with_a = word_lower.ends_with('a');
        let _ends_with_e = word_lower.ends_with('e');  // prefisso con _
        
        let gender = eval_lazy(
            ends_with_o,
            || Some("masculine".to_string()),
            || eval_lazy(
                ends_with_a,
                || Some("feminine".to_string()),
                || None
            )
        );
        
        let number = eval_lazy(
            word_lower.ends_with('i') || word_lower.ends_with('e'),
            || Some("plural".to_string()),
            || Some("singular".to_string())
        );
        
        Some(AdjectiveInfo {
            word: word.to_string(),
            gender,
            number,
        })
    }
}

pub fn extract_adjectives(words: &[String]) -> Vec<AdjectiveInfo> {
    words.iter()
        .filter_map(|w| AdjectiveInfo::from_word(w))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_masculine_adj() {
        let adj = AdjectiveInfo::from_word("bello").unwrap();
        assert_eq!(adj.gender, Some("masculine".to_string()));
        assert_eq!(adj.number, Some("singular".to_string()));
    }

    #[test]
    fn test_feminine_adj() {
        let adj = AdjectiveInfo::from_word("bella").unwrap();
        assert_eq!(adj.gender, Some("feminine".to_string()));
        assert_eq!(adj.number, Some("singular".to_string()));
    }

    #[test]
    fn test_plural_adj() {
        let adj = AdjectiveInfo::from_word("belli").unwrap();
        // "belli" termina con 'i' → plurale
        assert_eq!(adj.number, Some("plural".to_string()));
        // Il genere non viene rilevato perché non finisce con 'o' o 'a'
        assert_eq!(adj.gender, None);
    }

    #[test]
    fn test_extract_adjectives() {
        let words = vec![
            "bello".to_string(),
            "bella".to_string(),
            "belli".to_string(),
            "casa".to_string(),
            "correre".to_string(),
        ];
        let adjs = extract_adjectives(&words);
        // Tutte le parole che finiscono con vocale vengono considerate
        // "casa" e "correre" finiscono con 'a' e 'e'
        assert_eq!(adjs.len(), 5);
    }
}
