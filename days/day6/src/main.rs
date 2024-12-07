use std::collections::BTreeSet;

use aoc2024::{AocHelper, RequestedAocInputType};
use rayon::prelude::*;
use tracing::{debug, info};
use tracing_subscriber::filter::combinator::Or;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Orientation {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Day6 {
    raw: String,
    pub obstacles: BTreeSet<(usize, usize)>,
    pub start_pos: (usize, usize),
    pub start_orientation: Option<Orientation>,
    pub dimensions: (usize, usize),
}

impl Day6 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            obstacles: BTreeSet::new(),
            start_pos: (0, 0),
            start_orientation: None,
            dimensions: (0, 0),
        }
    }

    pub fn parse(&mut self) {
        let lines = self.raw.lines();
        let mut row_max: usize = 0;
        let mut col_max: usize = 0;
        for (row, line) in lines.into_iter().enumerate() {
            for (col, char) in line.char_indices() {
                match char {
                    '#' => {
                        self.obstacles.insert((row, col));
                    }
                    '<' => {
                        self.start_pos = (row, col);
                        self.start_orientation = Some(Orientation::West);
                    }
                    '^' => {
                        self.start_pos = (row, col);
                        self.start_orientation = Some(Orientation::North);
                    }
                    'v' => {
                        self.start_pos = (row, col);
                        self.start_orientation = Some(Orientation::South);
                    }
                    '>' => {
                        self.start_pos = (row, col);
                        self.start_orientation = Some(Orientation::East);
                    }
                    _ => {}
                }
                col_max = col;
            }
            row_max = row;
        }
        self.dimensions = (row_max + 1, col_max + 1);
    }

    pub fn walk_the_pattern(&self) -> usize {
        let mut cur_pos = self.start_pos.clone();
        let mut cur_orientation = self.start_orientation.unwrap().clone();

        fn in_range(p: (usize, usize), q: (usize, usize)) -> bool {
            p.0 > 0 && p.1 > 0 && p.0 < q.0 && p.1 < q.1
        }

        let mut naive_set: BTreeSet<(usize, usize)> = BTreeSet::new();
        while in_range(cur_pos, self.dimensions) {
            debug!("Currently at {:?} facing {:?}", cur_pos, cur_orientation);
            let mut next_pos = cur_pos;
            let next_orientation: Orientation;
            match cur_orientation {
                Orientation::North => {
                    next_pos.0 -= 1;
                    next_orientation = Orientation::East;
                }
                Orientation::South => {
                    next_pos.0 += 1;
                    next_orientation = Orientation::West;
                }
                Orientation::East => {
                    next_pos.1 += 1;
                    next_orientation = Orientation::South;
                }
                Orientation::West => {
                    next_pos.1 -= 1;
                    next_orientation = Orientation::North;
                }
            }

            if self.obstacles.contains(&next_pos) {
                debug!(
                    "WILL HIT SOMETHING, TURNING INSTEAD to {:?}",
                    next_orientation
                );
                cur_orientation = next_orientation;
            } else {
                naive_set.insert(cur_pos);
                cur_pos = next_pos;
            }
        }

        info!("set: {:?}, size: {}", naive_set, naive_set.len());

        naive_set.len()
    }
}

fn find_a_loop(
    start_pos: (usize, usize),
    test_pos: (usize, usize),
    dimensions: (usize, usize),
    start_orientation: Orientation,
    obstacles: &BTreeSet<(usize, usize)>,
) -> bool {
    let mut cur_pos = start_pos;
    let mut cur_orientation = start_orientation;

    fn in_range(p: (usize, usize), q: (usize, usize)) -> bool {
        p.0 > 0 && p.1 > 0 && p.0 < q.0 && p.1 < q.1
    }

    let mut less_naive_set: BTreeSet<(usize, usize, Orientation)> = BTreeSet::new();
    while in_range(cur_pos, dimensions) {
        // debug!("Currently at {:?} facing {:?}", cur_pos, cur_orientation);
        let mut next_pos = cur_pos;
        let next_orientation: Orientation;
        match cur_orientation {
            Orientation::North => {
                next_pos.0 -= 1;
                next_orientation = Orientation::East;
            }
            Orientation::South => {
                next_pos.0 += 1;
                next_orientation = Orientation::West;
            }
            Orientation::East => {
                next_pos.1 += 1;
                next_orientation = Orientation::South;
            }
            Orientation::West => {
                next_pos.1 -= 1;
                next_orientation = Orientation::North;
            }
        }

        if obstacles.contains(&next_pos) || next_pos == test_pos {
            // debug!("WILL HIT SOMETHING, TURNING INSTEAD to {:?}", next_orientation);
            cur_orientation = next_orientation;
        } else {
            let insert_status = less_naive_set.insert((cur_pos.0, cur_pos.1, cur_orientation));
            if !insert_status {
                return true;
            }
            cur_pos = next_pos;
        }
    }
    false
}

fn find_all_loops(day: &Day6) -> Vec<(usize, usize)> {
    let dims = day.dimensions.clone();
    let obstacles = day.obstacles.clone();
    let start_pos = day.start_pos.clone();
    let start_orientation = day.start_orientation.unwrap().clone();

    let mut test_positions: Vec<(usize, usize)> = Vec::new();

    for row in 0..dims.0 {
        for col in 0..dims.1 {
            info!("Testing {row} {col}");
            if !obstacles.contains(&(row, col)) {
                test_positions.push((row, col));
            }
        }
    }

    let loopers = test_positions
        .into_par_iter()
        .filter(|tp| find_a_loop(start_pos, *tp, dims, start_orientation, &obstacles));

    // let ans: Vec<(usize, usize)> = loopers.map(|pos| {pos}).collect();
    let ans: Vec<(usize, usize)> = loopers.collect();

    ans
}

fn main() {
    let aoc: AocHelper = AocHelper::new(6, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d6 = Day6::new(&test_input);
    d6.parse();
    debug!("{:?}", d6);
    let ans = d6.walk_the_pattern();
    info!("Got answer: {}", ans);

    let mut d6 = Day6::new(&real_input);
    d6.parse();
    debug!("{:?}", d6);
    let ans = d6.walk_the_pattern();
    info!("Got answer: {}", ans);

    let mut d6 = Day6::new(&test_input);
    d6.parse();
    debug!("{:?}", d6);
    let loops = find_all_loops(&d6);
    info!("Loops at: {:?}, total: {}", loops, loops.len());

    let mut d6 = Day6::new(&real_input);
    d6.parse();
    debug!("{:?}", d6);
    let loops = find_all_loops(&d6);
    info!("Loops at: {:?}, total: {}", loops, loops.len());
}
