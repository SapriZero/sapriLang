//! Parsing dei token in Abstract Syntax Tree

use super::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Product(Vec<Ast>),
    Ident(String),
    Number(f64),
}

pub fn parse(tokens: &[Token]) -> Result<Ast, String> {
    if tokens.is_empty() {
        return Err("Espressione vuota".to_string());
    }

    let mut factors = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Ident(name) => {
                factors.push(Ast::Ident(name.clone()));
                i += 1;
            }
            Token::Number(n) => {
                factors.push(Ast::Number(*n));
                i += 1;
            }
            Token::Star => {
                i += 1;
                if i >= tokens.len() {
                    return Err("Operatore '*' senza operando".to_string());
                }
                continue;
            }
            Token::LParen => {
                let mut depth = 1;
                let mut end = i + 1;
                while end < tokens.len() && depth > 0 {
                    match tokens[end] {
                        Token::LParen => depth += 1,
                        Token::RParen => depth -= 1,
                        _ => {}
                    }
                    end += 1;
                }
                if depth != 0 {
                    return Err("Parentesi non bilanciate".to_string());
                }
                let sub_expr = parse(&tokens[i+1..end-1])?;
                factors.push(sub_expr);
                i = end;
            }
            Token::RParen => return Err("Parentesi chiusa inaspettata".to_string()),
        }
    }

    if factors.len() == 1 {
        Ok(factors.remove(0))
    } else {
        Ok(Ast::Product(factors))
    }
}
