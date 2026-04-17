//! Analisi morfologica funzionale
//! Usa sapri_diz per configurazione e vocabolario

pub mod verb;
pub mod noun;
pub mod adj;
pub mod export;

pub use verb::{VerbInfo, ConjugationType, Tense, Person, Number};
pub use noun::NounInfo;
pub use adj::AdjectiveInfo;

// Re-export da sapri_diz per comodità
pub use sapri_diz::{diz, load_diz, text, code, validate_name};

#[inline(always)]
pub fn eval_lazy<T, F1, F2>(condition: bool, then_fn: F1, else_fn: F2) -> T
where
    F1: FnOnce() -> T,
    F2: FnOnce() -> T,
{
    if condition { then_fn() } else { else_fn() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_lazy() {
        let result = eval_lazy(true, || 42, || 0);
        assert_eq!(result, 42);
        
        let result = eval_lazy(false, || 42, || 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_eval_lazy_with_closure() {
        let x = 10;
        let result = eval_lazy(x > 5, || x * 2, || x / 2);
        assert_eq!(result, 20);
    }

   /* #[test]
    fn test_import_from_diz() {
        // Verifica che i re-export funzionino
        let _ = load_diz();
        let _ = text::filter_words::LIST;
        let _ = code::structs::ALLOWED;
    } */
}
