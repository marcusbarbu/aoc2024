use std::collections::{BTreeMap, BTreeSet};

use aoc2024::{AocHelper, RequestedAocInputType};
use tracing::{debug, error, info};

type Point = (i32, i32);

#[derive(Debug)]
struct Day10 {
    raw: String,
    matrix: Vec<Vec<i32>>,
    map: BTreeMap<Point, i32>,
    num_rows: i32,
    num_cols: i32,
    trailheads: Vec<Point>,
}

#[derive(Clone, Debug)]
struct Answer {
    _path: Vec<Point>,
    endpoint: Point,
}

impl Day10 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            matrix: Vec::new(),
            num_cols: 0,
            num_rows: 0,
            map: BTreeMap::new(),
            trailheads: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        let mut max_row = 0;
        let mut max_col = 0;
        for (row, line) in self.raw.lines().into_iter().enumerate() {
            let mut row_vec: Vec<i32> = Vec::new();
            for (col, hc) in line.chars().into_iter().enumerate() {
                if hc != '.' {
                    let height: i32 = hc.to_string().parse::<i32>().unwrap();
                    row_vec.push(height);
                    if height == 0 {
                        self.trailheads.push((row as i32, col as i32));
                    }

                    self.map.insert((row as i32, col as i32), height);
                }

                max_col = col;
            }
            max_row = row;
            self.matrix.push(row_vec);
        }
        self.num_rows = (max_row + 1) as i32;
        self.num_cols = (max_col + 1) as i32;
    }

    fn reach_nines(
        map: &BTreeMap<Point, i32>,
        cur_point: Point,
        bounds: Point,
        visited: Option<Vec<Point>>,
    ) -> Option<Vec<Answer>> {
        let mut _visited: Vec<Point>;
        if visited.is_none() {
            _visited = Vec::new();
        } else {
            _visited = visited.unwrap().clone();
        }

        debug!(
            "Trying to find nines from {:?} visited: {:?}",
            cur_point, _visited
        );

        _visited.push(cur_point);

        let Some(height) = map.get(&cur_point) else {
            return None;
        };

        if *height == 9 {
            let ans = Answer {
                _path: _visited,
                endpoint: cur_point,
            };

            return Some(vec![ans]);
        }

        let up = (cur_point.0 - 1, cur_point.1);
        let down = (cur_point.0 + 1, cur_point.1);
        let left = (cur_point.0, cur_point.1 - 1);
        let right = (cur_point.0, cur_point.1 + 1);

        let in_bounds = |p: Point| p.0 >= 0 && p.0 < bounds.0 && p.1 >= 0 && p.1 < bounds.1;

        let height_check_next = |p: Point| {
            let Some(next_height) = map.get(&p) else {
                return false;
            };
            debug!("cur {height} next {next_height}");
            return *next_height == (*height) + 1;
        };

        let mut solutions: Vec<Answer> = Vec::new();
        for next in [up, down, left, right] {
            if in_bounds(next) && !_visited.contains(&next) && height_check_next(next) {
                let Some(res) = Day10::reach_nines(map, next, bounds, Some(_visited.clone()))
                else {
                    continue;
                };
                res.iter().for_each(|r| solutions.push(r.clone()));
            }
        }

        if solutions.len() > 0 {
            error!("Found an answer!");
            return Some(solutions.clone());
        }
        None
    }

    fn get_score(answers: Vec<Answer>) -> usize {
        let mut ends: BTreeSet<Point> = BTreeSet::new();
        for ele in answers.iter() {
            ends.insert(ele.endpoint);
        }
        ends.len()
    }

    pub fn score_trailheads(&self) -> (usize, usize) {
        let mut scores: Vec<usize> = Vec::new();
        let mut totals: Vec<usize> = Vec::new();
        for head in self.trailheads.iter() {
            let res = Day10::reach_nines(&self.map, *head, (self.num_rows, self.num_cols), None);
            debug!("Found res {:?} for head {:?}", res, head);
            let Some(answers) = res else {
                scores.push(0);
                continue;
            };
            let total = answers.len();
            let score = Day10::get_score(answers);
            debug!("Score is {} total paths is {}", score, total);
            scores.push(score);
            totals.push(total);
        }
        info!("Scores: {:?}", scores);
        info!("Totals: {:?}", totals);
        (
            scores.iter().fold(0, |acc, score| acc + score),
            totals.iter().fold(0, |acc, score| acc + score),
        )
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(10, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d10 = Day10::new(&test_input);
    d10.parse();
    debug!("{:?}", d10);
    let ans = d10.score_trailheads();
    info!("Answer: {:?}", ans);

    // return;

    let mut d10 = Day10::new(&real_input);
    d10.parse();
    debug!("{:?}", d10);
    let ans = d10.score_trailheads();
    info!("Answer: {:?}", ans);
}
