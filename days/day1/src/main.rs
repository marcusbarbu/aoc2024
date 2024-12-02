use std::{collections::BTreeMap, env, iter::zip};
struct Day1PartA {
    input_path: String,
    a_vec: Vec<i32>,
    b_vec: Vec<i32>
}

impl Day1PartA {
    pub fn new(path: &str) -> Self {
        let p: String = path.to_string();
        Day1PartA{
            input_path: p,
            a_vec: Vec::new(),
            b_vec: Vec::new()
        }
    }

    pub fn parse(&mut self){
        let raw = std::fs::read_to_string(self.input_path.clone());
        let raw = raw.unwrap();

        raw.lines().for_each(|line| {
            let mut parts = line.split_ascii_whitespace();
            self.a_vec.push(parts.next().unwrap().parse::<i32>().unwrap());
            self.b_vec.push(parts.next().unwrap().parse::<i32>().unwrap());
        });

    }

    pub fn sort_lists(&mut self){
        self.a_vec.sort();
        self.b_vec.sort();
    }

    pub fn get_diff_sum(&mut self) -> i32{

        let ans: i32 = zip(&mut self.a_vec, &mut self.b_vec).fold(0, |acc, (a, b)| {
            acc + a.abs_diff(*b) as i32
        });
        ans
    }
}

struct Day1PartB {
    input_path: String,
    a_vec: Vec<i32>,
    b_counter: BTreeMap<i32, u32>
}

impl Day1PartB {
    pub fn new(path: &str) -> Self {
        let p: String = path.to_string();
        Day1PartB{
            input_path: p,
            a_vec: Vec::new(),
            b_counter: BTreeMap::new()
        }
    }

    pub fn parse(&mut self){
        let raw = std::fs::read_to_string(self.input_path.clone());
        let raw = raw.unwrap();

        raw.lines().for_each(|line| {
            let mut parts = line.split_ascii_whitespace();
            self.a_vec.push(parts.next().unwrap().parse::<i32>().unwrap());
            let b = parts.next().unwrap().parse::<i32>().unwrap();
            self.b_counter.entry(b).and_modify(|cur| *cur += 1).or_insert(1);
        });

    }

    pub fn get_score(&self) -> i32 {
        self.a_vec.iter().fold(0, |acc, cur| {
            let cur_score = *(self.b_counter.get(cur).unwrap_or(&0)) as i32;
            acc + *cur * cur_score
        })
    }

}




fn main() {
    let _ = dotenvy::dotenv();
    let base_path: String = env::var("STATIC_BASE_PATH").unwrap();
    let test_path = base_path.clone() + "/day1/test_input";
    let mut day1_part_a_test: Day1PartA = Day1PartA::new(&test_path);
    day1_part_a_test.sort_lists();
    day1_part_a_test.parse();

    let ans = day1_part_a_test.get_diff_sum();
    println!("Found diff answer: {}", ans);

    let real_path = base_path + "/day1/real_input";
    let mut day1_part_a_real: Day1PartA = Day1PartA::new(&real_path);
    day1_part_a_real.parse();
    day1_part_a_real.sort_lists();

    let ans = day1_part_a_real.get_diff_sum();
    println!("Found diff answer: {}", ans);

    let mut day1_partb_test: Day1PartB = Day1PartB::new(&test_path);
    day1_partb_test.parse();
    println!("Counter: {:?}", day1_partb_test.b_counter);
    let ans = day1_partb_test.get_score();
    println!("Score: {}", ans);

    let mut day1_partb_real: Day1PartB = Day1PartB::new(&real_path);
    day1_partb_real.parse();
    println!("Counter: {:?}", day1_partb_real.b_counter);
    let ans = day1_partb_real.get_score();
    println!("Score: {}", ans);

}
