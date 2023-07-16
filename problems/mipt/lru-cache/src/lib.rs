#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    capacity: usize,
    counter: usize,
    
    cache: BTreeMap<usize, (K, V)>,
    map: HashMap<K, usize>,
}

impl<K: Clone + Hash + Ord, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            capacity,
            counter: 0,
            cache: BTreeMap::new(),
            map: HashMap::new(),
        }
    }
    
    pub(self) fn add(&mut self, key: K, value: V) {
        self.counter += 1;
        self.map.insert(key.clone(), self.counter);
        self.cache.insert(self.counter, (key, value));
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(&counter) = self.map.get(key) {
            let (old_key, value) = self.cache.remove(&counter).unwrap();
            self.add(old_key, value);
            
            Some(self.cache.get(&self.counter).map(|(_, v)| v).unwrap())
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut result = None;

        if let Some(&counter) = self.map.get(&key) {
            result = Some(self.cache.remove(&counter).map(|(_, v)| v).unwrap());
        } else if self.map.len() == self.capacity {
            let (_, (rem_key, _)) = self.cache.pop_first().unwrap();
            self.map.remove(&rem_key);
        }
        self.add(key, value);
        
        result
    }
}
