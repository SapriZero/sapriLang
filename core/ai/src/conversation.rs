//! Gestione della cronologia delle conversazioni

use std::time::{SystemTime, UNIX_EPOCH};

/// Ruolo del messaggio
#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    User,
    Assistant,
    System,
}

/// Singolo messaggio
#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub timestamp: u64,
}

impl Message {
    pub fn new(role: Role, content: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            role,
            content,
            timestamp,
        }
    }
}

/// Cronologia della conversazione
#[derive(Debug, Clone, Default)]
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

    pub fn with_max_history(max: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_history: max,
        }
    }

    pub fn add_user_message(&mut self, content: &str) {
        self.add_message(Role::User, content);
    }

    pub fn add_assistant_message(&mut self, content: &str) {
        self.add_message(Role::Assistant, content);
    }

    pub fn add_system_message(&mut self, content: &str) {
        self.add_message(Role::System, content);
    }

    fn add_message(&mut self, role: Role, content: &str) {
        self.messages.push(Message::new(role, content.to_string()));
        if self.messages.len() > self.max_history {
            self.messages.remove(0);
        }
    }

    pub fn get_last_user_message(&self) -> Option<&Message> {
        self.messages.iter().rev().find(|m| m.role == Role::User)
    }

    pub fn get_last_assistant_message(&self) -> Option<&Message> {
        self.messages.iter().rev().find(|m| m.role == Role::Assistant)
    }

    pub fn get_history(&self) -> &[Message] {
        &self.messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }
}
