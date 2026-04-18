//! Modelli dati per l'estrazione veloce di Wikipedia
//! Definisce le strutture per articoli, categorie, template, reindirizzamenti, portali e progetti

use serde::{Serialize, Deserialize};

// ============================================
// ARTICOLI (namespace 0)
// ============================================

/// Articolo Wikipedia (namespace 0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    /// ID univoco della pagina
    pub id: u32,
    /// Titolo dell'articolo
    pub title: String,
    /// Posizione di inizio nel file bz2 compresso
    pub byte_start: u64,
    /// Posizione di fine nel file bz2 compresso
    pub byte_end: u64,
    /// Lista delle categorie di appartenenza
    pub categories: Vec<String>,
}

impl Article {
    pub fn new(id: u32, title: String, byte_start: u64, byte_end: u64) -> Self {
        Self {
            id,
            title,
            byte_start,
            byte_end,
            categories: Vec::new(),
        }
    }
    
    pub fn add_category(&mut self, category: String) {
        if !self.categories.contains(&category) {
            self.categories.push(category);
        }
    }
    
    /// Formato CSV: id|title|byte_start|byte_end|categories
    pub fn to_csv(&self) -> String {
        let cats = self.categories.join(",");
        format!("{}|{}|{}|{}|{}", self.id, self.title, self.byte_start, self.byte_end, cats)
    }
}

// ============================================
// CATEGORIE (namespace 14)
// ============================================

/// Categoria Wikipedia (namespace 14)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    /// ID univoco della pagina
    pub id: u32,
    /// Nome della categoria (con prefisso "Categoria:")
    pub title: String,
    /// Categoria padre (da cui dipende)
    pub parent_category: String,
}

impl Category {
    pub fn new(id: u32, title: String, parent_category: String) -> Self {
        Self {
            id,
            title,
            parent_category,
        }
    }
    
    /// Rimuove il prefisso "Categoria:" dal titolo
    pub fn short_name(&self) -> String {
        self.title.replace("Categoria:", "")
    }
    
    /// Formato CSV: id|title|parent_category
    pub fn to_csv(&self) -> String {
        format!("{}|{}|{}", self.id, self.title, self.parent_category)
    }
}

// ============================================
// TEMPLATE (namespace 10)
// ============================================

/// Template Wikipedia (namespace 10)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    /// ID univoco della pagina
    pub id: u32,
    /// Nome del template (con prefisso "Template:")
    pub title: String,
    /// Posizione di inizio nel file bz2 compresso
    pub byte_start: u64,
    /// Posizione di fine nel file bz2 compresso
    pub byte_end: u64,
}

impl Template {
    pub fn new(id: u32, title: String, byte_start: u64, byte_end: u64) -> Self {
        Self {
            id,
            title,
            byte_start,
            byte_end,
        }
    }
    
    /// Rimuove il prefisso "Template:" dal titolo
    pub fn short_name(&self) -> String {
        self.title.replace("Template:", "")
    }
    
    /// Formato CSV: id|title|byte_start|byte_end
    pub fn to_csv(&self) -> String {
        format!("{}|{}|{}|{}", self.id, self.title, self.byte_start, self.byte_end)
    }
}

// ============================================
// REINDIRIZZAMENTI
// ============================================

/// Reindirizzamento Wikipedia
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redirect {
    /// Titolo della pagina che reindirizza
    pub from: String,
    /// Titolo della pagina di destinazione
    pub to: String,
}

impl Redirect {
    pub fn new(from: String, to: String) -> Self {
        Self { from, to }
    }
    
    /// Formato CSV: from|to
    pub fn to_csv(&self) -> String {
        format!("{}|{}", self.from, self.to)
    }
}

// ============================================
// PORTALI (namespace 100)
// ============================================

/// Portale Wikipedia (namespace 100)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portal {
    /// ID univoco della pagina
    pub id: u32,
    /// Nome del portale (con prefisso "Portale:")
    pub title: String,
}

impl Portal {
    pub fn new(id: u32, title: String) -> Self {
        Self { id, title }
    }
    
    /// Rimuove il prefisso "Portale:" dal titolo
    pub fn short_name(&self) -> String {
        self.title.replace("Portale:", "")
    }
    
    /// Formato CSV: id|title
    pub fn to_csv(&self) -> String {
        format!("{}|{}", self.id, self.title)
    }
}

// ============================================
// PROGETTI (namespace 102)
// ============================================

/// Progetto Wikipedia (namespace 102)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// ID univoco della pagina
    pub id: u32,
    /// Nome del progetto (con prefisso "Progetto:")
    pub title: String,
}

impl Project {
    pub fn new(id: u32, title: String) -> Self {
        Self { id, title }
    }
    
    /// Rimuove il prefisso "Progetto:" dal titolo
    pub fn short_name(&self) -> String {
        self.title.replace("Progetto:", "")
    }
    
    /// Formato CSV: id|title
    pub fn to_csv(&self) -> String {
        format!("{}|{}", self.id, self.title)
    }
}

// ============================================
// COLLEZIONI
// ============================================

/// Contenitore per tutti i dati estratti
#[derive(Debug, Clone, Default)]
pub struct ExtractedData {
    pub articles: Vec<Article>,
    pub categories: Vec<Category>,
    pub templates: Vec<Template>,
    pub redirects: Vec<Redirect>,
    pub portals: Vec<Portal>,
    pub projects: Vec<Project>,
}

impl ExtractedData {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn is_empty(&self) -> bool {
        self.articles.is_empty() && 
        self.categories.is_empty() && 
        self.templates.is_empty() && 
        self.redirects.is_empty() &&
        self.portals.is_empty() &&
        self.projects.is_empty()
    }
    
    pub fn total_count(&self) -> usize {
        self.articles.len() + 
        self.categories.len() + 
        self.templates.len() + 
        self.redirects.len() +
        self.portals.len() +
        self.projects.len()
    }
}

// ============================================
// TEST
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_article_to_csv() {
        let mut article = Article::new(12345, "Italia".to_string(), 1048576, 2097152);
        article.add_category("Stati".to_string());
        article.add_category("Europa".to_string());
        
        let csv = article.to_csv();
        assert_eq!(csv, "12345|Italia|1048576|2097152|Stati,Europa");
    }
    
    #[test]
    fn test_category_short_name() {
        let cat = Category::new(23456, "Categoria:Stati".to_string(), "Categorie principali".to_string());
        assert_eq!(cat.short_name(), "Stati");
        assert_eq!(cat.to_csv(), "23456|Categoria:Stati|Categorie principali");
    }
    
    #[test]
    fn test_template_short_name() {
        let tmpl = Template::new(34567, "Template:Infobox".to_string(), 6291457, 7340032);
        assert_eq!(tmpl.short_name(), "Infobox");
        assert_eq!(tmpl.to_csv(), "34567|Template:Infobox|6291457|7340032");
    }
    
    #[test]
    fn test_redirect_to_csv() {
        let redir = Redirect::new("Italia".to_string(), "Italia".to_string());
        assert_eq!(redir.to_csv(), "Italia|Italia");
    }
    
    #[test]
    fn test_portal_short_name() {
        let portal = Portal::new(45678, "Portale:Musica".to_string());
        assert_eq!(portal.short_name(), "Musica");
        assert_eq!(portal.to_csv(), "45678|Portale:Musica");
    }
    
    #[test]
    fn test_project_short_name() {
        let project = Project::new(56789, "Progetto:Matematica".to_string());
        assert_eq!(project.short_name(), "Matematica");
        assert_eq!(project.to_csv(), "56789|Progetto:Matematica");
    }
}
