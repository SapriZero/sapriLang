use urcm_interpreter::{Parser, VM};
use std::fs;

fn main() -> anyhow::Result<()> {
println!("🔌 URCM Runtime Example");
println!("=======================");

// Carica script URCM
let code = fs::read_to_string("examples/test.urcm")?;

// Parser
let mut parser = Parser::new();
parser.parse(&code)?;

// VM
let mut vm = VM::new(parser);
vm.run()?;

// Leggi risultati
println!("nRisultati:");
if let Some(c) = vm.get_value("C") {
println!("Comfort (C) = {}", c);
}
if let Some(s) = vm.get_value("S") {
println!("Sensazione (S) = {}", s);
}

// Simula cambio input
println!("n🔄 Cambio temperatura a 30...");
vm.update_input("t", 30.0)?;

if let Some(c) = vm.get_value("C") {
println!("Nuovo Comfort = {}", c);
}

Ok(())
}

