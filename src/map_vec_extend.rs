use std::collections::{BTreeMap, BTreeSet};

pub fn append_to_mapping<K: Ord, V>(btm: &mut BTreeMap<K, Vec<V>>, k: K, v: V) {
    match btm.get_mut(&k) {
        Some(vv) => vv.push(v),
        None => {
            let mut new_vec = Vec::new();
            new_vec.push(v);
            btm.insert(k, new_vec);
        }
    }
}

pub fn append_to_mapping_set<K: Ord, V: Ord>(btm: &mut BTreeMap<K, BTreeSet<V>>, k: K, v: V) {
    match btm.get_mut(&k) {
        Some(vv) => {
            vv.insert(v);
        }
        None => {
            let mut new_set: BTreeSet<V> = BTreeSet::new();
            new_set.insert(v);
            btm.insert(k, new_set);
        }
    }
}
