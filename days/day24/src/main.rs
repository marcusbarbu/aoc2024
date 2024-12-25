use std::io::prelude::*;
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
};

use aoc2024::{AocHelper, RequestedAocInputType};
use itertools::{concat, Itertools};
use regex::Regex;
use tracing::{debug, error, info};

#[derive(Debug)]
enum WireCombo {
    Static { value: bool },
    Or { a: String, b: String },
    And { a: String, b: String },
    Xor { a: String, b: String },
}

#[derive(Debug)]
struct SingleOutput {
    name: String,
    predecessors: WireCombo,
    calculated_value: Option<bool>,
}

#[derive(Debug)]
struct Day24 {
    raw: String,
    start_state: HashMap<String, bool>,
    gates: HashMap<String, RefCell<SingleOutput>>,
    missing_count: usize,
}

const COMBINATION_REGEX_STR: &str = r"(\w+)\s*(AND|OR|XOR)\s*(\w+)\s*->\s*(\w+)";

impl Day24 {
    pub fn new(s: &String) -> Self {
        Self {
            raw: s.clone(),
            start_state: HashMap::new(),
            gates: HashMap::new(),
            missing_count: 0,
        }
    }

    pub fn parse(&mut self) {
        let (init_state, gates) = self.raw.split_once("\n\n").unwrap();
        debug!("IS: {:?} |||| G: {:?}", init_state, gates);
        for line in init_state.lines() {
            let (var_name, value) = line.split_once(": ").unwrap();
            let bool_val: bool = value.parse::<i32>().unwrap() == 1;
            self.start_state.insert(var_name.to_string(), bool_val);
            let static_output = SingleOutput {
                name: var_name.to_string(),
                predecessors: WireCombo::Static { value: bool_val },
                calculated_value: Some(bool_val),
            };
            self.gates
                .insert(var_name.to_string(), RefCell::new(static_output));
        }

        let gate_regex = Regex::new(COMBINATION_REGEX_STR).unwrap();
        for line in gates.lines() {
            let Some(caps) = gate_regex.captures(line) else {
                continue;
            };
            let (full, [a, op, b, out]) = caps.extract();
            debug!("Full: {full} a: {a} op: {op} b: {b} out: {out}");
            let combo: WireCombo;
            match op {
                "AND" => {
                    combo = WireCombo::And {
                        a: a.to_string(),
                        b: b.to_string(),
                    };
                }
                "XOR" => {
                    combo = WireCombo::Xor {
                        a: a.to_string(),
                        b: b.to_string(),
                    };
                }
                "OR" => {
                    combo = WireCombo::Or {
                        a: a.to_string(),
                        b: b.to_string(),
                    };
                }
                _ => {
                    error!("Op found was {op}, moving on");
                    continue;
                }
            }

            let so = SingleOutput {
                name: out.to_string(),
                calculated_value: None,
                predecessors: combo,
            };
            self.gates.insert(out.to_string(), RefCell::new(so));
            self.missing_count += 1;
        }
    }

    pub fn single_step(&mut self) {
        for (name, output_val) in self.gates.iter() {
            if let Some(val) = output_val.borrow().calculated_value {
                continue;
            };
            let a_name: String;
            let b_name: String;
            match &output_val.borrow().predecessors {
                WireCombo::Static { value } => {
                    output_val.borrow_mut().calculated_value = Some(*value);
                    continue;
                }
                WireCombo::Or { a, b } => {
                    a_name = a.clone();
                    b_name = b.clone();
                }
                WireCombo::And { a, b } => {
                    a_name = a.clone();
                    b_name = b.clone();
                }
                WireCombo::Xor { a, b } => {
                    a_name = a.clone();
                    b_name = b.clone();
                }
            }
            let Some(a_val) = self.gates.get(&a_name).unwrap().borrow().calculated_value else {
                continue;
            };
            let Some(b_val) = self.gates.get(&b_name).unwrap().borrow().calculated_value else {
                continue;
            };
            let new_val: bool = match &output_val.borrow().predecessors {
                WireCombo::Static { value } => {
                    error!("Tried to combine a static?");
                    continue;
                }
                WireCombo::Or { a, b } => a_val | b_val,
                WireCombo::And { a, b } => a_val & b_val,
                WireCombo::Xor { a, b } => a_val ^ b_val,
            };
            output_val.borrow_mut().calculated_value = Some(new_val);
            self.missing_count -= 1;
            debug!("Set {name} ({:?}) to {new_val}", output_val);
        }
    }

    fn get_by_name_in_order(&self, start: &str) -> Vec<String> {
        let mut zs: Vec<String> = Vec::new();
        for name in self.gates.keys() {
            if name.starts_with(start) {
                zs.push(name.clone());
            }
        }
        zs.sort();
        zs
    }

    fn get_zs_in_order(&self) -> Vec<String> {
        self.get_by_name_in_order("z")
    }

    fn get_res(&self) -> u128 {
        let zs = self.get_zs_in_order();

        let mut output: u128 = 0;
        for (i, z) in zs.iter().enumerate() {
            let Some(tf) = self.gates.get(z).unwrap().borrow().calculated_value else {
                error!("Tried to calculate result with missing value for {z}");
                continue;
            };
            if (tf) {
                output |= (1 << i);
            }
        }
        output
    }

    fn calc_value(&self, start: &str) -> u128 {
        let zs = self.get_by_name_in_order(start);
        for z in zs.iter() {
            debug!("{z}");
        }

        let mut output: u128 = 0;
        for (i, z) in zs.iter().enumerate() {
            let Some(tf) = self.gates.get(z).unwrap().borrow().calculated_value else {
                error!("Tried to calculate result with missing value for {z}");
                continue;
            };
            if (tf) {
                output |= (1 << i);
            }
        }
        output
    }

    pub fn solve_p1(&mut self) -> u128 {
        while self.missing_count > 0 {
            self.single_step();
        }
        self.get_res()
    }

    pub fn check_layers(&self) {
        let zs = self.get_zs_in_order();
        for key in zs.iter() {
            let ins = &self.gates.get(key).unwrap().borrow().predecessors;
            debug!("{} has combo {:?}", key, ins);
        }
    }

    pub fn correct_output(&mut self) {
        let x_val = self.calc_value("x");
        let y_val = self.calc_value("y");

        let correct = x_val + y_val;
        let calced = self.solve_p1();

        info!("Correct: {correct} calced: {calced}");
        let delta = correct ^ calced;
        info!("Wrong bits: {:#32X?}", delta);
    }

    pub fn find_inputs(
        &self,
        key: &str,
        cache: &mut HashMap<String, HashSet<String>>,
    ) -> HashSet<String> {
        if let Some(out) = cache.get(key) {
            return out.clone();
        };
        let mut out: HashSet<String> = HashSet::new();
        let mut cur_name = key.to_string();

        let mut to_check: HashSet<String> = HashSet::new();
        let mut cur_point = self.gates.get(key).unwrap().borrow();
        match &cur_point.predecessors {
            WireCombo::Static { value } => {
                out.insert(cur_name.clone());
            }
            WireCombo::Or { a, b } => {
                to_check.insert(a.clone());
                to_check.insert(b.clone());
            }
            WireCombo::And { a, b } => {
                to_check.insert(a.clone());
                to_check.insert(b.clone());
            }
            WireCombo::Xor { a, b } => {
                to_check.insert(a.clone());
                to_check.insert(b.clone());
            }
        };
        for check in to_check.iter() {
            out.insert(check.clone());
            let res = self.find_inputs(check.as_str(), cache);
            out.extend(res.into_iter());
        }

        cache.insert(cur_name, out.clone());
        out
    }

    pub fn get_inputs_per_output_bit(&self) -> HashSet<String> {
        let xs = self.get_by_name_in_order("x");
        let ys = self.get_by_name_in_order("y");
        let zs = self.get_by_name_in_order("z");

        let mut cache = HashMap::new();
        let all_keys = self.gates.keys();
        for (index, key) in all_keys.clone().enumerate() {
            if xs.contains(key) || ys.contains(key) {
                continue;
            }
            let res = self.find_inputs(key, &mut cache);
            let mut res_vec: Vec<String> = res.into_iter().collect();
            res_vec.sort();
            info!("{key} relies on {:?}", res_vec);
        }
        let mut pot_swaps: HashSet<String> = HashSet::new();
        for key in zs.iter() {
            let deps = cache.get(key).unwrap();
            let top_gate = &self.gates.get(key).unwrap().borrow().predecessors;
            info!(
                "{key} ({:?}) relies on {} deps: {:?}",
                top_gate,
                deps.len(),
                deps
            );
            match top_gate {
                WireCombo::Or { a, b } => {
                    pot_swaps.extend(deps.clone().into_iter());
                    pot_swaps.insert(key.clone());
                }
                WireCombo::And { a, b } => {
                    pot_swaps.extend(deps.clone().into_iter());
                    pot_swaps.insert(key.clone());
                }
                _ => { /*pass */ }
            }
        }
        info!(
            "Got {} potential swaps: {:?} out of {} keys",
            pot_swaps.len(),
            pot_swaps,
            self.gates.len()
        );
        pot_swaps
    }

    pub fn make_graphviz_graph(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        let xs = self.get_by_name_in_order("x");
        let ys = self.get_by_name_in_order("y");
        let zs = self.get_by_name_in_order("z");

        let mut xv: Vec<String> = Vec::new();
        lines.push(format!("{}", xs.join("\n")));
        lines.push(format!("{}", ys.join("\n")));
        lines.push(format!("{}", zs.join("\n")));

        for (key, gate) in self.gates.iter() {
            let mut ins: Vec<String> = Vec::new();
            match &gate.borrow().predecessors {
                WireCombo::Or { a, b } => {
                    ins.push(a.clone());
                    ins.push(b.clone());
                }
                WireCombo::And { a, b } => {
                    ins.push(a.clone());
                    ins.push(b.clone());
                }
                WireCombo::Xor { a, b } => {
                    ins.push(a.clone());
                    ins.push(b.clone());
                }
                _ => {}
            }
            for i in ins {
                lines.push(format!("{} -> {}", i, key));
            }
        }

        lines.join("\n")
    }

    pub fn get_names(w: &WireCombo) -> Vec<String> {
        let mut ins: Vec<String> = Vec::new();
        match w {
            WireCombo::Or { a, b } => {
                ins.push(a.clone());
                ins.push(b.clone());
            }
            WireCombo::And { a, b } => {
                ins.push(a.clone());
                ins.push(b.clone());
            }
            WireCombo::Xor { a, b } => {
                ins.push(a.clone());
                ins.push(b.clone());
            }
            _ => {}
        }
        ins
    }

    pub fn find_input_xor_and_gates(&self) -> (HashMap<String, String>, HashMap<String, String>) {
        let mut out_net_to_logical: HashMap<String, String> = HashMap::new();
        let mut out_logical_to_net: HashMap<String, String> = HashMap::new();
        for (name, gate) in self.gates.iter() {
            let mut ins: Vec<String> = Vec::new();
            let mut name_base: String = String::new();
            match &gate.borrow().predecessors {
                WireCombo::Or { a, b } => {
                    ins.push(a.clone());
                    ins.push(b.clone());
                    name_base.push_str("OR");
                }
                WireCombo::And { a, b } => {
                    ins.push(a.clone());
                    ins.push(b.clone());
                    name_base.push_str("AND");
                }
                WireCombo::Xor { a, b } => {
                    ins.push(a.clone());
                    ins.push(b.clone());
                    name_base.push_str("XOR");
                }
                _ => {}
            }
            if ins.len() == 0 {
                continue;
            }
            if ins.iter().all(|s| s.starts_with("x") || s.starts_with("y")) {
                let new_name = format!("{}_{}_{}", ins[0], name_base, ins[1]);
                out_net_to_logical.insert(name.clone(), new_name.clone());
                out_logical_to_net.insert(new_name.clone(), name.clone());
            }
        }
        (out_net_to_logical, out_logical_to_net)
    }

    pub fn work_from_z1(&self, ntl: &HashMap<String, String>) {
        let zs = self.get_by_name_in_order("z");
        let mut ziter = zs.iter();
        ziter.next();

        let mut of_concern: Vec<String> = Vec::new();
        for z in ziter {
            let z_gate = &self.gates.get(z).unwrap().borrow().predecessors;
            // match z_gate {

            // }
        }
    }
}

fn main() {
    let aoc: AocHelper = AocHelper::new(24, None);
    let test_input = aoc
        .get_input_as_string(RequestedAocInputType::Test)
        .unwrap();
    let real_input = aoc
        .get_input_as_string(RequestedAocInputType::Real)
        .unwrap();

    let mut d24 = Day24::new(&test_input);
    d24.parse();
    debug!("{:?}", d24);
    d24.single_step();
    let res = d24.solve_p1();
    info!("Answer: {res}");

    let mut d24 = Day24::new(&real_input);
    d24.parse();
    debug!("{:?}", d24);
    d24.single_step();
    let res = d24.solve_p1();
    info!("Answer: {res}");

    let mut d24 = Day24::new(&real_input);
    d24.parse();
    d24.check_layers();
    d24.correct_output();
    d24.get_inputs_per_output_bit();

    let gv = d24.make_graphviz_graph();
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(gv.as_bytes());

    let name_map = d24.find_input_xor_and_gates();
    info!("{:#?}", name_map);
}
