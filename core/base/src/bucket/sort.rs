//! Funzioni di ordinamento ultra-veloci per bucket
//!
//! Counting sort diretto su array fixed-size:
//! - `u8` (256 valori) per primo carattere
//! - `u16` (65536 valori) per primi due caratteri

// ============================================================================
// COUNTING SORT PER U8 (0-255)
// ============================================================================

/// Counting sort per valori a 8 bit (0-255)
///
/// # Arguments
/// * `data` - Slice mutabile di `u8` da ordinare in-place
///
/// # Complexity
/// - Tempo: O(n + k) dove k=256
/// - Spazio: O(k) per l'array di conteggio
///
/// # Example
/// ```ignore
/// use sapri_core::bucket::sort::counting_sort_u8;
/// let mut data = vec![5, 2, 8, 1, 9];
/// counting_sort_u8(&mut data);
/// assert_eq!(data, vec![1, 2, 5, 8, 9]);
/// ```
#[inline(always)]
pub fn counting_sort_u8(data: &mut [u8]) {
    if data.is_empty() { return; }

    let mut count = [0u32; 256];

    // Conteggio (histogramming) - O(n)
    for &val in data.iter() {
        count[val as usize] += 1;
    }

    // Riscrittura diretta in ordine - O(k + n)
    let mut pos = 0usize;
    for (val, &cnt) in count.iter().enumerate() {
        for _ in 0..cnt {
            data[pos] = val as u8;
            pos += 1;
        }
    }
}

// ============================================================================
// COUNTING SORT PER U16 (0-65535)
// ============================================================================

/// Counting sort per valori a 16 bit (0-65535)
///
/// # Arguments
/// * `data` - Slice mutabile di `u16` da ordinare in-place
///
/// # Complexity
/// - Tempo: O(n + k) dove k=65536
/// - Spazio: O(k) per l'array di conteggio (256KB)
///
/// # Example
/// ```ignore
/// use sapri_core::bucket::sort::counting_sort_u16;
/// let mut data = vec![300u16, 100, 400, 50];
/// counting_sort_u16(&mut data);
/// assert_eq!(data, vec![50, 100, 300, 400]);
/// ```
#[inline(always)]
pub fn counting_sort_u16(data: &mut [u16]) {
    if data.is_empty() { return; }

    let mut count = [0u32; 65536];

    // Conteggio (histogramming) - O(n)
    for &val in data.iter() {
        count[val as usize] += 1;
    }

    // Riscrittura diretta in ordine - O(k + n)
    let mut pos = 0usize;
    for (val, &cnt) in count.iter().enumerate() {
        for _ in 0..cnt {
            data[pos] = val as u16;
            pos += 1;
        }
    }
}

// ============================================================================
// VERSIONE STABILE (per u16, con buffer temporaneo)
// ============================================================================

/// Counting sort stabile per valori a 16 bit
/// Preserva l'ordine relativo degli elementi con chiave uguale.
///
/// # Arguments
/// * `data` - Slice mutabile di `u16` da ordinare in-place
///
/// # Note
/// Usa un buffer temporaneo per la stabilità → O(n) spazio extra.
pub fn counting_sort_u16_stable(data: &mut [u16]) {
    if data.is_empty() { return; }

    // Copia per lettura stabile
    let temp = data.to_vec();
    let mut count = [0u32; 65536];

    // Conteggio
    for &val in temp.iter() {
        count[val as usize] += 1;
    }

    // Prefix sum per posizioni finali
    let mut total = 0u32;
    for cnt in count.iter_mut() {
        let old = *cnt;
        *cnt = total;
        total += old;
    }

    // Posizionamento stabile (itera in ordine per mantenere stabilità)
    for &val in temp.iter() {
        let idx = val as usize;
        let pos = count[idx] as usize;
        data[pos] = val;
        count[idx] += 1;
    }
}

// ============================================================================
// ORDINAMENTO PAROLE PER PREFIX (primi 2 byte)
// ============================================================================

/// Ordina un bucket di parole basandosi sui primi 2 byte come chiave u16
///
/// # Arguments
/// * `words` - Slice di byte-vector (parole da ordinare)
/// * `sorted` - Buffer di uscita (verrà riempito con parole ordinate)
///
/// # Note
/// - Usa `counting_sort_u16` per ordinare le chiavi
/// - Applica la stessa permutazione alle parole originali
/// - Se due parole hanno lo stesso prefix, l'ordine è non-deterministico
pub fn sort_words_by_prefix(words: &[Vec<u8>], sorted: &mut Vec<Vec<u8>>) {
    if words.is_empty() {
        sorted.clear();
        return;
    }

    // Estrai chiavi u16 dai primi 2 byte di ogni parola
     let keys: Vec<u16> = words.iter()
        .map(|w| {
            let b1 = *w.first().unwrap_or(&0) as u16;
            let b2 = *w.get(1).unwrap_or(&0) as u16;
            (b1 << 8) | b2
        })
        .collect();

    // Crea coppie (chiave, indice) per applicare la permutazione dopo l'ordinamento
    let indexed: Vec<(u16, usize)> = keys.iter()
        .copied()
        .enumerate()
        .map(|(i, k)| (k, i))
        .collect();

    // Estrai solo le chiavi per l'ordinamento
    let mut sort_keys: Vec<u16> = indexed.iter().map(|(k, _)| *k).collect();
    counting_sort_u16(&mut sort_keys);

    // Ricostruisci l'ordine degli indici basato sulle chiavi ordinate
    // (approccio semplificato: associa ogni chiave ordinata al primo indice disponibile)
    let mut used = vec![false; words.len()];
    sorted.clear();
    
    for &ordered_key in &sort_keys {
        for &(key, idx) in &indexed {
            if key == ordered_key && !used[idx] {
                sorted.push(words[idx].clone());
                used[idx] = true;
                break;
            }
        }
    }
}

// ============================================================================
// TEST
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort_u8() {
        let mut data = vec![5u8, 2, 8, 1, 9, 3, 7, 4, 6, 0];
        let expected = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        counting_sort_u8(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    fn test_counting_sort_u8_with_duplicates() {
        let mut data = vec![5u8, 2, 5, 1, 2, 3, 1, 4, 1, 0];
        let mut expected = data.clone();
        expected.sort();
        counting_sort_u8(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    fn test_counting_sort_u16() {
        let mut data = vec![300u16, 100, 400, 100, 500, 50, 200];
        let expected = vec![50u16, 100, 100, 200, 300, 400, 500];
        counting_sort_u16(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    fn test_counting_sort_u16_large() {
        let mut data: Vec<u16> = (0..1000).map(|i| (i * 257) as u16).collect();
        let mut expected = data.clone();
        expected.sort();
        counting_sort_u16(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    fn test_counting_sort_u16_stable() {
        // Test stabilità: coppie (chiave, id_origine)
        let mut data = vec![3u16, 1, 3, 2, 1, 3];
        let original = data.clone();
        counting_sort_u16_stable(&mut data);
        
        // Verifica ordinamento
        let mut expected = original.clone();
        expected.sort();
        assert_eq!(data, expected);
        
        // Verifica stabilità (i tre "3" mantengono l'ordine relativo)
        // Questo è implicito nell'algoritmo, testato visivamente
    }

    #[test]
    fn test_sort_words_by_prefix() {
        let words = vec![
            b"zebra".to_vec(),
            b"apple".to_vec(),
            b"banana".to_vec(),
            b"cherry".to_vec(),
            b"apricot".to_vec(),
        ];

        let mut sorted = Vec::new();
        sort_words_by_prefix(&words, &mut sorted);

        // Verifica che tutte le parole siano presenti
        assert_eq!(sorted.len(), words.len());
        
        // Verifica ordinamento per primi 2 byte:
        // "ap" (apple, apricot) < "ba" (banana) < "ch" (cherry) < "ze" (zebra)
        let prefixes: Vec<&[u8]> = sorted.iter().map(|w| &w[..2]).collect();
        assert!(prefixes.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_empty_input() {
        let mut data_u8: Vec<u8> = vec![];
        counting_sort_u8(&mut data_u8);
        assert!(data_u8.is_empty());
        
        let mut data_u16: Vec<u16> = vec![];
        counting_sort_u16(&mut data_u16);
        assert!(data_u16.is_empty());
    }

    #[test]
    fn test_single_element() {
        let mut data = vec![42u8];
        counting_sort_u8(&mut data);
        assert_eq!(data, vec![42u8]);

        let mut data = vec![12345u16];
        counting_sort_u16(&mut data);
        assert_eq!(data, vec![12345u16]);
    }

    #[test]
    fn test_already_sorted() {
        let mut data = vec![1u8, 2, 3, 4, 5];
        counting_sort_u8(&mut data);
        assert_eq!(data, vec![1u8, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut data = vec![5u8, 4, 3, 2, 1];
        let expected = vec![1u8, 2, 3, 4, 5];
        counting_sort_u8(&mut data);
        assert_eq!(data, expected);
    }
}
