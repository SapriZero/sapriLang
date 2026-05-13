//! Motore di inferenza basato su IRCM
use crate::knowledge::KnowledgeBase;

#[derive(Debug)]
pub struct Reasoner {
    knowledge: KnowledgeBase,
}

#[derive(Debug, Clone)]
pub struct Inference {
    pub query: String,
    pub answer: String,
    pub confidence: f64,
    pub s_score: f64,
}

impl Reasoner {
    pub fn new(knowledge: KnowledgeBase) -> Self {
        Self { knowledge }
    }

    pub fn query(&self, query: &str) -> Inference {
        let words = crate::utils::extract_words(query);
        if words.is_empty() {
            return Inference {
                query: query.to_string(),
                answer: "Non ho capito la domanda".to_string(),
                confidence: 0.0,
                s_score: 0.0,
            };
        }
        let mut answers = Vec::new();
        for word in &words {
            if let Some(category) = self.knowledge.get_category(word) {
                answers.push(format!("{} è un {}", word, category));
            }
            let relations = self.knowledge.find_by_relation(word, "is_a");
            for rel in relations {
                answers.push(format!("{} è un {}", word, rel));
            }
        }
        if answers.is_empty() {
            Inference {
                query: query.to_string(),
                answer: format!("Non conosco '{}'", words.join(", ")),
                confidence: 0.0,
                s_score: 0.0,
            }
        } else {
            Inference {
                query: query.to_string(),
                answer: answers.join("; "),
                confidence: 0.8,
                s_score: 0.95,
            }
        }
    }

    pub fn infer_analogy(&self, _source: &str, _target: &str) -> Option<String> {
        None
    }

    pub fn structural_similarity(&self, concept_a: &str, concept_b: &str) -> f64 {
        let relations_a = self.knowledge.find_by_relation(concept_a, "is_a");
        let relations_b = self.knowledge.find_by_relation(concept_b, "is_a");
        if relations_a.is_empty() || relations_b.is_empty() {
            return 0.0;
        }
        let common = relations_a.iter().filter(|r| relations_b.contains(r)).count();
        common as f64 / relations_a.len().max(relations_b.len()) as f64
    }

    pub fn knowledge(&self) -> &KnowledgeBase {
        &self.knowledge
    }
}
