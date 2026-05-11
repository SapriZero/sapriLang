//! Definizione tabella

use std::collections::HashMap;
use sapri_obj::Value;

/// Record di una tabella
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub id: usize,
    pub values: HashMap<String, Value>,
}

impl Record {
    /// Crea un nuovo record
    pub fn new(id: usize) -> Self {
        Self {
            id,
            values: HashMap::new(),
        }
    }

    /// Inserisce un valore nel record
    pub fn set(&mut self, key: &str, value: Value) {
        self.values.insert(key.to_string(), value);
    }

    /// Ottiene un valore dal record
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }
}

/// Tabella con accesso O(1)
#[derive(Debug, Clone)]
pub struct Table {
    header: String,
    index_bits: u8,
    records: Vec<Record>,
    indexes: HashMap<String, HashMap<String, usize>>,
}

impl Table {
    /// Crea una nuova tabella vuota
    pub fn new(header: &str, index_bits: u8) -> Self {
        Self {
            header: header.to_string(),
            index_bits,
            records: Vec::new(),
            indexes: HashMap::new(),
        }
    }

    /// Restituisce l'header della tabella
    pub fn header(&self) -> &str {
        &self.header
    }

    /// Restituisce i bit dell'indice
    pub fn index_bits(&self) -> u8 {
        self.index_bits
    }

    /// Ottiene un record per indice
    pub fn get(&self, index: usize) -> Option<&Record> {
        self.records.get(index)
    }

    /// Ottiene un record mutabile per indice
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Record> {
        self.records.get_mut(index)
    }

    /// Inserisce un nuovo record, restituisce l'indice
    pub fn insert(&mut self, mut record: Record) -> usize {
        let id = self.records.len();
        record.id = id;
        self.records.push(record);
        id
    }

    /// Aggiorna un record esistente
    pub fn update(&mut self, index: usize, record: Record) -> Option<Record> {
        if index < self.records.len() {
            let old = self.records[index].clone();
            self.records[index] = record;
            Some(old)
        } else {
            None
        }
    }

    /// Rimuove un record
    pub fn delete(&mut self, index: usize) -> Option<Record> {
        if index < self.records.len() {
            Some(self.records.remove(index))
        } else {
            None
        }
    }

    /// Numero di record
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Verifica se la tabella è vuota
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

	/// Converte un Value in una chiave per indice
	fn value_to_key(&self, value: &Value) -> String {
		match value {
			Value::String(s) => s.clone(),
			Value::Number(n) => n.to_string(),
			Value::Bool(b) => b.to_string(),
			Value::Null => "null".to_string(),
			Value::Obj(_) => "obj".to_string(),
			Value::Array(_) => "array".to_string(),
		}
	}

	pub fn add_index(&mut self, name: &str, field: &str) {
		let mut index_map = HashMap::new();
		for (idx, record) in self.records.iter().enumerate() {
			if let Some(value) = record.get(field) {
				let key = self.value_to_key(value);
				index_map.insert(key, idx);
			}
		}
		self.indexes.insert(name.to_string(), index_map);
	}
		
    /// Cerca per indice secondario
    pub fn search_index(&self, index_name: &str, value: &str) -> Option<usize> {
        self.indexes
            .get(index_name)
            .and_then(|idx| idx.get(value).copied())
    }
    
    pub fn records(&self) -> &[Record] {
        &self.records
    }

    /// Ottiene tutti i record mutabili
    pub fn records_mut(&mut self) -> &mut [Record] {
        &mut self.records
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sapri_obj::Value;

    #[test]
    fn test_record() {
        let mut record = Record::new(0);
        record.set("name", Value::from("Mario"));
        record.set("age", Value::from(30));

        assert_eq!(record.id, 0);
        assert_eq!(record.get("name").and_then(|v| v.as_str()), Some("Mario"));
        assert_eq!(record.get("age").and_then(|v| v.as_number()), Some(30.0));
        assert!(record.get("unknown").is_none());
    }

    #[test]
    fn test_table_new() {
        let table = Table::new("10", 16);
        assert_eq!(table.header(), "10");
        assert_eq!(table.index_bits(), 16);
        assert!(table.is_empty());
        assert_eq!(table.len(), 0);
    }

    #[test]
    fn test_table_insert_and_get() {
        let mut table = Table::new("10", 16);
        let mut record = Record::new(0);
        record.set("name", Value::from("Mario"));

        let id = table.insert(record);
        assert_eq!(id, 0);
        assert_eq!(table.len(), 1);
        assert!(!table.is_empty());

        let retrieved = table.get(0).unwrap();
        assert_eq!(retrieved.get("name").and_then(|v| v.as_str()), Some("Mario"));
    }

    #[test]
    fn test_table_update() {
        let mut table = Table::new("10", 16);
        let mut record1 = Record::new(0);
        record1.set("name", Value::from("Mario"));
        table.insert(record1);

        let mut record2 = Record::new(0);
        record2.set("name", Value::from("Luigi"));
        let old = table.update(0, record2).unwrap();
        assert_eq!(old.get("name").and_then(|v| v.as_str()), Some("Mario"));

        let updated = table.get(0).unwrap();
        assert_eq!(updated.get("name").and_then(|v| v.as_str()), Some("Luigi"));
    }

    #[test]
    fn test_table_delete() {
        let mut table = Table::new("10", 16);
        let mut record = Record::new(0);
        record.set("name", Value::from("Mario"));
        table.insert(record);
        assert_eq!(table.len(), 1);

        let deleted = table.delete(0).unwrap();
        assert_eq!(deleted.get("name").and_then(|v| v.as_str()), Some("Mario"));
        assert_eq!(table.len(), 0);
    }

    #[test]
    fn test_table_records_slice() {
        let mut table = Table::new("10", 16);
        
        for i in 0..3 {
            let mut record = Record::new(i);
            record.set("id", Value::from(i as f64));
            table.insert(record);
        }

        let records = table.records();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].get("id").and_then(|v| v.as_number()), Some(0.0));
        assert_eq!(records[1].get("id").and_then(|v| v.as_number()), Some(1.0));
        assert_eq!(records[2].get("id").and_then(|v| v.as_number()), Some(2.0));
    }

	#[test]
	fn test_table_index() {
		let mut table = Table::new("10", 16);
		
		let mut record1 = Record::new(0);
		record1.set("code", Value::from("A001"));
		table.insert(record1);
		
		let mut record2 = Record::new(0);
		record2.set("code", Value::from("A002"));
		table.insert(record2);

		table.add_index("code_idx", "code");
		
		assert_eq!(table.search_index("code_idx", "A001"), Some(0));
		assert_eq!(table.search_index("code_idx", "A002"), Some(1));
		assert_eq!(table.search_index("code_idx", "A003"), None);
	}
}
