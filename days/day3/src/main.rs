use aoc2024::{AocHelper, RequestedAocInputType};
use regex::Regex;
use tracing::{debug, info};
struct Day3 {
    raw: String,
    muls: Vec<[i32; 2]>,
}

const DO_DONT_REGEX: &str = r"(mul)\((\d+)\,(\d+)\)|(do)\(\)|(don't)\(\)";
const STANDARD_MUL_REGEX: &str = r"mul\((\d+)\,(\d+)\)";

impl Day3 {
    pub fn new(s: &String) -> Self {
        Day3 {
            raw: s.clone(),
            muls: Vec::new(),
        }
    }

    pub fn get_muls(&mut self) {
        let re = Regex::new(STANDARD_MUL_REGEX).unwrap();
        let muls: Vec<&str> = re.find_iter(&self.raw).map(|m| m.as_str()).collect();
        debug!("Found muls {:?}", muls);

        re.captures_iter(&self.raw).for_each(|caps| {
            debug!("Caps: {:?}", caps);
            let a = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let c = [a, b];
            info!("Found nums {:?}", c);
            self.muls.push([a, b]);
        });
    }

    pub fn get_optional_muls(&mut self) {
        let re: Regex = Regex::new(DO_DONT_REGEX).unwrap();

        let mut allowed: bool = true;
        for capture in re.captures_iter(&self.raw) {
            debug!("Found cap {:?}", capture);
            let cc: Vec<&str> = capture
                .iter()
                .map(|c| match c {
                    Some(cap) => return cap.as_str(),
                    None => {
                        return "";
                    }
                })
                .collect();

            debug!("1: {} 4: {} 5: {}", cc[1], cc[4], cc[5]);

            match [cc[1], cc[4], cc[5]] {
                ["mul", _, _] => {
                    if !allowed {
                        continue;
                    }
                    let a = cc[2].parse::<i32>().unwrap();
                    let b = cc[3].parse::<i32>().unwrap();
                    let c = [a, b];
                    info!("Found nums {:?}", c);
                    self.muls.push(c);
                }
                [_, "do", _] => {allowed = true;}
                [_, _, "don't"] => {allowed = false;}
                [_, _, _] => {}
            }
        }
    }

    pub fn get_mul_sum(&self) -> i32 {
        self.muls.iter().fold(0, |acc, x| acc + (x[0] * x[1]))
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(3, Some(vec!["dont_test".to_string()]));
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let second_test_input = aoc
        .get_input_as_string(RequestedAocInputType::CustomTest {
            fname: "dont_test".to_string(),
        })
        .unwrap();

    let mut d3 = Day3::new(&test_input);
    d3.get_muls();
    let total = d3.get_mul_sum();
    info!("Got answer: {total}");

    let mut d3 = Day3::new(&real_input);
    d3.get_muls();
    let total = d3.get_mul_sum();
    info!("Got answer: {total}");

    let mut d3 = Day3::new(&second_test_input);
    d3.get_optional_muls();
    let total = d3.get_mul_sum();
    info!("Got answer: {total}");

    let mut d3 = Day3::new(&real_input);
    d3.get_optional_muls();
    let total = d3.get_mul_sum();
    info!("Got answer: {total}");
}
