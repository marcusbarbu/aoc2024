use std::{
    collections::{HashMap, HashSet},
    path,
};

use aoc2024::{
    map_vec_extend::{append_to_hash_map, append_to_hash_set},
    AocHelper, RequestedAocInputType,
};
use dotenvy::from_path_iter;
use itertools::{repeat_n, Itertools};
use rayon::prelude::*;
use tracing::{debug, info};

const NUMBER_PAD: &str = "789
456
123
X0A";

const DIR_PAD: &str = "X^A
<v>";

type Point = (i32, i32);

fn sum_points(a: &Point, b: &Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

const ILLEGALS: [&str; 12] = [
    "<>", "><", "^v", "v^", "<v<", ">v>", ">^>", "<^<", "^<^", "v<v", "^>^", "v<v",
];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
    Press,
}

const WORKING_DIRS: [Direction; 5] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
    Direction::None,
];

impl Direction {
    pub fn get_offset(&self) -> Point {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::None => (0, 0),
            Direction::Press => (0, 0),
        }
    }

    pub fn get_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::None => '.',
            Direction::Press => '.',
        }
    }
}

const ALL_DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug)]
struct Keypad {
    raw: String,
    keys: Vec<(char, Point)>,
    p_to_k: HashMap<Point, char>,
    k_to_p: HashMap<char, Point>,
    bounds: Point,
    paths: HashMap<(char, char), HashSet<Vec<Direction>>>,
    point_set: HashSet<Point>,
    offlimits: Point,
    string_paths: HashMap<(char, char), Vec<String>>,
}

impl Keypad {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            keys: Vec::new(),
            bounds: (0, 0),
            paths: HashMap::new(),
            p_to_k: HashMap::new(),
            k_to_p: HashMap::new(),
            point_set: HashSet::new(),
            offlimits: (0, 0),
            string_paths: HashMap::new(),
        }
    }

    pub fn parse(&mut self) {
        for (row, line) in self.raw.lines().into_iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                let p: Point = (row as i32, col as i32);
                self.keys.push((char, p));
                self.p_to_k.insert(p, char);
                self.k_to_p.insert(char, p);
                self.point_set.insert(p);
                if char == 'X' {
                    self.offlimits = p;
                }
            }
        }
        let mr = self.raw.lines().count();
        let mc = self.raw.lines().next().unwrap().chars().count();
        self.bounds = (mr as i32, mc as i32);
    }

    fn eval_path(
        start: &Point,
        path: &Vec<&Direction>,
        valids: &HashSet<Point>,
        p_to_k: &HashMap<Point, char>,
    ) -> Option<(char, usize)> {
        let mut cur = start.clone();
        let mut len = valids.len();
        for (i, step) in path.iter().enumerate() {
            if !valids.contains(&cur) {
                return None;
            }
            if **step == Direction::None {
                len = i;
                break;
            }
            let offset = step.get_offset();
            cur = sum_points(&cur, &offset);
        }

        if !valids.contains(&cur) {
            return None;
        }
        let finish = p_to_k.get(&cur).unwrap();
        Some((*finish, len))
    }

    pub fn find_all(&mut self, max: usize) {
        let all = repeat_n(WORKING_DIRS.iter(), max).multi_cartesian_product();
        // let all = WORKING_DIRS.iter()
        // let avec = all.collect_vec();
        // debug!("ALL: {:?}", avec);

        let mut valid_set = self.point_set.clone();
        valid_set.remove(&self.offlimits);
        let ptk = self.p_to_k.clone();

        for start in self.point_set.iter() {
            if start == &self.offlimits {
                continue;
            }

            let start_char = self.p_to_k.get(start).unwrap();

            let working_all = all.clone();
            // let working_all = WORKING_DIRS.iter().combinations_with_replacement(max);
            for path in working_all {
                let Some(res) = Keypad::eval_path(start, &path, &valid_set, &ptk) else {
                    continue;
                };
                // path.clone().iter().filter_map(|val| {

                // });
                // let pp = path.clone();
                let mut pp: Vec<Direction> = path.clone().iter().map(|x| (**x).clone()).collect();
                if let Some(first_none) = pp
                    .iter()
                    .enumerate()
                    .find(|(ind, dir)| dir == &&Direction::None)
                {
                    let _ = pp.split_off(first_none.0);
                }

                if pp.len() > 0 {
                    append_to_hash_set(&mut self.paths, (*start_char, res.0), pp);
                }
            }
        }
    }

    pub fn get_paths_internal(&self, a: char, b: char) -> Vec<String> {
        debug!("looking for {a} {b}");
        let options = self.paths.get(&(a, b)).unwrap();
        let mut oo = options.clone();
        // let mut oo: Vec<Vec<Direction>> =
        // oo.sort_by(|a, b| {
        //     a.len().cmp(&b.len())
        // });
        // oo.dedup();
        // oo.dedup();
        let mut out: Vec<String> = Vec::new();
        for path in oo {
            let mut s: String = path.iter().map(|d| d.get_char()).collect();
            if ILLEGALS.iter().any(|ill| s.contains(*ill)) {
                continue;
            }
            s.push('A');
            out.push(s);
        }
        out.sort();
        out.dedup();
        out.sort();
        out.reverse();
        out
    }

    pub fn make_string_paths(&mut self) {
        let keys: Vec<char> = self.k_to_p.keys().cloned().collect();
        let all = repeat_n(keys.iter(), 2).multi_cartesian_product();
        for kpair in all {
            let path_key = (*kpair[0], *kpair[1]);
            if path_key.0 == 'X' || path_key.1 == 'X' {
                continue;
            }
            if path_key.0 == path_key.1 {
                self.string_paths.insert(path_key, vec!["".to_string()]);
                continue;
            }
            let res = self.get_paths_internal(path_key.0, path_key.1);
            self.string_paths.insert(path_key, res);
        }
    }

    pub fn get_string_path(&self, a: char, b: char) -> &Vec<String> {
        self.string_paths.get(&(a, b)).unwrap()
    }
}

#[derive(Debug)]
struct Day21 {
    raw: String,
    targets: Vec<String>,
    numpad: Keypad,
    dirpad: Keypad,
}

impl Day21 {
    pub fn new(s: &String) -> Self {
        let mut np = Keypad::new(&NUMBER_PAD.to_string());
        np.parse();
        np.find_all(6);
        debug!("{:?}", np.paths.keys());
        np.make_string_paths();

        let mut dp = Keypad::new(&DIR_PAD.to_string());
        dp.parse();
        dp.find_all(4);
        dp.make_string_paths();
        Self {
            raw: s.clone(),
            targets: Vec::new(),
            numpad: np,
            dirpad: dp,
        }
    }

    pub fn parse(&mut self) {
        self.raw
            .lines()
            .for_each(|l| self.targets.push(l.to_string()));
    }

    // return the shortest possible sequence for this path?
    pub fn test_keypad_path(
        &self,
        path: &String,
        pad_to_use: &Keypad,
        level: usize,
        tlc: &mut HashMap<(String, usize), usize>,
    ) -> usize {
        // let tabin = "\t".to_string().repeat(level);
        // debug!("{}Looking for path {path} at depth {level}", tabin);
        if level == 0 {
            // debug!("{}Path {path} has len {}", tabin, path.len());
            return path.len();
        }
        let path_string = path.to_string();
        if tlc.contains_key(&(path_string.clone(), level)) {
            return *tlc.get(&(path_string, level)).unwrap();
        }
        let with_start = "A".to_string() + path;
        let steps = with_start.chars().tuple_windows::<(char, char)>();
        let mut set_of_bests: Vec<String> = Vec::new();
        let mut option_sets: Vec<HashSet<String>> = Vec::new();
        let mut total = 0;
        for step in steps {
            // let step_option_set =
            if step.0 == step.1 {
                total += 1;
                continue;
            }
            let path_options = pad_to_use.get_string_path(step.0, step.1);
            // debug!("{} Path: {} All options for {} to {}: {:?}", tabin, with_start, step.0, step.1, path_options);
            let mut scores: Vec<usize> = Vec::new();
            for option in path_options.iter() {
                // let (best_len, best_path) = self.test_keypad_path(option, &self.dirpad, level - 1);
                let best_len = self.test_keypad_path(option, &self.dirpad, level - 1, tlc);
                // if best_len < min_len {
                //     min_len = best_len;
                //     min_path = best_path;
                // }
                scores.push(best_len);
            }
            // debug!("{}  Decided on {min_path}", tabin);
            // set_of_bests.push(min_path);
            scores.sort();
            total += scores[0];
        }
        // let best_of_bests = set_of_bests.concat();
        // debug!("{}Best path for {path} is {}", tabin, best_of_bests);
        tlc.insert((path_string, level), total);
        return total;
    }

    pub fn get_answer(&self, limit: usize) -> usize {
        let res = self
            .targets
            .par_iter()
            .fold(
                || 0,
                |acc, t| {
                    let mut tlc = HashMap::new();
                    let res = self.test_keypad_path(t, &self.numpad, limit, &mut tlc);
                    let num = t.split_once('A').unwrap().0.parse::<usize>().unwrap();
                    let score = res * num;
                    info!("Res for {t} is {res} numeric is {num} = {score}");
                    acc + score
                },
            )
            .sum::<usize>();

        res
    }
}

/*

Find the shortest paths on the number pad, represented as a vector of directions
- Precalculate paths between each point on the number and directional pads
- Each step on each pad always start at A
- if paths are precalculated, should be able to do this without a ton of back and forth
- each number is independent!!!!


- At each level, find the shortest paths on the dir pad, again as a vector of directions
- Repeat until at top most level, pruning out longer paths potentially?


*/

fn main() {
    let aoc: AocHelper = AocHelper::new(21, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d21 = Day21::new(&test_input);
    d21.parse();
    let ans = d21.get_answer(3);
    info!("Ans: {ans}");

    let mut d21 = Day21::new(&real_input);
    d21.parse();
    let ans = d21.get_answer(3);
    info!("Ans: {ans}");

    let mut d21 = Day21::new(&real_input);
    d21.parse();
    let ans = d21.get_answer(26);
    info!("Ans: {ans}");
}
