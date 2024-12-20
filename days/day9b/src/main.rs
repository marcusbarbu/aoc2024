use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    rc::Rc,
    thread::AccessError,
};

use aoc2024::{AocHelper, RequestedAocInputType};
use tracing::{debug, info};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Block {
    filled: bool,
    id: usize,
    len: usize,
    start_idx: usize,
    valid: bool,
}

// type ProtectedBlock = Rc<RefCell<Option<Block>>>;

// #[derive(Debug, Clone)]
// enum BlockList {
//     Nil,
//     BlockListElem(ProtectedBlock, Rc<BlockList>),

// }

#[derive(Debug)]
struct Day9 {
    raw: String,
    start_state: Vec<usize>,
    block_list: Vec<Block>,
    file_list: Vec<Block>,
    free_set: BTreeSet<Block>,
    free_heaps: Vec<BinaryHeap<Block>>,
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // self.start_idx.cmp(&other.start_idx)
        other.start_idx.cmp(&self.start_idx)
    }
}

impl Day9 {
    pub fn new(s: &String) -> Self {
        let mut heaps: Vec<BinaryHeap<Block>> = Vec::new();
        for i in 0..10 {
            heaps.push(BinaryHeap::new())
        }
        Self {
            raw: s.clone(),
            start_state: Vec::new(),
            block_list: Vec::new(),
            file_list: Vec::new(),
            free_set: BTreeSet::new(),
            free_heaps: heaps,
        }
    }

    pub fn parse(&mut self) {
        self.raw.chars().for_each(|c| {
            self.start_state
                .push(c.to_string().parse::<usize>().unwrap());
        });
    }

    pub fn build_blocks(&mut self) {
        // let mut rootblock = BlockList::BlockListElem(Rc::new(RefCell::new(None)), ())
        let mut rolling_index: usize = 0;
        for (index, size) in self.start_state.iter().enumerate() {
            debug!("Index {index} has size {size}");
            let id = index / 2;
            let b: Block;
            if index % 2 == 1 {
                b = Block {
                    filled: false,
                    id: 0,
                    len: *size,
                    start_idx: rolling_index,
                    valid: true,
                };
            } else {
                b = Block {
                    filled: true,
                    id: id,
                    len: *size,
                    start_idx: rolling_index,
                    valid: true,
                };
            }

            self.block_list.push(b);
            if b.filled {
                self.file_list.push(b)
            } else {
                if *size > 0 {
                    self.free_set.insert(b);
                }
                self.free_heaps[*size].push(b);
            }
            rolling_index += *size;
        }
        debug!("Free heaps: {:#?}", self.free_heaps);
        Day9::print_blocks(&self.block_list);
    }

    fn get_free_block(
        idx: usize,
        block_size: usize,
        bhv: &mut Vec<BinaryHeap<Block>>,
    ) -> Option<Block> {
        debug!("Searching for block of size {block_size}");
        let mut total_options: Vec<(usize, Block)> = Vec::new();
        for acceptable in block_size..bhv.len() {
            let heap = &mut bhv[acceptable];
            if let Some(block) = heap.peek() {
                if block.start_idx < idx {
                    total_options.push((block.start_idx, block.clone()));
                    // let ret_block = heap.pop().unwrap();
                    // return Some(ret_block);
                }
            }
        }
        if total_options.len() == 0 {
            return None;
        }
        debug!(
            "Total options for block of size {block_size} lt idx {idx}: {:?}",
            total_options
        );
        total_options.sort_by(|a, b| a.0.cmp(&b.0));
        debug!("{:?}", total_options);
        let target_heap = total_options.get(0).unwrap().1.len;
        bhv[target_heap].pop()
    }

    // fn print_blocks(orig_list: &Vec<Block>, new_list: &Vec<Block>) {
    fn print_blocks(orig_list: &Vec<Block>) {
        let mut output_block_list = orig_list.clone();
        // output_block_list.append(&mut new_list.clone());
        output_block_list.sort_by(|a, b| a.start_idx.cmp(&b.start_idx));

        // // for block in output_block_list {

        // // }

        // let mut orig_copy = orig_list.clone();
        // orig_copy.sort_by(|a, b| {
        //     a.start_idx.cmp(&b.start_idx)
        // });

        // let mut new_copy = new_list.clone();
        // new_copy.sort_by(|a, b| {
        //     a.start_idx.cmp(&b.start_idx)
        // });

        // let mut output_block_list: Vec<Block> = Vec::new();

        // for block in orig_copy {
        //     if new_copy.iter().find(|nb| {
        //         nb.start_idx ==
        //     })
        // }

        let sorted_string: String = output_block_list
            .iter()
            .map(|block| {
                if block.filled {
                    return block.id.to_string().repeat(block.len);
                } else {
                    return String::from(".").repeat(block.len);
                }
            })
            .collect();

        // info!("Blocks: {}", sorted_string);
    }

    pub fn defrag(&mut self) {
        let mut output_block_list: Vec<Block> = Vec::new();

        //rebuild the list from the back
        for back_block in self.block_list.iter_mut().rev() {
            debug!("Start new block ===============================================");
            if !back_block.filled {
                // if self.free_set.contains(&back_block){
                //     debug!("Free block {:?}", back_block);
                //     output_block_list.push(back_block.clone());
                // }
                // else {
                //     debug!("stale free block, skipping");
                // }
                // debug!("end new block ===============================================");
                continue;
            }
            debug!(
                "Trying to move block with id {}: {:?}",
                back_block.id, back_block
            );
            if let Some(free_block) =
                Day9::get_free_block(back_block.start_idx, back_block.len, &mut self.free_heaps)
            {
                self.free_set.remove(&free_block);

                if free_block.start_idx >= back_block.start_idx {
                    debug!("Overlap on back block {:?}", back_block);
                    output_block_list.push(back_block.clone());
                    continue;
                }

                let mut new_block = back_block.clone();
                let mut new_free_block = back_block.clone();
                let mut new_free_blocks: Vec<Block> = Vec::new();

                new_block.start_idx = free_block.start_idx;
                debug!(
                    "Moving block {:?} to idx {}",
                    back_block, new_block.start_idx
                );
                new_free_block.filled = false;

                if free_block.len != new_block.len {
                    let new_len = free_block.len - new_block.len;
                    let remainder_block = Block {
                        filled: false,
                        id: 0,
                        len: new_len,
                        start_idx: new_block.start_idx + new_block.len,
                        valid: true,
                    };
                    new_free_blocks.push(remainder_block);
                    debug!("Built new block {:?}", remainder_block);
                }

                output_block_list.push(new_block);

                new_free_blocks.push(new_free_block);
                new_free_blocks.iter().for_each(|nfb| {
                    debug!("Registering new free block: {:?}", nfb);
                    self.free_heaps[nfb.len].push(nfb.clone());
                    self.free_set.insert(nfb.clone());
                });
            } else {
                debug!("No block found for {:?}", back_block);
                output_block_list.push(back_block.clone());
            }

            Day9::print_blocks(&output_block_list);
            debug!("end new block ===============================================");
        }

        debug!("Adding free blocks: {:#?}", self.free_set);
        for ele in self.free_set.iter() {
            output_block_list.push(ele.clone());
        }

        Day9::print_blocks(&output_block_list);
        self.block_list = output_block_list;
    }

    pub fn get_score(&self) -> usize {
        let mut working_block_list = self.block_list.clone();
        working_block_list.sort_by(|a, b| a.start_idx.cmp(&b.start_idx));

        let mut total: usize = 0;
        for block in working_block_list {
            if block.filled {
                let mut block_score: usize = 0;
                for i in block.start_idx..block.start_idx + block.len {
                    block_score += (i * block.id);
                }
                total += block_score;
            }
        }

        total
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
    d9.build_blocks();
    debug!("{:?}", d9);
    d9.defrag();
    let ans = d9.get_score();
    info!("Answer? {}", ans);

    let mut d9 = Day9::new(&real_input);
    d9.parse();
    d9.build_blocks();
    debug!("{:?}", d9);
    d9.defrag();
    let ans = d9.get_score();
    info!("Answer? {}", ans);
}
