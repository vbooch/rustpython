use std::collections::HashMap;
use crate::table::FactsTable;
use rand::Rng;

pub struct FactsTableManager {    
    _instances: HashMap<u16, FactsTable>,    
}

impl FactsTableManager {

    pub fn init() -> Self {
        Self {
            _instances: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, dim_count: u16) -> &mut FactsTable {
        let mut rng = rand::thread_rng();
        let table_id = rng.gen::<u16>();
        
        let table = FactsTable::create(table_id, dim_count);      
        self._instances.insert(table_id, table);

        let inserted = self._instances.get_mut(&table_id).unwrap();
        
        inserted
    }

    pub fn get(&mut self, id: &u16) -> Option<&mut FactsTable> {
        if self._instances.contains_key(id) {
            return self._instances.get_mut(id);
        }            
        
        None
    }
}
