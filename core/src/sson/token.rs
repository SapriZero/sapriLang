//! Token per il lexer .sson

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Separatore di blocco: ============================================
    Separator,

    /// Header sezione: [path] o [_META]
    Section(String),

    /// Sottosezione relativa: [.child]
    Subsection(String),

    /// Coppia chiave-valore: key = value
    KeyValue(String, String),

    /// Lista di valori CSV: val1, val2, "val,3"
    CsvLine(Vec<String>),

    /// Commento: // testo o /// docstring
    Comment(bool, String),  // bool = true se docstring

    /// Fine file
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Separator => write!(f, "==="),
            Token::Section(s) => write!(f, "[{}]", s),
            Token::Subsection(s) => write!(f, "[.{}]", s),
            Token::KeyValue(k, v) => write!(f, "{}={}", k, v),
            Token::CsvLine(vals) => write!(f, "CSV[{}]", vals.join(",")),
            Token::Comment(doc, txt) => {
                if *doc { write!(f, "///{}", txt) } else { write!(f, "//{}", txt) }
            }
            Token::Eof => write!(f, "EOF"),
        }
    }
}
