//! Cervello principale dell'AI - Motore unificato

use crate::conversation::Conversation;
use crate::knowledge::KnowledgeBase;
use crate::memory::HolographicMemory;
use crate::reasoner::Reasoner;
use crate::reader::WikipediaReader;
use sapri_core::Config;

/// Cervello principale dell'AI
#[derive(Debug)]
pub struct Brain {
    config: Config,
    conversation: Conversation,
    knowledge: KnowledgeBase,
    memory: HolographicMemory,
    reasoner: Reasoner,
}

impl Brain {
    pub fn new() -> Result<Self, String> {
        Self::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Result<Self, String> {
        let knowledge = KnowledgeBase::with_config(config.as_obj());
        let reasoner = Reasoner::new(knowledge.clone());

        Ok(Self {
            config,
            conversation: Conversation::new(),
            knowledge,
            memory: HolographicMemory::new(),
            reasoner,
        })
    }

    /// Carica conoscenza da Wikipedia
    pub fn learn_from_wikipedia(&mut self, path: &str) -> Result<usize, String> {
        println!("🧠 Apprendimento da Wikipedia...");
        let mut reader = WikipediaReader::with_knowledge(self.knowledge.clone());
        let count = reader.read_articles(path)?;
        self.knowledge = reader.into_knowledge();
        self.reasoner = Reasoner::new(self.knowledge.clone());
        Ok(count)
    }

    /// Importa dizionario
    pub fn import_dictionary(&mut self, path: &str) -> Result<usize, String> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut count = 0;

        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            if let Some((word, _)) = line.split_once('/') {
                self.knowledge.add_word(word, "word")?;
                count += 1;
            }
        }

        Ok(count)
    }

    /// Parla: risponde a un input
    pub fn talk(&mut self, input: &str) -> String {
        self.conversation.add_user_message(input);

        // 1. Cerca in memoria (ricordo diretto)
        if let Some(response) = self.memory.recall(input) {
            self.conversation.add_assistant_message(&response);
            return response;
        }

        // 2. Usa il reasoner (inferenza + conoscenza)
        let inference = self.reasoner.query(input);

        let response = if inference.confidence > 0.5 {
            inference.answer
        } else {
            format!("Non conosco '{}'. Puoi insegnarmelo?", input)
        };

        // 3. Memorizza la nuova conoscenza (calcola S-score)
        let _ = self.memory.remember(input, &response);

        self.conversation.add_assistant_message(&response);
        response
    }

    /// Insegna una nuova conoscenza
    pub fn teach(&mut self, input: &str, output: &str) -> Result<(), String> {
        self.memory.remember(input, output)?;

        // Se sembra una definizione (es. "gatto è un animale")
        if input.contains(" è un ") {
            let parts: Vec<&str> = input.split(" è un ").collect();
            if parts.len() == 2 {
                self.knowledge.add_relation(parts[0].trim(), "is_a", parts[1].trim())?;
            }
        }

        Ok(())
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        self.knowledge.save(&format!("{}/knowledge.bin", path))?;
        self.memory.save(&format!("{}/memory.bin", path))?;
        Ok(())
    }

    pub fn load(&mut self, path: &str) -> Result<(), String> {
        self.knowledge.load(&format!("{}/knowledge.bin", path))?;
        self.memory.load(&format!("{}/memory.bin", path))?;
        self.reasoner = Reasoner::new(self.knowledge.clone());
        Ok(())
    }

    pub fn stats(&self) -> String {
        format!(
            "{}\n{}\n{}",
            self.knowledge.stats(),
            self.memory.stats(),
            self.conversation.stats()
        )
    }
}

impl Default for Brain {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
