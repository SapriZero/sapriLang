//! Comandi eseguibili dal runtime

/// Comandi supportati dal runtime
#[derive(Debug, Clone)]
pub enum Command {
    /// Ricarica la configurazione da file
    Reset,
    /// Carica un nuovo file .sson
    Load { path: String },
    /// Valuta un'espressione IRCM
    Eval { expr: String },
    /// Definisce un atomo
    Define { name: String, value: String },
    /// Legge un atomo
    Get { name: String },
    /// Mostra tutti gli atomi definiti
    List,
    /// Mostra statistiche
    Stats,
    /// Esce dal runtime
    Exit,
    /// Comando sconosciuto
    Unknown(String),
}

impl Command {
    /// Parsea una stringa in un comando
    pub fn parse(input: &str) -> Self {
        let input = input.trim();
        
        if input.is_empty() {
            return Command::Unknown("empty".to_string());
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts[0].to_lowercase();
        
        match cmd.as_str() {
            "reset" => Command::Reset,
            "load" => {
                if parts.len() > 1 {
                    Command::Load { path: parts[1].to_string() }
                } else {
                    Command::Unknown("load requires path".to_string())
                }
            }
            "eval" => {
                if parts.len() > 1 {
                    Command::Eval { expr: parts[1..].join(" ") }
                } else {
                    Command::Unknown("eval requires expression".to_string())
                }
            }
            "define" => {
                if parts.len() >= 3 {
                    Command::Define { 
                        name: parts[1].to_string(), 
                        value: parts[2..].join(" ") 
                    }
                } else {
                    Command::Unknown("define requires name and value".to_string())
                }
            }
            "get" => {
                if parts.len() > 1 {
                    Command::Get { name: parts[1].to_string() }
                } else {
                    Command::Unknown("get requires name".to_string())
                }
            }
            "list" => Command::List,
            "stats" => Command::Stats,
            "exit" | "quit" | "q" => Command::Exit,
            _ => Command::Unknown(input.to_string()),
        }
    }
}
