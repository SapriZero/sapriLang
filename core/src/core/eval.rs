use std::collections::HashMap;

pub fn eval_expr(expr: &str, values: &HashMap<char, f64>) -> Result<f64, String> {
let tokens: Vec<&str> = expr.split_whitespace().collect();
if tokens.is_empty() {
return Err("Espressione vuota".to_string());
}

let mut result = 0.0;
let mut current_op = '+';
let mut i = 0;

while i < tokens.len() {
let token = tokens[i];

match token {
"+" | "-" | "*" | "/" => {
current_op = token.chars().next().unwrap();
i += 1;
continue;
}
_ => {
let val = if let Ok(num) = token.parse::<f64>() {
num
} else if token.len() == 1 {
let c = token.chars().next().unwrap();
*values.get(&c).ok_or_else(|| format!("Atomo non trovato: {}", c))?
} else {
return Err(format!("Token non supportato: {}", token));
};

match current_op {
'+' => result += val,
'-' => result -= val,
'*' => {
if i == 0 { result = val; }
else { result *= val; }
}
'/' => {
if i == 0 { result = val; }
else { result /= val; }
}
_ => result = val,
}
}
}
i += 1;
}

Ok(result)
}

