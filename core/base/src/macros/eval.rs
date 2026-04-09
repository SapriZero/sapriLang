//! Valutazione condizionale pura e masking leggero.

/// Valuta un'espressione solo se la condizione è vera, altrimenti fallback.
#[macro_export]
macro_rules! eval {
    ($cond:expr, $then:expr, $else:expr) => {
        if $cond { $then } else { $else }
    };
    ($cond:expr, $then:expr) => {
        if $cond { Some($then) } else { None }
    };
}

/// Applica una maschera/trasformazione a un valore solo se soddisfa un predicato.
#[macro_export]
macro_rules! mask {
    ($val:expr, $pred:expr, $transform:expr) => {
        if $pred(&$val) { $transform($val) } else { $val }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_eval_conditional() {
        assert_eq!(eval!(true, 1 + 1, 0), 2);
        assert_eq!(eval!(false, panic!(), 42), 42); // short-circuit: panic! non eseguito
    }

    #[test]
    fn test_eval_option_form() {
        assert_eq!(eval!(true, "ok"), Some("ok"));
        assert_eq!(eval!(false, "nope"), None);
    }

    #[test]
    fn test_mask_predicate() {
        let x = 10;
        let masked = mask!(x, |&v| v > 5, |v| v * 2);
        assert_eq!(masked, 20);
        
        let y = 3;
        let unchanged = mask!(y, |&v| v > 5, |v| v * 2);
        assert_eq!(unchanged, 3);
    }
}
