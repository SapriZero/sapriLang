//! Definizione header a bit variabili

/// Header a lunghezza variabile (2-7 bit)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub bits: String,
    pub pattern: String,
    pub length: usize,
}

/// Trait per le operazioni sull'header
pub trait HeaderOps {
    fn new(bits: &str) -> Self;
    fn bits(&self) -> &str;
    fn pattern(&self) -> &str;
    fn len(&self) -> usize;
}

impl Header {
    /// Crea un nuovo header (metodo helper)
    pub fn create(bits: &str) -> Self {
        Self {
            bits: bits.to_string(),
            pattern: format!("header({})", bits),
            length: bits.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_new() {
        let h = Header::new("10");
        assert_eq!(h.bits(), "10");
        assert_eq!(h.len(), 2);
        assert_eq!(h.pattern(), "header(10)");
    }

    #[test]
    fn test_header_different_lengths() {
        let h2 = Header::new("110");
        assert_eq!(h2.bits(), "110");
        assert_eq!(h2.len(), 3);

        let h3 = Header::new("1110");
        assert_eq!(h3.bits(), "1110");
        assert_eq!(h3.len(), 4);
    }

    #[test]
    fn test_header_ops_trait() {
        use HeaderOps;
        let h = Header::new("10");
        assert_eq!(h.bits(), "10");
        assert_eq!(h.pattern(), "header(10)");
        assert_eq!(h.len(), 2);
    }
}
