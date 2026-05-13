//! Utility varie per l'AI

use std::time::{SystemTime, UNIX_EPOCH};

/// Ottiene timestamp corrente in secondi
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Normalizza un testo (lowercase, rimuove punteggiatura, etc.)
pub fn normalize_text(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect()
}

/// Estrae parole da un testo
pub fn extract_words(text: &str) -> Vec<String> {
    normalize_text(text)
        .split_whitespace()
        .filter(|w| w.len() > 2)
        .map(|w| w.to_string())
        .collect()
}

/// Calcola similarità tra due stringhe (versione semplice)
pub fn string_similarity(a: &str, b: &str) -> f64 {
    let a_norm = normalize_text(a);
    let b_norm = normalize_text(b);

    if a_norm.is_empty() && b_norm.is_empty() {
        return 1.0;
    }

    let common = a_norm
        .split_whitespace()
        .filter(|w| b_norm.split_whitespace().any(|bw| bw == w))
        .count();

    let total = a_norm.split_whitespace().count().max(b_norm.split_whitespace().count());

    if total == 0 { 0.0 } else { common as f64 / total as f64 }
}
