//! Implementazioni aggiuntive per Table

use super::table::Table;

impl Table {
    /// Cerca un record per valore di campo (scan lineare)
    /// Usare solo per tabelle piccole o debug
	pub fn find_by_field(&self, field: &str, value: &str) -> Vec<usize> {
			let records = self.records();  // Usa il metodo pubblico records()
			records
				.iter()
				.enumerate()
				.filter_map(|(idx, record)| {
					record
						.get(field)
						.and_then(|v| v.as_str())
						.filter(|s| *s == value)
						.map(|_| idx)
				})
				.collect()
		}
}
