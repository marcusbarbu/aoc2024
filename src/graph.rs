use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

use tracing::{debug, error};

#[derive(Debug)]
pub struct Graph<T: Hash + Eq + Copy + Clone + core::fmt::Debug + Ord> {
    pub points: HashSet<T>,
    edges: HashMap<T, HashMap<T, i32>>,
}

pub const SCORE_MAX: i32 = 1_234_567_890;
impl<T: Hash + Eq + Copy + Clone + core::fmt::Debug + Ord> Graph<T> {
    pub fn new() -> Self {
        Graph {
            points: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_point(&mut self, dp: T) {
        self.points.insert(dp);
    }

    pub fn add_edge(&mut self, dp_start: &T, dp_end: &T, cost: i32) -> bool {
        debug!(
            "Inserting edge between {:?} and {:?} cost: {cost}",
            dp_start, dp_end
        );
        if !self.points.contains(dp_start) || !self.points.contains(dp_end) {
            error!("Couldn't insert edge b/c one point does not exist");
            return false;
        }

        let mut working_map: &mut HashMap<T, i32>;
        if !self.edges.contains_key(dp_start) {
            self.edges.insert(dp_start.clone(), HashMap::new());
        }
        working_map = self.edges.get_mut(&dp_start).unwrap();
        working_map.insert(dp_end.clone(), cost);

        true
    }

    pub fn get_neighbors(&self, p: &T) -> Option<Vec<(T, i32)>> {
        let Some(edges) = self.edges.get(p) else {
            return None;
        };

        let out: Vec<(T, i32)> = edges.iter().map(|(p, s)| (p.clone(), s.clone())).collect();

        Some(out)
    }

    fn reconstruct_path(start_ref: &T, end: &T, prevs: &HashMap<T, Option<T>>) -> Vec<T> {
        let mut out: Vec<T> = Vec::new();
        let mut cur: &T = end;
        while let Some(Some(prev)) = prevs.get(&cur) {
            out.push(cur.clone());
            cur = prev;
        }
        out.iter().map(|x| *x).rev().collect()
    }

    pub fn shortest_path_len(&self, start_ref: &T, end: &T) -> (i32, Vec<T>) {
        let mut working_set: HashSet<T>;
        let mut working_q: BinaryHeap<(T, i32)> = BinaryHeap::new();
        let mut prev: HashMap<T, Option<T>> = HashMap::new();
        let mut distances: HashMap<T, i32> = HashMap::new();
        let start = start_ref.clone();

        working_set = self.points.iter().map(|p| p.clone()).collect();
        debug!("start: {:?}", start);

        // let Some(cur) = working_q.pop() else {return;};

        for p in working_set.iter() {
            if *p == start {
                debug!("Skipping {:?}", p);
                continue;
            }
            distances.insert(p.clone(), SCORE_MAX);
            prev.insert(p.clone(), None);
            working_q.push((p.clone(), SCORE_MAX));
        }
        debug!("Distances: {:?}", distances);
        distances.insert(start.clone(), 0);
        let dbstart = distances.get(&start);
        debug!("{:?}", dbstart);
        prev.insert(start.clone(), None);
        working_q.push((start.clone(), 0));

        debug!("Distances: {:?}", distances);
        while let Some(cur) = working_q.pop() {
            // debug!("Working queue state: {:?}", working_q);
            working_set.remove(&cur.0);
            debug!("Starting from {:?}", cur);
            let Some(neighbors) = self.get_neighbors(&cur.0) else {
                debug!("No neighbors for {:?} ??", cur);
                continue;
            };
            debug!("Neighbors for {:?} => {:?}", cur, neighbors);
            for n in neighbors {
                debug!("\t Working on {:?}", n);
                let alt_dist_to_n = cur.1 + n.1; // is it cheaper to get to n via cur?
                let n_prev_score = distances.get(&n.0).unwrap();
                if alt_dist_to_n < *n_prev_score {
                    debug!("New score for neighbor {:?}", n);
                    distances.insert(n.0, alt_dist_to_n);
                    prev.insert(n.0, Some(cur.0));
                    let nsp = (n.0, alt_dist_to_n);
                    debug!("Inserting {:?}", nsp);
                    working_q.push(nsp);
                    working_set.insert(n.0);
                } else {
                    debug!(
                        "{:?} prev score {} new score {}, no change",
                        n, n_prev_score, alt_dist_to_n
                    );
                }
            }
        }

        debug!("Done! Scores: {:?} Prev: {:?}", distances, prev);

        let res = distances.get(end);
        debug!("Res: {:?}", res);
        let path = Graph::reconstruct_path(start_ref, end, &prev);
        if res.is_some() {
            return (*res.unwrap(), path);
        }
        (0, Vec::new())
    }
}
