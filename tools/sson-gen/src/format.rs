// ==========================================
// 1. FUNZIONI PURE DI FORMATTAZIONE (ATOMICHE)
// ==========================================

/// Determina il tipo di file dall'estensione
fn get_file_type(name: &str) -> &'static str {
    if name.ends_with(".rs") {
        "rust"
    } else if name.ends_with(".toml") {
        "toml"
    } else if name.ends_with(".md") {
        "markdown"
    } else {
        ""
    }
}

/// Genera la riga per una directory
fn format_dir_line(name: &str, depth: usize) -> String {
    let indent = "  ".repeat(depth);
    format!("{}{}/\n", indent, name)
}

/// Genera la riga per un file (con o senza tipo)
fn format_file_line(name: &str, depth: usize) -> String {
    let indent = "  ".repeat(depth);
    let file_type = get_file_type(name);
    
    if file_type.is_empty() {
        format!("{}{}\n", indent, name)
    } else {
        format!("{}{} ({})\n", indent, name, file_type)
    }
}

// ==========================================
// 2. FUNZIONE PRINCIPALE (STRATEGIA)
// ==========================================

/// Strategia di formattazione: decide come formattare in base al tipo
fn format_node(node: &TreeNode, depth: usize, include_src: bool) -> String {
    // Array di funzioni di formattazione (strategie)
    let formatters: [fn(&TreeNode, usize) -> String; 2] = [
        // Formatter per directory
        |node, depth| format_dir_line(&node.name, depth),
        // Formatter per file
        |node, depth| format_file_line(&node.name, depth),
    ];
    
    // Seleziona il formatter appropriato
    let formatter = if node.is_dir {
        formatters[0]
    } else {
        formatters[1]
    };
    
    formatter(node, depth)
}

// ==========================================
// 3. FUNZIONE RICORSIVA (PURA)
// ==========================================

/// Versione funzionale pura di format
fn format_node_recursive(node: &TreeNode, depth: usize, include_src: bool) -> String {
    let mut result = String::new();
    
    // Formatta il nodo corrente se non è la root
    if depth > 0 {
        result.push_str(&format_node(node, depth, include_src));
    }
    
    // Formatta ricorsivamente i figli
    let children_results: Vec<String> = node.children
        .values()
        .map(|child| format_node_recursive(child, depth + 1, include_src))
        .collect();
    
    result.push_str(&children_results.concat());
    result
}

// ==========================================
// 4. VERSIONE CON PIPELINE (FUNZIONALE ESTREMA)
// ==========================================

/// Versione con pipeline funzionale
fn format_node_pipeline(node: &TreeNode, depth: usize, include_src: bool) -> String {
    // Funzione per processare un nodo e i suoi figli
    fn process(node: &TreeNode, depth: usize) -> Vec<String> {
        let mut lines = Vec::new();
        
        // Aggiungi riga per questo nodo se non è root
        if depth > 0 {
            let line = if node.is_dir {
                format_dir_line(&node.name, depth)
            } else {
                format_file_line(&node.name, depth)
            };
            lines.push(line);
        }
        
        // Aggiungi righe dei figli
        let child_lines: Vec<String> = node.children
            .values()
            .flat_map(|child| process(child, depth + 1))
            .collect();
        
        lines.extend(child_lines);
        lines
    }
    
    process(node, depth).concat()
}

// ==========================================
// 5. VERSIONE CON FOLDL (ACCUMULATORE)
// ==========================================

/// Versione con fold (accumulatore)
fn format_node_fold(node: &TreeNode, depth: usize, include_src: bool) -> String {
    fn fold_node(acc: String, node: &TreeNode, depth: usize) -> String {
        let mut acc = acc;
        
        if depth > 0 {
            let line = if node.is_dir {
                format_dir_line(&node.name, depth)
            } else {
                format_file_line(&node.name, depth)
            };
            acc.push_str(&line);
        }
        
        node.children.values().fold(acc, |acc, child| {
            fold_node(acc, child, depth + 1)
        })
    }
    
    fold_node(String::new(), node, depth)
}

// ==========================================
// 6. SOSTITUISCI LA FUNZIONE ORIGINALE
// ==========================================

/// Genera rappresentazione testuale con indentazione
fn format(&self, depth: usize, include_src: bool) -> String {
    // Scegli una delle versioni sopra
    format_node_recursive(self, depth, include_src)
    
    // Oppure:
    // format_node_pipeline(self, depth, include_src)
    // format_node_fold(self, depth, include_src)
}
