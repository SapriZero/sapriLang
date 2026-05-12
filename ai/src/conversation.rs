//! Gestione conversazioni

#[derive(Debug, Clone)]
pub struct Message {
    pub role: String,  // "user" o "assistant"
    pub content: String,
}

impl Message {
    pub fn user(content: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: content.to_string(),
        }
    }
    
    pub fn assistant(content: &str) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.to_string(),
        }
    }
    
    pub fn to_sson(&self) -> String {
        format!(
            "[message]\nrole: {}\ncontent: {}\n",
            self.role, self.content
        )
    }
}

#[derive(Debug, Clone)]
pub struct Conversation {
    messages: Vec<Message>,
    max_history: usize,
}

impl Conversation {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            max_history: 100,
        }
    }
    
    pub fn with_max_history(max_history: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_history,
        }
    }
    
    pub fn add_user_message(&mut self, content: &str) {
        self.messages.push(Message::user(content));
        self.trim();
    }
    
    pub fn add_assistant_message(&mut self, content: &str) {
        self.messages.push(Message::assistant(content));
        self.trim();
    }
    
    pub fn add_pair(&mut self, user: &str, assistant: &str) {
        self.add_user_message(user);
        self.add_assistant_message(assistant);
    }
    
    pub fn history(&self) -> &[Message] {
        &self.messages
    }
    
    pub fn last_user_message(&self) -> Option<&Message> {
        self.messages.iter().rev().find(|m| m.role == "user")
    }
    
    pub fn last_assistant_message(&self) -> Option<&Message> {
        self.messages.iter().rev().find(|m| m.role == "assistant")
    }
    
    pub fn clear(&mut self) {
        self.messages.clear();
    }
    
    pub fn len(&self) -> usize {
        self.messages.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
    
    fn trim(&mut self) {
        while self.messages.len() > self.max_history {
            self.messages.remove(0);
        }
    }
}

impl Default for Conversation {
    fn default() -> Self {
        Self::new()
    }
}
