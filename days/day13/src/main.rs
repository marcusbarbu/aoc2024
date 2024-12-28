use aoc2024::{AocHelper, RequestedAocInputType};
use regex::Regex;
use tracing::{error, info};

type Button = (i64, i64);
type Point = (i64, i64);

#[derive(Debug)]
struct SysEq {
    a: Button,
    b: Button,
    target: Point,
}

#[derive(Debug)]
struct Day13 {
    raw: String,
    problems: Vec<SysEq>,
}

// r"Button A: X+17, Y+86"

const BUTTON_REGEX: &str = r"Button [A|B]: X(.*), Y(.*)";
const TARGET_REGEX: &str = r"Prize: X=(\d+), Y=(\d+)";

const FLOAT_ISSUE_LIMIT: f64 = 0.00000000000005;
const P2_OFFSET: i64 = 10000000000000;

impl SysEq {
    pub fn solve(&self, is_b: bool) -> (i64, i64) {
        let ax: f64 = self.a.0 as f64;
        let ay: f64 = self.a.1 as f64;
        let bx: f64 = self.b.0 as f64;
        let by: f64 = self.b.1 as f64;
        let tx: f64 = self.target.0 as f64;
        let ty: f64 = self.target.1 as f64;

        let ratio = ((ax / tx) - (ay / ty)) / ((by / ty) - (bx / tx));
        // debug!("Ratio is {ratio}");

        let solve_a: f64 = tx / (ax + (bx * ratio));
        let solve_b: f64 = solve_a * ratio;
        // debug!("a is {solve_a} b is {solve_b}");

        let check_a = solve_a.round() as i64;
        let check_b = solve_b.round() as i64;

        let check_tx = (check_a * self.a.0) + (check_b * self.b.0);
        let check_ty = (check_a * self.a.1) + (check_b * self.b.1);

        // debug!("CHECKING a: {check_a} b: {check_b} = {check_tx} vs {}", self.target.0);
        let is_neg = (check_a < 0) || (check_b < 0);
        let is_gt_100 = (check_a > 100) || (check_b > 100);

        if is_neg || (is_gt_100 && !is_b) {
            error!(
                "NEGATIVE a: {solve_a} b: {solve_b} a: {check_a} b: {check_b} = {check_tx} vs {}",
                self.target.0
            );
            return (0, 0);
        }

        if check_tx != self.target.0 {
            error!(
                "FAILED X a: {solve_a} b: {solve_b} a: {check_a} b: {check_b} = {check_tx} vs {}",
                self.target.0
            );
            return (0, 0);
        }

        if check_ty != self.target.1 {
            error!(
                "FAILED Y a: {solve_a} b: {solve_b} a: {check_a} b: {check_b} = {check_ty} vs {}",
                self.target.1
            );
            return (0, 0);
        }

        (check_a, check_b)
    }
}

impl Day13 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            problems: Vec::new(),
        }
    }

    pub fn parse(&mut self, offset: i64) {
        let button_regex = Regex::new(BUTTON_REGEX).unwrap();
        let target_regex = Regex::new(TARGET_REGEX).unwrap();
        let lv: Vec<String> = self.raw.lines().map(|s| s.to_string()).collect();
        lv.chunks(4).for_each(|chunk| {
            // debug!("Chunk: {:?}", chunk);
            let a_match = button_regex.captures(&chunk[0]).unwrap();
            let b_match = button_regex.captures(&chunk[1]).unwrap();
            let target_match = target_regex.captures(&chunk[2]).unwrap();

            // debug!("A {:?} B {:?} Target: {:?}", a_match, b_match, target_match);
            let x = a_match.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = a_match.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let a = (x, y);

            let x = b_match.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = b_match.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let b = (x, y);

            let x = target_match
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap();
            let y = target_match
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap();
            let target = (x + offset, y + offset);

            let s = SysEq {
                a: a,
                b: b,
                target: target,
            };

            self.problems.push(s);
        });
    }

    pub fn get_score(&self, is_b: bool) -> i64 {
        self.problems.iter().fold(0, |acc, prob| {
            let (a, b) = prob.solve(is_b);
            let score = (3 * a) + b;
            // debug!("Ans for {:?} is {:?} score: {}", prob, (a,b), score);
            acc + score
        })
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(13, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d13 = Day13::new(&test_input);
    d13.parse(0);
    let ans = d13.get_score(false);
    info!("Score: {ans}");

    let mut d13 = Day13::new(&real_input);
    d13.parse(0);
    let ans = d13.get_score(false);
    info!("Score: {ans}");

    let mut d13 = Day13::new(&test_input);
    d13.parse(P2_OFFSET);
    let ans = d13.get_score(true);
    info!("Score: {ans}");

    let mut d13 = Day13::new(&real_input);
    d13.parse(P2_OFFSET);
    let ans = d13.get_score(true);
    info!("Score: {ans}");
}
