//! Runtime principale del sistema

use std::io::{self, Write};
use crate::config::Config;
use crate::executor::Executor;
use crate::hot_reload::HotReload;
use crate::command::Command;
use sapri_rust_dsl::AtomValue;

pub struct Runtime {
    config: Config,
    executor: Executor,
    hot_reload: Option<HotReload>,
    running: bool,
}

impl Runtime {
    pub fn new(config_path: Option<&str>) -> Result<Self, String> {
        let config = if let Some(path) = config_path {
            Config::load(path)?
        } else {
            Config::default()
        };
        
        let executor = Executor::with_initial(config.initial_atoms.clone());
        
        let hot_reload = if let Some(path) = config_path {
            Some(HotReload::new(path)?)
        } else {
            None
        };
        
        Ok(Self {
            config,
            executor,
            hot_reload,
            running: true,
        })
    }
    
    pub fn execute(&mut self, cmd: Command) -> Result<String, String> {
        match cmd {
            Command::Reset => {
                if let Some(hr) = &mut self.hot_reload {
                    let new_config = hr.reset()?;
                    self.config = new_config.clone();
                    Ok("Configuration reloaded".to_string())
                } else {
                    Err("No config file to reload".to_string())
                }
            }
            
            Command::Load { path } => {
                let new_config = Config::load(&path)?;
                self.config = new_config;
                Ok(format!("Loaded configuration from {}", path))
            }
            
            Command::Eval { expr } => {
                let result = self.executor.eval(&expr)?;
                Ok(format!("{} = {:?}", expr, result))
            }
            
            Command::Define { name, value } => {
                let atom_value = if let Ok(n) = value.parse::<f64>() {
                    AtomValue::Number(n)
                } else {
                    AtomValue::String(value)
                };
                let display_value = atom_value.clone();
                self.executor.define(&name, atom_value);
                Ok(format!("Defined {} = {:?}", name, display_value))
            }
            
            Command::Get { name } => {
                match self.executor.get(&name) {
                    Some(value) => Ok(format!("{} = {:?}", name, value)),
                    None => Err(format!("Atom '{}' not found", name)),
                }
            }
            
            Command::List => {
                let atoms = self.executor.list_atoms();
                if atoms.is_empty() {
                    Ok("No atoms defined".to_string())
                } else {
                    let lines: Vec<String> = atoms.iter()
                        .map(|(k, v)| format!("  {} = {}", k, v))
                        .collect();
                    Ok(format!("Atoms:\n{}", lines.join("\n")))
                }
            }
            
            Command::Stats => {
                Ok(format!(
                    "Mode: {:?}\nMax depth: {}\nHistory size: {}\nVersion: {}",
                    self.config.mode,
                    self.config.max_depth,
                    self.executor.history().len(),
                    self.config.version
                ))
            }
            
            Command::Exit => {
                self.running = false;
                Ok("Goodbye!".to_string())
            }
            
            Command::Unknown(cmd) => {
                Err(format!("Unknown command: {}", cmd))
            }
        }
    }
    
    pub fn run(&mut self) {
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║                    SAPRI CORE v{}                          ║", env!("CARGO_PKG_VERSION"));
        println!("║  Commands: reset, load <file>, eval <expr>, define <n> <v>║");
        println!("║            get <name>, list, stats, exit                  ║");
        println!("╚════════════════════════════════════════════════════════════╝\n");
        
        while self.running {
            if let Some(hr) = &mut self.hot_reload {
                match hr.apply_if_needed() {
                    Ok(Some(new_config)) => {
                        self.config = new_config.clone();
                        println!("\n🔄 Configuration reloaded (hot-reload)\n");
                    }
                    Ok(None) => {}
                    Err(e) => eprintln!("Hot-reload error: {}", e),
                }
            }
            
            print!("> ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }
            
            let cmd = Command::parse(&input);
            match self.execute(cmd) {
                Ok(output) => println!("{}", output),
                Err(err) => eprintln!("Error: {}", err),
            }
        }
    }
}
