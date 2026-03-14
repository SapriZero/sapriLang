//! Parser .sson

use crate::sson::ast::SsonDocument;
use crate::sson::error::SsonError;
use crate::sson::token::SsonTokenizer;
use std::collections::HashMap;

pub struct SsonParser {
    tokenizer: SsonTokenizer,
}

impl SsonParser {
    pub fn new() -> Self {
        Self {
            tokenizer: SsonTokenizer::new(),
        }
    }
    
    pub fn parse(&mut self, input: &str) -> Result<SsonDocument, SsonError> {
        self.tokenizer.reset(input);
        
        let metadata = HashMap::new();
        let tables = Vec::new();
        
        // TODO: implementare parsing completo
        // Per ora restituisce documento vuoto
        
        Ok(SsonDocument {
            metadata,
            tables,
        })
    }
}

pub fn parse_sson(input: &str) -> Result<SsonDocument, SsonError> {
    let mut parser = SsonParser::new();
    parser.parse(input)
}
