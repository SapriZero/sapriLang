//! Tokenizzazione delle espressioni stringa

/// Tipi di token riconosciuti dallo scanner
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(f64),
    Star,
    LParen,
    RParen,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        if c.is_ascii_digit() || c == '.' {
            let mut num_str = String::new();
            let mut has_dot = false;
            while i < chars.len() {
                let ch = chars[i];
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                    i += 1;
                } else if ch == '.' && !has_dot {
                    num_str.push(ch);
                    has_dot = true;
                    i += 1;
                } else {
                    break;
                }
            }
            let num = num_str.parse::<f64>()
                .map_err(|_| format!("Numero non valido: {}", num_str))?;
            tokens.push(Token::Number(num));
            continue;
        }

		if c.is_alphabetic() {
            // In URCM, ogni singolo carattere è un identificatore atomico
            // Quindi produciamo un token per ogni carattere, non accumuliamo stringhe
            tokens.push(Token::Ident(c.to_string()));
            i += 1;
            continue;
        }

        match c {
            '*' => { tokens.push(Token::Star); i += 1; }
            '(' => { tokens.push(Token::LParen); i += 1; }
            ')' => { tokens.push(Token::RParen); i += 1; }
            _ => return Err(format!("Carattere non riconosciuto: '{}'", c)),
        }
    }
    Ok(tokens)
}
