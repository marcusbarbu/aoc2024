use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
    hash::Hash,
    i32,
};

use aoc2024::{AocHelper, RequestedAocInputType};
use tracing::{debug, error, info};

const SCORE_MAX: i32 = 1_000_000_000;

#[derive(Debug, Clone, Copy, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn get_delta(&self) -> Point {
        match self {
            Direction::North => (1, 0),
            Direction::South => (-1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    pub fn get_opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match [self, other] {
            [Direction::North, Direction::North] => true,
            [Direction::South, Direction::South] => true,
            [Direction::East, Direction::East] => true,
            [Direction::West, Direction::West] => true,
            [_, _] => false,
        }
    }
}
impl Eq for Direction {}

type Point = (i32, i32);

#[derive(Debug, Clone, Copy, Hash)]
struct DirPoint((i32, i32), Direction);

impl PartialEq for DirPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for DirPoint {}

impl PartialOrd for DirPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for DirPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScoringPoint(DirPoint, i32);
impl PartialOrd for ScoringPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ScoringPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

#[derive(Debug, Clone)]
struct Graph {
    pub points: HashSet<DirPoint>,
    edges: HashMap<DirPoint, HashMap<DirPoint, i32>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            points: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_point(&mut self, dp: DirPoint) {
        self.points.insert(dp);
    }

    pub fn add_edge(&mut self, dp_start: &DirPoint, dp_end: &DirPoint, cost: i32) -> bool {
        debug!(
            "Inserting edge between {:?} and {:?} cost: {cost}",
            dp_start, dp_end
        );
        if !self.points.contains(dp_start) || !self.points.contains(dp_end) {
            error!("Couldn't insert edge b/c one point does not exist");
            return false;
        }

        let working_map: &mut HashMap<DirPoint, i32>;
        if !self.edges.contains_key(dp_start) {
            self.edges.insert(dp_start.clone(), HashMap::new());
        }
        working_map = self.edges.get_mut(&dp_start).unwrap();
        working_map.insert(dp_end.clone(), cost);

        true
    }

    pub fn get_neighbors(&self, p: &DirPoint) -> Option<Vec<(DirPoint, i32)>> {
        let Some(edges) = self.edges.get(p) else {
            return None;
        };

        let out: Vec<(DirPoint, i32)> = edges.iter().map(|(p, s)| (p.clone(), s.clone())).collect();

        Some(out)
    }
}

#[derive(Debug, Clone)]
struct Day16 {
    raw: String,
    walls: BTreeSet<Point>,
    start: Point,
    goal: Point,
    bounds: Point,
    start_dir: Direction,
    graph: Graph,
}

impl Day16 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            walls: BTreeSet::new(),
            start: (0, 0),
            goal: (0, 0),
            bounds: (0, 0),
            start_dir: Direction::East,
            graph: Graph::new(),
        }
    }

    pub fn parse(&mut self) {
        let working = self.raw.clone();
        let mut mr = 0;
        let mut mc = 0;
        for (row, line) in working.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let p: Point = (row as i32, col as i32);
                match c {
                    '#' => {
                        self.walls.insert(p);
                    }
                    'S' => {
                        self.start = p;
                    }
                    'E' => {
                        self.goal = p;
                    }
                    _ => {}
                }
                mc = col;
            }
            mr = row;
        }

        self.bounds = ((mr + 1) as i32, (mc + 1) as i32);

        for row in 0..self.bounds.0 {
            for col in 0..self.bounds.1 {
                if self.walls.contains(&(row, col)) {
                    continue;
                }
                for dir in [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ] {
                    self.graph.add_point(DirPoint((row, col), dir));
                }
            }
        }

        let dir_points = self.graph.points.clone();
        for point in dir_points {
            for dir in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                if point.1 == dir.get_opposite() {
                    continue;
                }
                let del = dir.get_delta();
                let adj: Point = (point.0 .0 + del.0, point.0 .1 + del.1);

                let ndp = DirPoint(adj, dir);
                if dir == point.1 {
                    self.graph.add_edge(&point, &ndp, 1);
                } else {
                    self.graph.add_edge(&point, &ndp, 1001);
                }
            }
        }
    }

    pub fn traverse(&self) -> i32 {
        let mut working_set: BTreeSet<DirPoint> = BTreeSet::new();
        let mut working_q: BinaryHeap<ScoringPoint> = BinaryHeap::new();
        let mut prev: BTreeMap<DirPoint, Option<DirPoint>> = BTreeMap::new();
        let mut distances: HashMap<DirPoint, i32> = HashMap::new();

        working_set = self.graph.points.iter().map(|p| p.clone()).collect();
        let start: DirPoint = DirPoint(self.start, Direction::East);
        debug!("start: {:?}", start);

        // let Some(cur) = working_q.pop() else {return;};

        for p in working_set.iter() {
            if p.0 == start.0 && p.1 == start.1 {
                debug!("Skipping {:?}", p);
                continue;
            }
            distances.insert(p.clone(), SCORE_MAX);
            prev.insert(p.clone(), None);
            working_q.push(ScoringPoint(p.clone(), SCORE_MAX));
        }
        debug!("Distances: {:?}", distances);
        distances.insert(start, 0);
        let dbstart = distances.get(&start);
        debug!("{:?}", dbstart);
        prev.insert(start, None);
        working_q.push(ScoringPoint(start, 0));

        debug!("Distances: {:?}", distances);
        while let Some(cur) = working_q.pop() {
            // debug!("Working queue state: {:?}", working_q);
            working_set.remove(&cur.0);
            debug!("Starting from {:?}", cur);
            let Some(neighbors) = self.graph.get_neighbors(&cur.0) else {
                debug!("No neighbors for {:?} ??", cur);
                continue;
            };
            debug!("Neighbors for {:?} => {:?}", cur, neighbors);
            for n in neighbors {
                debug!("\t Working on {:?}", n);
                let alt_dist_to_n = cur.1 + n.1; // is it cheaper to get to n via cur?
                let n_prev_score = distances.get(&n.0).unwrap();
                if alt_dist_to_n < *n_prev_score {
                    debug!("New score for neighbor {:?}", n);
                    distances.insert(n.0, alt_dist_to_n);
                    prev.insert(n.0, Some(cur.0));
                    let nsp = ScoringPoint(n.0, alt_dist_to_n);
                    debug!("Inserting {:?}", nsp);
                    working_q.push(nsp);
                    working_set.insert(n.0);
                } else {
                    debug!(
                        "{:?} prev score {} new score {}, no change",
                        n, n_prev_score, alt_dist_to_n
                    );
                }
            }
        }

        debug!("Done! Scores: {:?} Prev: {:?}", distances, prev);

        let variants: Vec<DirPoint> = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .iter()
        .map(|dir| DirPoint(self.goal, *dir))
        .collect();

        for var in variants.iter() {
            info!(
                "Path for {:?}: {:?} {:?}",
                var,
                distances.get(&var),
                prev.get(&var)
            );
        }

        let mut final_scores: Vec<i32> = variants
            .iter()
            .map(|var| *distances.get(var).unwrap_or(&SCORE_MAX))
            .collect();

        final_scores.sort();
        *final_scores.get(0).unwrap_or(&SCORE_MAX)
    }

    fn build_all_paths_from_prev_list(
        p: &DirPoint,
        target: &DirPoint,
        prevs: &HashMap<DirPoint, Vec<DirPoint>>,
    ) -> Vec<Vec<DirPoint>> {
        let mut all: Vec<Vec<DirPoint>> = Vec::new();

        let mut working_vec: Vec<DirPoint> = Vec::new();
        let mut cur = p.clone();
        while cur != *target {
            working_vec.push(cur);
            let Some(options) = prevs.get(&cur) else {
                return vec![working_vec];
            };

            if options.len() == 1 {
                cur = options[0].clone();
            } else {
                for o in options.iter() {
                    let mut res = Day16::build_all_paths_from_prev_list(o, target, prevs);
                    for inner_path in res.iter_mut() {
                        let mut new = working_vec.clone();
                        new.append(inner_path);
                        all.push(new);
                    }
                }
                break;
            }
        }
        all.push(working_vec);
        all
    }

    pub fn traverse_with_options(&self) -> i32 {
        let mut working_set: BTreeSet<DirPoint> = BTreeSet::new();
        let mut working_q: BinaryHeap<ScoringPoint> = BinaryHeap::new();
        let mut prev: HashMap<DirPoint, Vec<DirPoint>> = HashMap::new();
        let mut distances: HashMap<DirPoint, i32> = HashMap::new();

        working_set = self.graph.points.iter().map(|p| p.clone()).collect();
        let start: DirPoint = DirPoint(self.start, Direction::East);
        debug!("start: {:?}", start);

        // let Some(cur) = working_q.pop() else {return;};

        for p in working_set.iter() {
            if p.0 == start.0 && p.1 == start.1 {
                debug!("Skipping {:?}", p);
                continue;
            }
            distances.insert(p.clone(), SCORE_MAX);
            prev.insert(p.clone(), Vec::new());
            working_q.push(ScoringPoint(p.clone(), SCORE_MAX));
        }
        debug!("Distances: {:?}", distances);
        distances.insert(start, 0);
        let dbstart = distances.get(&start);
        debug!("{:?}", dbstart);
        prev.insert(start, Vec::new());
        working_q.push(ScoringPoint(start, 0));

        debug!("Distances: {:?}", distances);
        while let Some(cur) = working_q.pop() {
            // debug!("Working queue state: {:?}", working_q);
            working_set.remove(&cur.0);
            debug!("Starting from {:?}", cur);
            let Some(neighbors) = self.graph.get_neighbors(&cur.0) else {
                debug!("No neighbors for {:?} ??", cur);
                continue;
            };
            debug!("Neighbors for {:?} => {:?}", cur, neighbors);
            for n in neighbors {
                debug!("\t Working on {:?}", n);
                let alt_dist_to_n = cur.1 + n.1; // is it cheaper to get to n via cur?
                let n_prev_score = distances.get(&n.0).unwrap();
                if alt_dist_to_n == *n_prev_score {
                    prev.get_mut(&n.0).unwrap().push(cur.0);
                } else if alt_dist_to_n < *n_prev_score {
                    debug!("New score for neighbor {:?}", n);
                    distances.insert(n.0, alt_dist_to_n);
                    prev.insert(n.0, vec![cur.0]);
                    let nsp = ScoringPoint(n.0, alt_dist_to_n);
                    debug!("Inserting {:?}", nsp);
                    working_q.push(nsp);
                    working_set.insert(n.0);
                } else {
                    debug!(
                        "{:?} prev score {} new score {}, no change",
                        n, n_prev_score, alt_dist_to_n
                    );
                }
            }
        }

        debug!("Done! Scores: {:?} Prev: {:?}", distances, prev);

        let mut variants: Vec<DirPoint> = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .iter()
        .map(|dir| DirPoint(self.goal, *dir))
        .collect();

        for var in variants.iter() {
            info!(
                "Path for {:?}: {:?} {:?}",
                var,
                distances.get(&var),
                prev.get(&var)
            );
        }

        let final_scores: Vec<i32> = variants
            .iter()
            .map(|var| *distances.get(var).unwrap_or(&SCORE_MAX))
            .collect();

        debug!("variants pre sort {:?}", variants);
        variants.sort_by(|a, b| {
            let da = distances.get(a).unwrap_or(&SCORE_MAX);
            let db = distances.get(b).unwrap_or(&SCORE_MAX);
            da.cmp(db)
        });
        debug!("variants post sort {:?}", variants);

        let best_path_start = variants[0];
        debug!("Best path start: {:?}", best_path_start);

        let all_paths = Day16::build_all_paths_from_prev_list(&best_path_start, &start, &prev);
        for path in all_paths.iter() {
            debug!("Path: {:?}", path)
        }

        let all_points: HashSet<Point> = all_paths.iter().flatten().map(|x| x.0).collect();
        self.debug_points_on_path(&all_points);
        all_points.len() as i32
    }

    fn debug_points_on_path(&self, pset: &HashSet<Point>) {
        for row in 0..self.bounds.0 {
            let mut rv: String = String::new();
            for col in 0..self.bounds.1 {
                let p: Point = (row, col);
                if pset.contains(&p) {
                    rv.push('O');
                } else if self.walls.contains(&p) {
                    rv.push('#');
                } else {
                    rv.push('.')
                }
            }
            println!("{:?}", rv);
        }
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(16, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d16 = Day16::new(&test_input);
    d16.parse();
    let ans = d16.traverse();
    info!("Ans: {ans}");

    let mut d16 = Day16::new(&real_input);
    d16.parse();
    let ans = d16.traverse();
    info!("Ans: {ans}");

    let mut d16 = Day16::new(&test_input);
    d16.parse();
    let ans = d16.traverse_with_options();
    info!("Ans: {ans}");

    let mut d16 = Day16::new(&real_input);
    d16.parse();
    let ans = d16.traverse_with_options() + 1;
    info!("Ans: {ans}");
}
