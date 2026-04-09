
//! Bucket array system per URCM
//!
//! Fornisce strutture per bucket array fixed-size (65536 default)
//! con lookup O(1) e zero-copy tramite Cow.


pub mod array;      // pub, non private
pub mod sort;       // pub

pub use array::{BucketArray, BucketStats, BucketError};
pub use sort::{counting_sort_u8, counting_sort_u16, counting_sort_u16_stable};

// pub use scanner::{BucketScanner, run_engine_bucket};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_integrity() {
        // Verifica che i tipi principali siano accessibili
        let _bucket: BucketArray<i32> = BucketArray::new("test");
        let _stats: BucketStats;
        let _error: BucketError;

        // Verifica funzioni di sorting
        let mut data = vec![5, 2, 8, 1];
        counting_sort_u8(&mut data);
    }
}
