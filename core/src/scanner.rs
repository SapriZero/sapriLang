//! Scanner ottimizzato con bucket array 65535
//! Mantiene compatibilità con scanner.rs esistente

use crate::bucket::array::BucketArray;
use crate::bucket::strategy::BucketStrategy;
use crate::bucket::BucketStats;  // usa il re-export
use std::borrow::Cow;
use std::marker::PhantomData;

/// Struttura per i parametri adattivi del loop di scansione
#[derive(Debug, Clone, Copy)]
pub struct ScanParams {
    pub chunk_size: usize,
    pub prog_step: usize, 
}

impl Default for ScanParams {
    fn default() -> Self {
        Self {
            chunk_size: 64 * 1024, // 64KB default
            prog_step: 10,
        }
    }
}

pub trait TokenStrategy {
    fn extract<'a>(slice: &'a [u8], i: usize, end: usize) -> (Option<Cow<'a, [u8]>>, usize);
}

#[inline(always)]
pub fn apply_filters(word: &[u8], range: (Option<&[u8]>, Option<&[u8]>)) -> bool {
    match range {
        (Some(min), Some(max)) => word >= min && word <= max,
        (Some(min), None) => word >= min,
        (None, Some(max)) => word <= max,
        (None, None) => true,
    }
}

/// Scanner basato su bucket array 65535
pub struct BucketScanner<'a, S: TokenStrategy> {
    /// Bucket array per parole (65536 slot)
    buckets: BucketArray<Vec<Cow<'a, [u8]>>, 65536>,
    /// Parametri di scansione
    params: ScanParams,
    /// Strategy marker
    _strategy: PhantomData<S>,
}

impl<'a, S: TokenStrategy + BucketStrategy> BucketScanner<'a, S> {
    /// Crea un nuovo scanner
    pub fn new(params: ScanParams) -> Self {
        Self {
            buckets: BucketArray::new("word_buckets"),
            params,
            _strategy: PhantomData,
        }
    }

    /// Esegue la scansione dei dati
    pub fn scan<F>(&mut self, data: &'a [u8], range: (Option<&[u8]>, Option<&[u8]>), mut progress: F)
    where
        F: FnMut(f32),
    {
        let total_len = data.len();
        let mut i = 0;

        while i < total_len {
            if let (Some(word), next_i) = S::extract(data, i, total_len) {
                if apply_filters(&word, range) {
                    let idx = S::bucket_index(word.as_ref());

                    // Ottieni o crea il bucket
                    if self.buckets.get(idx).is_none() {
                        // Inizializza con capacità stimata
                        let _ = self.buckets.insert(idx, Vec::with_capacity(32));
                    }

                    if let Some(bucket) = self.buckets.get_mut(idx) {
                        bucket.push(word);
                    }
                }
                i = next_i;
            } else {
                i += 1;
            }

            if i % self.params.prog_step == 0 {
                progress((i as f32 / total_len as f32) * 100.0);
            }
        }

        progress(100.0);
    }

    /// Restituisce le parole raccolte, già parzialmente ordinate per bucket
    pub fn into_sorted_words(mut self) -> Vec<Cow<'a, [u8]>> {
        let mut result = Vec::with_capacity(65536 * 8);  // stima

        // Scansiona tutti i bucket in ordine
        for idx in 0..65536 {
            if let Some(mut bucket) = self.buckets.remove(idx) {
                if !bucket.is_empty() {
                    // Sort locale (bucket già piccolo)
                    bucket.sort_unstable();
                    result.extend(bucket);
                }
            }
        }

        result
    }

    /// Restituisce statistiche di utilizzo
    pub fn stats(&self) -> Vec<BucketStats> {
        vec![self.buckets.stats()]
    }

    /// Resetta lo scanner
    pub fn clear(&mut self) {
        self.buckets.clear();
    }
}

/// Versione semplificata per uso diretto (wrapper di run_engine_bucket)
pub fn run_engine_bucket<'a, F, S>(
    data: &'a [u8],
    params: ScanParams,
    range: (Option<&[u8]>, Option<&[u8]>),
    progress: &mut F,
) -> Vec<Cow<'a, [u8]>>
where
    F: FnMut(f32),
    S: TokenStrategy + BucketStrategy,
{
    let mut scanner = BucketScanner::<S>::new(params);
    scanner.scan(data, range, |p| progress(p));
    scanner.into_sorted_words()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Implementazione minima per i test
    pub struct TestStrategy;
    
    impl TokenStrategy for TestStrategy {
        fn extract<'a>(slice: &'a [u8], i: usize, end: usize) -> (Option<Cow<'a, [u8]>>, usize) {
            if i >= end {
                return (None, end);
            }
            
            // Salta spazi
            let mut start = i;
            while start < end && slice[start].is_ascii_whitespace() {
                start += 1;
            }
            
            if start >= end {
                return (None, end);
            }
            
            // Trova fine parola
            let mut next = start;
            while next < end && !slice[next].is_ascii_whitespace() {
                next += 1;
            }
            
            (Some(Cow::Borrowed(&slice[start..next])), next)
        }
    }
    
    #[test]
    fn test_bucket_scanner_simple() {
        let data = b"hello world hello rust world";
        let params = ScanParams {
            chunk_size: 1024,
            prog_step: 10,
        };
        
        let mut progress_calls = 0;
        let mut progress = |p: f32| {
            progress_calls += 1;
            assert!(p >= 0.0 && p <= 100.0);
        };
        
        let words = run_engine_bucket::<_, TestStrategy>(
            data,
            params,
            (None, None),
            &mut progress
        );
        
        assert!(!words.is_empty());
        assert!(words.len() >= 4);
        assert!(progress_calls > 0);
    }
    
    #[test]
    fn test_bucket_scanner_with_filter() {
        let data = b"apple banana apple cherry banana date";
        let params = ScanParams {
            chunk_size: 1024,
            prog_step: 10,
        };
        
        let range = (Some(b"b" as &[u8]), Some(b"c" as &[u8]));
        let words = run_engine_bucket::<_, TestStrategy>(
            data,
            params,
            range,
            &mut |_| {}
        );
        
        assert!(!words.is_empty());
        for word in &words {
            let w = String::from_utf8_lossy(word);
            assert!(w.starts_with('b') || w.starts_with('c'));
        }
    }
    
    #[test]
    fn test_bucket_scanner_empty() {
        let data = b"";
        let params = ScanParams {
            chunk_size: 1024,
            prog_step: 10,
        };
        
        let words = run_engine_bucket::<_, TestStrategy>(
            data,
            params,
            (None, None),
            &mut |_| {}
        );
        
        assert!(words.is_empty());
    }
    
    #[test]
    fn test_bucket_scanner_stats() {
        let data = b"hello world hello";
        let params = ScanParams::default();
        
        let mut scanner = BucketScanner::<TestStrategy>::new(params);
        scanner.scan(data, (None, None), &mut |_| {});
        
        let stats = scanner.stats();
        assert_eq!(stats.len(), 1);
    }
    
    #[test]
    fn test_bucket_scanner_clear() {
        let data = b"hello world";
        let params = ScanParams::default();
        
        let mut scanner = BucketScanner::<TestStrategy>::new(params);
        scanner.scan(data, (None, None), &mut |_| {});
        
        assert!(scanner.stats()[0].used > 0);
        
        scanner.clear();
        assert_eq!(scanner.stats()[0].used, 0);
    }
    
    #[test]
    fn test_bucket_index() {
        assert_eq!(TestStrategy::bucket_index(b"ab"), (b'a' as usize) << 8 | (b'b' as usize));
        assert_eq!(TestStrategy::bucket_index(b"a"), (b'a' as usize) << 8);
        assert_eq!(TestStrategy::bucket_index(b""), 0);
    }
}
