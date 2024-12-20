use std::collections::{BTreeMap, BTreeSet, HashMap};

use aoc2024::{map_vec_extend::append_to_mapping, AocHelper, RequestedAocInputType};
use tracing::{debug, error, info, Level};

#[derive(Debug)]
struct Day19 {
    raw: String,
    pieces: BTreeMap<char, Vec<String>>,
    targets: Vec<String>,
    checked: BTreeMap<(String, String), bool>,
}

impl Day19 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            pieces: BTreeMap::new(),
            targets: Vec::new(),
            checked: BTreeMap::new(),
        }
    }

    pub fn parse(&mut self) {
        let (mut pieces, mut targets) = self.raw.split_once("\n\n").unwrap();
        pieces.split(", ").for_each(|p| {
            let fc = p.chars().next().unwrap();
            append_to_mapping(&mut self.pieces, fc, p.to_string());
        });
        targets.lines().for_each(|line| {
            self.targets.push(line.to_string());
        });
    }

    pub fn is_possible(
        &self,
        working: &String,
        target: &String,
        checked_map: &mut BTreeMap<(String, String), bool>,
    ) -> bool {
        if let Some(res) = checked_map.get(&(working.to_string(), target.to_string())) {
            return *res;
        };
        if *working == *target {
            return true;
        }
        info!("Checking if there's a possible path to {target} from {working}");
        // let tchars: Vec<char> = target.chars().collect();
        // let cur_char = tchars[index];
        let wlen = working.len();
        let cur_char = target.chars().nth(wlen).unwrap();
        let Some(possibles) = self.pieces.get(&cur_char) else {
            checked_map.insert((working.clone(), target.clone()), false);
            return false;
        };
        info!("Found {} options", possibles.len());
        debug!("Looking for strings that start with {cur_char}");
        for p in possibles.iter() {
            let total_option = working.to_owned() + p;
            let ll = total_option.len();

            debug!("\t Option {p} results in option {total_option} with len {ll}");
            if ll > target.len() {
                continue;
            }

            if total_option == target[0..ll] {
                debug!("\t\t Recursing");
                let res = self.is_possible(&total_option, target, checked_map);
                if res {
                    return res;
                }
            }
        }
        checked_map.insert((working.clone(), target.clone()), false);
        false
    }

    pub fn count_possible(&mut self) -> usize {
        let mut check_map: BTreeMap<(String, String), bool> = BTreeMap::new();
        self.targets.iter().fold(0, |acc, target| {
            let start: String = String::from("");
            let res = self.is_possible(&start, target, &mut check_map);
            error!("Target {target}: {res}");

            if res {
                return acc + 1;
            }
            acc
        })
    }

    pub fn count_all_ways(
        &self,
        working: &String,
        target: &String,
        checked_map: &mut BTreeMap<(String, String), usize>,
    ) -> usize {
        // check if we made it to a match
        //      if yes, return 1 (we've got one option)
        // if not
        //      get all of the options that start with the next character
        //          for each option
        //              check if current + option = target[0..working.len()]
        //                  if yes:
        //                      add working score + count_all_ways(option)
        //

        if let Some(res) = checked_map.get(&(working.to_string(), target.to_string())) {
            return *res;
        };

        // this is the first one that worked out
        if *working == *target {
            checked_map.insert((working.to_string(), target.to_string()), 1);
            return 1;
        }

        let working_index = working.len();
        let Some(cur_char) = target.chars().nth(working_index) else {
            return 0;
        };
        let Some(options) = self.pieces.get(&cur_char) else {
            return 0;
        };

        let mut count: usize = 0;

        for option in options {
            let test = working.clone() + option;
            let len_test = test.len();

            if len_test > target.len() {
                // count += 0;
                continue;
            }

            if len_test == target.len() {
                if test == *target {
                    count += 1;
                }
                continue;
            }

            if test == target[0..len_test] {
                count += self.count_all_ways(&test, target, checked_map);
            }
        }
        checked_map.insert((working.to_string(), target.to_string()), count);
        count
    }

    pub fn count_all_perms(&self) -> usize {
        let mut check_map: BTreeMap<(String, String), usize> = BTreeMap::new();
        self.targets.iter().fold(0, |acc, target| {
            let start: String = String::from("");
            // info!("Looking for {target}");
            // if self.is_possible(&start, target, &mut check_map) {
            //     return acc + 1;
            // }
            let count = self.count_all_ways(&start, target, &mut check_map);
            info!("Count {count} for {target}");
            acc + count
        })
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(19, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();
    // AocHelper::setup_subscriber(Level::INFO);

    let mut d19 = Day19::new(&test_input);
    d19.parse();
    debug!("{:?}", d19);
    let ans = d19.count_possible();
    info!("Ans: {ans}");

    let mut d19 = Day19::new(&real_input);
    d19.parse();
    debug!("{:?}", d19);
    let ans = d19.count_possible();
    info!("Ans: {ans}");

    let mut d19 = Day19::new(&test_input);
    d19.parse();
    debug!("{:?}", d19);
    let ans = d19.count_all_perms();
    info!("Ans: {ans}");

    let mut d19 = Day19::new(&real_input);
    d19.parse();
    debug!("{:?}", d19);
    let ans = d19.count_all_perms();
    info!("Ans: {ans}");
}
