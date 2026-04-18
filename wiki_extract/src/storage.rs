//! Storage - Salvataggio dei dati estratti in formato CSV
//! Formato: separatore '|', liste separate da ','

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;

use crate::models::{
    Article, Category, Template, Redirect, Portal, Project, ExtractedData
};

// ============================================
// COSTANTI
// ============================================

const ARTICLES_HEADER: &str = "# id|title|byte_start|byte_end|categories\n";
const CATEGORIES_HEADER: &str = "# id|title|parent_category\n";
const TEMPLATES_HEADER: &str = "# id|title|byte_start|byte_end\n";
const REDIRECTS_HEADER: &str = "# from|to\n";
const PORTALS_HEADER: &str = "# id|title\n";
const PROJECTS_HEADER: &str = "# id|title\n";

// ============================================
// SALVATAGGIO SINGOLI FILE
// ============================================

pub fn save_articles(articles: &[Article], output_dir: &str) -> Result<(), String> {
    let path = Path::new(output_dir).join("articles.sson");
    let file = File::create(&path).map_err(|e| format!("Errore creazione {}: {}", path.display(), e))?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(ARTICLES_HEADER.as_bytes()).map_err(|e| e.to_string())?;
    
    for article in articles {
        let csv_line = format!("{}|{}|{}|{}|{}\n", 
            article.id, 
            article.title, 
            article.byte_start, 
            article.byte_end,
            article.categories.join(",")
        );
        writer.write_all(csv_line.as_bytes()).map_err(|e| e.to_string())?;
    }
    
    println!("✅ Salvati {} articoli in {}", articles.len(), path.display());
    Ok(())
}

pub fn save_categories(categories: &[Category], output_dir: &str) -> Result<(), String> {
    let path = Path::new(output_dir).join("categories.sson");
    let file = File::create(&path).map_err(|e| format!("Errore creazione {}: {}", path.display(), e))?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(CATEGORIES_HEADER.as_bytes()).map_err(|e| e.to_string())?;
    
    for category in categories {
        let csv_line = format!("{}|{}|{}\n", category.id, category.title, category.parent_category);
        writer.write_all(csv_line.as_bytes()).map_err(|e| e.to_string())?;
    }
    
    println!("✅ Salvate {} categorie in {}", categories.len(), path.display());
    Ok(())
}

pub fn save_templates(templates: &[Template], output_dir: &str) -> Result<(), String> {
    let path = Path::new(output_dir).join("templates.sson");
    let file = File::create(&path).map_err(|e| format!("Errore creazione {}: {}", path.display(), e))?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(TEMPLATES_HEADER.as_bytes()).map_err(|e| e.to_string())?;
    
    for template in templates {
        let csv_line = format!("{}|{}|{}|{}\n", template.id, template.title, template.byte_start, template.byte_end);
        writer.write_all(csv_line.as_bytes()).map_err(|e| e.to_string())?;
    }
    
    println!("✅ Salvati {} template in {}", templates.len(), path.display());
    Ok(())
}

pub fn save_redirects(redirects: &[Redirect], output_dir: &str) -> Result<(), String> {
    let path = Path::new(output_dir).join("redirects.sson");
    let file = File::create(&path).map_err(|e| format!("Errore creazione {}: {}", path.display(), e))?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(REDIRECTS_HEADER.as_bytes()).map_err(|e| e.to_string())?;
    
    for redirect in redirects {
        let csv_line = format!("{}|{}\n", redirect.from, redirect.to);
        writer.write_all(csv_line.as_bytes()).map_err(|e| e.to_string())?;
    }
    
    println!("✅ Salvati {} reindirizzamenti in {}", redirects.len(), path.display());
    Ok(())
}

pub fn save_portals(portals: &[Portal], output_dir: &str) -> Result<(), String> {
    let path = Path::new(output_dir).join("portals.sson");
    let file = File::create(&path).map_err(|e| format!("Errore creazione {}: {}", path.display(), e))?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(PORTALS_HEADER.as_bytes()).map_err(|e| e.to_string())?;
    
    for portal in portals {
        let csv_line = format!("{}|{}\n", portal.id, portal.title);
        writer.write_all(csv_line.as_bytes()).map_err(|e| e.to_string())?;
    }
    
    println!("✅ Salvati {} portali in {}", portals.len(), path.display());
    Ok(())
}

pub fn save_projects(projects: &[Project], output_dir: &str) -> Result<(), String> {
    let path = Path::new(output_dir).join("projects.sson");
    let file = File::create(&path).map_err(|e| format!("Errore creazione {}: {}", path.display(), e))?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(PROJECTS_HEADER.as_bytes()).map_err(|e| e.to_string())?;
    
    for project in projects {
        let csv_line = format!("{}|{}\n", project.id, project.title);
        writer.write_all(csv_line.as_bytes()).map_err(|e| e.to_string())?;
    }
    
    println!("✅ Salvati {} progetti in {}", projects.len(), path.display());
    Ok(())
}

// ============================================
// SALVATAGGIO COLLETTIVO
// ============================================

pub fn save_all(data: &ExtractedData, output_dir: &str) -> Result<(), String> {
    std::fs::create_dir_all(output_dir).map_err(|e| format!("Errore creazione directory {}: {}", output_dir, e))?;
    
    if !data.articles.is_empty() {
        save_articles(&data.articles, output_dir)?;
    }
    if !data.categories.is_empty() {
        save_categories(&data.categories, output_dir)?;
    }
    if !data.templates.is_empty() {
        save_templates(&data.templates, output_dir)?;
    }
    if !data.redirects.is_empty() {
        save_redirects(&data.redirects, output_dir)?;
    }
    if !data.portals.is_empty() {
        save_portals(&data.portals, output_dir)?;
    }
    if !data.projects.is_empty() {
        save_projects(&data.projects, output_dir)?;
    }
    
    println!("\n📊 Riepilogo salvataggio:");
    println!("   Articoli: {}", data.articles.len());
    println!("   Categorie: {}", data.categories.len());
    println!("   Template: {}", data.templates.len());
    println!("   Reindirizzamenti: {}", data.redirects.len());
    println!("   Portali: {}", data.portals.len());
    println!("   Progetti: {}", data.projects.len());
    println!("   Totale: {}", data.total_count());
    
    Ok(())
}

// ============================================
// CHECKPOINT
// ============================================

pub fn save_checkpoint(
    output_dir: &str,
    page_count: u64,
    byte_pos: u64,
    last_title: &str,
    data: &ExtractedData,
) -> Result<(), String> {
    let checkpoint_path = Path::new(output_dir).join("checkpoint.json");
    
    let checkpoint = serde_json::json!({
        "page_count": page_count,
        "byte_position": byte_pos,
        "last_page_title": last_title,
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        "articles_count": data.articles.len(),
        "categories_count": data.categories.len(),
        "templates_count": data.templates.len(),
        "redirects_count": data.redirects.len(),
        "portals_count": data.portals.len(),
        "projects_count": data.projects.len(),
    });
    
    let json = serde_json::to_string_pretty(&checkpoint).map_err(|e| e.to_string())?;
    std::fs::write(&checkpoint_path, json).map_err(|e| e.to_string())?;
    
    Ok(())
}

pub fn load_checkpoint(output_dir: &str) -> Result<Option<serde_json::Value>, String> {
    let checkpoint_path = Path::new(output_dir).join("checkpoint.json");
    
    if !checkpoint_path.exists() {
        return Ok(None);
    }
    
    let json = std::fs::read_to_string(&checkpoint_path).map_err(|e| e.to_string())?;
    let checkpoint = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    
    Ok(Some(checkpoint))
}

// ============================================
// FUNZIONI LEGACY (per compatibilità)
// ============================================

pub fn save_words(_words: &std::collections::HashMap<String, u32>, _path: &str) -> Result<(), String> {
    Ok(())
}

pub fn save_verbs(_verbs: &std::collections::HashMap<String, u32>, _path: &str) -> Result<(), String> {
    Ok(())
}

pub fn save_nouns(_nouns: &std::collections::HashMap<String, u32>, _path: &str) -> Result<(), String> {
    Ok(())
}

// ============================================
// PROGRESS BAR
// ============================================

pub struct ProgressBar {
    total_bytes: u64,
    processed_bytes: u64,
    start_time: Instant,
    last_update: Instant,
}

impl ProgressBar {
    pub fn new(total_bytes: u64) -> Self {
        Self {
            total_bytes,
            processed_bytes: 0,
            start_time: Instant::now(),
            last_update: Instant::now(),
        }
    }
    
    pub fn update(&mut self, bytes_read: u64) {
        self.processed_bytes = bytes_read;
        
        let now = Instant::now();
        if now.duration_since(self.last_update).as_millis() >= 500 {
            self.last_update = now;
            self.display();
        }
    }
    
    pub fn display(&self) {
        let percent = (self.processed_bytes as f64 / self.total_bytes as f64) * 100.0;
        let mb_processed = self.processed_bytes as f64 / 1024.0 / 1024.0;
        let mb_total = self.total_bytes as f64 / 1024.0 / 1024.0;
        let elapsed = self.start_time.elapsed();
        let speed = mb_processed / elapsed.as_secs_f64();
        let eta = if speed > 0.0 {
            (mb_total - mb_processed) / speed
        } else {
            0.0
        };
        
        // Barra di progresso di 30 caratteri
        let bar_len = 30;
        let filled = (bar_len as f64 * percent / 100.0) as usize;
        let bar = "█".repeat(filled) + &"░".repeat(bar_len - filled);
        
        print!("\r[{bar}] {:.1}% | {:.1}/{:.0} MB | {:.1} MB/s | ETA: {:.0}s    ",
            percent, mb_processed, mb_total, speed, eta);
        std::io::stdout().flush().unwrap();
    }
    
    pub fn finish(&self) {
        println!();
    }
}
