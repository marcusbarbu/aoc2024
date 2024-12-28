use std::collections::VecDeque;

use aoc2024::{AocHelper, RequestedAocInputType};
use tracing::{debug, info};

#[derive(Clone, Debug)]
struct Equation {
    total: usize,
    operands: VecDeque<usize>,
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
    Start,
}

#[derive(Debug)]
struct Day7 {
    raw: String,
    equations: Vec<Equation>,
}

fn solve_equation(round: usize, current: usize, target: usize, numbers: VecDeque<usize>) -> bool {
    if round == 0 {
        let mut next_nums: VecDeque<usize> = numbers.clone();
        let val = next_nums.pop_front().unwrap();
        return solve_equation(round + 1, val, target, next_nums);
    }
    if current == target && numbers.len() == 0 {
        return true;
    }

    if numbers.len() == 0 {
        return false;
    }

    let mut next_nums: VecDeque<usize> = numbers.clone();
    let val = next_nums.pop_front().unwrap();
    let add_ans = solve_equation(round + 1, current + val, target, next_nums.clone());
    let mul_ans = solve_equation(round + 1, current * val, target, next_nums);

    return add_ans || mul_ans;
}

fn concat(a: usize, b: usize) -> usize {
    let mut power_of_ten: usize = 0;
    while 10_usize.pow(power_of_ten as u32) <= b {
        power_of_ten += 1;
    }

    let mathy = (a * 10_usize.pow(power_of_ten as u32)) + b;

    // let stringy = format!("{}{}", a, b).parse::<usize>().unwrap();

    // if mathy != stringy {
    //     error!("Mathy {mathy} != {stringy} A: {a} B: {b} power: {power_of_ten}");
    // }

    return mathy;
}

fn solve_equation_concat(
    round: usize,
    current: usize,
    target: usize,
    numbers: VecDeque<usize>,
    my_op: Operator,
) -> Option<Vec<Operator>> {
    if round == 0 {
        let mut next_nums: VecDeque<usize> = numbers.clone();
        let val = next_nums.pop_front().unwrap();
        return solve_equation_concat(round + 1, val, target, next_nums, Operator::Start);
    }
    if current == target && numbers.len() == 0 {
        let v = vec![my_op];
        return Some(v);
    }

    if numbers.len() == 0 {
        return None;
    }

    let mut next_nums: VecDeque<usize> = numbers.clone();
    let val = next_nums.pop_front().unwrap();
    let add_ans = solve_equation_concat(
        round + 1,
        current + val,
        target,
        next_nums.clone(),
        Operator::Add,
    );
    let mul_ans = solve_equation_concat(
        round + 1,
        current * val,
        target,
        next_nums.clone(),
        Operator::Mul,
    );
    let concat_ans = solve_equation_concat(
        round + 1,
        concat(current, val),
        target,
        next_nums,
        Operator::Concat,
    );

    let answers = [add_ans, mul_ans, concat_ans];
    let best = answers.iter().find(|ans| ans.is_some());

    if let Some(internal_option) = best {
        let mut ans = internal_option.clone().unwrap();
        ans.push(my_op);
        return Some(ans);
    }

    None
}

impl Day7 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            equations: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        self.raw.lines().into_iter().for_each(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let total = parts[0].parse::<usize>().unwrap();
            let mut operands: VecDeque<usize> = VecDeque::new();
            parts[1].split_ascii_whitespace().for_each(|num| {
                operands.push_back(num.parse::<usize>().unwrap());
            });

            self.equations.push(Equation { total, operands })
        });
    }

    pub fn get_answer(&mut self) -> usize {
        self.equations.iter().fold(0, |acc, eq| {
            if solve_equation(0, 0, eq.total, eq.operands.clone()) {
                return acc + eq.total;
            }
            acc
        })
    }

    pub fn get_part2_answer(&mut self) -> usize {
        let mut fail_count = 0;
        let ans = self.equations.iter().fold(0, |acc, eq| {
            if solve_equation(0, 0, eq.total, eq.operands.clone()) {
                return acc + eq.total;
            } else {
                let concat_ans =
                    solve_equation_concat(0, 0, eq.total, eq.operands.clone(), Operator::Start);
                if concat_ans.is_some() {
                    debug!("Had to call concat");
                    debug!("Ans: {:?}", concat_ans.unwrap());
                    return acc + eq.total;
                } else {
                    fail_count += 1;
                }
            }
            acc
        });

        info!("Failed {fail_count} lines");
        return ans;
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(7, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d7: Day7 = Day7::new(&test_input);
    d7.parse();
    debug!("Contents: {:?}", d7);
    let ans = d7.get_answer();
    info!("Answer: {ans}");

    let mut d7: Day7 = Day7::new(&real_input);
    d7.parse();
    // debug!("Contents: {:?}", d7);
    let ans = d7.get_answer();
    info!("Answer: {ans}");

    let mut d7: Day7 = Day7::new(&test_input);
    d7.parse();
    debug!("Contents: {:?}", d7);
    let ans = d7.get_part2_answer();
    info!("Answer: {ans}");

    // return;

    let mut d7: Day7 = Day7::new(&real_input);
    d7.parse();
    // debug!("Contents: {:?}", d7);
    let ans = d7.get_part2_answer();
    info!("Answer: {ans}");
}
