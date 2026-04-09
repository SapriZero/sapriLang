//! Compilazione da AST a Atom<AtomValue>

use crate::atom_value::AtomValue;
use crate::context::Context;
use sapri_base::Atom;
use super::parser::Ast;

pub fn compile(ast: &Ast, ctx: &Context) -> Result<Atom<AtomValue>, String> {
    match ast {
        Ast::Ident(name) => {
            ctx.get(name)
                .ok_or_else(|| format!("Identificatore non trovato: '{}'", name))
        }
        Ast::Number(n) => {
            Ok(Atom::resolved(AtomValue::Number(*n)))
        }
        Ast::Product(factors) => {
            if factors.is_empty() {
                return Err("Prodotto vuoto".to_string());
            }

            let mut compiled = Vec::new();
            for factor in factors {
                compiled.push(compile(factor, ctx)?);
            }

            if compiled.len() == 1 {
                Ok(compiled.remove(0))
            } else {
                // Crea un atomo che moltiplica i valori numerici
                Ok(Atom::resolved(AtomValue::Number(
                    compiled.iter().fold(1.0, |acc, atom| {
                        acc * atom.get().as_number().unwrap_or(0.0)
                    })
                )))
            }
        }
    }
}

#[allow(dead_code)] 
pub fn compile_str(expr: &str, ctx: &Context) -> Result<Atom<AtomValue>, String> {
    let tokens = super::token::tokenize(expr)?;
    let ast = super::parser::parse(&tokens)?;
    compile(&ast, ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;

    #[test]
    fn test_compile_ident() {
        let mut ctx = Context::new();
        ctx.set("a", Atom::resolved(AtomValue::Number(10.0)));
        let ast = Ast::Ident("a".to_string());
        let atom = compile(&ast, &ctx).unwrap();
        assert_eq!(atom.get().as_number(), Some(10.0));
    }

    #[test]
    fn test_compile_number() {
        let ctx = Context::new();
        let ast = Ast::Number(42.0);
        let atom = compile(&ast, &ctx).unwrap();
        assert_eq!(atom.get().as_number(), Some(42.0));
    }

    #[test]
    fn test_compile_product() {
        let mut ctx = Context::new();
        ctx.set("a", Atom::resolved(AtomValue::Number(10.0)));
        ctx.set("b", Atom::resolved(AtomValue::Number(20.0)));
        let ast = Ast::Product(vec![
            Ast::Ident("a".to_string()),
            Ast::Ident("b".to_string()),
        ]);
        let atom = compile(&ast, &ctx).unwrap();
        assert_eq!(atom.get().as_number(), Some(200.0));
    }

    #[test]
    fn test_compile_str() {
        let mut ctx = Context::new();
        ctx.set("a", Atom::resolved(AtomValue::Number(10.0)));
        ctx.set("b", Atom::resolved(AtomValue::Number(20.0)));
        let atom = compile_str("a * b", &ctx).unwrap();
        assert_eq!(atom.get().as_number(), Some(200.0));
        let atom2 = compile_str("ab", &ctx).unwrap();
        assert_eq!(atom2.get().as_number(), Some(200.0));
    }
}
