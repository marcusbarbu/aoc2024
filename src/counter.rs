use std::collections::{btree_map::Keys, BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct BTreeCounter<K: Ord + Clone> {
    inner: BTreeMap<K, usize>,
}

impl<K: Ord + Clone> BTreeCounter<K> {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub fn add_n(&mut self, k: K, n: usize) {
        let start = self.inner.get(&k).unwrap_or(&0);
        self.inner.insert(k, start + n);
    }

    pub fn add(&mut self, k: K) {
        self.add_n(k, 1)
    }

    pub fn get(&self, k: &K) -> Option<usize> {
        self.inner.get(k).copied()
    }

    pub fn keys(&self) -> Keys<'_, K, usize> {
        self.inner.keys()
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, K, usize> {
        self.inner.iter()
    }
}

#[derive(Debug)]
pub struct HashMapCounter<K: Hash + Clone + Eq> {
    inner: HashMap<K, usize>,
}

impl<K: Clone + Hash + Eq> HashMapCounter<K> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add_n(&mut self, k: K, n: usize) {
        let start = self.inner.get(&k).unwrap_or(&0);
        self.inner.insert(k, start + n);
    }

    pub fn add(&mut self, k: K) {
        self.add_n(k, 1)
    }

    pub fn get(&self, k: &K) -> Option<usize> {
        self.inner.get(k).copied()
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, K, usize> {
        self.inner.keys()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, usize> {
        self.inner.iter()
    }
}
