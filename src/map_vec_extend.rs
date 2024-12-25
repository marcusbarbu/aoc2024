use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

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

pub fn append_to_hash_map<K, V>(hm: &mut HashMap<K, Vec<V>>, k: K, v: V)
where
    K: Ord + std::hash::Hash,
{
    match hm.get_mut(&k) {
        Some(vv) => vv.push(v),
        None => {
            let mut new_vec = Vec::new();
            new_vec.push(v);
            hm.insert(k, new_vec);
        }
    }
}

pub fn append_to_hash_set<K, V>(hm: &mut HashMap<K, HashSet<V>>, k: K, v: V)
where
    K: std::hash::Hash + Eq,
    V: std::hash::Hash + Eq,
{
    match hm.get_mut(&k) {
        Some(vv) => vv.insert(v),
        None => {
            let mut new_set: HashSet<V> = HashSet::new();
            new_set.insert(v);
            hm.insert(k, new_set);
            true
        }
    };
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
