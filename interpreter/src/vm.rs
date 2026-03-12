use std::collections::HashMap;
use crate::parser::{Parser, Node};

#[derive(Debug)]
pub struct VM {
    parser: Parser,
    values: HashMap<String, f64>,
    reactives: HashMap<String, Vec<String>>,
}

impl VM {
    pub fn new(parser: Parser) -> Self {
        let mut vm = VM {
            parser,
            values: HashMap::new(),
            reactives: HashMap::new(),
        };
        vm.build_dependencies();
        vm
    }
    
    fn build_dependencies(&mut self) {
        for (name, node) in self.parser.get_nodes() {
            if node.is_react {
                if let Some(expr) = &node.expr {
                    for part in expr.split('*') {
                        let part = part.trim();
                        if !part.chars().next().unwrap().is_numeric() {
                            self.reactives.entry(part.to_string())
                                .or_insert_with(Vec::new)
                                .push(name.clone());
                        }
                    }
                }
            }
        }
    }
    
   pub fn run(&mut self) -> anyhow::Result<()> {
    let nodes = self.parser.get_nodes().clone();
    
    println!("🔍 Ordine valutazione:");
    for (name, node) in &nodes {
        println!("  {} = {:?}", name, node.expr);
    }
    
    // Calcola in ordine, gestendo dipendenze
    let mut changed = true;
    let mut iteration = 0;
    while changed && iteration < 10 {
        changed = false;
        iteration += 1;
        println!("\n📐 Iterazione {}", iteration);
        
        for (name, node) in &nodes {
            if let Some(expr) = &node.expr {
                match self.parser.eval_expr(expr, &self.values) {
                    Ok(new_val) => {
                        let old_val = self.values.get(name).copied().unwrap_or(0.0);
                        if (new_val - old_val).abs() > 1e-10 {
                            println!("  {} cambia: {} -> {}", name, old_val, new_val);
                            self.values.insert(name.clone(), new_val);
                            changed = true;
                        }
                    }
                    Err(e) => {
                        println!("  {} non valutabile: {}", name, e);
                    }
                }
            }
        }
    }
    
    println!("\n✅ Valori finali:");
    for (k, v) in &self.values {
        println!("  {} = {}", k, v);
    }
    
    Ok(())
}
    
    pub fn get_value(&self, name: &str) -> Option<f64> {
        self.values.get(name).copied()
    }
    
    pub fn update_input(&mut self, name: &str, value: f64) -> anyhow::Result<()> {
        self.values.insert(name.to_string(), value);
        
        if let Some(deps) = self.reactives.get(name) {
            let nodes = self.parser.get_nodes().clone();
            let mut changed = true;
            
            while changed {
                changed = false;
                for dep in deps {
                    if let Some(node) = nodes.get(dep) {
                        if let Some(expr) = &node.expr {
                            let new_val = self.parser.eval_expr(expr, &self.values)?;
                            let old_val = self.values.get(dep).copied().unwrap_or(0.0);
                            
                            if (new_val - old_val).abs() > 1e-10 {
                                self.values.insert(dep.clone(), new_val);
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub fn dump(&self) {
        println!("\n📊 Stato VM:");
        let mut sorted: Vec<_> = self.values.iter().collect();
        sorted.sort_by_key(|(k, _)| *k);
        
        for (k, v) in sorted {
            let react = if self.parser.get_nodes().get(k).map(|n| n.is_react).unwrap_or(false) {
                "⚡"
            } else {
                "  "
            };
            println!("  {}{} = {}", react, k, v);
        }
    }
}
