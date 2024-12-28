use std::{collections::HashSet};

use aoc2024::{AocHelper, RequestedAocInputType};
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use tracing::{debug, info};

#[derive(Debug)]
struct Day25 {
    raw: String,
    all_maps: Vec<[usize;5]>,
    keys: HashSet<[usize;5]>,
    locks: HashSet<[usize;5]>
}

impl Day25 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            all_maps: Vec::new(),
            keys: HashSet::new(),
            locks: HashSet::new(),
        }
    }

    pub fn parse(&mut self) {
        let grids = self.raw.split("\n\n");
        for grid in grids {
            let mut code: [usize;5] = [0,0,0,0,0];
            let mut lines = grid.lines();
            let zeroth = lines.next().unwrap();
            let is_lock: bool;

            if zeroth.contains("#") {
                is_lock = true;
            }
            else {
                is_lock = false;
            }

            for (row, line) in lines.enumerate() {
                let code_height = if is_lock { row + 1 } else  { 5 - row };
                for (col, val) in line.chars().enumerate() {
                    if val == '.' {
                        continue;
                    }

                    if code[col] >= code_height {
                        continue;
                    }

                    code[col] = code_height;
                }
            }

            self.all_maps.push(code.clone());
            if is_lock{
                self.locks.insert(code);
            }
            else {
                self.keys.insert(code);
            }
            debug!("IS_LOCK: {is_lock} Grid: 
{}
        has code: {:?}", grid, code);
        }
    }

    pub fn find_match_count(&self) -> usize {
        self.keys.par_iter().fold(|| 0, |acc, key_code| {
            let mut match_count = 0;
            for lock in self.locks.iter() {
                debug!("Testing lock: {:?} with key: {:?}", lock, key_code);
                let mut matches = true;
                for tumbler in 0..5 {
                    if lock[tumbler] + key_code[tumbler] > 5 {
                        debug!("{:?} and {:?} overlap!", lock, key_code);
                        matches = false;
                        break;
                    }
                }
                if matches {
                    match_count += 1;
                }
            }
            acc + match_count
        }).sum::<usize>()
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(25, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d25 = Day25::new(&test_input);
    d25.parse();
    debug!("{:?}", d25);
    let ans = d25.find_match_count();
    info!("Ans: {}", ans);

    let mut d25 = Day25::new(&real_input);
    d25.parse();
    debug!("{:?}", d25);
    let ans = d25.find_match_count();
    info!("Ans: {}", ans);
}
