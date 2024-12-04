use std::{borrow::Borrow, collections::HashSet};

use aoc2024::{AocHelper, RequestedAocInputType};
use tracing::{debug, error, info};

struct Day2A {
    raw: String,
    pub rows: Vec<Vec<i32>>,
    a: Vec<Vec<i32>>,
    b: Vec<Vec<i32>>,
}

impl Day2A {
    pub fn new(s: &String) -> Self {
        Day2A {
            raw: s.clone(),
            rows: Vec::new(),
            a: Vec::new(),
            b: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        let lines = self.raw.lines();
        lines.for_each(|line| {
            let ii = line
                .split_ascii_whitespace()
                .map(|l| l.parse::<i32>().unwrap());
            let v: Vec<i32> = Vec::from_iter(ii);
            self.rows.push(v);
        });
    }

    fn get_row_unsafe_index(row: &Vec<i32>) -> Option<usize> {
        let mut ri = row.iter().enumerate();
        let (_, last_ref) = ri.next().unwrap();
        let mut last = *last_ref;
        let mut dir: Option<bool> = None;
        for (index, cur_row) in ri {
            let new_dir = *cur_row > last;
            if dir.is_some() && new_dir != dir.unwrap() {
                // error!("Direction changed during row: {:?}", row);
                return Some(index);
            }
            dir = Some(new_dir);
            let diff = last.abs_diff(*cur_row);
            // debug!("Diff of last: {} and cur: {} is {}", last, cur_row, diff);
            match diff {
                1..4 => {
                    last = *cur_row;
                }
                _ => {
                    // error!("Diff {} in row {:?} is too large", diff, row);
                    return Some(index);
                }
            }
        }
        None
    }

    pub fn get_safe_rows(&self) -> i32 {
        let res = self.rows.iter().fold(0, |acc, row| {
            let unsafe_idx = Day2A::get_row_unsafe_index(row);
            match unsafe_idx {
                Some(_) => acc,
                None => acc + 1,
            }
        });
        res
    }

    pub fn get_semi_safe_rows(&mut self) -> i32 {
        let mut safe = Vec::new();
        let res = self.rows.iter().fold(0, |acc, row| {
            let base = row.clone();
            let unsafe_idx = Day2A::get_row_unsafe_index(row);
            if unsafe_idx.is_none() {
                safe.push(row.clone());
                return acc + 1;
            } else {
                let unsafe_idx = unsafe_idx.unwrap();
                for i in unsafe_idx - 1..unsafe_idx + 2 {
                    info!("i is {} unsafe was {}", i, unsafe_idx);
                    let mut test_row = base.clone();
                    test_row.remove(i);
                    let ans = Day2A::get_row_unsafe_index(&test_row);
                    if ans.is_none() {
                        safe.push(test_row);
                        return acc + 1;
                    }
                }
            }
            acc
        });
        self.a = safe;
        res
    }

    fn single_brute_force(row: &Vec<i32>) -> bool {
        let base = row.clone();
        if Day2A::get_row_unsafe_index(row).is_none() {
            return true;
        }

        for i in 0..row.len() {
            let mut cur = base.clone();
            cur.remove(i);
            if Day2A::get_row_unsafe_index(&cur).is_none() {
                return true;
            }
        }

        false
    }

    pub fn forget_it_brute_force_it(&mut self) -> i32 {
        let mut succ = Vec::new();
        let res = self.rows.iter().fold(0, |acc, row| {
            if Day2A::single_brute_force(row) {
                succ.push(row.clone());
                return acc + 1;
            }
            acc
        });
        self.b = succ;
        res
    }

    pub fn what_even(&self) {
        info!("Lengths: b {} a {}", self.b.len(), self.a.len());
        let mut b_hash = HashSet::new();
        let mut a_hash = HashSet::new();
        self.b.iter().for_each(|row| {
            b_hash.insert(format!("{:?}", row));
        });
        self.a.iter().for_each(|row| {
            a_hash.insert(format!("{:?}", row));
        });

        let diff = b_hash.difference(&a_hash);
        info!("Diff: len {} {:?}", diff.clone().count(), diff);
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(2, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d2a_test = Day2A::new(&test_input);
    d2a_test.parse();
    info!("Current: {:?}", d2a_test.rows.clone());
    let safe = d2a_test.get_safe_rows();
    let semi_safe = d2a_test.get_semi_safe_rows();
    info!("Safe row count: {}", safe);
    info!("Semi Safe row count: {}", semi_safe);

    // return;
    let mut d2a_real = Day2A::new(&real_input);
    d2a_real.parse();
    info!("Current: {:?}", d2a_real.rows.clone());
    let safe = d2a_real.get_safe_rows();
    info!("Safe row count: {}", safe);
    let semi_safe = d2a_real.get_semi_safe_rows();
    info!("Semi Safe row count: {}", semi_safe);
    let bf = d2a_real.forget_it_brute_force_it();
    info!("Brute force count: {}", bf);

    d2a_real.what_even();
}
