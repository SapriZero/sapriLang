// wiki_extract/src/flag_map.rs

use std::collections::HashMap;

pub fn get_flag_map() -> HashMap<char, &'static str> {
    let mut map = HashMap::new();
    
    // Verbi
    map.insert('A', "Verbo 1a coniugazione (-are)");
    map.insert('B', "Verbo 2a coniugazione (-ere)");
    map.insert('C', "Verbo 3a coniugazione (-ire)");
    map.insert('D', "Gerundio");
    map.insert('E', "Participio passato");
    map.insert('K', "Verbo (forme imperative)");
    map.insert('L', "Verbo (forme con pronomi)");
    map.insert('M', "Verbo riflessivo");
    map.insert('P', "Verbo (forme con pronomi)");
    map.insert('V', "Verbo (forme con pronomi)");
    map.insert('X', "Verbo riflessivo");
    map.insert('Z', "Verbo (forme irregolari)");
    map.insert('h', "Verbo (forme con pronomi)");
    map.insert('j', "Verbo (forme con pronomi)");
    map.insert('l', "Verbo (forme con pronomi)");
    map.insert('m', "Verbo (forme con pronomi)");
    map.insert('t', "Verbo incoativo (-sc-)");
    map.insert('v', "Verbo (forme con pronomi)");
    map.insert('z', "Verbo (forme alternative)");
    
    // Nomi
    map.insert('N', "Nome (suffissi diminutivi)");
    map.insert('Q', "Nome femminile");
    map.insert('S', "Nome maschile");
    map.insert('a', "Nome (forme alterate)");
    map.insert('b', "Nome (forme alterate)");
    map.insert('c', "Nome (suffissi)");
    map.insert('n', "Nome (forme alterate)");
    
    // Aggettivi
    map.insert('G', "Aggettivo");
    map.insert('H', "Aggettivo");
    map.insert('O', "Aggettivo (forme base)");
    map.insert('R', "Aggettivo (forme base)");
    map.insert('W', "Aggettivo (superlativo -issimo)");
    map.insert('Y', "Avverbio (-mente)");
    map.insert('g', "Aggettivo (forme alterate)");
    map.insert('o', "Aggettivo (forme base)");
    map.insert('p', "Aggettivo (forme base)");
    map.insert('u', "Aggettivo (forme alterate)");
    
    // Numeri
    map.insert('|', "Numeri cardinali (uno, due, tre)");
    map.insert('!', "Numeri cardinali (dieci, venti)");
    map.insert('"', "Numeri cardinali (mille, duemila)");
    map.insert('£', "Numeri romani (i, ii, iii)");
    map.insert('$', "Numeri romani (x, l, c, m)");
    map.insert('@', "Ordinali (-esimo)");
    map.insert('&', "Ordinali (unesimo, duesimo)");
    map.insert('(', "Ordinali (decimo, undicesimo)");
    map.insert(')', "Ordinali (-enne)");
    map.insert('=', "Ordinali (unenne, duenne)");
    
    // Preposizioni/Articoli
    map.insert('T', "Preposizioni articolate (del, della)");
    map.insert('U', "Articoli (un', bell')");
    map.insert('i', "Preposizioni articolate (l')");
    map.insert('q', "Prefisso (bell')");
    map.insert('r', "Prefisso (brav')");
    map.insert('s', "Prefisso (buon')");
    map.insert('^', "Prefisso (sant')");
    
    // Prefissi
    map.insert('d', "Prefisso (stra-)");
    map.insert('e', "Prefisso (pre-)");
    
    map
}
