//! Apprendimento

use std::collections::HashMap;

pub struct Learner {
    stats: HashMap<String, usize>,
}

impl Learner {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }
    
    pub fn learn(&mut self, input: &str, response: &str) {
        let key = format!("{}→{}", input, response);
        *self.stats.entry(key).or_insert(0) += 1;
    }
    
    pub fn confidence(&self, input: &str, response: &str) -> f64 {
        let key = format!("{}→{}", input, response);
        let count = *self.stats.get(&key).unwrap_or(&0) as f64;
        let total: f64 = self.stats.values().sum::<usize>() as f64;
        if total == 0.0 { 0.0 } else { count / total }
    }
    
    pub fn most_common_response(&self, input: &str) -> Option<String> {
        let mut best: Option<(&str, usize)> = None;
        for (key, count) in &self.stats {
            if let Some(prefix) = key.split('→').next() {
                if prefix == input {
                    if let Some((_, best_count)) = best {
                        if *count > best_count {
                            best = Some((key, *count));
                        }
                    } else {
                        best = Some((key, *count));
                    }
                }
            }
        }
        
        best.and_then(|(key, _)| {
            key.split('→').nth(1).map(|s| s.to_string())
        })
    }
}

impl Default for Learner {
    fn default() -> Self {
        Self::new()
    }
}
