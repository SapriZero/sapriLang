#[macro_export]
macro_rules! urcm {
// Forma 1: valori immediati (moltiplicazione)
($($val:expr),+ $(,)?) => {{
let mut result = 1.0;
$(result *= $val as f64;)+
result
}};

// Forma 2: lazy con closure
(|$($param:ident),+| $expr:expr) => {{
move |$($param),+| {
let _ = ($($param),+);
$expr
}
}};

// Forma 3: stringa con contesto
($expr:expr, $ctx:expr) => {{
$ctx.eval($expr)
}};
}

#[macro_export]
macro_rules! urcm_c {
($ctx:expr) => {{
move |expr: &str| $ctx.eval(expr)
}};
}

#[macro_export]
macro_rules! pipe {
($val:expr) => { $val };
($val:expr, $f:expr) => { $f($val) };
($val:expr, $f:expr, $($rest:expr),+) => {
$crate::pipe!($f($val), $($rest),+)
};
}

