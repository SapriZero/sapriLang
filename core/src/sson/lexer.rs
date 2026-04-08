//! Lexer elastico per .sson
//! Riconosce sezioni, proprietà _: , righe CSV quote-aware, commenti.

use crate::sson::token::Token;
use crate::sson::error::{Result, SsonError};

pub struct Lexer<'a> {
	#[allow(dead_code)] 
    input: &'a str,
    chars: std::vec::IntoIter<char>,
    line: usize,
    col: usize,
    peek: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().collect::<Vec<_>>().into_iter();
        let peek = chars.next();
        Self { input, chars, line: 1, col: 1, peek }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek.take();
        if let Some(ch) = c {
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else if ch != '\r' {
                self.col += 1;
            }
            self.peek = self.chars.next();
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek {
            if c.is_whitespace() && c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_until(&mut self, delimiter: char) -> String {
        let mut buf = String::new();
        while let Some(c) = self.peek {
            if c == delimiter || c == '\n' {
                break;
            }
            buf.push(c);
            self.advance();
        }
        buf
    }

    fn read_quoted(&mut self, quote: char) -> Result<String> {
        self.advance(); // consuma apertura
        let mut buf = String::new();
        while let Some(c) = self.peek {
            match c {
                ch if ch == quote => {
                    self.advance(); // consuma chiusura
                    return Ok(buf);
                }
                '\\' => {
                    self.advance();
                    if let Some(esc) = self.peek {
                        buf.push(match esc {
                            'n' => '\n', 't' => '\t', 'r' => '\r',
                            '\\' => '\\', '"' => '"', '\'' => '\'',
                            _ => esc,
                        });
                        self.advance();
                    }
                }
                _ => {
                    buf.push(c);
                    self.advance();
                }
            }
        }
        Err(SsonError::LexerError {
            line: self.line, col: self.col,
            message: "Stringa non chiusa".into(),
        })
    }

    fn read_identifier(&mut self) -> String {
        let mut buf = String::new();
        while let Some(c) = self.peek {
            if c.is_alphanumeric() || c == '_' || c == '.' || c == '-' {
                buf.push(c);
                self.advance();
            } else {
                break;
            }
        }
        buf
    }

    fn read_csv_line(&mut self) -> Result<Vec<String>> {
        let mut cells = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;

        while let Some(c) = self.peek {
            match c {
                '"' if !in_quotes => { in_quotes = true; self.advance(); }
                '"' if in_quotes => { in_quotes = false; self.advance(); }
                ',' if !in_quotes => {
                    cells.push(Self::parse_value(&current));
                    current.clear();
                    self.advance();
                    self.skip_whitespace();
                }
                '\n' | '\r' => break,
                _ => {
                    current.push(c);
                    self.advance();
                }
            }
        }
        if !current.is_empty() || !cells.is_empty() {
            cells.push(Self::parse_value(&current));
        }
        Ok(cells)
    }

    fn parse_value(raw: &str) -> String {
        let t = raw.trim();
        if t.starts_with('"') && t.ends_with('"') && t.len() >= 2 {
            t[1..t.len()-1].to_string()
        } else {
            t.to_string()
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        loop {
            self.skip_whitespace();
            let Some(c) = self.peek else { return Ok(Token::Eof); };

            // Separatore
            if c == '=' {
                while self.peek == Some('=') { self.advance(); }
                return Ok(Token::Separator);
            }

            // Commento
            if c == '/' {
                self.advance();
                if self.peek == Some('/') {
                    self.advance();
                    let is_doc = self.peek == Some('/');
                    if is_doc { self.advance(); }
                    let txt = self.read_until('\n');
                    return Ok(Token::Comment(is_doc, txt));
                }
                return Err(SsonError::LexerError {
                    line: self.line, col: self.col,
                    message: "Carattere '/' non atteso".into(),
                });
            }

            // Sezione [path]
            if c == '[' {
                self.advance();
                let is_sub = self.peek == Some('.');
                if is_sub { self.advance(); }
                let name = self.read_identifier();
                if self.peek != Some(']') {
                    return Err(SsonError::LexerError {
                        line: self.line, col: self.col,
                        message: "']' mancante in sezione".into(),
                    });
                }
                self.advance(); // consuma ]
                return Ok(if is_sub {
                    Token::Subsection(name)
                } else {
                    Token::Section(name)
                });
            }

            // Lista - item (per dipendenze)
            if c == '-' {
                self.advance();
                self.skip_whitespace();
                let item = self.read_until('\n');
                return Ok(Token::CsvLine(vec![item.trim().to_string()]));
            }

            // Key = value o riga CSV
            if c.is_alphabetic() || c == '_' {
                let key = self.read_identifier();
                self.skip_whitespace();

                if self.peek == Some('=') {
                    self.advance(); // consuma =
                    self.skip_whitespace();
                    let value = if self.peek == Some('"') || self.peek == Some('\'') {
                        let q = self.peek.unwrap();
                        self.read_quoted(q)?
                    } else {
                        self.read_until('\n').trim().to_string()
                    };
                    return Ok(Token::KeyValue(key, value));
                }

                // Altrimenti è inizio riga CSV
                self.advance(); // torna al primo char dopo key
                let mut cells = vec![key];
                cells.extend(self.read_csv_line()?);
                return Ok(Token::CsvLine(cells));
            }

            // Riga CSV pura
            if c.is_numeric() || c == '"' || c == ',' {
                return Ok(Token::CsvLine(self.read_csv_line()?));
            }

            // Carattere non riconosciuto
            return Err(SsonError::LexerError {
                line: self.line, col: self.col,
                message: format!("Carattere non riconosciuto: '{}'", c),
            });
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            if matches!(token, Token::Eof) {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        Ok(tokens)
    }
}
