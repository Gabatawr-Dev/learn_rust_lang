#![forbid(unsafe_code)]

use std::{borrow::Borrow, iter::FromIterator, ops::Index};

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FlatMap<K, V>(Vec<(K, V)>);

impl<K: Ord, V> FlatMap<K, V> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn as_slice(&self) -> &[(K, V)] {
        self.0.as_slice()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.0.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(index) => {
                let (_, old_value) = core::mem::replace(&mut self.0[index], (key, value));
                Some(old_value)
            }
            Err(index) => {
                self.0.insert(index, (key, value));
                None
            }
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized
    {
        match self.0.binary_search_by_key(&key, |(k, _)| k.borrow()) {
            Ok(index) => {
                let (_, v) = &self.0[index];
                Some(v)
            }
            Err(_) => None,
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized
    {
        match self.0.binary_search_by_key(&key, |(k, _)| k.borrow()) {
            Ok(index) => {
                let (_, v) = self.0.remove(index);
                Some(v)
            }
            Err(_) => None,
        }
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized
    {
        match self.0.binary_search_by_key(&key, |(k, _)| k.borrow()) {
            Ok(index) => Some(self.0.remove(index)),
            Err(_) => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<K, V, Q> Index<&Q> for FlatMap<K, V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
{
    type Output = V;

    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key).expect("no entry found for key")
    }
}

impl<K: Ord, V> Extend<(K, V)> for FlatMap<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<K: Ord, V> From<Vec<(K, V)>> for FlatMap<K, V> {
    fn from(vec: Vec<(K, V)>) -> Self {
        let mut map = FlatMap::new();
        map.extend(vec);
        map
    }
}

impl<K: Ord, V> From<FlatMap<K, V>> for Vec<(K, V)> {
    fn from(map: FlatMap<K, V>) -> Self {
        map.0
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for FlatMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = FlatMap::new();
        map.extend(iter);
        map
    }
}

impl<K, V> IntoIterator for FlatMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
