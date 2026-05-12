//! Cervello principale dell'AI

use crate::conversation::Conversation;
use crate::memory::HolographicMemory;
use crate::learning::Learner;
use crate::knowledge::KnowledgeBase;
use sapri_core::{Runtime, Command};

pub struct Brain {
    runtime: Runtime,
    knowledge: KnowledgeBase,
    memory: HolographicMemory,
    learner: Learner,
    conversation: Conversation,
}

impl Brain {
    pub fn new(config_path: Option<&str>) -> Result<Self, String> {
        let mut brain = Self {
            runtime: Runtime::new(config_path)?,
            knowledge: KnowledgeBase::new(),
            memory: HolographicMemory::new(),
            learner: Learner::new(),
            conversation: Conversation::new(),
        };
        
        // Carica parole da Wikipedia (formato pulito)
        let data_dir = "data";
        
        // Prova a caricare il file pulito
        let clean_words_path = format!("{}/wikipedia_words_clean.txt", data_dir);
        if std::path::Path::new(&clean_words_path).exists() {
            match brain.knowledge.load_words(&clean_words_path) {
                Ok(count) => println!("📚 Caricate {} parole da Wikipedia (pulite)", count),
                Err(e) => eprintln!("⚠️ Errore caricamento parole pulite: {}", e),
            }
        } else {
            // Fallback al file grezzo
            let raw_words_path = format!("{}/wikipedia_words.txt", data_dir);
            if std::path::Path::new(&raw_words_path).exists() {
                match brain.knowledge.load_words(&raw_words_path) {
                    Ok(count) => println!("📚 Caricate {} parole da Wikipedia (grezze)", count),
                    Err(e) => eprintln!("⚠️ Errore caricamento parole grezze: {}", e),
                }
            } else {
                println!("⚠️ Nessun file parole trovato in {}", data_dir);
            }
        }
        
        // Opzionale: carica dizionario Hunspell
        let dic_path = format!("{}/italiano_2_4_2007_09_01/it_IT.dic", data_dir);
        if std::path::Path::new(&dic_path).exists() {
            match brain.knowledge.load_dic(&dic_path) {
                Ok(count) => println!("📚 Caricate {} parole dal dizionario Hunspell", count),
                Err(e) => eprintln!("⚠️ Errore caricamento dizionario: {}", e),
            }
        }
        
        println!("{}", brain.knowledge.stats());
        
        Ok(brain)
    }
    
    pub fn talk(&mut self, input: &str) -> String {
        self.conversation.add_user_message(input);
        
        // Cerca nella memoria prima
        if let Some(response) = self.memory.recall(input) {
            self.conversation.add_assistant_message(&response);
            return response;
        }
        
        // Cerca nella conoscenza
        let response = self.knowledge.answer(input);
        
        if response.is_empty() {
            // Prova a valutare come espressione
            match self.runtime.execute(Command::Eval { expr: input.to_string() }) {
                Ok(result) => {
                    // Se il risultato è diverso dall'input, è una valutazione
                    if result != input {
                        self.conversation.add_assistant_message(&result);
                        return result;
                    }
                }
                Err(_) => {}
            }
            
            // Risposta generica per parole sconosciute
            let unknown_response = format!("Non conosco '{}'. Puoi insegnarmelo?", input);
            self.conversation.add_assistant_message(&unknown_response);
            unknown_response
        } else {
            self.conversation.add_assistant_message(&response);
            response
        }
    }
    
    pub fn learn(&mut self, input: &str, response: &str) {
        self.learner.learn(input, response);
        self.knowledge.add_terzina(input, response);
        self.memory.remember(input, response);
    }
    
    pub fn save(&self, path: &str) -> Result<(), String> {
        self.knowledge.save(&format!("{}/knowledge.sson", path))?;
        self.memory.save(&format!("{}/memory.sson", path))?;
        Ok(())
    }
    
    pub fn load(&mut self, path: &str) -> Result<(), String> {
        let knowledge_path = format!("{}/knowledge.sson", path);
        let memory_path = format!("{}/memory.sson", path);
        
        if std::path::Path::new(&knowledge_path).exists() {
            self.knowledge.load(&knowledge_path)?;
        }
        if std::path::Path::new(&memory_path).exists() {
            // Per ora, la memoria si ricostruisce dalle conversazioni
        }
        Ok(())
    }
    
    pub fn stats(&self) -> String {
        format!(
            "{}\nMemory: {} entries\nConversation: {} messages",
            self.knowledge.stats(),
            self.memory.len(),
            self.conversation.len()
        )
    }
}
