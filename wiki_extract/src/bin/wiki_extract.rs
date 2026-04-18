//! Eseguibile per estrarre metadati da Wikipedia con checkpoint

use bzip2::read::BzDecoder;
use quick_xml::Reader;
use quick_xml::events::Event;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::time::Instant;

use sapri_wiki_extract::models::{
    Article, Category, Template, Redirect, Portal, Project, ExtractedData
};
use sapri_wiki_extract::storage::{
    save_all, save_checkpoint, load_checkpoint
};

// ============================================
// COSTANTI
// ============================================

lazy_static::lazy_static! {
    static ref CATEGORY_RE: Regex = Regex::new(r"\[\[Categoria:([^\]\|]+)").unwrap();
}

// ============================================
// STRUTTURE
// ============================================

struct BzBufReader<R: Read> {
    decoder: BzDecoder<R>,
    buffer: Vec<u8>,
    pos: usize,
}

impl<R: Read> BzBufReader<R> {
    fn new(reader: R) -> Self {
        Self {
            decoder: BzDecoder::new(reader),
            buffer: Vec::new(),
            pos: 0,
        }
    }
    
    fn fill_buffer(&mut self) -> std::io::Result<usize> {
        let mut chunk = vec![0u8; 8192];
        match self.decoder.read(&mut chunk) {
            Ok(n) if n > 0 => {
                self.buffer = chunk[..n].to_vec();
                self.pos = 0;
                Ok(n)
            }
            Ok(_) => Ok(0),
            Err(e) => Err(e),
        }
    }
}

impl<R: Read> Read for BzBufReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.pos >= self.buffer.len() {
            if self.fill_buffer()? == 0 {
                return Ok(0);
            }
        }
        let available = &self.buffer[self.pos..];
        let to_copy = buf.len().min(available.len());
        buf[..to_copy].copy_from_slice(&available[..to_copy]);
        self.pos += to_copy;
        Ok(to_copy)
    }
}

impl<R: Read> BufRead for BzBufReader<R> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if self.pos >= self.buffer.len() {
            if self.fill_buffer()? == 0 {
                return Ok(&[]);
            }
        }
        Ok(&self.buffer[self.pos..])
    }
    
    fn consume(&mut self, amt: usize) {
        self.pos += amt;
    }
}

// ============================================
// FUNZIONI DI PARSING
// ============================================

fn extract_categories(text: &str) -> Vec<String> {
    let mut categories = Vec::new();
    for cap in CATEGORY_RE.captures_iter(text) {
        let cat = cap[1].trim().to_string();
        if !categories.contains(&cat) {
            categories.push(cat);
        }
    }
    categories
}

fn should_process(ns: i32) -> bool {
    matches!(ns, 0 | 14 | 10 | 100 | 102)
}

fn get_input_path(args: &[String]) -> String {
    if args.len() > 1 && !args[1].starts_with('-') {
        args[1].clone()
    } else {
        "itwiki-latest-pages-articles.xml.bz2".to_string()
    }
}

fn get_output_dir(args: &[String]) -> String {
    let mut output_dir = "data".to_string();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--output" || args[i] == "-o" {
            if i + 1 < args.len() {
                output_dir = args[i + 1].clone();
                break;
            }
        }
        i += 1;
    }
    output_dir
}

fn use_checkpoint(args: &[String]) -> bool {
    !args.contains(&"--no-resume".to_string())
}

// ============================================
// MAIN
// ============================================

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = get_input_path(&args);
    let output_dir = get_output_dir(&args);
    let resume = use_checkpoint(&args);
    
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║           SAPRI WIKIPEDIA EXTRACTOR v0.2.0                ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    println!("📁 File input: {}", input_file);
    println!("📁 Output dir: {}", output_dir);
    
    if !Path::new(&input_file).exists() {
        eprintln!("❌ File non trovato: {}", input_file);
        std::process::exit(1);
    }
    
    std::fs::create_dir_all(&output_dir).unwrap();
    
    let start_time = Instant::now();
    
    // Variabili per il checkpoint
    let (mut page_count, mut byte_pos, mut data) = if resume {
        if let Ok(Some(checkpoint)) = load_checkpoint(&output_dir) {
            println!("📌 Checkpoint trovato! Ripresa elaborazione...\n");
            let pc = checkpoint.get("page_count").and_then(|v| v.as_u64()).unwrap_or(0);
            let bp = checkpoint.get("byte_position").and_then(|v| v.as_u64()).unwrap_or(0);
            println!("⏩ Ripresa dalla pagina {} (byte {})", pc, bp);
            (pc, bp, ExtractedData::new())
        } else {
            println!("🆕 Nuova elaborazione...\n");
            (0, 0, ExtractedData::new())
        }
    } else {
        println!("🆕 Elaborazione da capo (--no-resume)...\n");
        (0, 0, ExtractedData::new())
    };
    
    let file = File::open(&input_file).expect("File non trovato");
    let mut reader = BufReader::new(file);
    
    if byte_pos > 0 {
        reader.seek(SeekFrom::Start(byte_pos)).unwrap();
    }
    
    let bz_reader = BzBufReader::new(reader);
    let mut xml_reader = Reader::from_reader(bz_reader);
    let mut buf = Vec::new();
    
    let mut in_page = false;
    let mut in_title = false;
    let mut in_ns = false;
    let mut in_id = false;
    let mut in_text = false;
    
    let mut current_title = String::new();
    let mut current_ns = 0;
    let mut current_id = 0;
    let mut current_redirect = String::new();
    let mut current_text = String::new();
    let mut page_start = 0u64;
    
    let mut last_save = Instant::now();
    let mut pages_since_save = 0;
    
    println!("📖 Elaborazione in corso... (Ctrl+C per interrompere)\n");
    
    loop {
	    let pos = xml_reader.buffer_position() as u64;
	    
	    match xml_reader.read_event_into(&mut buf) {
	        Ok(Event::Start(ref e)) if e.name().as_ref() == b"page" => {
	            page_start = pos;
	            current_title.clear();
	            current_ns = 0;
	            current_id = 0;
	            current_redirect.clear();
	            current_text.clear();
	        }
	        Ok(Event::Start(ref e)) if e.name().as_ref() == b"title" => {
	            in_title = true;
	        }
	        Ok(Event::Start(ref e)) if e.name().as_ref() == b"ns" => {
	            in_ns = true;
	        }
	        Ok(Event::Start(ref e)) if e.name().as_ref() == b"id" => {
	            in_id = true;
	        }
	        Ok(Event::Start(ref e)) if e.name().as_ref() == b"text" => {
	            in_text = true;
	            current_text.clear();
	        }
	        Ok(Event::Empty(ref e)) if e.name().as_ref() == b"redirect" => {
	            if let Ok(Some(attr)) = e.try_get_attribute(b"title") {
	                if let Ok(attr_val) = attr.unescape_value() {
	                    current_redirect = attr_val.to_string();
	                }
	            }
	        }
	        Ok(Event::Text(e)) => {
	            let text = e.unescape().unwrap();
	            if in_title {
	                current_title = text.to_string();
	            } else if in_ns {
	                current_ns = text.parse().unwrap_or(0);
	            } else if in_id {
	                current_id = text.parse().unwrap_or(0);
	            } else if in_text {
	                current_text.push_str(&text);
	            }
	        }
	        Ok(Event::End(ref e)) if e.name().as_ref() == b"title" => {
	            in_title = false;
	        }
	        Ok(Event::End(ref e)) if e.name().as_ref() == b"ns" => {
	            in_ns = false;
	        }
	        Ok(Event::End(ref e)) if e.name().as_ref() == b"id" => {
	            in_id = false;
	        }
	        Ok(Event::End(ref e)) if e.name().as_ref() == b"text" => {
	            in_text = false;
	        }
	        Ok(Event::End(ref e)) if e.name().as_ref() == b"page" => {
	            let page_end = xml_reader.buffer_position() as u64;
	            page_count += 1;
	            pages_since_save += 1;
	            
	            if should_process(current_ns) {
	                let categories = extract_categories(&current_text);
	                
	                match current_ns {
	                    0 => {
	                        let mut article = Article::new(current_id, current_title.clone(), page_start, page_end);
	                        for cat in categories {
	                            article.add_category(cat);
	                        }
	                        data.articles.push(article);
	                    }
	                    14 => {
	                        let parent = if categories.is_empty() { String::new() } else { categories[0].clone() };
	                        data.categories.push(Category::new(current_id, current_title.clone(), parent));
	                    }
	                    10 => {
	                        data.templates.push(Template::new(current_id, current_title.clone(), page_start, page_end));
	                    }
	                    100 => {
	                        data.portals.push(Portal::new(current_id, current_title.clone()));
	                    }
	                    102 => {
	                        data.projects.push(Project::new(current_id, current_title.clone()));
	                    }
	                    _ => {}
	                }
	                
	                if !current_redirect.is_empty() {
	                    data.redirects.push(Redirect::new(current_title.clone(), current_redirect.clone()));
	                }
	            }
	            
	            // Salva ogni 1000 pagine o 30 secondi
	            if pages_since_save >= 1000 || last_save.elapsed().as_secs() > 30 {
	                let _ = save_checkpoint(&output_dir, page_count, page_end, &current_title, &data);
	                let _ = save_all(&data, &output_dir);
	                last_save = Instant::now();
	                pages_since_save = 0;
	                
	                let elapsed = start_time.elapsed();
	                let rate = page_count as f64 / elapsed.as_secs_f64();
	                println!("📊 Pagine: {} | Articoli: {} | Velocità: {:.1} pag/s",
	                         page_count, data.articles.len(), rate);
	            }
	        }
	        Ok(Event::Eof) => break,
	        Err(e) => {
	            eprintln!("Errore durante la lettura: {}", e);
	            break;
	        }
	        _ => {}
	    }
	    
	    byte_pos = xml_reader.buffer_position() as u64;
	    buf.clear();
	}
    
    println!("\n💾 Salvataggio finale...");
    let _ = save_all(&data, &output_dir);
    let _ = save_checkpoint(&output_dir, page_count, byte_pos, "EOF", &data);
    
    let elapsed = start_time.elapsed();
    println!("\n✅ Elaborazione completata!");
    println!("   Pagine: {}", page_count);
    println!("   Articoli: {}", data.articles.len());
    println!("   Categorie: {}", data.categories.len());
    println!("   Template: {}", data.templates.len());
    println!("   Reindirizzamenti: {}", data.redirects.len());
    println!("   Portali: {}", data.portals.len());
    println!("   Progetti: {}", data.projects.len());
    println!("   Tempo: {:.2?}", elapsed);
}
