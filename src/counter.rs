use std::collections::{btree_map::Keys, BTreeMap};

#[derive(Debug)]
pub struct Counter<K: Ord + Clone> {
    inner: BTreeMap<K, usize>,
}

impl<K: Ord + Clone> Counter<K> {
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
