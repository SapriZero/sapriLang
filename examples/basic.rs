use urcm::{urcm, urcm_c, pipe, UrcmCtx};

fn main() {
// Forma 1: eager
let t = 25.0;
let u = 60.0;
let comfort = urcm!(t * u / 100);
println!("Comfort eager: {}", comfort);

// Forma 2: lazy
let formula = urcm!(|t, u| t * u / 100);
let comfort = formula(25.0, 60.0);
println!("Comfort lazy: {}", comfort);

// Forma 3: contesto
let mut ctx = UrcmCtx::new(());
ctx.def_number('t', "temperatura", 25.0);
ctx.def_number('u', "umidita", 60.0);

let comfort = ctx.eval("t * u / 100").unwrap();
println!("Comfort contesto: {}", comfort);

// Pipe
let res = pipe!(10, |x| x * 2, |x| x + 5);
println!("Pipe risultato: {}", res);

// Currying
let urcm = urcm_c!(ctx);
let c1 = urcm("t * u");
let c2 = urcm("t + u");
println!("C1: {:?}, C2: {:?}", c1, c2);
}

