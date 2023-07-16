#![forbid(unsafe_code)]

use std::collections::VecDeque;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct MinQueue<T> {
    data: VecDeque<T>,
    min_map: BTreeMap<T, u32>
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
            min_map: BTreeMap::new()
        }
    }

    pub fn push(&mut self, val: T) {
        match self.min_map.get_mut(&val) {
            Some(min_count) => {
                *min_count += 1;
            },
            None => {
                self.min_map.insert(val.clone(), 1);
            },
        };
        self.data.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        let opt_val = self.data.pop_front();

        match &opt_val {
            Some(val) => {
                if let Some(min_count) = self.min_map.get_mut(val) {
                    if *min_count == 1 {
                        self.min_map.remove(&val);
                    } else { 
                        *min_count -= 1; 
                    }
                }
                opt_val
            },
            None => opt_val
        }
    }

    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn min(&self) -> Option<&T> {
        match self.min_map.first_key_value() {
            Some(min_pair) => Some(&min_pair.0),
            None => None
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
