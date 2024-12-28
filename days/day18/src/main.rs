use std::collections::HashSet;

use aoc2024::{
    graph::{Graph, SCORE_MAX},
    AocHelper, RequestedAocInputType,
};
use tracing::{debug, info};

type Point = (i32, i32);

// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
// struct Point(i32, i32);

#[derive(Debug)]
struct Day18 {
    raw: String,
    walls_list: Vec<Point>,
    walls: HashSet<Point>,
    bounds: Point,
    graph: Graph<Point>,
}

const NEIGHBOR_OFFSETS: [Point; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Day18 {
    pub fn new(s: &String, rows: i32, cols: i32) -> Self {
        Self {
            raw: s.clone(),
            walls: HashSet::new(),
            walls_list: Vec::new(),
            bounds: (rows + 1, cols + 1),
            graph: Graph::new(),
        }
    }

    pub fn parse(&mut self) {
        for line in self.raw.lines().into_iter() {
            let v: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            // everything is in row,col (like everything else this year)
            // self.walls.insert((v[1],v[0]));
            self.walls_list.push((v[1], v[0]));

            // if self.walls.len() >= point_count {
            //     return;
            // }
        }
    }

    pub fn make_graph(&mut self, point_count: usize) {
        self.graph = Graph::new();
        let mut ww_list = self.walls_list.clone();
        ww_list.truncate(point_count);
        let working_walls: HashSet<Point> = ww_list.iter().map(|w| *w).collect();
        self.walls = working_walls.clone();
        for row in 0..self.bounds.0 {
            for col in 0..self.bounds.1 {
                let p = (row, col);
                if working_walls.contains(&p) {
                    continue;
                }

                self.graph.add_point(p);
            }
        }

        let non_walls = self.graph.points.clone();
        for non_wall in non_walls {
            NEIGHBOR_OFFSETS
                .iter()
                .filter_map(|offset| {
                    let np = (non_wall.0 + offset.0, non_wall.1 + offset.1);
                    let inbounds =
                        np.0 >= 0 && np.0 < self.bounds.0 && np.1 >= 0 && np.1 < self.bounds.1;
                    if inbounds && !self.walls.contains(&np) {
                        return Some(np);
                    }
                    None
                })
                .for_each(|valid| {
                    self.graph.add_edge(&non_wall, &valid, 1);
                    // self.graph.add_edge(&valid, &non_wall, 1);
                });
        }
    }

    pub fn print_grid(&self, path: &Vec<Point>) {
        for row in 0..self.bounds.0 {
            let mut rv: String = String::new();
            for col in 0..self.bounds.1 {
                if self.walls.contains(&(row, col)) {
                    rv.push('#');
                } else if path.contains(&(row, col)) {
                    rv.push('O');
                } else {
                    rv.push('.');
                }
            }
            println!("{}", rv);
        }
    }

    pub fn get_shortest_path(&self) -> i32 {
        let actual_corner = (self.bounds.0 - 1, self.bounds.1 - 1);
        let (score, path) = self.graph.shortest_path_len(&(0, 0), &actual_corner);
        // debug!("{:?}", path);
        // self.print_grid(&path);
        score
    }

    pub fn find_impossible_byte(&mut self) -> (i32, i32) {
        let total = self.walls_list.len();
        let mut midpoint = total / 2;
        let mut stepsize = midpoint;
        let actual_corner = (self.bounds.0 - 1, self.bounds.1 - 1);

        while stepsize > 1 {
            info!("Trying to find impossible with half = {midpoint} step size = {stepsize}");
            self.make_graph(midpoint);
            let (score, _) = self.graph.shortest_path_len(&(0, 0), &actual_corner);

            stepsize = stepsize / 2 - 1;
            if score != SCORE_MAX {
                // still good, go higher
                info!("Score {score} is still good, going up");
                midpoint = midpoint + stepsize;
            } else {
                info!("Score {score} is no good, going down");
                // stepsize = stepsize / 2;
                midpoint = midpoint - stepsize;
            }
        }

        for idx in midpoint - 3..midpoint + 3 {
            self.make_graph(idx);
            let (score, _) = self.graph.shortest_path_len(&(0, 0), &actual_corner);
            info!(
                "Score: {score} idx: {idx} point: {:?}",
                self.walls_list[idx]
            );
        }

        self.walls_list[midpoint - 1]
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(18, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d18 = Day18::new(&test_input, 6, 6);
    d18.parse();
    debug!("d18: {:?}", d18);
    d18.make_graph(12);
    debug!("d18: {:?}", d18);
    d18.print_grid(&Vec::new());
    let path_len = d18.get_shortest_path();
    info!("Path len: {path_len}");

    let mut d18 = Day18::new(&real_input, 70, 70);
    d18.parse();
    // debug!("d18: {:?}", d18);
    d18.make_graph(1024);
    // debug!("d18: {:?}", d18);
    d18.print_grid(&Vec::new());
    let path_len = d18.get_shortest_path();
    info!("Path len: {path_len}");

    let mut d18 = Day18::new(&test_input, 6, 6);
    d18.parse();
    let ans = d18.find_impossible_byte();
    info!("Ans : {:?}", ans);
    info!("Answer: {}, {}", ans.1, ans.0);

    let mut d18 = Day18::new(&real_input, 70, 70);
    d18.parse();
    let ans = d18.find_impossible_byte();
    info!("Ans : {:?}", ans);
    info!("Answer: {}, {}", ans.1, ans.0);
}
