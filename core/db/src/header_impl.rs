//! Implementazione header

use super::header::{Header, HeaderOps};

impl HeaderOps for Header {
    fn new(bits: &str) -> Self {
        let length = bits.len();
        let pattern = format!("header({})", bits);
        Self {
            bits: bits.to_string(),
            pattern,
            length,
        }
    }

    fn bits(&self) -> &str {
        &self.bits
    }

    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn len(&self) -> usize {
        self.length
    }
}
