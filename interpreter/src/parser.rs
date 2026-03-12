use std::collections::HashMap;
use crate::{PHI, PI, SQRT2, H, ALPHA, MU};

#[derive(Debug, Clone)]
pub enum Op {
    Mul,
    Add,
    Sub,
    Div,
    Eval,
    Assign,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub is_react: bool,
    pub value: Option<f64>,
    pub expr: Option<String>,
}

#[derive(Debug)]
pub struct Parser {
    nodes: HashMap<String, Node>,
    constants: HashMap<String, f64>,
}

impl Parser {
    pub fn new() -> Self {
        let mut p = Parser {
            nodes: HashMap::new(),
            constants: HashMap::new(),
        };
        p.init_constants();
        p
    }

    fn init_constants(&mut self) {
        self.constants.insert("φ".to_string(), PHI);
        self.constants.insert("π".to_string(), PI);
        self.constants.insert("√2".to_string(), SQRT2);
        self.constants.insert("h".to_string(), H);
        self.constants.insert("α".to_string(), ALPHA);
        self.constants.insert("μ".to_string(), MU);
    }

    pub fn parse(&mut self, code: &str) -> anyhow::Result<()> {
        for line in code.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Cerca il primo =
            if let Some(equal_pos) = line.find('=') {
                let left = line[..equal_pos].trim();
                let right = line[equal_pos+1..].trim();
                
                if left.is_empty() || right.is_empty() {
                    anyhow::bail!("Sintassi non valida: '{}'", line);
                }

                // Estrai il nome variabile (ultima parola a sinistra)
                let left_parts: Vec<&str> = left.split_whitespace().collect();
                let name = left_parts.last().unwrap().to_string();
                
                let is_react = name.chars().next().unwrap().is_uppercase();

                self.nodes.insert(name.clone(), Node {
                    name,
                    is_react,
                    value: None,
                    expr: Some(right.to_string()),
                });
            } else {
                anyhow::bail!("Sintassi non valida: '{}'", line);
            }
        }

        Ok(())
    }

pub fn eval_expr(&self, expr: &str, context: &HashMap<String, f64>) -> anyhow::Result<f64> {
    // Prima dividi per operatori + e - (se vorrai)
    // Per ora gestiamo * e / in ordine
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    if tokens.is_empty() {
        return Ok(0.0);
    }
    
    let mut result = 0.0;
    let mut current_op = '+';
    let mut i = 0;
    
    while i < tokens.len() {
        let token = tokens[i];
        
        // Se è un operatore
        if token == "*" || token == "/" || token == "+" || token == "-" {
            current_op = token.chars().next().unwrap();
            i += 1;
            continue;
        }
        
        // Valuta il numero/variabile
        let val = if let Ok(num) = token.parse::<f64>() {
            num
        } else if let Some(&c) = self.constants.get(token) {
            c
        } else if let Some(&v) = context.get(token) {
            v
        } else {
            anyhow::bail!("Variabile non trovata: {}", token);
        };
        
        // Applica l'operatore corrente
        match current_op {
            '+' => result += val,
            '-' => result -= val,
            '*' => {
                if i == 1 { // primo valore
                    result = val;
                } else {
                    result *= val;
                }
            }
            '/' => {
                if i == 1 {
                    result = val;
                } else {
                    result /= val;
                }
            }
            _ => result = val,
        }
        
        i += 1;
    }
    
    Ok(result)
}

    pub fn get_nodes(&self) -> &HashMap<String, Node> {
        &self.nodes
    }

    pub fn get_constants(&self) -> &HashMap<String, f64> {
        &self.constants
    }
}
