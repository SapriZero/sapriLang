// wiki_extract/src/fast_extractor.rs

use bzip2::read::BzDecoder;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};

fn main() {
    let input_file = "itwiki-latest-pages-articles.xml.bz2";
    let file = File::open(input_file).unwrap();
    let decoder = BzDecoder::new(file);
    let mut reader = Reader::from_reader(decoder);
    let mut buf = Vec::new();
    
    let mut articles = Vec::new();
    let mut categories = HashMap::new();
    let mut redirects = Vec::new();
    
    let mut in_page = false;
    let mut in_title = false;
    let mut in_ns = false;
    let mut in_text = false;
    let mut current_title = String::new();
    let mut current_ns = 0;
    let mut current_id = 0;
    let mut current_redirect = String::new();
    let mut current_categories = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"page" => {
                in_page = true;
                current_title.clear();
                current_ns = 0;
                current_id = 0;
                current_redirect.clear();
                current_categories.clear();
            }
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"title" => {
                in_title = true;
            }
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"ns" => {
                in_ns = true;
            }
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"text" => {
                in_text = true;
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap();
                if in_title {
                    current_title = text.to_string();
                } else if in_ns {
                    current_ns = text.parse().unwrap_or(0);
                } else if in_text {
                    // Estrai solo categorie (non salvare tutto il testo)
                    for cap in CATEGORY_RE.captures_iter(&text) {
                        current_categories.push(cap[1].to_string());
                    }
                }
            }
            Ok(Event::Empty(ref e)) if e.name().as_ref() == b"redirect" => {
                if let Some(attr) = e.try_get_attribute(b"title") {
                    if let Ok(attr_val) = attr {
                        current_redirect = attr_val.unescape_value().unwrap().to_string();
                    }
                }
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"title" => {
                in_title = false;
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"ns" => {
                in_ns = false;
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"text" => {
                in_text = false;
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"page" => {
                in_page = false;
                
                // Filtra per namespace
                if should_process(current_ns) {
                    articles.push(Article {
                        title: current_title.clone(),
                        ns: current_ns,
                        id: current_id,
                        redirect: current_redirect.clone(),
                        categories: current_categories.clone(),
                    });
                    
                    if !current_redirect.is_empty() {
                        redirects.push((current_title.clone(), current_redirect.clone()));
                    }
                }
                
                if current_ns == 14 && !current_categories.is_empty() {
                    categories.insert(current_title, current_categories.clone());
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("Errore: {}", e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }
    
    // Salva output
    save_articles(&articles);
    save_categories(&categories);
    save_redirects(&redirects);
}

fn should_process(ns: i32) -> bool {
    matches!(ns, 0 | 14 | 10 | 100 | 102)
}
