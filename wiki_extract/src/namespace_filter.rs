// wiki_extract/src/namespace_filter.rs

pub struct NamespaceFilter {
    include_keys: Vec<i32>,
    exclude_keys: Vec<i32>,
}

impl NamespaceFilter {
    pub fn new() -> Self {
        Self {
            include_keys: vec![0, 14, 10, 100, 102],  // Articoli, Categorie, Template, Portali, Progetti
            exclude_keys: vec![
                -2, -1,  // Media, Speciale
                1, 2, 3, 4, 5, 6, 7, 8, 9,  // Discussioni, Utente, File, MediaWiki
                11, 12, 13, 15,  // Template talk, Aiuto, Aiuto talk, Categoria talk
                101, 103,  // Portale talk, Progetto talk
                828, 829,  // Modulo, Modulo talk
                2300, 2301, 2302, 2303,  // Gadget
                2600,  // Topic
            ],
        }
    }
    
    pub fn should_process(&self, ns: i32) -> bool {
        if self.exclude_keys.contains(&ns) {
            return false;
        }
        self.include_keys.contains(&ns)
    }
    
    pub fn is_article(&self, ns: i32) -> bool {
        ns == 0
    }
    
    pub fn is_category(&self, ns: i32) -> bool {
        ns == 14
    }
    
    pub fn is_template(&self, ns: i32) -> bool {
        ns == 10
    }
    
    pub fn get_namespace_name(&self, ns: i32) -> &'static str {
        match ns {
            0 => "Articolo",
            14 => "Categoria",
            10 => "Template",
            100 => "Portale",
            102 => "Progetto",
            _ => "Altro",
        }
    }
}
