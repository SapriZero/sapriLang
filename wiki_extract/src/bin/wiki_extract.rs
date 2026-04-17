//! Eseguibile per estrarre parole da Wikipedia con checkpoint
//! 
//! Uso: wiki-extract [FILE] [--resume]
//! 
//!   FILE    - File Wikipedia .bz2 da processare (default: itwiki-latest-pages-articles.xml.bz2)
//!   --resume- Riprende elaborazione interrotta (ignora checkpoint esistente)

use bzip2::read::BzDecoder;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::time::Instant;

use sapri_wiki_extract::extractor::extract_from_page;
use sapri_wiki_extract::storage::{save_words, save_verbs, save_nouns, save_checkpoint, load_checkpoint};
use sapri_wiki_extract::checkpoint::Checkpoint;

/// Wrapper che implementa BufRead per BzDecoder
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

fn print_usage() {
    println!("Uso: wiki-extract [OPZIONI] [FILE]");
    println!();
    println!("Opzioni:");
    println!("  --help          Mostra questo aiuto");
    println!("  --resume        Riprende elaborazione interrotta (usa checkpoint)");
    println!("  --no-resume     Ignora checkpoint, ricomincia da capo");
    println!("  --output DIR    Directory output (default: data/)");
    println!();
    println!("FILE: File Wikipedia .bz2 da processare");
    println!("      (default: itwiki-latest-pages-articles.xml.bz2)");
}

fn parse_args() -> (String, bool, bool, String) {
    let args: Vec<String> = env::args().collect();
    let mut input_file = "itwiki-latest-pages-articles.xml.bz2".to_string();
    let mut resume = true;  // di default usa checkpoint se esiste
    let mut output_dir = "data".to_string();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            "--resume" => {
                resume = true;
                i += 1;
            }
            "--no-resume" => {
                resume = false;
                i += 1;
            }
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    output_dir = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("❌ --output richiede una directory");
                    std::process::exit(1);
                }
            }
            arg if arg.starts_with('-') => {
                eprintln!("❌ Opzione sconosciuta: {}", arg);
                print_usage();
                std::process::exit(1);
            }
            arg => {
                input_file = arg.to_string();
                i += 1;
            }
        }
    }
    
    (input_file, resume, true, output_dir)
}
fn main() {
    let (input_file, resume, _verbose, output_dir) = parse_args();
    
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║           SAPRI WIKIPEDIA EXTRACTOR v0.1.0                ║");
    println!("║  Estrae parole, verbi, nomi da Wikipedia italiana        ║");
    println!("║  Con checkpoint e salvataggio incrementale               ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    println!("📁 File input: {}", input_file);
    println!("📁 Output dir: {}", output_dir);
    
    // Verifica che il file esista
    if !Path::new(&input_file).exists() {
        eprintln!("❌ File non trovato: {}", input_file);
        eprintln!("\nPer scaricare Wikipedia italiana:");
        eprintln!("  wget https://dumps.wikimedia.org/itwiki/latest/itwiki-latest-pages-articles.xml.bz2");
        std::process::exit(1);
    }
    
    // Crea directory output
    std::fs::create_dir_all(&output_dir).unwrap();
    
    let start_time = Instant::now();
    
    // Verifica se esiste checkpoint
    let checkpoint_path = format!("{}/checkpoint.json", output_dir);
    let (mut page_count, mut byte_pos, mut word_counts, mut verb_counts, mut noun_counts) =
        if resume && Checkpoint::exists(&checkpoint_path) {
            println!("📌 Checkpoint trovato! Ripresa elaborazione...\n");
            let cp = load_checkpoint().unwrap();
            (
                cp.page_count,
                cp.byte_position,
                cp.word_counts.into_iter().collect(),
                cp.verb_counts.into_iter().collect(),
                cp.noun_counts.into_iter().collect(),
            )
        } else {
            if resume {
                println!("🆕 Nessun checkpoint trovato, nuova elaborazione...\n");
            } else {
                println!("🆕 Elaborazione da capo (--no-resume)...\n");
            }
            (0, 0, HashMap::new(), HashMap::new(), HashMap::new())
        };
    
    let file = File::open(&input_file).expect("File non trovato");
    let mut reader = BufReader::new(file);
    
    // Salta alla posizione del checkpoint
    if byte_pos > 0 {
        reader.seek(SeekFrom::Start(byte_pos)).unwrap();
        println!("⏩ Ripresa dalla posizione byte: {}", byte_pos);
    }
    
    let bz_reader = BzBufReader::new(reader);
    let mut xml_reader = Reader::from_reader(bz_reader);
    let mut buf = Vec::new();
    let mut in_text = false;
    let mut current_title = String::new();
    let mut current_text = String::new();
    let mut last_save = Instant::now();
    
    println!("📖 Elaborazione in corso... (Ctrl+C per interrompere, riprenderà dal checkpoint)\n");
    println!("💾 I file .sson vengono salvati ogni 1000 pagine\n");
    
    loop {
        match xml_reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"page" => {
                // Nuova pagina
            }
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"title" => {
                in_text = false;
            }
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"text" => {
                in_text = true;
                current_text.clear();
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"page" => {
                page_count += 1;
                
                if !current_text.is_empty() {
                    let extracted = extract_from_page(&current_text, &current_title);
                    
                    for (word, count) in extracted.words {
                        *word_counts.entry(word).or_insert(0) += count;
                    }
                    for (verb, count) in extracted.verbs {
                        *verb_counts.entry(verb).or_insert(0) += count;
                    }
                    for (noun, count) in extracted.nouns {
                        *noun_counts.entry(noun).or_insert(0) += count;
                    }
                }
                
                // Salva checkpoint e file .sson ogni 1000 pagine o ogni 30 secondi
                if page_count % 1000 == 0 || last_save.elapsed().as_secs() > 30 {
                    // Salva checkpoint
                    let _ = save_checkpoint(&output_dir, page_count, byte_pos, &current_title,
                                            &word_counts, &verb_counts, &noun_counts);
                    
                    // Salva file .sson in modo incrementale
                    let _ = save_words(&word_counts, &format!("{}/words.sson", output_dir));
                    let _ = save_verbs(&verb_counts, &format!("{}/verbs.sson", output_dir));
                    let _ = save_nouns(&noun_counts, &format!("{}/nouns.sson", output_dir));
                    
                    last_save = Instant::now();
                    
                    let elapsed = start_time.elapsed();
                    let rate = page_count as f64 / elapsed.as_secs_f64();
                    println!("📊 Pagine: {} | Parole uniche: {} | Velocità: {:.1} pag/s",
                             page_count, word_counts.len(), rate);
                    println!("💾 Checkpoint e .sson salvati");
                }
                
                current_title.clear();
                current_text.clear();
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap();
                if in_text {
                    current_text.push_str(&text);
                } else {
                    current_title = text.to_string();
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("Errore: {}", e);
                break;
            }
            _ => {}
        }
        
        // Aggiorna posizione byte (approssimativa)
        byte_pos = xml_reader.buffer_position() as u64;
        buf.clear();
    }
    
    // Salvataggio finale
    println!("\n💾 Salvataggio finale...");
    let _ = save_words(&word_counts, &format!("{}/words.sson", output_dir));
    let _ = save_verbs(&verb_counts, &format!("{}/verbs.sson", output_dir));
    let _ = save_nouns(&noun_counts, &format!("{}/nouns.sson", output_dir));
    
    let elapsed = start_time.elapsed();
    println!("\n✅ Elaborazione completata!");
    println!("   Pagine: {}", page_count);
    println!("   Parole uniche: {}", word_counts.len());
    println!("   Verbi unici: {}", verb_counts.len());
    println!("   Nomi unici: {}", noun_counts.len());
    println!("   Tempo: {:.2?}", elapsed);
    println!();
    println!("📁 Output salvato in: {}/", output_dir);
    println!("   - words.sson (aggiornato ogni 1000 pagine)");
    println!("   - verbs.sson (aggiornato ogni 1000 pagine)");
    println!("   - nouns.sson (aggiornato ogni 1000 pagine)");
    println!("   - checkpoint.json (stato per ripresa)");
}
