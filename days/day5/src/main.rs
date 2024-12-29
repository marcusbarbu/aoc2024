use std::{
    collections::{BTreeMap, VecDeque},
    ops::Div,
    usize,
};

use aoc2024::{AocHelper, RequestedAocInputType};
use tracing::{debug, info};

struct Day5 {
    raw: String,
    deps_list: BTreeMap<i32, Vec<i32>>,
    anti_deps_list: BTreeMap<i32, Vec<i32>>,
    print_list: Vec<Vec<i32>>,
}

impl Day5 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            deps_list: BTreeMap::new(),
            anti_deps_list: BTreeMap::new(),
            print_list: Vec::new(),
        }
    }

    fn append_to_mapping(btm: &mut BTreeMap<i32, Vec<i32>>, k: i32, v: i32) {
        match btm.get_mut(&k) {
            Some(vv) => vv.push(v),
            None => {
                let mut new_vec = Vec::new();
                new_vec.push(v);
                btm.insert(k, new_vec);
            }
        }
    }

    pub fn parse(&mut self) {
        let lines: Vec<&str> = self.raw.lines().collect();
        let mut is_split = false;
        for l in lines {
            if l.trim().len() == 0 {
                is_split = true;
                continue;
            }
            if !is_split {
                let parts: Vec<&str> = l.split('|').collect();
                let a = parts[0].parse::<i32>().unwrap();
                let b = parts[1].parse::<i32>().unwrap();
                Day5::append_to_mapping(&mut self.deps_list, b, a);
                Day5::append_to_mapping(&mut self.anti_deps_list, a, b);
            } else {
                let mut print_set = Vec::new();
                for num in l.split(',') {
                    print_set.push(num.parse::<i32>().unwrap());
                }
                self.print_list.push(print_set);
            }
        }
    }

    fn is_valid_print(&self, v: &Vec<i32>) -> bool {
        let mut already_printed: Vec<i32> = Vec::new();
        for x in v {
            let anti_deps = self.anti_deps_list.get(&x);
            match anti_deps {
                Some(antis) => {
                    for ele in antis.iter() {
                        if already_printed.contains(ele) {
                            info!(
                                "Vec {:?} is invalid because {} came after anti dep {}",
                                v, x, ele
                            );
                            return false;
                        }
                    }
                }
                None => {}
            }
            already_printed.push(*x);
        }
        true
    }

    pub fn find_valid_middle_sum(&self) -> i32 {
        self.print_list
            .iter()
            .filter(|plist| {
                let good = self.is_valid_print(plist);
                info!("List was good? {}", good);
                good
            })
            .fold(0, |acc, found| acc + found.get(found.len().div(2)).unwrap())
    }

    fn correct_order(&self, v: &mut Vec<i32>) -> VecDeque<i32> {
        let mut answer: VecDeque<i32> = VecDeque::new();
        let empty_vec: Vec<i32> = Vec::new();

        for (_, val) in v.iter().enumerate() {
            // get all deps and anti deps for val
            let deps = self.deps_list.get(val).unwrap_or(&empty_vec);
            let anti_deps = self.anti_deps_list.get(val).unwrap_or(&empty_vec);

            if answer.len() == 0 {
                debug!("Empty answer, pushed {val} to back");
                answer.push_back(*val);
                continue;
            }

            // index inserted at must be greater than min index
            let mut min_index: usize = usize::MIN;
            // index inserted at must be less than max index
            let mut max_index: usize = usize::MAX;
            let mut found_min = false;
            for (di, existing) in answer.iter().enumerate() {
                if deps.contains(existing) {
                    debug!("Found dep {existing} at {di}");
                    found_min = true;
                    min_index = min_index.max(di);
                }
                if anti_deps.contains(existing) {
                    debug!("Found antidep {existing} at {di}");
                    max_index = max_index.min(di);
                }
            }

            info!("Testing {val} Min index {min_index} max index {max_index}");
            if !found_min {
                debug!("pushed front");
                answer.push_front(*val);
            } else if max_index == usize::MAX {
                debug!("pushed back");
                answer.push_back(*val);
            } else {
                info!("Tricky insert: {val} between {min_index} an {max_index}");
                answer.insert(min_index + 1, *val);
            }
        }

        let test_vec: Vec<i32> = Vec::from(answer.clone());
        let valid = self.is_valid_print(&test_vec);
        info!("New answer: {:?} is valid? {}", answer, valid);
        if !valid {
            panic!("Not valid!");
        }
        answer
    }

    pub fn fix_em_all(&self) -> i32 {
        self.print_list
            .iter()
            .filter(|plist| {
                let good = self.is_valid_print(plist);
                info!("List was good? {}", good);
                !good
            })
            .fold(0, |acc, bad_list| {
                let mut n = bad_list.clone();
                let corr = self.correct_order(&mut n);
                // found.get(found.len().div(2)).unwrap()

                acc + corr.get(corr.len().div(2)).unwrap()
            })
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(5, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d5 = Day5::new(&test_input);
    d5.parse();
    debug!("Deps: {:?}", d5.deps_list);
    debug!("Prints: {:?}", d5.print_list);
    let s = d5.find_valid_middle_sum();
    info!("Found answer: {s}");

    let mut d5 = Day5::new(&real_input);
    d5.parse();
    debug!("Deps: {:?}", d5.deps_list);
    debug!("Prints: {:?}", d5.print_list);
    let s = d5.find_valid_middle_sum();
    info!("Found answer: {s}");

    let mut d5 = Day5::new(&test_input);
    d5.parse();
    debug!("Deps: {:?}", d5.deps_list);
    debug!("Prints: {:?}", d5.print_list);
    let fixed_ans = d5.fix_em_all();
    info!("Found ans: {fixed_ans}");

    let mut d5 = Day5::new(&real_input);
    d5.parse();
    debug!("Deps: {:?}", d5.deps_list);
    debug!("Prints: {:?}", d5.print_list);
    let fixed_ans = d5.fix_em_all();
    info!("Found ans: {fixed_ans}");
}
