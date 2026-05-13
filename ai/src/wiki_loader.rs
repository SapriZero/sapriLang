//! Caricamento dati da wiki_extract

use crate::knowledge::KnowledgeBase;
use wiki_extract::models::Article;
use wiki_extract::storage::load_articles;
use std::path::Path;

/// Caricatore di conoscenza da Wikipedia
pub struct WikiLoader;

impl WikiLoader {
    /// Carica tutti gli articoli dal file .sson di wiki_extract
    pub fn load_articles(knowledge: &mut KnowledgeBase, path: &str) -> Result<usize, String> {
        let articles = load_articles(path).map_err(|e| e.to_string())?;
        let mut count = 0;
        
        for article in articles {
            // Formato riga: id|title|start_byte|end_byte|categories
            let id = article.id.to_string();
            let title = article.title;
            let categories = article.categories.join(",");
            
            knowledge.add_article(&id, &title, &categories)?;
            count += 1;
            
            if count % 10000 == 0 {
                println!("📚 Caricati {} articoli...", count);
            }
        }
        
        Ok(count)
    }
    
    /// Carica solo le categorie
    pub fn load_categories(knowledge: &mut KnowledgeBase, path: &str) -> Result<usize, String> {
        use wiki_extract::storage::load_categories;
        let categories = load_categories(path).map_err(|e| e.to_string())?;
        let mut count = 0;
        
        for cat in categories {
            knowledge.add_word(&cat.name, "category")?;
            count += 1;
        }
        
        Ok(count)
    }
}
