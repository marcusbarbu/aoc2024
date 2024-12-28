use std::collections::{BTreeMap, HashMap, HashSet};

use aoc2024::{
    counter::BTreeCounter, map_vec_extend::append_to_mapping, AocHelper,
    RequestedAocInputType,
};
use tracing::{debug, info};

type Point = (i32, i32);
const NEIGHBOR_OFFSETS: [Point; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Debug)]
struct Day20 {
    raw: String,
    path: Vec<Point>,
    path_map: HashMap<Point, usize>,
    walls: HashSet<Point>,
    start: Point,
    end: Point,
    bounds: Point,
}

fn add_points(a: &Point, b: &Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

impl Day20 {
    pub fn new(s: &String) -> Self {
        let p = (0, 0);
        Self {
            raw: s.clone(),
            path: Vec::new(),
            walls: HashSet::new(),
            path_map: HashMap::new(),
            start: p.clone(),
            end: p.clone(),
            bounds: p.clone(),
        }
    }

    pub fn parse(&mut self) {
        let mut path_set: HashSet<Point> = HashSet::new();
        for (row, line) in self.raw.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let p: Point = (row as i32, col as i32);
                match c {
                    '#' => {
                        self.walls.insert(p);
                    }
                    '.' => {
                        path_set.insert(p);
                    }
                    'S' => {
                        self.start = p;
                        path_set.insert(p);
                    }
                    'E' => {
                        self.end = p;
                        path_set.insert(p);
                    }
                    _ => {}
                }
            }
        }
        let mr = self.raw.lines().count();
        let mc = self.raw.lines().next().unwrap().chars().count();
        self.bounds = (mr as i32, mc as i32);
        let mut cur: Point = self.start;
        while cur != self.end {
            let mut next_found = false;
            self.path.push(cur);
            self.path_map.insert(cur, self.path.len() - 1);
            let neighbors = NEIGHBOR_OFFSETS.map(|ofs| (cur.0 + ofs.0, cur.1 + ofs.1));
            for n in neighbors {
                if self.path.contains(&n) {
                    continue;
                }
                if path_set.contains(&n) {
                    cur = n;
                    next_found = true;
                    break;
                }
            }
            if !next_found {
                panic!(
                    "Couldn't find a neighbor for {:?}. Checked: {:?}",
                    cur, neighbors
                );
            }
        }
        self.path.push(cur);
        self.path_map.insert(cur, self.path.len() - 1);
    }

    fn check_len(&self, a: Point, b: Point) -> i32 {
        let a_ind = self.path_map.get(&a).unwrap();
        let b_ind = self.path_map.get(&b).unwrap();
        (*b_ind as i32) - (*a_ind as i32)
    }

    pub fn find_p1_cheats_for_point(&self, p: &Point) -> Option<HashSet<Point>> {
        let mut res = HashSet::new();
        for step_1_offset in NEIGHBOR_OFFSETS {
            for step_2_offset in NEIGHBOR_OFFSETS {
                let mut target = add_points(p, &step_1_offset);
                target = add_points(&target, &step_2_offset);
                if self.path_map.contains_key(&target) {
                    res.insert(target);
                }
            }
        }
        Some(res)
    }

    pub fn find_p1_cheats_and_scores(&self) -> BTreeCounter<i32> {
        let mut score_counter: BTreeCounter<i32> = BTreeCounter::new();
        let mut cheat_set: HashMap<(Point, Point), i32> = HashMap::new();
        for p in self.path.clone() {
            let Some(cheats) = self.find_p1_cheats_for_point(&p) else {
                continue;
            };
            for c in cheats {
                let score = self.check_len(p, c) - 2;
                if score > 0 {
                    cheat_set.insert((p, c), score);
                    score_counter.add(score);
                }
            }
        }
        info!("Cheat set: {:?}", cheat_set);
        info!("Cheat counter: {:?}", score_counter);

        score_counter
    }

    pub fn get_min_savings_count_p1(&self, min_score: i32) -> usize {
        let mut res = 0;
        let counter = self.find_p1_cheats_and_scores();
        let scores: Vec<&i32> = counter.keys().filter(|k| **k >= min_score).collect();

        for s in scores {
            let c = counter.get(s).unwrap();
            res += c;
            info!("There are {c} cheats that save {s} picoseconds");
        }

        res
    }

    pub fn find_dynamic_cheats(&self, p: &Point, steps: usize) -> HashMap<(i32, i32), usize> {
        // mapping between endpoint of cheat and cheat len (iter count)
        let mut res: BTreeMap<Point, Vec<usize>> = BTreeMap::new();
        let mut wall_pieces: HashSet<Point> = HashSet::new();
        wall_pieces.insert(p.clone());

        for step in 0..steps {
            // debug!("\t search step {step} looking at {:?} pieces", wall_pieces);
            let mut next_pieces: HashSet<Point> = HashSet::new();
            for piece in wall_pieces.iter() {
                let targets: Vec<Point> = NEIGHBOR_OFFSETS
                    .iter()
                    .map(|x| add_points(&piece, x))
                    .collect();
                for t in targets {
                    if self.path_map.contains_key(&t) {
                        append_to_mapping(&mut res, t, step);
                        next_pieces.insert(t);
                    } else if self.walls.contains(&t) {
                        next_pieces.insert(t);
                    } else {
                        // debug!("{:?} is not a wall or path", t);
                        // debug!("Walls: {:?}", self.walls);
                    }
                }
            }
            wall_pieces = next_pieces;
        }

        let mut out: HashMap<Point, usize> = HashMap::new();
        res.iter().for_each(|(point, scores)| {
            let mut ss = scores.clone();
            ss.sort();
            let score = ss.first().unwrap();
            out.insert(point.clone(), *score);
        });

        out
    }

    pub fn find_p2_cheats_and_scores(&self, steps: usize) -> BTreeCounter<i32> {
        let mut score_counter: BTreeCounter<i32> = BTreeCounter::new();
        let mut cheat_set: HashMap<(Point, Point), i32> = HashMap::new();
        for (i, p) in self.path.clone().iter().enumerate() {
            debug!("Iter: {i} looking at {:?}", p);
            let dynamic_cheats = self.find_dynamic_cheats(&p, steps);
            for (exit, s) in dynamic_cheats {
                // if exit == self.end {
                //     let score = (self.path.len() as i32) - (*self.path_map.get(&p).unwrap() as i32);
                //     debug!("End point score: {score}");
                //     cheat_set.insert((*p, exit), score);
                //     score_counter.add(score);
                // }
                if true {
                    let score = self.check_len(*p, exit) - ((s + 1) as i32);
                    if score > 0 {
                        cheat_set.insert((*p, exit), score);
                        score_counter.add(score);
                    }
                }
            }
        }
        info!("Cheat set: {:?}", cheat_set);
        info!("Cheat counter: {:?}", score_counter);

        score_counter
    }

    pub fn get_min_savings_count_p2(&self, min_score: i32, steps: usize) -> usize {
        let mut res = 0;
        let counter = self.find_p2_cheats_and_scores(steps);
        let scores: Vec<&i32> = counter.keys().filter(|k| **k >= min_score).collect();

        for s in scores {
            let c = counter.get(s).unwrap();
            res += c;
            info!("There are {c} cheats that save {s} picoseconds");
        }

        res
    }

    fn render_board(&self) {
        let mut out: String = "".to_string();
        for row in 0..self.bounds.0 {
            let mut rs: String = "".to_string();
            for col in 0..self.bounds.1 {
                let p = (row, col);
                if self.walls.contains(&p) {
                    rs.push('#');
                } else if self.path.contains(&p) {
                    rs.push('.');
                } else {
                    rs.push('?');
                }
            }
            out.push('\n');
            out.push_str(&rs);
        }
        println!("{}", out);
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(20, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    // let mut d20 = Day20::new(&test_input);
    // d20.parse();
    // debug!("{:?}", d20);
    // let total_len = d20.check_len(d20.start, d20.end);
    // info!("{} is total path len", total_len);
    // d20.find_p1_cheats_and_scores();
    // let ans = d20.get_min_savings_count_p1(23);
    // info!("Ans: {ans}");

    // let mut d20 = Day20::new(&real_input);
    // d20.parse();
    // debug!("{:?}", d20);
    // let total_len = d20.check_len(d20.start, d20.end);
    // info!("{} is total path len", total_len);
    // d20.find_p1_cheats_and_scores();
    // let ans = d20.get_min_savings_count_p1(100);
    // info!("Ans: {ans}");

    let mut d20 = Day20::new(&test_input);
    d20.parse();
    debug!("{:?}", d20);
    let total_len = d20.check_len(d20.start, d20.end);
    info!("{} is total path len", total_len);
    d20.render_board();
    let ans = d20.get_min_savings_count_p2(50, 21);
    info!("Ans: {ans}");

    let mut d20 = Day20::new(&real_input);
    d20.parse();
    debug!("{:?}", d20);
    let total_len = d20.check_len(d20.start, d20.end);
    info!("{} is total path len", total_len);
    d20.render_board();
    let ans = d20.get_min_savings_count_p2(100, 20);
    info!("Ans: {ans}");
}
