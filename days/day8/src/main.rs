use std::collections::{BTreeMap, BTreeSet};

use aoc2024::{AocHelper, RequestedAocInputType};
use tracing::{debug, info};

#[derive(Debug)]
struct Day8 {
    raw: String,
    antenna_locations: BTreeMap<char, Vec<(i32, i32)>>,
    num_rows: usize,
    num_cols: usize,
    _antinodes: BTreeSet<Point>,
    freqs: BTreeMap<char, Frequency>,
}

type Point = (i32, i32);
type PointPair = ((i32, i32), (i32, i32));

#[derive(Debug)]
struct Frequency {
    _name: char,
    arms: Vec<FrequencyArms>,
}

#[derive(Debug)]
struct FrequencyArms {
    root: Point,
    first_point: Point,
    potential_points: Vec<Point>,
    _antinode_points: Vec<Point>,
}

impl Day8 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            antenna_locations: BTreeMap::new(),
            num_cols: 0,
            num_rows: 0,
            _antinodes: BTreeSet::new(),
            freqs: BTreeMap::new(),
        }
    }

    // TODO: put this in the library
    fn append_to_mapping(btm: &mut BTreeMap<char, Vec<(i32, i32)>>, k: char, v: (i32, i32)) {
        match btm.get_mut(&k) {
            Some(vv) => {
                vv.push(v);
                vv.sort()
            }
            None => {
                let mut new_vec = Vec::new();
                new_vec.push(v);
                btm.insert(k, new_vec);
            }
        }
    }

    pub fn parse(&mut self) {
        let mut mr = 0;
        let mut mc = 0;
        for (row, line) in self.raw.lines().into_iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c.is_ascii_alphanumeric() {
                    Day8::append_to_mapping(
                        &mut self.antenna_locations,
                        c,
                        (row as i32, col as i32),
                    );
                }
                mc = col;
            }
            mr = row;
        }
        self.num_cols = mc + 1;
        self.num_rows = mr + 1;
    }

    fn generate_pairs(v: &Vec<(i32, i32)>) -> Vec<((i32, i32), (i32, i32))> {
        let mut out = Vec::new();
        for i in 0..v.len() {
            let first = v[i];
            for last in i + 1..v.len() {
                let second = v[last];
                let pair = (first, second);
                out.push(pair);
            }
        }

        out
    }

    fn distance(a: (i32, i32), b: (i32, i32)) -> f64 {
        let rowdiff: i32 = (b.0 as i32) - (a.0 as i32);
        let coldiff = (b.1 as i32) - (a.1 as i32);

        let diffsq: f64 = (rowdiff.pow(2) + coldiff.pow(2)) as f64;
        diffsq.sqrt()
    }

    fn possible_points_on_line(num_rows: usize, num_cols: usize, p: PointPair) -> Vec<(i32, i32)> {
        let root = p.0;
        let first_point = p.1;

        let rowdiff: i32 = (first_point.0 as i32) - (root.0 as i32);
        let coldiff = (first_point.1 as i32) - (root.1 as i32);

        // go in one step direction, then flip both signs and go the other way

        let in_range = |point: (i32, i32)| {
            point.0 < num_rows as i32 && point.0 >= 0 && point.1 < num_cols as i32 && point.1 >= 0
        };

        let mut options: Vec<(i32, i32)> = Vec::new();
        let mut cur: (i32, i32) = (first_point.0 as i32, first_point.1 as i32);
        cur.0 += rowdiff;
        cur.1 += coldiff;
        while in_range(cur) {
            options.push(cur);
            cur.0 += rowdiff;
            cur.1 += coldiff;
        }
        let mut cur: (i32, i32) = (root.0 as i32, root.1 as i32);
        cur.0 -= rowdiff;
        cur.1 -= coldiff;
        while in_range(cur) {
            options.push(cur);
            cur.0 -= rowdiff;
            cur.1 -= coldiff;
        }
        options.sort();
        options
    }

    pub fn build_frequencies(&mut self) {
        self.antenna_locations.iter().for_each(|(signal, loc_vec)| {
            let pairs = Day8::generate_pairs(loc_vec);
            let mut freq: Frequency = Frequency {
                _name: *signal,
                arms: Vec::new(),
            };
            debug!("Pairs for {signal}: {:?}", pairs);

            for ele in pairs.iter() {
                let points = Day8::possible_points_on_line(self.num_rows, self.num_cols, *ele);
                let fa: FrequencyArms = FrequencyArms {
                    root: ele.0,
                    first_point: ele.1,
                    potential_points: points,
                    _antinode_points: Vec::new(),
                };
                freq.arms.push(fa);
            }
            self.freqs.insert(*signal, freq);
        });
    }

    pub fn calculate_antinodes(&mut self) -> BTreeSet<Point> {
        let mut universal_antinodes: BTreeSet<Point> = BTreeSet::new();
        self.freqs.iter().for_each(|(_key, freq)| {
            for arm in freq.arms.iter() {
                // let arm_antinodes: Vec<(i32,i32)> =
                arm.potential_points
                    .iter()
                    .filter_map(|point| {
                        let root_dist = Day8::distance(*point, arm.root);
                        let fp_dist = Day8::distance(*point, arm.first_point);
                        let greatest = root_dist.max(fp_dist);
                        let least = root_dist.min(fp_dist);
                        let good = greatest == (least * 2.0);
                        debug!("Greatest {greatest} least {least}: {good}");
                        if good {
                            return Some((point.0, point.1));
                        }
                        None
                    })
                    .for_each(|p| {
                        universal_antinodes.insert(p);
                    });
                // debug!("Arm antinodes: {:?}", arm_antinodes);
            }
        });
        universal_antinodes
    }

    pub fn get_total_line_points(&self) -> BTreeSet<Point> {
        let mut out: BTreeSet<Point> = BTreeSet::new();

        self.freqs.iter().for_each(|(_, freq)| {
            freq.arms.iter().for_each(|arm| {
                arm.potential_points.iter().for_each(|point| {
                    out.insert((point.0, point.1));
                });
                out.insert(arm.root);
                out.insert(arm.first_point);
            });
        });

        out
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(8, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d8: Day8 = Day8::new(&test_input);
    d8.parse();
    debug!("d8: {:?}", d8);
    d8.build_frequencies();
    debug!("{:?}", d8);
    let antinodes = d8.calculate_antinodes();
    let all_lines = d8.get_total_line_points();
    info!("All antinodes: {:?} len: {}", antinodes, antinodes.len());
    info!("All lines? {:?} len: {}", all_lines, all_lines.len());

    let mut d8: Day8 = Day8::new(&real_input);
    d8.parse();
    debug!("d8: {:?}", d8);
    d8.build_frequencies();
    debug!("{:?}", d8);
    let antinodes = d8.calculate_antinodes();
    let all_lines = d8.get_total_line_points();
    info!("All antinodes: {:?} len: {}", antinodes, antinodes.len());
    info!("All lines? {:?} len: {}", all_lines, all_lines.len());
}
