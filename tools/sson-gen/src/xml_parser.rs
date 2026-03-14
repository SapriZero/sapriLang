//! Funzioni per il parsing dell'XML generato da riveter

/// Decodifica le entità HTML più comuni
pub fn decode_html_entities(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '&' {
            // Prova a riconoscere un'entità
            let mut entity = String::new();
            while let Some(&next) = chars.peek() {
                if next == ';' {
                    chars.next();
                    break;
                }
                entity.push(chars.next().unwrap());
            }

            // Decodifica entità comuni
            match entity.as_str() {
                "lt" => result.push('<'),
                "gt" => result.push('>'),
                "amp" => result.push('&'),
                "quot" => result.push('"'),
                "apos" => result.push('\''),
                _ => {
                    // Non riconosciuta, lascia com'era
                    result.push('&');
                    result.push_str(&entity);
                    result.push(';');
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Estrae il contenuto di un tag XML
pub fn extract_xml_tag(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);

    xml.find(&open).and_then(|start| {
        let start = start + open.len();
        xml[start..].find(&close).map(|end| {
            xml[start..start+end].trim().to_string()
        })
    })
}

/// Estrae la struttura ad albero dagli attributi XML
pub fn extract_tree(xml: &str, output: &mut String, depth: usize) {
    let indent = "  ".repeat(depth);
    let mut pos = 0;

    while let Some(dir_start) = xml[pos..].find("<dir") {
        let dir_start = pos + dir_start;

        // Cerca l'attributo name
        if let Some(name_start) = xml[dir_start..].find("name=\"") {
            let name_start = dir_start + name_start + 6;
            if let Some(name_end) = xml[name_start..].find('"') {
                let name = &xml[name_start..name_start+name_end];
                output.push_str(&format!("{}{}/\n", indent, name));

                // Trova la fine del tag <dir ...> per continuare la ricerca
                if let Some(tag_end) = xml[dir_start..].find('>') {
                    let tag_end = dir_start + tag_end + 1;

                    // Cerca contenuti fino a </dir>
                    if let Some(close_pos) = xml[tag_end..].find("</dir>") {
                        let content = &xml[tag_end..tag_end+close_pos];

                        // Cerca file e subdirectory nel contenuto
                        extract_files_from_content(content, output, depth + 1);
                        extract_tree(content, output, depth + 1);

                        pos = tag_end + close_pos + 6;
                        continue;
                    }
                }
            }
        }
        pos = dir_start + 4;
    }
}

/// Estrae i file dal contenuto di una directory
fn extract_files_from_content(content: &str, output: &mut String, depth: usize) {
    let indent = "  ".repeat(depth);
    let mut pos = 0;

    while let Some(file_start) = content[pos..].find("<file") {
        let file_start = pos + file_start;

        // Cerca l'attributo name
        if let Some(name_start) = content[file_start..].find("name=\"") {
            let name_start = file_start + name_start + 6;
            if let Some(name_end) = content[name_start..].find('"') {
                let name = &content[name_start..name_start+name_end];
                output.push_str(&format!("{}{}\n", indent, name));
            }
        }

        // Vai oltre questo file
        if let Some(tag_end) = content[file_start..].find("/>") {
            pos = file_start + tag_end + 2;
        } else if let Some(tag_end) = content[file_start..].find('>') {
            pos = file_start + tag_end + 1;
        } else {
            break;
        }
    }
}

/// Conta i file totali
pub fn count_files(xml: &str) -> usize {
    xml.matches("<file").count()
}

/// Estrae lista file completa
pub fn extract_all_files(xml: &str, output: &mut String) {
    let mut pos = 0;
    while let Some(file_start) = xml[pos..].find("<file") {
        let file_start = pos + file_start;

        // Cerca l'attributo name
        if let Some(name_start) = xml[file_start..].find("name=\"") {
            let name_start = file_start + name_start + 6;
            if let Some(name_end) = xml[name_start..].find('"') {
                let name = &xml[name_start..name_start+name_end];
                output.push_str(&format!("  {}\n", name));
            }
        }

        // Vai oltre questo file
        if let Some(tag_end) = xml[file_start..].find("/>") {
            pos = file_start + tag_end + 2;
        } else if let Some(tag_end) = xml[file_start..].find('>') {
            pos = file_start + tag_end + 1;
        } else {
            break;
        }
    }
}
