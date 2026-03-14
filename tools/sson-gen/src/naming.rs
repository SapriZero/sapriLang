//! Generazione naming gerarchico per elementi del codice

/// Genera un nome gerarchico a partire dal path del file e dal nome della funzione
/// 
/// # Esempio
/// ```
/// let name = generate_function_name("src/bucket/sort.rs", "counting_sort_u8");
/// assert_eq!(name, "bucket.sort.counting_sort_u8");
/// ```
pub fn generate_function_name(file_path: &str, function_name: &str) -> String {
    let path_parts: Vec<&str> = file_path
        .trim_start_matches("src/")
        .trim_end_matches(".rs")
        .split('/')
        .collect();
    
    let mut name_parts = path_parts;
    name_parts.push(function_name);
    name_parts.join(".")
}

/// Genera nome per struct/enum/trait
pub fn generate_type_name(file_path: &str, type_name: &str) -> String {
    let path_parts: Vec<&str> = file_path
        .trim_start_matches("src/")
        .trim_end_matches(".rs")
        .split('/')
        .collect();
    
    let mut name_parts = path_parts;
    name_parts.push(type_name);
    name_parts.join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_function_name() {
        assert_eq!(
            generate_function_name("src/bucket/sort.rs", "counting_sort_u8"),
            "bucket.sort.counting_sort_u8"
        );
        
        assert_eq!(
            generate_function_name("src/sson/token.rs", "next_token"),
            "sson.token.next_token"
        );
    }

    #[test]
    fn test_generate_type_name() {
        assert_eq!(
            generate_type_name("src/bucket/array.rs", "BucketArray"),
            "bucket.array.BucketArray"
        );
    }
}
