//! Motore di inferenza basato su IRCM

use crate::knowledge::KnowledgeBase;
use crate::analyzer::{MsasAnalyzer, IrcmProjector, MsasResult, IrcmProjection};

/// Motore di inferenza
#[derive(Debug)]
pub struct Reasoner {
    knowledge: KnowledgeBase,
    msas: MsasAnalyzer,
    ircm: IrcmProjector,
}

/// Risultato di inferenza
#[derive(Debug, Clone)]
pub struct Inference {
    pub query: String,
    pub answer: String,
    pub confidence: f64,
    pub s_score: f64,
}

impl Reasoner {
    pub fn new(knowledge: KnowledgeBase) -> Self {
        Self {
            knowledge,
            msas: MsasAnalyzer::new(),
            ircm: IrcmProjector::new(),
        }
    }

    /// Risponde a una query usando la conoscenza e l'inferenza
    pub fn query(&self, query: &str) -> Inference {
        // 1. Analizza la query (trova parole chiave)
        let words = crate::utils::extract_words(query);

        if words.is_empty() {
            return Inference {
                query: query.to_string(),
                answer: "Non ho capito la domanda".to_string(),
                confidence: 0.0,
                s_score: 0.0,
            };
        }

        // 2. Cerca nella conoscenza
        let mut answers = Vec::new();

        for word in &words {
            if let Some(category) = self.knowledge.get_category(word) {
                answers.push(format!("{} è un {}", word, category));
            }

            // Cerca relazioni
            let relations = self.knowledge.find_by_relation(word, "is_a");
            for rel in relations {
                answers.push(format!("{} è un {}", word, rel));
            }
        }

        // 3. Se non trova, usa inferenza IRCM
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

    /// Inferisce per analogia (IRCM projection)
    pub fn infer_analogy(&self, source: &str, target: &str) -> Option<String> {
        // TODO: Implementare analogia usando proiezione IRCM
        // Esempio: se A sta a B come C sta a ?
        None
    }

    /// Calcola similarità strutturale tra due concetti (IRCM)
    pub fn structural_similarity(&self, concept_a: &str, concept_b: &str) -> f64 {
        let relations_a = self.knowledge.find_by_relation(concept_a, "is_a");
        let relations_b = self.knowledge.find_by_relation(concept_b, "is_a");

        if relations_a.is_empty() || relations_b.is_empty() {
            return 0.0;
        }

        let common: usize = relations_a.iter()
            .filter(|r| relations_b.contains(r))
            .count();

        common as f64 / relations_a.len().max(relations_b.len()) as f64
    }

    pub fn knowledge(&self) -> &KnowledgeBase {
        &self.knowledge
    }
}
