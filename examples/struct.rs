use urcm::{UrcmCtx, urcm_ctx};
use std::collections::HashSet;

struct MyData {
t: f64,
u: f64,
}

impl MyData {
fn comfort(&self) -> f64 {
self.t * self.u / 100.0
}

fn media(&self) -> f64 {
(self.t + self.u) / 2.0
}
}

fn main() {
let data = MyData { t: 25.0, u: 60.0 };

// Crea contesto con atomi
let mut ctx = urcm_ctx!(data);
ctx.def_field('t', "temperatura", "t")
.def_field('u', "umidita", "u")
.def_function('C', "comfort", MyData::comfort, HashSet::from(['t', 'u']))
.def_function('M', "media", MyData::media, HashSet::from(['t', 'u']));

// Usa le funzioni
let comfort = ctx.eval("C").unwrap();
let media = ctx.eval("M").unwrap();
println!("Comfort: {}, Media: {}", comfort, media);

// Formula composta
let doppio_comfort = ctx.eval("C * 2").unwrap();
println!("Doppio comfort: {}", doppio_comfort);
}
