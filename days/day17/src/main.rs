use std::collections::HashSet;

use aoc2024::{AocHelper, RequestedAocInputType};
use rayon::prelude::*;
use tracing::{debug, error, info};

type RegisterInt = i32;

#[derive(Debug)]
struct Day17 {
    raw: String,
    program_asm: Vec<u8>,
    reg_start_state: [RegisterInt; 3],
}

#[derive(Debug, Clone)]
enum Operation {
    Adv { raw_operand: usize },
    Bxl { raw_operand: usize },
    Bst { raw_operand: usize },
    Jnz { raw_operand: usize },
    Bxc { raw_operand: usize },
    Out { raw_operand: usize },
    Bdv { raw_operand: usize },
    Cdv { raw_operand: usize },
    INVALID,
}

impl Operation {
    pub fn from_opcode_operand(opcode: u8, operand: u8) -> Self {
        match opcode {
            0 => Self::Adv {
                raw_operand: operand as usize,
            },
            1 => Self::Bxl {
                raw_operand: operand as usize,
            },
            2 => Self::Bst {
                raw_operand: operand as usize,
            },
            3 => Self::Jnz {
                raw_operand: operand as usize,
            },
            4 => Self::Bxc {
                raw_operand: operand as usize,
            },
            5 => Self::Out {
                raw_operand: operand as usize,
            },
            6 => Self::Bdv {
                raw_operand: operand as usize,
            },
            7 => Self::Cdv {
                raw_operand: operand as usize,
            },

            _ => Self::INVALID,
        }
    }
}

#[derive(Debug)]
struct AocMachine {
    ip: usize,
    regs: [RegisterInt; 3],
    program: Vec<u8>,
    output: Vec<RegisterInt>,
}

impl AocMachine {
    pub fn get_combo_operand(&self, raw: usize) -> RegisterInt {
        match raw {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.regs[0],
            5 => self.regs[1],
            6 => self.regs[2],
            _ => {
                panic!("Raw operand {raw}")
            }
        }
    }

    fn messy_division(num: RegisterInt, denom_operand: RegisterInt) -> RegisterInt {
        // let two: RegisterInt = 2;
        // let denom = two.pow(denom_operand as u32);

        // let fnum = num as f64;
        // let dnum = denom as f64;

        // let res = fnum/dnum;
        // debug!("{} / {} = {}", num, denom, res);
        // res as RegisterInt
        num >> denom_operand
    }

    pub fn act(&mut self, op: Operation) -> bool {
        debug!("Running {:?}", op);
        match op {
            Operation::Adv { raw_operand } => {
                let combo_operand = self.get_combo_operand(raw_operand);
                let res = AocMachine::messy_division(self.regs[0], combo_operand);
                self.regs[0] = res;
            }
            Operation::Bxl { raw_operand } => {
                self.regs[1] ^= raw_operand as i32;
            }
            Operation::Bst { raw_operand } => {
                let combo_operand = self.get_combo_operand(raw_operand);
                self.regs[1] = combo_operand % 8;
            }
            Operation::Jnz { raw_operand } => {
                if self.regs[0] != 0 {
                    self.ip = raw_operand;
                    return false;
                }
            }
            Operation::Bxc { raw_operand } => {
                let res = self.regs[1] ^ self.regs[2];
                self.regs[1] = res;
            }
            Operation::Out { raw_operand } => {
                let combo_operand = self.get_combo_operand(raw_operand);
                self.output.push(combo_operand % 8);
            }
            Operation::Bdv { raw_operand } => {
                let combo_operand = self.get_combo_operand(raw_operand);
                let res = AocMachine::messy_division(self.regs[0], combo_operand);
                self.regs[1] = res;
            }
            Operation::Cdv { raw_operand } => {
                let combo_operand = self.get_combo_operand(raw_operand);
                let res = AocMachine::messy_division(self.regs[0], combo_operand);
                self.regs[2] = res;
            }
            Operation::INVALID => {
                error!("INvalid operation found, continuing?");
            }
        }
        true
    }

    pub fn dump_state(&self) {
        debug!("State: Regs: {:?} Ip: {}", self.regs, self.ip);
    }

    pub fn run(&mut self) {
        while self.ip < self.program.len() - 1 {
            self.dump_state();
            let opc = self.program[self.ip];
            let operand = self.program[self.ip + 1];

            let op: Operation = Operation::from_opcode_operand(opc, operand);
            let step = self.act(op);
            if step {
                self.ip += 2;
            }
        }
    }
}

impl Day17 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            program_asm: Vec::new(),
            reg_start_state: [0; 3],
        }
    }

    pub fn parse(&mut self) {
        for (idx, line) in self.raw.lines().enumerate() {
            if idx < 3 {
                self.reg_start_state[idx] = line
                    .split(": ")
                    .last()
                    .unwrap()
                    .parse::<RegisterInt>()
                    .unwrap();
                continue;
            }
            if idx == 3 {
                continue;
            }
            line.split("Program: ")
                .last()
                .unwrap()
                .split(",")
                .for_each(|opc| {
                    debug!("Testing {opc}");
                    self.program_asm.push(opc.parse::<u8>().unwrap())
                });
        }
    }

    pub fn run_machine(&self) -> String {
        let mut machine = AocMachine {
            ip: 0,
            regs: self.reg_start_state,
            program: self.program_asm.clone(),
            output: Vec::new(),
        };

        machine.run();
        println!("{:?}", machine.output);
        let output: String = machine.output.iter().map(|o| o.to_string() + ",").collect();

        output
        // info!("Output: {}", machine.output.map.join(","));
    }
}

type BruteInt = u64;

fn single_step(x: BruteInt) -> (BruteInt, BruteInt) {
    let out: BruteInt = (((x & 7) ^ 2) ^ 7) ^ (x >> ((x & 7) ^ 2)) & 7;
    (out, x >> 3)
}

const brute_ans: [BruteInt; 16] = [2, 4, 1, 2, 7, 5, 1, 7, 4, 4, 0, 3, 5, 5, 3, 0];

fn brute_force() {
    let start: BruteInt = (1 << 16);
    let mut answers: Vec<BruteInt> = Vec::new();
    ((1 as BruteInt) << 45..((1 as BruteInt) << 63))
        .into_par_iter()
        .for_each(|x| {
            // info!("testing {x}");
            let mut i = 0;
            let mut cur = x;
            while i < brute_ans.len() {
                let (res, follow) = single_step(cur);
                if brute_ans[i] != res {
                    return;
                }
                if (follow == 0) {
                    info!("Answer could be {x}, i: {i} res {res}");
                }
                i += 1;
                cur = follow;
            }
            info!("Answer possible at {x}");
        });
}

fn reverse_out() {
    let mut valids: HashSet<u64> = HashSet::new();
    valids.insert(0);
    for target in brute_ans.iter().rev() {
        info!("Looking for target {target}");
        debug!("Looking for target {target}");
        let mut next_valid: HashSet<u64> = HashSet::new();
        for v in valids.iter() {
            for i in 0..8 {
                let test = (*v << 3) + i;
                let res = single_step(test).0;
                debug!("Test {test} found res {res}");
                if res == *target {
                    info!("Found working value {test} for {target}");
                    next_valid.insert(test);
                }
            }
        }
        valids = next_valid;
    }
    info!("valids: {:?}", valids);
    info!("{:?}", valids.iter().min());
}

fn main() {
    let aoc: AocHelper = AocHelper::new(17, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    // let mut d17 = Day17::new(&test_input);
    // d17.parse();
    // debug!("{:?}", d17);
    // let ans = d17.run_machine();
    // info!("Ans: {ans}");

    // let mut d17 = Day17::new(&real_input);
    // d17.parse();
    // debug!("{:?}", d17);
    // let ans = d17.run_machine();
    // info!("Ans: {ans}");

    info!("wah");
    reverse_out();
    info!("wtf");
    // brute_force();
    // d17.reg_start_state[0] = 7552015;
    // let ans = d17.run_machine();
    // info!("Ans: {ans}");
}
