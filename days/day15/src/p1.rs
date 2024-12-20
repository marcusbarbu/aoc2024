use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Add,
};

use aoc2024::{AocHelper, RequestedAocInputType};
use tracing::{debug, info};

// type Point = (i32, i32);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point(i32, i32);
type BoundedSet = BTreeMap<usize, Vec<Point>>;

#[derive(Debug, Clone)]
struct Day15 {
    raw: String,
    box_starts: BTreeSet<Point>,
    boxes: BTreeSet<Point>,
    robot_start: Point,
    robot_loc: Point,
    bounds: Point,
    walls: BTreeSet<Point>,
    actions: Vec<MoveDir>,
}

#[derive(Debug, Clone)]
enum MoveDir {
    Up,
    Down,
    Left,
    Right,
}

impl MoveDir {
    pub fn get_delta(&self) -> Point {
        match self {
            MoveDir::Up => Point(-1, 0),
            MoveDir::Down => Point(1, 0),
            MoveDir::Left => Point(0, -1),
            MoveDir::Right => Point(0, 1),
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Day15 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            box_starts: BTreeSet::new(),
            boxes: BTreeSet::new(),
            robot_start: Point(0, 0),
            robot_loc: Point(0, 0),
            bounds: Point(0, 0),
            walls: BTreeSet::new(),
            actions: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        let mut sp = self.raw.split("\n\n");
        let mut mr: usize = 0;
        let mut mc: usize = 0;
        for (row, line) in sp.next().unwrap().lines().enumerate() {
            debug!("Input line {}", line);
            if line.len() == 0 {
                break;
            }
            for (col, char) in line.chars().enumerate() {
                let point = Point(row as i32, col as i32);
                match char {
                    '#' => {
                        self.walls.insert(point);
                    }
                    '@' => {
                        self.robot_start = point.clone();
                        self.robot_loc = point.clone();
                    }
                    'O' => {
                        self.box_starts.insert(point.clone());
                        self.boxes.insert(point);
                    }
                    _ => {}
                }
                mc = col;
            }
            mr = row;
        }
        mc += 1;
        mr += 1;
        self.bounds = Point(mr as i32, mc as i32);

        for dir in sp.next().unwrap().chars() {
            match dir {
                '^' => self.actions.push(MoveDir::Up),
                '<' => self.actions.push(MoveDir::Left),
                '>' => self.actions.push(MoveDir::Right),
                'v' => self.actions.push(MoveDir::Down),
                _ => {}
            }
        }
    }

    fn render(&self) {
        let mut mat: Vec<String> = Vec::new();
        for r in 0..self.bounds.0 {
            let mut row: String = String::from("");
            for c in 0..self.bounds.1 {
                let mut cout = '.';
                if self.robot_loc == Point(r, c) {
                    cout = '@';
                } else if self.walls.contains(&Point(r, c)) {
                    cout = '#';
                } else if self.boxes.contains(&Point(r, c)) {
                    cout = 'O';
                }
                row.push(cout);
            }
            mat.push(row);
        }
        let out = mat.join("\n");
        println!("{}", out);
    }

    pub fn part1_walk(&mut self) {
        for act in &self.actions {
            let delta = act.get_delta();
            info!(
                "Robot starting at {:?} with move {:?}-{:?}",
                self.robot_loc, act, delta
            );
            let mut boi: Vec<Point> = Vec::new();
            let mut target = self.robot_loc.clone() + delta.clone();
            while self.boxes.contains(&target) {
                boi.push(target.clone());
                target = target + delta.clone();
            }

            if self.walls.contains(&target) {
                debug!(
                    "Hit a wall at Target:{:?} with boxes {:?}, done for this move",
                    target, boi
                );
                continue;
            } else {
                debug!("No wall, shifting boxes and moving robot");
                if boi.len() > 0 {
                    let first = boi[0].clone();
                    self.boxes.remove(&first);
                    self.boxes.insert(target);
                }
                self.robot_loc = self.robot_loc.clone() + delta;
            }
            self.render();
        }
    }

    pub fn part1_get_score(&self) -> usize {
        self.boxes.iter().fold(0, |acc, bx| {
            let row: usize = bx.0 as usize;
            let col: usize = bx.1 as usize;
            acc + (100 * row + col)
        })
    }
}

pub fn p1() {
    let aoc: AocHelper = AocHelper::new(15, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d15 = Day15::new(&test_input);
    d15.parse();
    debug!("{:?}", d15);
    d15.part1_walk();
    debug!("{:?}", d15);
    let ans = d15.part1_get_score();
    info!("Score: {ans}");

    let mut d15 = Day15::new(&real_input);
    d15.parse();
    debug!("{:?}", d15);
    d15.part1_walk();
    debug!("{:?}", d15);
    let ans = d15.part1_get_score();
    info!("Score: {ans}");
}
