use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

use aoc2024::{
    map_vec_extend::{self, append_to_mapping, append_to_mapping_set},
    AocHelper, RequestedAocInputType,
};
use tracing::{debug, info};

// type Point = (usize, usize);
type Point = (i32, i32);
type PointVec = Vec<Point>;
type PointSet = BTreeSet<Point>;

#[derive(Debug)]
struct Day12 {
    raw: String,
    point_sets: BTreeMap<char, PointSet>,
    region_sets: BTreeMap<char, Vec<PointSet>>,
    bounds: Point,
}

fn flood_from_seed(
    seed: Point,
    bounds: Point,
    valids: &BTreeSet<Point>,
    visited: Option<BTreeSet<Point>>,
) -> Option<BTreeSet<Point>> {
    let next = [
        (seed.0, seed.1 + 1),
        (seed.0, seed.1 - 1),
        (seed.0 + 1, seed.1),
        (seed.0 - 1, seed.1),
    ];

    let in_bounds = |p: &Point| p.0 >= 0 && p.0 < bounds.0 && p.1 >= 0 && p.1 < bounds.1;

    let mut _visited;
    if visited.is_none() {
        _visited = BTreeSet::new();
    } else {
        _visited = visited.unwrap();
    }

    _visited.insert(seed);
    next.iter().for_each(|p: &Point| {
        if !in_bounds(p) || _visited.contains(p) || !valids.contains(p) {
            return;
        }
        let res = flood_from_seed(*p, bounds, valids, Some(_visited.clone()));
        if res.is_some() {
            _visited.append(&mut res.unwrap().clone());
        }
    });

    Some(_visited)
}

enum KernelMatch {
    Yes { i: usize },
    No { i: usize },
    Any,
}

const kernel_set: [[KernelMatch; 4]; 8] = [
    [
        KernelMatch::No { i: 0 },
        KernelMatch::Yes { i: 1 },
        KernelMatch::Yes { i: 3 },
        KernelMatch::Yes { i: 4 },
    ],
    [
        KernelMatch::No { i: 2 },
        KernelMatch::Yes { i: 1 },
        KernelMatch::Yes { i: 5 },
        KernelMatch::Yes { i: 4 },
    ],
    [
        KernelMatch::No { i: 8 },
        KernelMatch::Yes { i: 7 },
        KernelMatch::Yes { i: 5 },
        KernelMatch::Yes { i: 4 },
    ],
    [
        KernelMatch::No { i: 6 },
        KernelMatch::Yes { i: 7 },
        KernelMatch::Yes { i: 3 },
        KernelMatch::Yes { i: 4 },
    ],
    [
        KernelMatch::No { i: 3 },
        KernelMatch::No { i: 1 },
        KernelMatch::Any,
        KernelMatch::Any,
    ],
    [
        KernelMatch::No { i: 5 },
        KernelMatch::No { i: 1 },
        KernelMatch::Any,
        KernelMatch::Any,
    ],
    [
        KernelMatch::No { i: 5 },
        KernelMatch::No { i: 7 },
        KernelMatch::Any,
        KernelMatch::Any,
    ],
    [
        KernelMatch::No { i: 3 },
        KernelMatch::No { i: 7 },
        KernelMatch::Any,
        KernelMatch::Any,
    ],
];

fn get_3_by_3(start: &Point, connected: &BTreeSet<Point>) -> Vec<bool> {
    let mut res: Vec<bool> = Vec::new();
    let mut options: Vec<Point> = Vec::new();

    for row in -1..2 {
        for col in -1..2 {
            // if row == col && col == 0 {
            //     continue;
            // }
            options.push((start.0 + row, start.1 + col))
        }
    }

    debug!("Options: {:?}", options);

    for opt in options {
        res.push(connected.contains(&opt))
    }

    res
}

fn get_match_count(v: Vec<bool>) -> usize {
    kernel_set.iter().fold(0, |acc, kset| {
        let mut passes: bool = true;
        for km in kset {
            match km {
                KernelMatch::Yes { i } => passes = v[*i],
                KernelMatch::No { i } => passes = !v[*i],
                KernelMatch::Any => passes = true,
            }
            if !passes {
                break;
            }
        }
        if passes {
            return acc + 1;
        }
        acc
    })
}

fn count_sides(s: &PointSet, bounds: Point) -> usize {
    let in_bounds = |p: &Point| p.0 >= 0 && p.0 < bounds.0 && p.1 >= 0 && p.1 < bounds.1;
    let edge_pieces: PointSet = s
        .iter()
        .filter_map(|p| {
            let next = [
                (p.0, p.1 + 1),
                (p.0, p.1 - 1),
                (p.0 + 1, p.1),
                (p.0 - 1, p.1),
            ];

            let outsides: Vec<&Point> = next
                .iter()
                .filter(|n| !in_bounds(n) || !s.contains(n))
                .collect();
            if outsides.len() >= 1 {
                return Some(*p);
            }
            None
        })
        .collect();

    debug!("Edges: {:?}", edge_pieces);

    let edge_comp = edge_pieces.clone();

    let mut side_count = 0;
    for p in s.iter() {
        // let next = [
        //     (p.0, p.1+1),
        //     (p.0, p.1-1),
        //     (p.0+1, p.1),
        //     (p.0-1, p.1),
        // ];

        // let connecteds: Vec<&Point> = next.iter().filter(|n| {s.contains(n)}).collect();
        // debug!("cons: {:?} len: {}", connecteds, connecteds.len());
        // let len = connecteds.len();
        // match len {
        //     0 => {sides += 4;}
        //     1 => {sides += 2;}
        //     2 => {
        //         // if the connections are in a line
        //         if connecteds[0].0 == connecteds[1].0 || connecteds[0].1 == connecteds[1].1 {
        //             debug!("Striaght side")
        //         }
        //         else {
        //             sides += 1;
        //         }
        //     }
        //     _ => {
        //         info!("What to do when con is {len}");
        //     }
        // }

        let filled_kernel = get_3_by_3(p, s);
        let single_score = get_match_count(filled_kernel);
        debug!("Got corner count: {single_score} for piece {:?}", p);
        side_count += single_score;
    }

    side_count
}

impl Day12 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            point_sets: BTreeMap::new(),
            region_sets: BTreeMap::new(),
            bounds: (0, 0),
        }
    }

    fn find_islands(&mut self, c: char) -> Option<Vec<PointSet>> {
        let Some(all_points) = self.point_sets.get(&c) else {
            return None;
        };
        let mut regions: Vec<PointSet> = Vec::new();
        let mut non_regioned: PointSet = all_points.clone();
        let seed = all_points.first().unwrap();
        let mut res = flood_from_seed(*seed, self.bounds, all_points, None);
        while res.is_some() {
            debug!("Res of first flood for {}: {:?}", c, res);
            for ele in res.clone().unwrap().iter() {
                non_regioned.remove(ele);
            }
            regions.push(res.clone().unwrap());

            let seed = non_regioned.first();
            if seed.is_some() {
                res = flood_from_seed(*seed.unwrap(), self.bounds, &non_regioned, None);
            } else {
                break;
            }
        }
        info!("Regions: {:?}", regions);
        Some(regions)
    }

    fn calculate_perimeter(&self, s: &PointSet) -> usize {
        let mut perimeter = 0;

        for seed in s {
            let next = [
                (seed.0, seed.1 + 1),
                (seed.0, seed.1 - 1),
                (seed.0 + 1, seed.1),
                (seed.0 - 1, seed.1),
            ];

            let in_bounds =
                |p: &Point| p.0 >= 0 && p.0 < self.bounds.0 && p.1 >= 0 && p.1 < self.bounds.1;

            for np in next {
                if !in_bounds(&np) {
                    perimeter += 1;
                } else if !s.contains(&np) {
                    perimeter += 1;
                }
            }
        }

        perimeter
    }

    fn calculate_area_perimter_for_char(&self, c: char) -> usize {
        let Some(regions) = self.region_sets.get(&c) else {
            return 0;
        };

        let score = regions.iter().fold(0, |acc, reg| {
            let p = self.calculate_perimeter(reg);
            let a = reg.len();
            debug!("reg {:?} has perimeter {} area {}", reg, p, a);
            acc + (a * p)
        });
        info!("Perimeter total for {} is {}", c, score);
        score
    }

    pub fn find_total_score_p1(&self) -> usize {
        let chars: Vec<char> = self.point_sets.keys().clone().map(|k| *k).collect();
        chars
            .iter()
            .fold(0, |acc, k| acc + self.calculate_area_perimter_for_char(*k))
    }

    pub fn find_all_islands(&mut self) {
        let chars: Vec<char> = self.point_sets.keys().clone().map(|k| *k).collect();
        chars.iter().for_each(|k| {
            if let Some(res) = self.find_islands(*k) {
                self.region_sets.insert(*k, res);
            };
        });
    }

    pub fn parse(&mut self) {
        let mut mr = 0;
        let mut mc = 0;
        for (row, line) in self.raw.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                // debug!("Char {} @ ({}, {})", char, row, col);
                append_to_mapping_set(&mut self.point_sets, char, (row as i32, col as i32));
                mc = col;
            }
            mr = row
        }
        self.bounds = ((mr + 1) as i32, (mc + 1) as i32);
    }

    fn calculate_area_sides_for_char(&self, c: char) -> usize {
        let Some(regions) = self.region_sets.get(&c) else {
            return 0;
        };

        let mut area_total = 0;
        let mut side_total = 0;
        let score = regions.iter().fold(0, |acc, reg| {
            let p = count_sides(reg, self.bounds);
            side_total += p;
            let a = reg.len();
            area_total += a;
            // debug!("reg {:?} has sides {} area {}", reg, p, a);
            debug!("Score for {} is {} * {} = {}", c, a, p, a * p);
            acc + (a * p)
        });
        info!(
            "Score for {} is {} * {} = {}",
            c, area_total, side_total, score
        );
        score
    }

    pub fn find_total_score_p2(&self) -> usize {
        let chars: Vec<char> = self.point_sets.keys().clone().map(|k| *k).collect();
        chars
            .iter()
            .fold(0, |acc, k| acc + self.calculate_area_sides_for_char(*k))
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(12, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d12 = Day12::new(&test_input);
    d12.parse();
    debug!("{:?}", d12);
    d12.find_all_islands();
    debug!("{:?}", d12);
    let score = d12.find_total_score_p1();
    info!("Score is {}", score);
    let score = d12.find_total_score_p2();
    info!("Score is {}", score);
    // return;

    let mut d12 = Day12::new(&real_input);
    d12.parse();
    debug!("{:?}", d12);
    d12.find_all_islands();
    debug!("{:?}", d12);
    let score = d12.find_total_score_p1();
    info!("Score is {}", score);
    let score = d12.find_total_score_p2();
    info!("Score is {}", score);
}
