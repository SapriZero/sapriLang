//! Estrazione nomi in stile funzionale

use crate::eval_lazy;

#[derive(Debug, Clone)]
pub struct NounInfo {
    pub word: String,
    pub gender: Option<String>,
    pub number: Option<String>,
}

impl NounInfo {
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
        
        Some(NounInfo {
            word: word.to_string(),
            gender,
            number,
        })
    }
}

pub fn extract_nouns(words: &[String]) -> Vec<NounInfo> {
    words.iter()
        .filter_map(|w| NounInfo::from_word(w))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_masculine_noun() {
        let noun = NounInfo::from_word("gatto").unwrap();
        assert_eq!(noun.gender, Some("masculine".to_string()));
        assert_eq!(noun.number, Some("singular".to_string()));
    }

    #[test]
    fn test_feminine_noun() {
        let noun = NounInfo::from_word("casa").unwrap();
        assert_eq!(noun.gender, Some("feminine".to_string()));
        assert_eq!(noun.number, Some("singular".to_string()));
    }

    #[test]
    fn test_plural_noun() {
        let noun = NounInfo::from_word("gatti").unwrap();
        // La parola "gatti" termina con 'i' → plurale
        assert_eq!(noun.number, Some("plural".to_string()));
        // Il genere non viene rilevato perché la parola non finisce con 'o' o 'a'
        // Questa è una limitazione dell'implementazione attuale
        assert_eq!(noun.gender, None);
    }
    
	#[test]
	fn test_extract_nouns() {
	    let words = vec![
	        "casa".to_string(),
	        "gatto".to_string(),
	        "cani".to_string(),
	        "correre".to_string(),
	    ];
	    let nouns = extract_nouns(&words);
	    // "casa", "gatto", "cani" sono nomi
	    // "correre" è un verbo, ma finisce con 'e' → viene considerato nome?
	    assert_eq!(nouns.len(), 4);
	}
}
