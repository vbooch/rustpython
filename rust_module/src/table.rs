use std::collections::HashMap;
use md5::{Md5, Digest};

pub struct FactsTable {    
    _facts: HashMap<u128, f32>,
    _addresses: HashMap<Vec<i32>, u128>,
    _dim_count: u16,    
    pub id: u16,
}

impl FactsTable {

    pub fn create(tab_id: u16, dim_count: u16 ) -> FactsTable {
        FactsTable {
            _facts: HashMap::new(),
            _addresses: HashMap::new(),
            _dim_count: dim_count,
            id: tab_id,
        }
    }

    pub fn count(&self) -> usize {        
        return self._facts.len();
    }

    fn _get_addr_hash(&self, address: &[i32]) -> u128
    {
        let mut bytes = Vec::with_capacity(4 * address.len());
    
        for value in address {
            bytes.extend(&value.to_be_bytes());
        }

        let binding = bytes.into_iter();
        let addr_bytes = binding.as_slice();
        
        let mut hasher = Md5::new();
        hasher.update(addr_bytes);
        
        return u128::from_be_bytes(hasher.finalize().into());
    }

    pub fn get(&self, address: &[i32]) -> Option<&f32> {
        
        let hash = &self._get_addr_hash(address);        
                
        if self._facts.contains_key(hash) {
            return self._facts.get(hash);
        }

        None
    }

    pub fn set(&mut self, address: &[i32], value: f32) {
        let hash = &self._get_addr_hash(address);
        self._facts.insert(*hash, value);
    }
}