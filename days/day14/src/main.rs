use std::{collections::BTreeMap, thread::sleep, time::Duration};

use aoc2024::{map_vec_extend::append_to_mapping, AocHelper, RequestedAocInputType};
use rayon::iter::{IntoParallelRefIterator, Map};
use regex::Regex;
use tracing::{debug, info};
use tracing_subscriber::field::debug;

use rayon::prelude::*;

type Point = (i32, i32);

const BUTTON_REGEX: &str = r"Button [A|B]: X(.*), Y(.*)";
//                      p=6,3 v=-1,-3
const PV_REGEX: &str = r"p=(.*),(.*) v=(.*),(.*)";

#[derive(Debug, Clone)]
struct Robot {
    start_loc: Point,
    current_loc: Point,
    velocity: Point,
}

fn display_grid(p: &[Point], bounds: Point) {
    let mut matrix: Vec<String> = Vec::new();
    for row in 0..bounds.0 {
        let mut rowstr: String = String::new();
        for col in 0..bounds.1 {
            if p.contains(&(row, col)) {
                rowstr.push('X');
            } else {
                rowstr.push('.');
            }
        }
        matrix.push(rowstr);
    }

    println!("{}", matrix.join("\n"));
}

fn debug_point_on_grid(p: Point, bounds: Point) {
    display_grid(&[p], bounds);
}

// fn debug_point_on_grid(p: Point, bounds: Point) {
//     let mut matrix: Vec<String> = Vec::new();
//     for row in 0..bounds.0 {
//         let mut rowstr: String = String::new();
//         for col in 0 .. bounds.1 {
//             if p.0 == row && p.1 == col {
//                 rowstr.push('X');
//             }
//             else {
//                 rowstr.push('.');
//             }
//         }
//         matrix.push(rowstr);
//     }

//     println!("{}", matrix.join("\n"));

// }

impl Robot {
    pub fn step_one(&mut self, bounds: Point) {
        self.current_loc.0 += self.velocity.0;
        self.current_loc.0 = self.current_loc.0.rem_euclid(bounds.0);
        self.current_loc.1 += self.velocity.1;
        self.current_loc.1 = self.current_loc.1.rem_euclid(bounds.1);
    }

    pub fn find_cycle_time(&self, bounds: Point) -> usize {
        let mut count: usize = 1;
        let mut working = self.start_loc.clone();
        working.0 += self.velocity.0;
        working.0 = working.0.rem_euclid(bounds.0);

        working.1 += self.velocity.1;
        working.1 = working.1.rem_euclid(bounds.1);

        while working != self.start_loc {
            // debug!("After {count} steps: {:?}", working);
            // debug_point_on_grid(working, bounds);

            working.0 += self.velocity.0;
            working.0 = working.0.rem_euclid(bounds.0);

            working.1 += self.velocity.1;
            working.1 = working.1.rem_euclid(bounds.1);

            count += 1;
        }

        debug!("cycle is {count}");
        count
    }

    pub fn location_after_steps(&self, n: usize, bounds: Point) -> Point {
        let mut working = self.start_loc.clone();
        debug!("Moving robot {:?}", self);
        for i in 0..n {
            // debug!("After {i} steps: {:?}", working);
            // debug_point_on_grid(working, bounds);

            working.0 += self.velocity.0;
            working.0 = working.0.rem_euclid(bounds.0);

            working.1 += self.velocity.1;
            working.1 = working.1.rem_euclid(bounds.1);
        }

        // debug!("After {n} steps: {:?}", working);
        // debug_point_on_grid(working, bounds);
        working
    }
}

#[derive(Debug, Clone)]
struct Day14 {
    raw: String,
    bounds: Point,
    robots: Vec<Robot>,
}

impl Day14 {
    pub fn new(s: &String, rows: i32, cols: i32) -> Self {
        Day14 {
            raw: s.clone(),
            bounds: (rows, cols),
            robots: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        let reg = Regex::new(PV_REGEX).unwrap();
        self.raw.lines().for_each(|line| {
            let caps = reg.captures(line).unwrap();
            let p0: i32 = caps[1].parse::<i32>().unwrap();
            let p1: i32 = caps[2].parse::<i32>().unwrap();
            let v0: i32 = caps[3].parse::<i32>().unwrap();
            let v1: i32 = caps[4].parse::<i32>().unwrap();

            debug!("{p0}, {p1} -> {v0}, {v1}");
            let r = Robot {
                start_loc: (p1, p0),
                current_loc: (p1, p0),
                velocity: (v1, v0),
            };

            self.robots.push(r);
        });
    }

    pub fn get_score(&mut self, steps: usize) -> i32 {
        let mut final_positions: Vec<Point> = Vec::new();
        self.robots
            .par_iter()
            .map(|robot| robot.location_after_steps(steps, self.bounds.clone()))
            .collect_into_vec(&mut final_positions);

        let middle: Point = (self.bounds.0 / 2, self.bounds.1 / 2);

        debug!("Final positions: {:?}", final_positions);

        debug!("Bounds: {:?} Midpoint: {:?}", self.bounds, middle);
        let mut scores: [i32; 4] = [0, 0, 0, 0];

        for pos in final_positions {
            if pos.0 == middle.0 || pos.1 == middle.1 {
                continue;
            }
            let top_bottom = pos.0 < middle.0;
            let left_right = pos.1 < middle.1;

            debug!("Pos: {:?} tb: {top_bottom} lr: {left_right}", pos);

            match (top_bottom, left_right) {
                (true, true) => {
                    scores[0] += 1;
                }
                (true, false) => {
                    scores[1] += 1;
                }
                (false, true) => {
                    scores[2] += 1;
                }
                (false, false) => {
                    scores[3] += 1;
                }
            }
        }

        debug!("Scores: {:?}", scores);

        let score = scores.iter().fold(1, |acc, s| acc * s);

        score
    }

    pub fn step_all_and_display(&mut self) {
        let mut step = 0;
        loop {
            step += 1;
            self.robots.par_iter_mut().for_each(|robot| {
                robot.step_one(self.bounds);
            });

            let robs: Vec<Point> = self.robots.iter().map(|robot| robot.current_loc).collect();
            let mut line_map: BTreeMap<i32, Vec<Point>> = BTreeMap::new();

            robs.iter().for_each(|point| {
                append_to_mapping(&mut line_map, point.0, *point);
            });

            if line_map.iter().any(|mp| mp.1.len() > 20) {
                info!("Step: {step}");
                display_grid(&robs, self.bounds);
            }

            let mut line_map: BTreeMap<i32, Vec<Point>> = BTreeMap::new();

            robs.iter().for_each(|point| {
                append_to_mapping(&mut line_map, point.1, *point);
            });

            if line_map.iter().any(|mp| mp.1.len() > 20) {
                info!("Step: {step}");
                display_grid(&robs, self.bounds);
            }

            // sleep(Duration::from_millis(100));
        }
    }

    pub fn find_cycles(&self) {
        let mut cycle_times: Vec<usize> = Vec::new();
        cycle_times = self
            .robots
            .iter()
            .map(|robot| robot.find_cycle_time(self.bounds))
            .collect(); //.collect_into_vec(&mut cycle_times);

        debug!("Cycle times: {:?}", cycle_times);
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(14, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d14 = Day14::new(&test_input, 7, 11);
    d14.parse();
    debug!("{:?}", d14);
    // d14.robots[10].location_after_steps(5, d14.bounds);
    let ans = d14.get_score(100);
    info!("Score: {ans}");

    let mut d14 = Day14::new(&real_input, 103, 101);
    d14.parse();
    debug!("{:?}", d14);
    // d14.robots[10].location_after_steps(5, d14.bounds);
    let ans = d14.get_score(100);
    info!("Score: {ans}");

    let mut d14 = Day14::new(&real_input, 103, 101);
    d14.parse();
    d14.step_all_and_display();
    // d14.find_cycles();

    // let x: i32 = -18;
    // let xt: i32 = x.rem_euclid(7 as i32);

    // debug!("{xt}" )
}
