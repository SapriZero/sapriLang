use sapri_diz::diz;

fn main() {
    // Inizializza il dizionario
   // sapri_diz::init_diz();
    
    // Accesso diretto con macro
    let stopwords: Vec<String> = diz!(lang.it.stopwords.list -> Vec<String>).unwrap();
    let suffixes: Vec<String> = diz!(lang.it.suffixes.list -> Vec<String>).unwrap();
    let bits: u8 = diz!(lang.it.charmap.bits -> u8).unwrap();
    let escape_code: u8 = diz!(lang.it.charmap.escape_code -> u8).unwrap();
    
    println!("Stopwords: {} parole", stopwords.len());
    println!("Suffissi: {} suffissi", suffixes.len());
    println!("Charmap bits: {}", bits);
    println!("Escape code: {}", escape_code);
}
