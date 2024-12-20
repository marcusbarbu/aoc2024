use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use aoc2024::{counter::Counter, AocHelper, RequestedAocInputType};
use tracing::{debug, error, info};
use tracing_subscriber::fmt::format;

// struct TreeNode {
//     val: usize,
//     left: Option<TreeNodeRef>,
//     right: Option<TreeNodeRef>
// }

// type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, Clone)]
enum Stone {
    Single { value: usize, rep: String },
    Multi { stones: Vec<Stone> },
}

#[derive(Debug)]
struct Day11 {
    raw: String,
    // roots: Vec<TreeNodeRef>
    stones: Vec<Stone>,
}

impl Day11 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            stones: Vec::new(),
        }
    }
    pub fn parse(&mut self) {
        self.raw
            .split_ascii_whitespace()
            .into_iter()
            .for_each(|num| {
                let val = num.parse::<usize>().unwrap();
                // let root = Day11::build_root_node_ref(val);
                // self.roots.push(root)
                let stone = Stone::Single {
                    value: val,
                    rep: val.to_string(),
                };
                self.stones.push(stone);
            });
    }

    fn split_number(n: &str) -> ((usize, usize), (String, String)) {
        let mp = n.len() / 2;
        let (l, r) = n.split_at(mp);
        let ln = l.parse::<usize>().unwrap();
        let rn = r.parse::<usize>().unwrap();
        let ls: String = ln.to_string();
        let rs: String = rn.to_string();
        ((ln, rn), (ls, rs))
    }

    pub fn single_step(&mut self) {
        for stone_section in self.stones.iter_mut() {
            match stone_section {
                Stone::Single { value, rep } => {
                    if *value == 0 {
                        *value = 1;
                        *rep = String::from("1");
                    } else if rep.len() % 2 == 0 {
                        let (new_nums, new_reps) = Day11::split_number(rep);
                        let left = Stone::Single {
                            value: new_nums.0,
                            rep: new_reps.0.to_string(),
                        };
                        let right = Stone::Single {
                            value: new_nums.1,
                            rep: new_reps.1.to_string(),
                        };
                        *stone_section = Stone::Multi {
                            stones: vec![left, right],
                        }
                    } else {
                        let new_num = *value * 2024;
                        let new_rep = new_num.to_string();
                        *value = new_num;
                        *rep = new_rep;
                    }
                }
                Stone::Multi { stones } => {
                    error!("Should've been flattened somehow?")
                }
            }
        }

        let mut new_stones: Vec<Stone> = Vec::new();
        for section in self.stones.iter() {
            match section {
                Stone::Single { value, rep } => {
                    new_stones.push(section.clone());
                }
                Stone::Multi { stones } => {
                    for ele in stones {
                        new_stones.push(ele.clone());
                    }
                }
            }
        }

        self.stones = new_stones;
    }

    pub fn print_stones(&self) -> String {
        let stone_string: String = self
            .stones
            .iter()
            .map(|stone| match stone {
                Stone::Single { value, rep } => return format!("{} ", rep),
                Stone::Multi { stones } => todo!(),
            })
            .collect();
        return stone_string;
    }

    pub fn multi_blink(&mut self, steps: usize) -> usize {
        for i in 1..steps + 1 {
            self.single_step();
            // let stone_string = self.print_stones();
            // debug!("After {} Total: {} steps: {}", i, self.stones.len(), stone_string);
            debug!("After {} Total: {}", i, self.stones.len());
        }

        self.stones.len()
    }
}

#[derive(Debug)]
struct D11Part2 {
    raw: String,
    stones: Counter<usize>,
}
impl D11Part2 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            stones: Counter::new(),
        }
    }

    pub fn parse(&mut self) {
        self.raw
            .split_ascii_whitespace()
            .into_iter()
            .for_each(|num| {
                let val = num.parse::<usize>().unwrap();
                self.stones.add(val)
            });
    }

    fn split_number(n: &str) -> ((usize, usize), (String, String)) {
        let mp = n.len() / 2;
        let (l, r) = n.split_at(mp);
        let ln = l.parse::<usize>().unwrap();
        let rn = r.parse::<usize>().unwrap();
        let ls: String = ln.to_string();
        let rs: String = rn.to_string();
        ((ln, rn), (ls, rs))
    }

    pub fn single_step(&mut self) {
        let mut new_counter: Counter<usize> = Counter::new();
        self.stones.iter().for_each(|(stone_number, count)| {
            let rep = stone_number.to_string();
            let mut new_number;
            if *stone_number == 0 {
                new_number = 1;
                new_counter.add_n(new_number, *count);
            } else if rep.len() % 2 == 0 {
                let (new_nums, new_reps) = D11Part2::split_number(&rep);
                new_counter.add_n(new_nums.0, *count);
                new_counter.add_n(new_nums.1, *count);
            } else {
                new_number = *stone_number * 2024;
                new_counter.add_n(new_number, *count);
            }
        });
        self.stones = new_counter;
    }

    fn stone_length(&self) -> usize {
        self.stones.iter().fold(0, |acc, (k, v)| acc + *v)
    }

    pub fn multi_blink(&mut self, steps: usize) -> usize {
        for i in 1..steps + 1 {
            self.single_step();
            // let stone_string = self.print_stones();
            // debug!("After {} Total: {} steps: {}", i, self.stones.len(), stone_string);
            debug!("After {} Total: {}", i, self.stone_length());
        }
        self.stone_length()
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(11, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d11 = Day11::new(&test_input);
    d11.parse();
    debug!("{:?}", d11);
    d11.print_stones();
    let ans = d11.multi_blink(25);
    // info!("{}", d11.print_stones());
    info!("Answer: {ans}");

    // let mut d11 = Day11::new(&real_input);
    // d11.parse();
    // debug!("{:?}", d11);
    // d11.print_stones();
    // let ans = d11.multi_blink(25);
    // // info!("{}", d11.print_stones());
    // info!("Answer: {ans}");

    let mut d11 = D11Part2::new(&test_input);
    d11.parse();
    debug!("{:?}", d11);
    // d11.single_step();
    // debug!("{:?}", d11);
    let ans = d11.multi_blink(75);
    info!("Answer: {ans}");

    let mut d11 = D11Part2::new(&real_input);
    d11.parse();
    debug!("{:?}", d11);
    // d11.single_step();
    // debug!("{:?}", d11);
    let ans = d11.multi_blink(75);
    info!("Answer: {ans}")
}
