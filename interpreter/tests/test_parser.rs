use urcm_interpreter::parser::Parser;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_simple_assignment() {
        let mut parser = Parser::new();
        let code = "temperatura t = 25";
        
        parser.parse(code).unwrap();
        let nodes = parser.get_nodes();
        
        assert_eq!(nodes.len(), 1);
        let node = nodes.get("t").unwrap();
        assert_eq!(node.name, "t");
        assert_eq!(node.is_react, false);
        assert_eq!(node.expr, Some("25".to_string()));
    }

    #[test]
    fn test_parse_react_variable() {
        let mut parser = Parser::new();
        let code = "Comfort C = t * u";
        
        parser.parse(code).unwrap();
        let nodes = parser.get_nodes();
        
        assert_eq!(nodes.len(), 1);
        let node = nodes.get("C").unwrap();
        assert_eq!(node.name, "C");
        assert_eq!(node.is_react, true);
        assert_eq!(node.expr, Some("t * u".to_string()));
    }

    #[test]
    fn test_parse_multiple_lines() {
        let mut parser = Parser::new();
        let code = r#"
# commento
temperatura t = 25
umidita u = 60
Comfort C = t * u
"#;
        
        parser.parse(code).unwrap();
        let nodes = parser.get_nodes();
        
        assert_eq!(nodes.len(), 3);
        assert!(nodes.contains_key("t"));
        assert!(nodes.contains_key("u"));
        assert!(nodes.contains_key("C"));
    }

    #[test]
    fn test_parse_with_constants() {
        let mut parser = Parser::new();
        let code = "risultato R = φ * π";
        
        parser.parse(code).unwrap();
        let nodes = parser.get_nodes();
        
        let node = nodes.get("R").unwrap();
        assert_eq!(node.expr, Some("φ * π".to_string()));
    }

    #[test]
    fn test_eval_expression() {
        let parser = Parser::new();
        let mut context = HashMap::new();
        context.insert("t".to_string(), 5.0);
        context.insert("u".to_string(), 10.0);
        
        let val = parser.eval_expr("t * u", &context).unwrap();
        assert_eq!(val, 50.0);
    }

    #[test]
    fn test_ignore_comments_and_empty() {
        let mut parser = Parser::new();
        let code = r#"

# solo commento

variabile v = 42

# altro commento
"#;
        
        parser.parse(code).unwrap();
        let nodes = parser.get_nodes();
        
        assert_eq!(nodes.len(), 1);
        assert!(nodes.contains_key("v"));
    }

    #[test]
    fn test_division() {
        let mut parser = Parser::new();
        let code = "rapporto R = 10 / 2";
        
        parser.parse(code).unwrap();
        let nodes = parser.get_nodes();
        
        let node = nodes.get("R").unwrap();
        assert_eq!(node.expr, Some("10 / 2".to_string()));
    }

    #[test]
    fn test_complex_expression() {
        let mut parser = Parser::new();
        let code = "calcolo C = (t + u) * 2 / φ";
        
        parser.parse(code).unwrap();
        let nodes = parser.get_nodes();
        
        assert_eq!(nodes.len(), 1);
        assert!(nodes.contains_key("C"));
    }

    #[test]
    fn test_invalid_syntax() {
        let mut parser = Parser::new();
        let code = "questa non è una assegnazione valida";
        
        let result = parser.parse(code);
        assert!(result.is_err());
    }
}
