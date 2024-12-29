use aoc2024::{AocHelper, RequestedAocInputType};
use tracing::{debug, info};

#[derive(Debug)]
struct Day9 {
    raw: String,
    total_occupied: i128,
    start_state: Vec<i64>,
    expanded_state: Vec<i128>,
    free_spots: Vec<(usize, usize)>, // first_index, size)
    file_list: Vec<(usize, usize)>,
    block_list: Vec<Block>,
}

#[derive(Debug, Clone, Copy)]
struct Block {
    _filled: bool,
    _id: usize,
    _len: usize,
    _start_idx: usize,
}

impl Day9 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            start_state: Vec::new(),
            expanded_state: Vec::new(),
            free_spots: Vec::new(),
            total_occupied: 0,
            file_list: Vec::new(),
            block_list: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        self.start_state = self
            .raw
            .as_bytes()
            .iter()
            .map(|b| (b - 0x30) as i64)
            .collect();
    }

    pub fn expand(&mut self) {
        // self.start_state.iter().enumerate().for_each(|(i, num)|{

        let mut total_index: usize = 0;
        for (i, num) in self.start_state.iter().enumerate() {
            let contents: i128;
            if i % 2 == 0 {
                contents = i as i128 / 2;
                self.total_occupied += *num as i128;
                self.file_list.push((contents as usize, *num as usize));
                let b = Block {
                    _filled: true,
                    _id: contents as usize,
                    _len: *num as usize,
                    _start_idx: total_index,
                };
                self.block_list.push(b);
            } else {
                self.free_spots.push((total_index, *num as usize));
                contents = -1;
                let b = Block {
                    _filled: false,
                    _id: 0,
                    _len: *num as usize,
                    _start_idx: total_index,
                };
                self.block_list.push(b);
            }
            for _ in 0..*num {
                self.expanded_state.push(contents);
            }
            total_index += *num as usize;
        }
    }

    pub fn naive_replace(&mut self) -> i128 {
        let mut working_array = self.expanded_state.clone();
        let mut start = 0;
        let mut end = working_array.len();

        while start <= end {
            debug!("Loop start {start} <> {end}");
            let cur = working_array[start];
            if cur == -1 {
                end -= 1;
                while working_array[end] == -1 {
                    end -= 1;
                }
                let last = working_array[end];
                if start < end {
                    debug!("Putting {last} from {end} to {start}");
                    working_array[end] = -1;
                    working_array[start] = last;
                }
            }
            start += 1;
        }

        let new_occupied: i128 = 0;
        let _vc: String = working_array
            .iter()
            .map(|val| {
                if *val >= 0 {
                    // return (*val % 10 + 0x30) as u8 as char;
                    return format!("|{}|", val.to_string());
                    // new_occupied += 1;
                    // return '#'
                }
                '.'.to_string()
            })
            .collect();
        // info!("After replace: {:?}", vc);
        info!(
            "New occupied {} Orig occupied {}",
            new_occupied, self.total_occupied
        );

        let ans: i128 = working_array
            .iter()
            .enumerate()
            .fold(0, |acc, (index, value)| {
                if *value < 0 {
                    return acc;
                }
                acc + (index as i128 * *value as i128)
            });
        ans
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(9, Some(vec!["second_test".to_string()]));
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let _test2_input = aoc
        .get_input_as_string(RequestedAocInputType::CustomTest {
            fname: "second_test".to_string(),
        })
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d9 = Day9::new(&test_input);
    d9.parse();
    debug!("{:?}", d9);
    d9.expand();
    debug!("{:?}", d9);
    let res = d9.naive_replace();
    debug!("{:?}", d9);
    info!("Ans: {res}");

    let mut d9 = Day9::new(&real_input);
    d9.parse();
    debug!("{:?}", d9);
    d9.expand();
    debug!("{:?}", d9);
    let res = d9.naive_replace();
    // debug!("{:?}", d9);
    info!("Ans: {res}");
}
