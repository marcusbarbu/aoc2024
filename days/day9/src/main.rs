use std::{borrow::BorrowMut, collections::BTreeMap};

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
    filled: bool,
    id: usize,
    len: usize,
    start_idx: usize,
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
                    filled: true,
                    id: contents as usize,
                    len: *num as usize,
                    start_idx: total_index,
                };
                self.block_list.push(b);
            } else {
                self.free_spots.push((total_index, *num as usize));
                contents = -1;
                let b = Block {
                    filled: false,
                    id: 0,
                    len: *num as usize,
                    start_idx: total_index,
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
        let vc: String = working_array
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

    fn find_fit_block(start: usize, size: usize, v: &Vec<(usize, usize)>) -> usize {
        let ans: Option<usize> = v.iter().enumerate().find_map(|(index, freespot)| {
            if freespot.1 >= size && freespot.0 < start {
                return Some(index);
            }
            None
        });
        if ans.is_some() {
            return ans.unwrap();
        }

        v.len()
    }

    // TODO: put this in the library
    fn append_to_mapping(btm: &mut BTreeMap<usize, Vec<Block>>, k: usize, v: Block) {
        match btm.get_mut(&k) {
            Some(vv) => vv.push(v),
            None => {
                let mut new_vec = Vec::new();
                new_vec.push(v);
                btm.insert(k, new_vec);
            }
        }
    }

    pub fn real_defrag(&self) {
        // refactor internal representation to use a block struct
        // which includes information about name, size, free/not free
        // then go through all file blocks in reverse order, try to match them with appropriate free blocks
        // replace a free block with a new file block and a free block if necessary
        // recalculate by using stored index info in the block structs
        let free_counts: BTreeMap<usize, Vec<Block>> = BTreeMap::new();

        let mut working_block_list = self.block_list.clone();

        // for block in working_block_list {
        //     if !block.filled{
        //         Day9::append_to_mapping(&mut free_counts, block.len, block);
        //     }
        // }

        let mut reverse_file_list = self.block_list.clone();
        reverse_file_list.reverse();
        let mut reverse_file_list = reverse_file_list.iter_mut().enumerate().filter_map(|b| {
            if b.1.filled {
                let orig_idx = self.block_list.len() - (b.0 + 1);
                return Some((orig_idx, b.1));
            }
            None
        });

        let mut idx_offset = 0;
        for (rev_idx, file) in reverse_file_list.borrow_mut() {
            debug!("Working on file {:?} at orig idx {}", file, rev_idx);
            let mut idx = None;
            for (vec_idx, block) in working_block_list.iter().enumerate() {
                if block.filled {
                    continue;
                }
                if block.len >= file.len {
                    idx = Some(vec_idx);
                    break;
                }
            }

            if idx.is_some() {
                let idx = idx.unwrap();
                debug!("Found an index at {idx}");
                let working_block = working_block_list.get_mut(idx).unwrap();
                let len_diff = working_block.len - file.len;
                working_block.len = file.len;
                working_block.filled = true;
                working_block.id = file.id;
                let new_start_idx = working_block.start_idx + working_block.len;

                working_block_list.remove(rev_idx + idx_offset);
                if len_diff > 0 {
                    debug!("Building a new block!");
                    let b = Block {
                        filled: false,
                        id: 0,
                        len: len_diff,
                        start_idx: new_start_idx,
                    };
                    working_block_list.insert(idx + 1, b);
                    idx_offset += 1;
                }
                // (*file).filled = false;
            }
        }
        info!("After defrag: {:?}", working_block_list);
        let as_string: String = working_block_list
            .iter()
            .map(|block| {
                if block.filled {
                    return block.id.to_string().repeat(block.len);
                } else {
                    return ".".to_string();
                }
            })
            .collect();
        info!("After defrag: {:?}", as_string);
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(9, Some(vec!["second_test".to_string()]));
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let test2_input = aoc
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
    d9.real_defrag();

    return;
    let mut d9 = Day9::new(&test2_input);
    d9.parse();
    debug!("{:?}", d9);
    d9.expand();
    debug!("{:?}", d9);
    let res = d9.naive_replace();
    debug!("{:?}", d9);
    info!("Ans: {res}");
    d9.real_defrag();

    return;

    let mut d9 = Day9::new(&real_input);
    d9.parse();
    debug!("{:?}", d9);
    d9.expand();
    debug!("{:?}", d9);
    let res = d9.naive_replace();
    // debug!("{:?}", d9);
    info!("Ans: {res}");
}
