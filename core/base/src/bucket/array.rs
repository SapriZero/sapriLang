//! BucketArray: Array fixed-size con lookup O(1)
//! Ottimizzato per accesso diretto a 65536 slot

use std::fmt;

/// Bucket array con dimensione fissa a compile-time
#[derive(Clone)]
pub struct BucketArray<T, const N: usize = 65536> {
    /// Slot interni (Vec per non stack-overflow con 64k elementi)
    slots: Vec<Option<T>>,
    /// Numero di slot occupati
    count: usize,
    /// Nome del bucket (per debug)
    name: String,
}

impl<T, const N: usize> BucketArray<T, N> {
    /// Crea un nuovo bucket array vuoto
    pub fn new(name: &str) -> Self {
        let mut slots = Vec::with_capacity(N);
        slots.resize_with(N, || None);
        Self {
            slots,
            count: 0,
            name: name.to_string(),
        }
    }

    /// Inserisce un valore all'indice specificato
    #[inline(always)]
    pub fn insert(&mut self, idx: usize, value: T) -> Result<(), BucketError> {
        if idx >= N {
           // return Err(BucketError::IndexOutOfRange { idx, max: N });
             return Err(BucketError::ValueAlreadyExists { idx });
        }
        if self.slots[idx].is_some() {
               return Err(BucketError::ValueAlreadyExists { idx });
        }
        self.slots[idx] = Some(value);
        self.count += 1;
        Ok(())
    }

    /// Ottiene un riferimento al valore all'indice (O(1))
    #[inline(always)]
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= N {
            return None;
        }
        self.slots[idx].as_ref()
    }

    /// Ottiene un riferimento mutabile al valore all'indice
    #[inline(always)]
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx >= N {
            return None;
        }
        self.slots[idx].as_mut()
    }

    /// Rimuove e restituisce il valore all'indice
    #[inline(always)]
    pub fn remove(&mut self, idx: usize) -> Option<T> {
        if idx >= N {
            return None;
        }
        if self.slots[idx].is_some() {
            self.count -= 1;
        }
        self.slots[idx].take()
    }

    /// Aggiorna o inserisce un valore (sovrascrive se esistente)
    #[inline(always)]
    pub fn set(&mut self, idx: usize, value: T) -> Result<(), BucketError> {
        if idx >= N {
            return Err(BucketError::IndexOutOfRange { idx, max: N });
        }
        if self.slots[idx].is_none() {
            self.count += 1;
        }
        self.slots[idx] = Some(value);
        Ok(())
    }

    /// Restituisce il prossimo indice libero
    pub fn next_free(&self) -> Option<usize> {
        (0..N).find(|&i| self.slots[i].is_none())
    }

    /// Numero di slot occupati
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.count
    }

    /// Capacità massima
    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        N
    }

    /// È vuoto?
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Svuota il bucket
    pub fn clear(&mut self) {
        for slot in &mut self.slots {
            *slot = None;
        }
        self.count = 0;
    }

    /// Iteratore su (indice, valore)
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.slots.iter().enumerate()
            .filter_map(|(i, v)| v.as_ref().map(|v| (i, v)))
    }

    /// Statistiche del bucket
    pub fn stats(&self) -> BucketStats {
        BucketStats {
            name: self.name.clone(),
            used: self.count,
            capacity: N,
            utilization: (self.count as f64 / N as f64) * 100.0,
        }
    }
}

/// Statistiche di utilizzo del bucket
#[derive(Debug, Clone)]
pub struct BucketStats {
    pub name: String,
    pub used: usize,
    pub capacity: usize,
    pub utilization: f64,
}

impl fmt::Display for BucketStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bucket '{}': {}/{} slot used ({:.1}%)",
            self.name, self.used, self.capacity, self.utilization
        )
    }
}

/// Errori del bucket
#[derive(Debug, Clone, PartialEq)]
pub enum BucketError {
    IndexOutOfRange { idx: usize, max: usize },
    ValueAlreadyExists { idx: usize },
    BucketFull { max: usize },
}

impl fmt::Display for BucketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BucketError::IndexOutOfRange { idx, max } => write!(f, "Index {} out of range (max {})", idx, max),
            BucketError::ValueAlreadyExists { idx } => write!(f, "Value already exists at index {}", idx),
            BucketError::BucketFull { max } => write!(f, "Bucket is full (max {})", max),
        }
    }
}

impl std::error::Error for BucketError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_new() {
        let bucket = BucketArray::<i32, 1024>::new("test");
        assert_eq!(bucket.len(), 0);
        assert_eq!(bucket.capacity(), 1024);
        assert!(bucket.is_empty());
    }

    #[test]
    fn test_bucket_insert_and_get() {
        let mut bucket = BucketArray::<i32, 1024>::new("test");

        bucket.insert(42, 100).unwrap();
        assert_eq!(bucket.len(), 1);
        assert_eq!(bucket.get(42), Some(&100));

        // Indice fuori range
        assert!(bucket.insert(2000, 200).is_err());

        // Doppio inserimento
        assert!(bucket.insert(42, 200).is_err());
    }

    #[test]
    fn test_bucket_set() {
        let mut bucket = BucketArray::<i32, 1024>::new("test");

        bucket.set(42, 100).unwrap();
        assert_eq!(bucket.get(42), Some(&100));

        // Sovrascrittura
        bucket.set(42, 200).unwrap();
        assert_eq!(bucket.get(42), Some(&200));
        assert_eq!(bucket.len(), 1);  // count non aumenta
    }

    #[test]
    fn test_bucket_remove() {
        let mut bucket = BucketArray::<i32, 1024>::new("test");

        bucket.insert(42, 100).unwrap();
        assert_eq!(bucket.len(), 1);

        let removed = bucket.remove(42);
        assert_eq!(removed, Some(100));
        assert_eq!(bucket.len(), 0);
        assert_eq!(bucket.get(42), None);

        // Rimozione inesistente
        assert_eq!(bucket.remove(99), None);
    }

    #[test]
    fn test_next_free() {
        let mut bucket = BucketArray::<i32, 256>::new("test");

        assert_eq!(bucket.next_free(), Some(0));

        bucket.insert(0, 100).unwrap();
        assert_eq!(bucket.next_free(), Some(1));

        bucket.insert(1, 200).unwrap();
        bucket.insert(2, 300).unwrap();
        assert_eq!(bucket.next_free(), Some(3));
    }

    #[test]
    fn test_iter() {
        let mut bucket = BucketArray::<i32, 256>::new("test");

        bucket.insert(10, 100).unwrap();
        bucket.insert(20, 200).unwrap();
        bucket.insert(30, 300).unwrap();

        let items: Vec<_> = bucket.iter().collect();
        assert_eq!(items.len(), 3);
        assert!(items.contains(&(10, &100)));
        assert!(items.contains(&(20, &200)));
        assert!(items.contains(&(30, &300)));
    }

    #[test]
    fn test_stats() {
        let mut bucket = BucketArray::<i32, 1024>::new("test");

        for i in 0..256 {
            bucket.insert(i, i as i32).unwrap();
        }

        let stats = bucket.stats();
        assert_eq!(stats.name, "test");
        assert_eq!(stats.used, 256);
        assert_eq!(stats.capacity, 1024);
        assert!((stats.utilization - 25.0).abs() < 0.01);
    }
}
