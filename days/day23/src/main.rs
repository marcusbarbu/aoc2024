use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use aoc2024::{
    counter::{BTreeCounter, HashMapCounter},
    map_vec_extend::append_to_hash_set,
    AocHelper, RequestedAocInputType,
};
use itertools::Itertools;
use tracing::{debug, info};

#[derive(Debug)]
struct Day23 {
    raw: String,
    pairs: Vec<(String, String)>,
    maps: HashMap<String, HashSet<String>>,
}

impl Day23 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            pairs: Vec::new(),
            maps: HashMap::new(),
        }
    }

    pub fn parse(&mut self) {
        for ele in self.raw.lines() {
            let parts = ele.split_once('-').unwrap();
            self.pairs.push((parts.0.to_string(), parts.1.to_string()));
        }
    }

    pub fn make_sets(&mut self) {
        for ele in self.pairs.iter() {
            // append_to_hash_set(&mut self.maps, ele.0.clone(), ele.0.clone());
            append_to_hash_set(&mut self.maps, ele.0.clone(), ele.1.clone());
            append_to_hash_set(&mut self.maps, ele.1.clone(), ele.0.clone());
            // append_to_hash_set(&mut self.maps, ele.1.clone(), ele.1.clone());
        }
    }

    pub fn counter_sets(&self) -> usize {
        let mut c: HashMapCounter<String> = HashMapCounter::new();
        for (key, set) in self.maps.iter() {
            let starts_valid = key.starts_with('t');
            let mut valid_set: HashSet<String> = HashSet::new();
            for window in set.iter().combinations(2) {
                // debug!("Window {:?}", window);
                let mut valid: bool = starts_valid;
                let mut point_list_vec: Vec<String> = Vec::new();
                point_list_vec.push(key.clone());
                for point in window.iter() {
                    point_list_vec.push(point.to_string());
                    if point.starts_with('t') {
                        valid = true;
                    }
                }
                if valid {
                    point_list_vec.sort();
                    let pls = point_list_vec.join(",");
                    valid_set.insert(pls);
                }
            }
            for pls in valid_set.iter() {
                c.add(pls.to_string());
            }
        }
        let mut sats: Vec<String> = Vec::new();
        for (group, len) in c.iter() {
            debug!("Group {group} len {len}");
            if len >= &3 {
                debug!("Group {group} satisfies!");
                sats.push(group.to_string());
            }
        }
        for s in sats.iter() {
            info!("{}", s);
        }
        sats.len()
    }

    pub fn largest_continent(&self) -> String {
        let mut c: HashMapCounter<String> = HashMapCounter::new();
        for (key, set) in self.maps.iter() {
            let mut valid_set: HashSet<String> = HashSet::new();
            for window in set.iter().powerset() {
                // debug!("Window {:?}", window);
                let mut point_list_vec: Vec<String> = Vec::new();
                point_list_vec.push(key.clone());
                for point in window.iter() {
                    point_list_vec.push(point.to_string());
                    if point.starts_with('t') {}
                }
                point_list_vec.sort();
                let pls = point_list_vec.join(",");
                valid_set.insert(pls);
            }
            for pls in valid_set.iter() {
                c.add(pls.to_string());
            }
        }
        let mut sorted = c.iter().sorted_by(|a, b| b.1.cmp(&a.1));
        sorted.next().unwrap().0.to_string()
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(23, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d23 = Day23::new(&test_input);
    d23.parse();
    d23.make_sets();
    debug!("{:?}", d23);
    let ans = d23.counter_sets();
    info!("Answer: {ans}");

    let mut d23 = Day23::new(&real_input);
    d23.parse();
    d23.make_sets();
    debug!("{:?}", d23);
    let ans = d23.counter_sets();
    info!("Answer: {ans}");

    let mut d23 = Day23::new(&test_input);
    d23.parse();
    d23.make_sets();
    let ans = d23.largest_continent();
    info!("Answer: {ans}");

    let mut d23 = Day23::new(&real_input);
    d23.parse();
    d23.make_sets();
    let ans = d23.largest_continent();
    info!("Answer: {ans}");
}
