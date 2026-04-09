//! Estensione di TokenStrategy per supporto bucket 65535

use crate::scanner::TokenStrategy;

/// Estensione di TokenStrategy con supporto bucket
pub trait BucketStrategy: TokenStrategy {
    /// Calcola l'indice bucket (0-65535) basato sui primi due byte
    fn bucket_index(word: &[u8]) -> usize {
        let b1 = word.get(0).copied().unwrap_or(0) as usize;
        let b2 = word.get(1).copied().unwrap_or(0) as usize;
        (b1 << 8) | b2
    }

    /// Versione alternativa che usa hashing per distribuzione uniforme
    fn bucket_index_hash(word: &[u8]) -> usize {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        (hasher.finish() & 0xFFFF) as usize  // primi 16 bit
    }
}

// Implementa automaticamente per tutti i tipi che implementano TokenStrategy
impl<T: TokenStrategy> BucketStrategy for T {}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::scanner::{StandardStrategy, ExtendedStrategy};

    #[test]
    fn test_bucket_index() {
        // Test con parole semplici
        assert_eq!(StandardStrategy::bucket_index(b"ab"), (b'a' as usize) << 8 | (b'b' as usize));
        assert_eq!(StandardStrategy::bucket_index(b"a"), (b'a' as usize) << 8);
        assert_eq!(StandardStrategy::bucket_index(b""), 0);

        // Verifica range 0-65535
        let idx = StandardStrategy::bucket_index(b"\xFF\xFF");
        assert_eq!(idx, 0xFFFF);
    }

    #[test]
    fn test_bucket_index_hash() {
        let idx1 = StandardStrategy::bucket_index_hash(b"hello");
        let idx2 = StandardStrategy::bucket_index_hash(b"world");

        assert!(idx1 < 65536);
        assert!(idx2 < 65536);
        assert_ne!(idx1, idx2);  // Probabilmente diversi
    }
}
*/
