//! Servizio per la gestione dei file

use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modified: String,
}

pub fn list_files(dir: &Path) -> Result<Vec<FileInfo>, std::io::Error> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("sapri") {
                if let Ok(metadata) = fs::metadata(&path) {
                    files.push(FileInfo {
                        name: path.file_name().unwrap().to_string_lossy().to_string(),
                        path: path.to_string_lossy().to_string(),
                        size: metadata.len(),
                        modified: metadata.modified()
                            .map(|t| format!("{:?}", t))
                            .unwrap_or_default(),
                    });
                }
            }
        }
    }

    Ok(files)
}

pub fn read_file(dir: &Path, name: &str) -> Result<String, std::io::Error> {
    let path = dir.join(name);
    fs::read_to_string(path)
}

pub fn write_file(dir: &Path, name: &str, content: &str) -> Result<(), std::io::Error> {
    let path = dir.join(name);
    fs::write(path, content)
}

pub fn delete_file(dir: &Path, name: &str) -> Result<(), std::io::Error> {
    let path = dir.join(name);
    fs::remove_file(path)
}
