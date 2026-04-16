//! Hot-reload della configurazione a runtime

use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use crate::config::Config;

pub struct HotReload {
    config_path: PathBuf,
    current_config: Option<Config>,
    reload_rx: mpsc::Receiver<()>,
    _watcher: RecommendedWatcher,
}

impl HotReload {
    pub fn new(config_path: &str) -> Result<Self, String> {
        let config_path = PathBuf::from(config_path);
        let (reload_tx, reload_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        
        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = event_tx.send(res);
        }).map_err(|e| format!("Failed to create watcher: {}", e))?;
        
        watcher.watch(&config_path, RecursiveMode::NonRecursive)
            .map_err(|e| format!("Failed to watch file: {}", e))?;
        
        let reload_tx_clone = reload_tx.clone();
        thread::spawn(move || {
            for event in event_rx {
                match event {
                    Ok(notify::Event { kind: notify::EventKind::Modify(_), .. }) => {
                        thread::sleep(Duration::from_millis(100));
                        let _ = reload_tx_clone.send(());
                    }
                    _ => {}
                }
            }
        });
        
        let current_config = Config::load(config_path.to_str().unwrap()).ok();
        
        Ok(Self {
            config_path,
            current_config,
            reload_rx,
            _watcher: watcher,
        })
    }
    
    pub fn reload(&mut self) -> Result<&Config, String> {
        let new_config = Config::load(self.config_path.to_str().unwrap())?;
        self.current_config = Some(new_config);
        Ok(self.current_config.as_ref().unwrap())
    }
    
    pub fn reset(&mut self) -> Result<&Config, String> {
        self.reload()
    }
    
    pub fn current(&self) -> Option<&Config> {
        self.current_config.as_ref()
    }
    
    pub fn needs_reload(&self) -> bool {
        self.reload_rx.try_recv().is_ok()
    }
    
    pub fn apply_if_needed(&mut self) -> Result<Option<&Config>, String> {
        if self.needs_reload() {
            self.reload().map(Some)
        } else {
            Ok(None)
        }
    }
}
