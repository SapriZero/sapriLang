use std::path::Path;
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::time::Duration;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, EventKind, Config};
use std::fs;
use crate::parser::Parser;
use crate::vm::VM;

pub struct LiveInterpreter {
    file_path: String,
    parser: Parser,
    vm: Option<VM>,
    last_content: String,
}

impl LiveInterpreter {
    pub fn new(file_path: &str) -> anyhow::Result<Self> {
        let content = fs::read_to_string(file_path)?;
        
        Ok(LiveInterpreter {
            file_path: file_path.to_string(),
            parser: Parser::new(),
            vm: None,
            last_content: content,
        })
    }
    
    fn reload(&mut self) -> anyhow::Result<()> {
        println!("\n🔄 Ricarico file...");
        let content = fs::read_to_string(&self.file_path)?;
        
        if content == self.last_content {
            return Ok(());
        }
        
        self.last_content = content.clone();
        let mut parser = Parser::new();
        parser.parse(&content)?;
        
        let mut vm = VM::new(parser);
        vm.run()?;
        
        self.vm = Some(vm);
        println!("✅ Ricaricato");
        if let Some(vm) = &self.vm {
            vm.dump();
        }
        
        Ok(())
    }
    
    pub fn run(&mut self) -> anyhow::Result<()> {
        self.reload()?;
        
        let (tx, rx) = channel();
        
        let mut watcher: RecommendedWatcher = Watcher::new(
            move |res: notify::Result<notify::Event>| {
                if let Ok(event) = res {
                    if matches!(event.kind, EventKind::Modify(_)) {
                        let _ = tx.send(());
                    }
                }
            },
            Config::default()
        )?;
        
        watcher.watch(Path::new(&self.file_path), RecursiveMode::NonRecursive)?;
        
        println!("👀 In attesa di modifiche... (Ctrl+C per uscire)");
        
        loop {
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(()) => {
                    self.reload()?;
                }
                Err(RecvTimeoutError::Timeout) => {}
                Err(e) => {
                    eprintln!("Errore: {:?}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
}
