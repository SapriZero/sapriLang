//! Tokenizzazione .sson con bucket 65535

use crate::bucket::array::BucketArray;

/// Tipi di token
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    /// Caratteri speciali
    SectionStart,   // '['
    SectionEnd,     // ']'
    Equals,         // '='
    Comma,          // ','
    Hash,           // '#'
    Dot,            // '.'
    Colon,          // ':'

    /// Token con valore
    Word(u16),      // parola (indice in word_bucket)
    String(usize),  // stringa (indice in stringhe)
    Number(f64),

    /// Fine
    Eof,
}

/// Token con posizione
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub col: usize,
}

/// Tokenizer per .sson
pub struct SsonTokenizer {
    /// Bucket per caratteri speciali (0-255)
    char_bucket: BucketArray<TokenType, 256>,
    /// Bucket per parole (hash 0-65535)
    word_bucket: BucketArray<u16, 65536>,
    /// Stringhe raccolte
    strings: Vec<String>,
    /// Parole raccolte
    words: Vec<String>,

    /// Input corrente
    input: Vec<u8>,
    pos: usize,
    line: usize,
    col: usize,
}

impl SsonTokenizer {
    pub fn new() -> Self {
        let mut char_bucket = BucketArray::new("sson_chars");

        // Inizializza caratteri speciali
        let _ = char_bucket.insert(b'[' as usize, TokenType::SectionStart);
        let _ = char_bucket.insert(b']' as usize, TokenType::SectionEnd);
        let _ = char_bucket.insert(b'=' as usize, TokenType::Equals);
        let _ = char_bucket.insert(b',' as usize, TokenType::Comma);
        let _ = char_bucket.insert(b'#' as usize, TokenType::Hash);
        let _ = char_bucket.insert(b'.' as usize, TokenType::Dot);
        let _ = char_bucket.insert(b':' as usize, TokenType::Colon);

        Self {
            char_bucket,
            word_bucket: BucketArray::new("sson_words"),
            strings: Vec::new(),
            words: Vec::new(),
            input: Vec::new(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    /// Inizializza con nuovo input
    pub fn reset(&mut self, input: &str) {
        self.input = input.as_bytes().to_vec();
        self.pos = 0;
        self.line = 1;
        self.col = 1;
    }

    /// Registra una parola e restituisce ID
pub fn register_word(&mut self, word: &str) -> u16 {
    let bytes = word.as_bytes();
    let b1 = bytes.first().copied().unwrap_or(0) as usize;
    let b2 = bytes.get(1).copied().unwrap_or(0) as usize;
    let hash = (b1 << 8) | b2;
    
    // Cerca se già esiste - usa copied() per ottenere u16
    if let Some(id) = self.word_bucket.get(hash).copied() {
        return id;
    }
    
    // Nuova parola
    let id = self.words.len() as u16;
    self.words.push(word.to_string());
    let _ = self.word_bucket.insert(hash, id);
    id
}

    /// Registra una stringa e restituisce indice
    pub fn register_string(&mut self, s: String) -> usize {
        let idx = self.strings.len();
        self.strings.push(s);
        idx
    }

    /// Ottieni prossimo token
    pub fn next_token(&mut self) -> Result<Token, String> {
        // Salta spazi bianchi
        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            if !c.is_ascii_whitespace() {
                break;
            }
            if c == b'\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            self.pos += 1;
        }

        if self.pos >= self.input.len() {
            return Ok(Token {
                token_type: TokenType::Eof,
                line: self.line,
                col: self.col,
            });
        }

        let start_pos = self.pos;
        let start_line = self.line;
        let start_col = self.col;
        let c = self.input[self.pos];

        // Caratteri speciali
        if let Some(&token_type) = self.char_bucket.get(c as usize) {
            self.pos += 1;
            self.col += 1;
            return Ok(Token {
                token_type,
                line: start_line,
                col: start_col,
            });
        }

        // Numeri
        if c.is_ascii_digit() || c == b'-' {
            let mut end = self.pos + 1;
            while end < self.input.len() &&
                  (self.input[end].is_ascii_digit() || self.input[end] == b'.') {
                end += 1;
            }
            let num_str = std::str::from_utf8(&self.input[self.pos..end])
                .map_err(|_| format!("Invalid number at {}:{}", start_line, start_col))?;
            let num = num_str.parse::<f64>()
                .map_err(|_| format!("Invalid number at {}:{}", start_line, start_col))?;

            self.pos = end;
            self.col += end - start_pos;

            return Ok(Token {
                token_type: TokenType::Number(num),
                line: start_line,
                col: start_col,
            });
        }

        // Stringhe tra virgolette
        if c == b'"' {
            self.pos += 1; // salta il primo "
            let mut end = self.pos;
            while end < self.input.len() && self.input[end] != b'"' {
                end += 1;
            }
            if end >= self.input.len() {
                return Err(format!("Unclosed string at {}:{}", start_line, start_col));
            }

            let s = std::str::from_utf8(&self.input[self.pos..end])
                .map_err(|_| format!("Invalid string at {}:{}", start_line, start_col))?
                .to_string();

            let idx = self.register_string(s);

            self.pos = end + 1; // salta il " finale
            self.col += (end - start_pos) + 1;

            return Ok(Token {
                token_type: TokenType::String(idx),
                line: start_line,
                col: start_col,
            });
        }
        
        // Parole (alfanumeriche e underscore)
		if c.is_ascii_alphabetic() || c == b'_' {
		    let mut end = self.pos + 1;
		    while end < self.input.len() && 
		          (self.input[end].is_ascii_alphanumeric() || self.input[end] == b'_') {
		        end += 1;
		    }
		    
		    // Estrai i byte prima di chiamare metodi che richiedono &mut self
		    let word_bytes = &self.input[self.pos..end];
		    let word = std::str::from_utf8(word_bytes)
		        .map_err(|_| format!("Invalid word at {}:{}", start_line, start_col))?
		        .to_string();
		    
		    let id = self.register_word(&word);
		    
		    self.pos = end;
		    self.col += end - start_pos;
		    
		    return Ok(Token {
		        token_type: TokenType::Word(id),
		        line: start_line,
		        col: start_col,
		    });
	   }

        Err(format!("Unexpected character '{}' at {}:{}", c as char, start_line, start_col))
    }

    /// Ottieni la stringa per un indice
    pub fn get_string(&self, idx: usize) -> Option<&str> {
        self.strings.get(idx).map(|s| s.as_str())
    }

    /// Ottieni la parola per un ID
    pub fn get_word(&self, id: u16) -> Option<&str> {
        self.words.get(id as usize).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_simple() {
        let mut tokenizer = SsonTokenizer::new();
        tokenizer.reset("[users id name]");

        let token = tokenizer.next_token().unwrap();
        assert!(matches!(token.token_type, TokenType::SectionStart));

        let token = tokenizer.next_token().unwrap();
        assert!(matches!(token.token_type, TokenType::Word(_)));

        let token = tokenizer.next_token().unwrap();
        assert!(matches!(token.token_type, TokenType::Word(_)));

        let token = tokenizer.next_token().unwrap();
        assert!(matches!(token.token_type, TokenType::Word(_)));

        let token = tokenizer.next_token().unwrap();
        assert!(matches!(token.token_type, TokenType::SectionEnd));
    }

    #[test]
    fn test_tokenizer_strings() {
        let mut tokenizer = SsonTokenizer::new();
        tokenizer.reset("[test] \"hello world\", 123");

        let mut tokens = Vec::new();
        while let Ok(token) = tokenizer.next_token() {
            if matches!(token.token_type, TokenType::Eof) {
                break;
            }
            tokens.push(token);
        }

        assert!(!tokens.is_empty());
    }
}
