use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    fs::{File, OpenOptions},
    io::Write,
    ops::Add,
    rc::Rc,
};

use aoc2024::{AocHelper, RequestedAocInputType};
use tracing::{debug, error, info};

// type Point = (i32, i32);
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point(i32, i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BoxPiece(Point, char);

#[derive(Debug, Clone)]
struct Day15 {
    raw: String,
    boxes: BTreeSet<BoxPiece>,
    robot_loc: Point,
    bounds: Point,
    walls: BTreeSet<Point>,
    actions: Vec<MoveDir>,
    render_file: Rc<RefCell<File>>,
}

enum Contents {
    Robot,
    Wall,
    LeftBox,
    RightBox,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum MoveDir {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl MoveDir {
    pub fn get_delta(&self) -> Point {
        match self {
            MoveDir::Up => Point(-1, 0),
            MoveDir::Down => Point(1, 0),
            MoveDir::Left => Point(0, -1),
            MoveDir::Right => Point(0, 1),
            MoveDir::None => Point(0, 0),
        }
    }
}

impl Display for MoveDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = match self {
            MoveDir::Up => '^',
            MoveDir::Down => 'v',
            MoveDir::Left => '<',
            MoveDir::Right => '>',
            MoveDir::None => '?',
        };
        write!(f, "{}", c)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add for BoxPiece {
    type Output = BoxPiece;

    fn add(self, rhs: Self) -> Self::Output {
        let p = self.0 + rhs.0;
        BoxPiece(p, self.1)
    }
}

impl Add<Point> for BoxPiece {
    type Output = BoxPiece;

    fn add(self, rhs: Point) -> Self::Output {
        let new_point = self.0 + rhs;
        BoxPiece(new_point, self.1)
    }
}

impl Add<char> for BoxPiece {
    type Output = BoxPiece;

    fn add(self, rhs: char) -> Self::Output {
        BoxPiece(self.0, rhs)
    }
}

impl Day15 {
    pub fn new(s: &String, out: &String) -> Self {
        let file: File = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("day15b.txt")
            .unwrap();

        let frc = Rc::new(RefCell::new(file));

        Self {
            raw: s.clone(),
            boxes: BTreeSet::new(),
            robot_loc: Point(0, 0),
            bounds: Point(0, 0),
            walls: BTreeSet::new(),
            actions: Vec::new(),
            render_file: frc,
        }
    }

    pub fn parse(&mut self) {
        let mut expanded = self.raw.clone();
        expanded = expanded.replace("#", "##");
        expanded = expanded.replace(".", "..");
        expanded = expanded.replace("O", "[]");
        expanded = expanded.replace("@", "@.");
        let mut sp = expanded.split("\n\n");
        info!("Before:");
        println!("{}", self.raw);
        info!("After expansion:");
        println!("{}", expanded);
        let mut mr: usize = 0;
        let mut mc: usize = 0;
        for (row, line) in sp.next().unwrap().lines().enumerate() {
            debug!("Input line {}", line);
            if line.len() == 0 {
                break;
            }
            let vc: Vec<char> = line.chars().collect();
            for (col, char) in vc.iter().enumerate() {
                // let col: i32 = col_offset as i32 * 2;
                let point = Point(row as i32, col as i32);
                match char {
                    '#' => {
                        self.walls.insert(point.clone());
                    }
                    '@' => {
                        self.robot_loc = point.clone();
                    }
                    '[' => {
                        self.boxes.insert(BoxPiece(point.clone(), 'l'));
                    }
                    ']' => {
                        self.boxes.insert(BoxPiece(point.clone(), 'r'));
                    }
                    _ => {}
                }
                mc = col as usize;
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

    fn render(&mut self, last_move: &MoveDir, idx: usize) {
        let mut mat: Vec<String> = Vec::new();
        let f_out = &mut self.render_file;
        mat.push(format!("Step {} Direction: {}", idx, last_move));
        for r in 0..self.bounds.0 {
            let mut row: String = String::from("");
            for c in 0..self.bounds.1 {
                let mut cout = '.';
                let this_point = Point(r, c);
                let lcont = self.boxes.contains(&BoxPiece(this_point.clone(), 'l'));
                let rcont = self.boxes.contains(&BoxPiece(this_point.clone(), 'r'));

                if lcont && rcont {
                    error!("Left and right both present!!!");
                }

                if self.robot_loc == this_point {
                    cout = '@';
                } else if self.walls.contains(&this_point) {
                    cout = '#';
                } else if lcont {
                    cout = '[';
                } else if rcont {
                    cout = ']';
                }
                row.push(cout);
            }
            mat.push(row);
        }
        let out = mat.join("\n");
        // println!("{}", out.clone());
        write!(f_out.borrow_mut(), "{}", out + "\n");
    }

    fn get_targets(&self, start: BoxPiece, dir: &MoveDir) -> Option<BTreeSet<BoxPiece>> {
        debug!("Getting targets starting @ {:?} going {:?}", start, dir);
        let mut out: BTreeSet<BoxPiece> = BTreeSet::new();
        out.insert(start.clone());
        let orig_dir = start.1;
        let delta = dir.get_delta();

        let one_step = |start: &Point, delta: &Point| {
            let ntarget: Point = *start + *delta;
            let ltg = BoxPiece(ntarget.clone(), 'l');
            let rtg = BoxPiece(ntarget.clone(), 'r');
            debug!(
                "One step from {:?} in the {:?} dir is {:?} ({:?}, {:?})",
                start, delta, ntarget, ltg, rtg
            );

            return (ntarget, ltg, rtg);
        };

        let (mut imm_target, mut ltarget, mut rtarget) = one_step(&start.0, &delta);
        let mut lcont = self.boxes.contains(&ltarget);
        let mut rcont = self.boxes.contains(&rtarget);

        if self.walls.contains(&imm_target) {
            // this cloud will hit a wall, no dice
            return None;
        }

        if !(lcont || rcont) {
            // no box, we're just about done here
            return Some(out);
        }

        if *dir == MoveDir::Left || *dir == MoveDir::Right {
            while lcont || rcont {
                debug!(
                    "In l/r while: target: {:?} {:?} {:?} lcont: {} rcont: {}",
                    imm_target, ltarget, rtarget, lcont, rcont
                );
                if lcont {
                    error!("Adding {:?}", ltarget);
                    out.insert(ltarget);
                } else if rcont {
                    out.insert(rtarget);
                    error!("Adding {:?}", rtarget);
                }

                (imm_target, ltarget, rtarget) = one_step(&imm_target, &delta);
                lcont = self.boxes.contains(&ltarget);
                rcont = self.boxes.contains(&rtarget);
            }
            if self.walls.contains(&imm_target) {
                // after gathering our lefts and rights, we hit a wall in the end so we're dead
                return None;
            } else {
                // otherwise, here's our stack of walls we must move
                return Some(out);
            }
        } else {
            // if there's a left box above/below me, look at the right box as well
            let lb_targets;
            let rb_targets;
            let imm_bp = BoxPiece(imm_target, orig_dir);
            if lcont {
                lb_targets = self.get_targets(imm_bp + 'l', dir);
                rb_targets = self.get_targets(imm_bp + 'r' + Point(0, 1), dir);
            } else if rcont {
                rb_targets = self.get_targets(imm_bp + 'r', dir);
                lb_targets = self.get_targets(imm_bp + 'l' + Point(0, -1), dir);
            } else {
                return Some(out);
            }
            if lb_targets.is_none() || rb_targets.is_none() {
                return None;
            } else {
                let mut lb_out = lb_targets.unwrap();
                let mut rb_out = rb_targets.unwrap();
                out.append(&mut lb_out);
                out.append(&mut rb_out);
                return Some(out);
            }
        }
    }

    pub fn walk(&mut self) {
        let actions = self.actions.clone();
        for (idx, act) in actions.iter().enumerate() {
            let delta = act.get_delta();
            info!(
                "Robot starting at {:?} with move {:?}-{:?}",
                self.robot_loc, act, delta
            );
            let robot_bp = BoxPiece(self.robot_loc, 's');
            let targets = self.get_targets(robot_bp, act);
            debug!("targets: {:?}", targets);
            if targets.is_none() {
                info!("No move, skipping");
                continue;
            } else {
                let mut all_news: Vec<BoxPiece> = Vec::new();
                for bp in targets.unwrap() {
                    // remove the original piece from the box set
                    // generate the new point
                    // add that point to the box set
                    // scoot the robot

                    let new_box = bp + delta;
                    debug!("Got target {:?} becomes piece {:?}", bp, new_box);
                    let removed = self.boxes.remove(&bp);
                    debug!("Removed: {removed}");
                    if new_box.1 == 's' {
                        self.robot_loc = new_box.0;
                    } else {
                        all_news.push(new_box);
                    }
                }
                for bp in all_news {
                    self.boxes.insert(bp);
                }
            }
            self.render(act, idx);
        }
    }

    pub fn get_score(&self) -> i32 {
        self.boxes.iter().fold(0, |acc, b| {
            if b.1 != 'l' {
                return acc;
            }

            acc + (100 * b.0 .0) + b.0 .1
        })
    }
}

pub fn p2() {
    let aoc: AocHelper = AocHelper::new(15, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d15 = Day15::new(&test_input, &"day15b.txt".to_string());
    d15.parse();
    debug!("{:?}", d15);
    d15.render(&MoveDir::None, 0);
    d15.walk();
    let ans = d15.get_score();
    info!("Score: {ans}");

    let mut d15 = Day15::new(&real_input, &"day15b.txt".to_string());
    d15.parse();
    debug!("{:?}", d15);
    d15.render(&MoveDir::None, 0);
    d15.walk();
    let ans = d15.get_score();
    info!("Score: {ans}");
}
