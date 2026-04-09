use std::collections::HashSet;

pub struct ReactiveVar<T> {
value: f64,
deps: HashSet<char>,
compute: Box<dyn Fn(&T) -> f64 + Send + Sync>,
}

impl<T> ReactiveVar<T> {
pub fn new<F>(compute: F, deps: HashSet<char>) -> Self
where
F: Fn(&T) -> f64 + Send + Sync + 'static,
{
Self {
value: 0.0,
deps,
compute: Box::new(compute),
}
}

pub fn update(&mut self, data: &T) {
self.value = (self.compute)(data);
}

pub fn value(&self) -> f64 {
self.value
}

pub fn deps(&self) -> &HashSet<char> {
&self.deps
}
}

