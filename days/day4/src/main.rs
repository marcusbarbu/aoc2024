use aoc2024::{AocHelper, RequestedAocInputType};
use diagonal::{diagonal_pos_neg, diagonal_pos_pos};
use regex::Regex;
use tracing::{debug, error, info};

struct Day4 {
    raw: String,
    all_strings: Vec<Vec<String>>,
}

impl Day4 {
    pub fn new(s: &String) -> Self {
        Day4 {
            raw: s.clone(),
            all_strings: Vec::new(),
        }
    }

    fn set_char(v: &mut Vec<Vec<char>>, x: usize, y: usize, c: char) {
        if let Some(inner_vec) = v.get_mut(x) {
            if let Some(char_ref) = inner_vec.get_mut(y) {
                *char_ref = c;
            } else {
                error!("Failed to set char at y: {}", y);
            }
        } else {
            error!("Failed to set char at x: {}", x);
        }
    }

    pub fn make_all_strings(&mut self) {
        let row_count = self.raw.lines().count();
        let row_len = self.raw.lines().next().unwrap().len();

        info!("Row count: {} len: {}", row_count, row_len);

        let dimension = row_count.max(row_len);
        let diag_dimension = 2 * dimension;
        debug!("Dim: {dimension}");

        let empty_vec: Vec<char> = vec!['*'; dimension];

        let mut forward: Vec<String> = Vec::new();
        let mut updown: Vec<Vec<char>> = vec![empty_vec.clone(); dimension];

        debug!("Empty vec: {:?}", empty_vec);

        for (line_no, line) in self.raw.lines().enumerate() {
            forward.push(line.to_string());
            for (cno, cur_char) in line.to_string().chars().enumerate() {
                Day4::set_char(&mut updown, cno, line_no, cur_char);
            }
        }

        let botleft = diagonal_pos_neg(&updown);
        let botright = diagonal_pos_pos(&updown);
        debug!("bl: {:?}", botleft);
        debug!("br: {:?}", botright);

        let botleft: Vec<String> = botleft
            .iter()
            .map(|x| {
                let charss: String = x.iter().map(|c| **c).collect();
                charss
            })
            .collect();

        let botright: Vec<String> = botright
            .iter()
            .map(|x| {
                let charss: String = x.iter().map(|c| **c).collect();
                charss
            })
            .collect();

        let updown: Vec<String> = updown
            .iter()
            .map(|x| {
                let charss: String = x.iter().map(|c| *c).collect();
                charss
            })
            .collect();

        self.all_strings.push(forward.clone());
        self.all_strings.push(updown.clone());
        self.all_strings.push(botleft.clone());
        self.all_strings.push(botright.clone());
    }

    pub fn count_xmas(&self) -> usize {
        self.all_strings.iter().fold(0, |acc, vec_of_strings| {
            let mut total = 0;
            for string in vec_of_strings {
                let normal = Regex::new(r"XMAS").unwrap();
                let normal_count = normal.find_iter(string).count();
                let rev = Regex::new(r"SAMX").unwrap();
                let rev_count = rev.find_iter(string).count();
                total += normal_count + rev_count;
            }

            acc + total
        })
    }
}

struct Day4PartB {
    raw: String,
    char_vec: Vec<Vec<char>>,
}

impl Day4PartB {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            char_vec: Vec::new(),
        }
    }

    pub fn to_vv(&mut self) {
        self.char_vec = self
            .raw
            .lines()
            .into_iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
    }

    pub fn find_cross_mas(&self) -> usize {
        let dim = self.char_vec.len();
        let mut count = 0;
        for row in 0..dim - 2 {
            for col in 0..dim - 2 {
                let a = [(row, col), (row, col + 1), (row, col + 2)];
                let b = [(row + 1, col), (row + 1, col + 1), (row + 1, col + 2)];
                let c = [(row + 2, col), (row + 2, col + 1), (row + 2, col + 2)];

                let a_cstr: Vec<char> = a
                    .iter()
                    .map(|coords| self.char_vec[coords.0][coords.1])
                    .collect();
                let b_cstr: Vec<char> = b
                    .iter()
                    .map(|coords| self.char_vec[coords.0][coords.1])
                    .collect();
                let c_cstr: Vec<char> = c
                    .iter()
                    .map(|coords| self.char_vec[coords.0][coords.1])
                    .collect();

                let mat = vec![a_cstr.clone(), b_cstr.clone(), c_cstr.clone()];
                let pp = diagonal_pos_pos(&mat);
                let pn = diagonal_pos_neg(&mat);

                let pns: String = pn[2].iter().map(|c| **c).collect();
                let pps: String = pp[2].iter().map(|c| **c).collect();

                if (pns.as_str() == "SAM" || pns.as_str() == "MAS")
                    && (pps.as_str() == "SAM" || pps.as_str() == "MAS")
                {
                    count += 1
                }
            }
        }
        count
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(
        4,
        Some(vec!["second_test".to_string(), "p2_input".to_string()]),
    );
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();
    let second_test_input = aoc
        .get_input_as_string(RequestedAocInputType::CustomTest {
            fname: "second_test".to_string(),
        })
        .unwrap();

    let p2_input = aoc
        .get_input_as_string(RequestedAocInputType::CustomTest {
            fname: "p2_input".to_string(),
        })
        .unwrap();

    let mut d4 = Day4::new(&test_input);
    d4.make_all_strings();
    debug!("All strings: {:?}", d4.all_strings);
    let count = d4.count_xmas();
    info!("Count: {}", count);

    let mut d4 = Day4::new(&second_test_input);
    d4.make_all_strings();
    debug!("All strings: {:?}", d4.all_strings);
    let count = d4.count_xmas();
    info!("Count: {}", count);

    let mut d4 = Day4::new(&real_input);
    d4.make_all_strings();
    debug!("All strings: {:?}", d4.all_strings);
    let count = d4.count_xmas();
    info!("Count: {}", count);

    let mut d4b = Day4PartB::new(&p2_input);
    d4b.to_vv();
    debug!("vv: {:?}", d4b.char_vec);
    let c = d4b.find_cross_mas();
    info!("Cross count: {}", c);

    let mut d4b = Day4PartB::new(&real_input);
    d4b.to_vv();
    debug!("vv: {:?}", d4b.char_vec);
    let c = d4b.find_cross_mas();
    info!("Cross count: {}", c);
}
