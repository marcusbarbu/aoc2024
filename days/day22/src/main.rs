use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

use aoc2024::{AocHelper, RequestedAocInputType};
use itertools::Itertools;
use itertools::TupleWindows;
use tracing::{debug, info};

const PRUNE_CONST: i64 = 16777216 - 1;
type Sequence = [i64; 4];

#[derive(Debug)]
struct Day22 {
    raw: String,
    init_nums: Vec<i64>,
}

impl Day22 {
    pub fn new(s: &String) -> Self {
        let nums: Vec<i64> = s.lines().map(|l| l.parse::<i64>().unwrap()).collect();

        Self {
            raw: s.clone(),
            init_nums: nums,
        }
    }

    pub fn single_iter(start: i64) -> (i64, i64, i64) {
        let mut secret = start;
        let a_ones = start % 10;
        let a = start << 6;
        secret = (secret ^ a) & PRUNE_CONST;
        let b = secret >> 5;
        secret = (secret ^ b) & PRUNE_CONST;
        let c = secret << 11;
        secret = (secret ^ c) & PRUNE_CONST;
        let b_ones = secret % 10;

        let delta = b_ones - a_ones;

        (secret, b_ones, delta)
    }

    pub fn get_2kth_numbers(&self) -> i64 {
        self.init_nums.iter().fold(0, |acc, num| {
            let mut res = *num;
            for _ in 0..2000 {
                (res, _, _) = Day22::single_iter(res);
            }
            debug!("{num} becomes {res}");
            acc + res
        })
    }

    pub fn get_sequence_scores(start: i64, n: usize) -> HashMap<Sequence, i64> {
        let mut score_map: HashMap<Sequence, i64> = HashMap::new();
        let mut cur = start;
        let mut price: i64 = 0;
        let mut change: i64 = 0;

        type PriceChange = (i64, i64);
        let mut res_vec: Vec<(i64, i64)> = Vec::new();
        for _ in 0..n {
            (cur, price, change) = Day22::single_iter(cur);
            res_vec.push((price, change));
        }

        let pct = res_vec
            .iter()
            .tuple_windows::<(&PriceChange, &PriceChange, &PriceChange, &PriceChange)>();
        for p in pct {
            // debug!("Windows: {:?}", p);
            let sequence: [i64; 4];

            sequence = [p.0 .1, p.1 .1, p.2 .1, p.3 .1];
            if score_map.contains_key(&sequence) {
                continue;
            } else {
                score_map.insert(sequence, p.3 .0);
            }
        }

        // debug!("Score map: {:?}", score_map);

        score_map
    }

    pub fn get_best_score(&self) -> i64 {
        let mut total_scores: HashMap<Sequence, i64> = HashMap::new();
        for start in self.init_nums.iter() {
            let score_map = Day22::get_sequence_scores(*start, 2000);
            score_map.iter().for_each(|(seq, score)| {
                let tot = total_scores.get(seq).unwrap_or(&0) + score;
                total_scores.insert(*seq, tot);
            });
        }
        debug!("Score map: {:?}", total_scores);
        let mut scores: Vec<i64> = total_scores.iter().map(|(_, score)| *score).collect();
        scores.sort();
        debug!("All scores: {:?}", scores);
        *scores.last().unwrap()
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(22, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d22 = Day22::new(&test_input);
    debug!("{:?}", d22);
    let ans = d22.get_2kth_numbers();
    info!("Ans: {ans}");

    let mut d22 = Day22::new(&real_input);
    debug!("{:?}", d22);
    let ans = d22.get_2kth_numbers();
    info!("Ans: {ans}");

    let mut d22 = Day22::new(&test_input);
    debug!("{:?}", d22);
    let ans = d22.get_best_score();
    info!("Ans: {}", ans);

    let mut d22 = Day22::new(&real_input);
    debug!("{:?}", d22);
    let ans = d22.get_best_score();
    info!("Ans: {}", ans);
}
