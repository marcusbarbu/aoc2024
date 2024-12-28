use std::error::Error;
use std::hash::Hash;
use std::io::prelude::*;
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs::File,
};

use aoc2024::map_vec_extend::append_to_hash_map;
use aoc2024::{AocHelper, AocHelperError, AocResult, RequestedAocInputType};
use itertools::Itertools;
use regex::Regex;
use tracing::{debug, error, info};

#[derive(Debug, Clone)]
enum WireCombo {
    Static { value: bool },
    Or { a: String, b: String },
    And { a: String, b: String },
    Xor { a: String, b: String },
}

#[derive(Debug, Clone)]
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
            if tf {
                output |= 1 << i;
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
            if tf {
                output |= 1 << i;
            }
        }
        output
    }

    pub fn solve_p1(&mut self) -> AocResult<u128 >{
        let mut last = 0;
        let mut consec = 0;
        while self.missing_count > 0 {
            self.single_step();
            if self.missing_count == last {
                consec += 1;
            }
            last = self.missing_count;

            if consec >= 20 {
                return Err(AocHelperError::TimeoutError);
            }

        }
        Ok(self.get_res())
    }

    pub fn check_layers(&self) {
        let zs = self.get_zs_in_order();
        for key in zs.iter() {
            let ins = &self.gates.get(key).unwrap().borrow().predecessors;
            debug!("{} has combo {:?}", key, ins);
        }
    }

    pub fn swap(&mut self, a_name: &str, b_name: &str) {
        let a_val = self.gates.get(a_name).unwrap().clone();
        let b_val = self.gates.get(b_name).unwrap().clone();
        self.gates.remove(a_name);
        self.gates.remove(b_name);

        self.gates.insert(a_name.to_string(), b_val);
        self.gates.insert(b_name.to_string(), a_val);
    }

    pub fn correct_output(&mut self)->bool {
        let x_val = self.calc_value("x");
        let y_val = self.calc_value("y");

        let correct = x_val + y_val;
        let Ok(calced) = self.solve_p1() else {
            error!("didn't complete");
            return false;};

        info!("Correct: {correct} calced: {calced}");
        let delta = correct ^ calced;
        if delta == 0 {
            return true;
        }
        info!("Wrong bits: {:#32X?}", delta);
        false
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
        let cur_name = key.to_string();

        let mut to_check: HashSet<String> = HashSet::new();
        let cur_point = self.gates.get(key).unwrap().borrow();
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

        let xv: Vec<String> = Vec::new();
        lines.push(format!("
subgraph {{
    rank = \"same\"

{}
{}
    }}", xs.join("\n"), ys.join("\n")));

        // lines.push(format!("{}", ys.join("\n")));
        lines.push(format!("
subgraph {{
    rank = \"same\"

{}
    }}", zs.join("\n")));

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
            ins.sort();
            if ins.iter().all(|s| s.starts_with("x") || s.starts_with("y")) {
                let new_name = format!("{}_{}_{}", ins[0], name_base, ins[1]);
                out_net_to_logical.insert(name.clone(), new_name.clone());
                out_logical_to_net.insert(new_name.clone(), name.clone());
            }
        }
        (out_net_to_logical, out_logical_to_net)
    }

    pub fn calculate_cins(&self, ntl: &HashMap<String, String>) -> (HashMap<String, String>, HashMap<String, String>, Vec<String>) {
        let mut original_name_to_cin: HashMap<String, String> = HashMap::new();
        let mut cin_to_original_name: HashMap<String, String> = HashMap::new();
        let zs = self.get_by_name_in_order("z");
        let mut ziter = zs.iter();
        ziter.next();

        let mut of_concern: Vec<String> = Vec::new();
        for (i, z) in ziter.enumerate() {
            debug!("z: {} i: {}", z, i);
            let z_gate = &self.gates.get(z).unwrap().borrow().predecessors;
            let a_s: String;
            let b_s: String;
            match z_gate {
                WireCombo::Xor { a, b } => {
                    a_s = a.clone();
                    b_s = b.clone();
                }
                _ => {
                    of_concern.push(z.clone());
                    continue;
                }
            }
            debug!("{} or {}", a_s, b_s);
            let should_be_cin_leg: String;
            let target = &format!("x{:0>2}_XOR_y{:0>2}", i+1, i+1);
            let a_is_xor_leg = ntl.get(&a_s).is_some() && ntl.get(&a_s).unwrap() == target;
            let b_is_xor_leg = ntl.get(&b_s).is_some() && ntl.get(&b_s).unwrap() == target;
            if a_is_xor_leg && b_is_xor_leg {
                of_concern.push(z.clone());
                error!("Weird on {z}");
                continue;
            }
            else if a_is_xor_leg {
                should_be_cin_leg = b_s;
            }
            else if b_is_xor_leg {
                should_be_cin_leg = a_s;
            }
            else {
                of_concern.push(z.clone());
                error!("Z {z} has no xor leg");
                continue;
            }
            let cin_name = format!("CIN_{:0>2}", i+1);
            original_name_to_cin.insert(should_be_cin_leg.clone(), cin_name.clone());
            cin_to_original_name.insert(cin_name.clone(), should_be_cin_leg.clone());
        }
        error!("Of concern: {:?}", of_concern);

        (original_name_to_cin, cin_to_original_name, of_concern)
    }

    pub fn find_intermediate_ands(&self, cins: &HashMap<String, String>, xor_ands: &HashMap<String, String>) -> (HashMap<String,String> , HashMap<String,String>) {
        let mut interadds_name_logical: HashMap<String,String> = HashMap::new();
        let mut interadds_logical_name: HashMap<String,String> = HashMap::new();
        for (name, gate_struct) in self.gates.iter(){
            if xor_ands.contains_key(name){
                continue;
            }
            let inner = &gate_struct.borrow().predecessors;
            let a_cin;
            let b_cin;
            let a_xa ;
            let b_xa ;
            match inner {
                WireCombo::And { a, b } => {
                    a_cin = cins.get(a);
                    b_cin = cins.get(b);
                    a_xa = xor_ands.get(a);
                    b_xa = xor_ands.get(b);
                }
                _=>{continue;}
            };
            match [a_cin, b_cin, a_xa, b_xa] {
                [Some(a), None, None, Some(b)] => {
                    debug!("A: {a} B: {b}");
                    let cin_number = a.split_once("_").unwrap().1.parse::<i32>().unwrap();
                    let xx_num = b.split_once("_").unwrap().0.trim_matches('x').parse::<i32>().unwrap();
                    // debug!("Cin number: {cin_number} xx: {xx_num}");
                    if cin_number == xx_num {
                        interadds_name_logical.insert(name.clone(), format!("{:0>2}_INTER_ADD", xx_num));
                        interadds_logical_name.insert(format!("{:0>2}_INTER_ADD", xx_num), name.clone());
                    }
                    else {
                        error!("Number mismatch for {}: {cin_number} != {xx_num}", name)
                    }
                }
                [None, Some(a), Some(b), None] => {
                    debug!("A: {a} B: {b}");
                    let cin_number = a.split_once("_").unwrap().1.parse::<i32>().unwrap();
                    let xx_num = b.split_once("_").unwrap().0.trim_matches('x').parse::<i32>().unwrap();
                    // debug!("Cin number: {cin_number} xx: {xx_num}");
                    if cin_number == xx_num {
                        interadds_name_logical.insert(name.clone(), format!("{:0>2}_INTER_ADD", xx_num));
                        interadds_logical_name.insert(format!("{:0>2}_INTER_ADD", xx_num), name.clone());
                    }
                    else {
                        error!("Number mismatch for {}: {cin_number} != {xx_num}", name)
                    }
                }
                _ => {
                    error!("{} Set was AND {:?} ({:?})", name, [a_cin, b_cin, a_xa, b_xa], inner);
                }
            }
        }
        (interadds_name_logical, interadds_logical_name)
    }

    pub fn find_couts(&self, inter_ands: &HashMap<String, String>, xor_ands: &HashMap<String, String>) -> (HashMap<String, String>, HashMap<String,String>) {
        let mut cout_name_logical: HashMap<String,String> = HashMap::new();
        let mut cout_logical_name: HashMap<String,String> = HashMap::new();

        for (name, gate_struct) in self.gates.iter(){
            if xor_ands.contains_key(name){
                continue;
            }
            let inner = &gate_struct.borrow().predecessors;
            let a_ia;
            let b_ia;
            let a_xa ;
            let b_xa ;
            match inner {
                WireCombo::Or { a, b } => {
                    a_ia = inter_ands.get(a);
                    b_ia = inter_ands.get(b);
                    a_xa = xor_ands.get(a);
                    b_xa = xor_ands.get(b);
                }
                _=>{continue;}
            };
            match [a_ia, b_ia, a_xa, b_xa] {
                [Some(a), None, None, Some(b)] => {
                    debug!("A: {a} B: {b}");
                    let ia_number = a.split_once("_").unwrap().0.parse::<i32>().unwrap();
                    let xx_num = b.split_once("_").unwrap().0.trim_matches('x').parse::<i32>().unwrap();
                    // debug!("ia number: {ia_number} xx: {xx_num}");
                    if ia_number == xx_num {
                        cout_name_logical.insert(name.clone(), format!("COUT_{:0>2}", xx_num+1));
                        cout_logical_name.insert(format!("COUT_{:0>2}", xx_num+1), name.clone());
                    }
                    else {
                        error!("Number mismatch for {}: {ia_number} != {xx_num}", name)
                    }
                }
                [None, Some(a), Some(b), None] => {
                    debug!("A: {a} B: {b}");
                    let ia_number = a.split_once("_").unwrap().0.parse::<i32>().unwrap();
                    let xx_num = b.split_once("_").unwrap().0.trim_matches('x').parse::<i32>().unwrap();
                    // debug!("ia number: {ia_number} xx: {xx_num}");
                    if ia_number == xx_num {
                        cout_name_logical.insert(name.clone(), format!("COUT_{:0>2}", xx_num+1));
                        cout_logical_name.insert(format!("COUT_{:0>2}", xx_num+1), name.clone());
                    }
                    else {
                        error!("Number mismatch for {}: {ia_number} != {xx_num}", name)
                    }
                }
                _ => {
                    error!("{} Set was AND {:?} ({:?})", name, [a_ia, b_ia, a_xa, b_xa], inner);
                }
            }
        }

        (cout_name_logical, cout_logical_name)
    }

    pub fn combine_maps(&self, v: Vec<&HashMap<String,String>>) -> (Vec<(String, Vec<String>)>, HashMap<String, Vec<String>>) {
        let mut comb: HashMap<String, Vec<String>> = HashMap::new();
        for hm in v.iter() {
            hm.iter().for_each(|(k, v)| {
                append_to_hash_map(&mut comb, k.clone(), v.clone());
            });
        }
        let mut out: Vec<(String, Vec<String>)> = Vec::new();
        for key in comb.keys() {
            let val = comb.get(key).unwrap().to_owned();
            out.push((key.clone(), val));
        }
        out.sort_by_key(|x| x.0.clone());
        (out, comb)
    }

    pub fn find_unsafes(&self, comb_hm: &HashMap<String, Vec<String>>) -> HashSet<String> {
        let mut pot_swaps: HashSet<String> = HashSet::new();
        for (key, all_names) in comb_hm.iter() {
            let mut ani = all_names.iter();
            let c_in =ani.clone().any(|x| x.starts_with("CIN"));
            let c_out=ani.clone().any(|x| x.starts_with("COUT"));

            match [c_in, c_out] {
                [true, true] => {}
                [false, false] => {
                    if all_names.len() > 1 {
                        error!("{key} No c/inout but multiples: {:?}", all_names);
                    }
                }
                _ => {
                    info!("{key} One c(in|out) {:?}", all_names);
                    pot_swaps.insert(key.clone());
                }
            }

        }
        info!("Cout only swaps: {:?}", pot_swaps);
        pot_swaps
    }

    pub fn re_render(&self, gate_name: &str, combined_map: &HashMap<String,Vec<String>>) {
        let b = vec!["NO_COOL_NAME".to_string()];
        let nice_name = combined_map.get(gate_name).unwrap_or(&b);
        let g = &self.gates.get(gate_name).unwrap().borrow();
        match &g.predecessors {
            WireCombo::Or { a, b } => {
                let aa= a;
                let bb = b;
                let a_name = combined_map.get(a).unwrap_or(&vec![aa.clone()]).to_owned();
                let b_name = combined_map.get(b).unwrap_or(&vec![bb.clone()]).to_owned();
                info!("{gate_name}: ({:?}) {:?} OR {:?}", nice_name, a_name, b_name);
            }
            WireCombo::And { a, b } => {
                let aa= a;
                let bb = b;
                let a_name = combined_map.get(a).unwrap_or(&vec![aa.clone()]).to_owned();
                let b_name = combined_map.get(b).unwrap_or(&vec![bb.clone()]).to_owned();
                info!("{gate_name}: ({:?}) {:?} AND {:?}", nice_name, a_name, b_name);
            }
            WireCombo::Xor { a, b } => {
                let aa= a;
                let bb = b;
                let a_name = combined_map.get(a).unwrap_or(&vec![aa.clone()]).to_owned();
                let b_name = combined_map.get(b).unwrap_or(&vec![bb.clone()]).to_owned();
                info!("{gate_name}: ({:?}) {:?} XOR {:?}", nice_name, a_name, b_name);

            }
            _ => {}
        }


    }


}

fn run_a_check(real_input: &String, swaps: Vec<Vec<&String>>) -> usize {
    let mut d24 = Day24::new(real_input);
    d24.parse();
    for s in swaps.iter() {
        d24.swap(s[0].as_str(), s[1].as_str());
    }


    d24.check_layers();
    // if !d24.correct_output() {
    //     return 12345;
    // }
    let name_map = d24.find_input_xor_and_gates();
    info!("{:#?}", name_map);

    let cin_maps = d24.calculate_cins(&name_map.0);
    info!("{:#?}", cin_maps);

    let interadd_maps = d24.find_intermediate_ands(&cin_maps.0, &name_map.0);
    info!("{:#?}", interadd_maps);

    let cout_maps = d24.find_couts(&interadd_maps.0, &name_map.0);
    info!("{:#?}", cout_maps);

    let combined0 = d24.combine_maps(vec![&name_map.0, &cin_maps.0, &interadd_maps.0, &cout_maps.0]);
    let combined1= d24.combine_maps(vec![&name_map.1, &cin_maps.1, &interadd_maps.1, &cout_maps.1]);

    info!("{:?}", combined0);
    for cmb in combined0.0 {
        info!("{}: {:?}", cmb.0, cmb.1);
    }
    for cmb in combined1.0 {
        info!("{}: {:?}", cmb.0, cmb.1);
    }

    let p_swaps = d24.find_unsafes(&combined0.1);
    let of_concern = cin_maps.2;
    info!("P Swaps: {:?}", p_swaps);
    info!("Z Swaps: {:?}", of_concern);

    let mut total_swaps: Vec<String> = Vec::new();
    for p in p_swaps.iter() {
        if p != "rvh" && p != "z45" {
            total_swaps.push(p.clone())
        }
    }
    for p in of_concern.iter() {
        if p != "rvh" && p != "z45" {
            total_swaps.push(p.clone())
        }
    }

    info!("{} total swap targets: {:?}", total_swaps.len(), total_swaps);

    for s in &total_swaps {
        d24.re_render(s.as_str(), &combined0.1);
    }

    total_swaps.len()
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
    let res = d24.solve_p1().unwrap();
    info!("Answer: {res}");

    let mut d24 = Day24::new(&real_input);
    d24.parse();
    debug!("{:?}", d24);
    d24.single_step();
    let res = d24.solve_p1().unwrap();
    info!("Answer: {res}");

    // d24.get_inputs_per_output_bit();

    let gv = d24.make_graphviz_graph();
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(gv.as_bytes());

    let mut d24 = Day24::new(&real_input);
    d24.parse();
    // d24.swap("jdr", "z31");
    // d24.swap("pgt", "z22");
    // d24.swap("ffj", "z08");
    // d24.swap("gjh", "z22");
    // d24.swap("jdr", "z31");
    d24.swap("ffj", "z08");
    d24.swap("gjh", "z22");
    d24.swap("jdr", "z31");
    d24.swap("dwp", "kfm");
    d24.make_graphviz_graph();
    d24.check_layers();
    d24.correct_output();
    let name_map = d24.find_input_xor_and_gates();
    info!("{:#?}", name_map);

    let cin_maps = d24.calculate_cins(&name_map.0);
    info!("{:#?}", cin_maps);

    let interadd_maps = d24.find_intermediate_ands(&cin_maps.0, &name_map.0);
    info!("{:#?}", interadd_maps);

    let cout_maps = d24.find_couts(&interadd_maps.0, &name_map.0);
    info!("{:#?}", cout_maps);

    let combined0 = d24.combine_maps(vec![&name_map.0, &cin_maps.0, &interadd_maps.0, &cout_maps.0]);
    let combined1= d24.combine_maps(vec![&name_map.1, &cin_maps.1, &interadd_maps.1, &cout_maps.1]);

    info!("{:?}", combined0);
    for cmb in combined0.0 {
        info!("{}: {:?}", cmb.0, cmb.1);
    }
    for cmb in combined1.0 {
        info!("{}: {:?}", cmb.0, cmb.1);
    }

    let p_swaps = d24.find_unsafes(&combined0.1);
    let of_concern = cin_maps.2;
    info!("P Swaps: {:?}", p_swaps);
    info!("Z Swaps: {:?}", of_concern);

    let mut total_swaps: Vec<String> = Vec::new();
    for p in p_swaps.iter() {
        if p != "rvh" && p != "z45" {
            total_swaps.push(p.clone())
        }
    }
    for p in of_concern.iter() {
        if p != "rvh" && p != "z45" {
            total_swaps.push(p.clone())
        }
    }

    info!("{} total swap targets: {:?}", total_swaps.len(), total_swaps);

    for s in total_swaps.iter() {
        d24.re_render(s.as_str(), &combined0.1);
    }
    // d24.calc_value(start);
    let out = d24.correct_output();
    info!("Out: {out}");

    // let mut all_eights = total_swaps.iter().combinations(8);
    // // let all_keys = d24.gates.keys();
    // // let mut all_eights = all_keys.combinations(8);

    // for window in all_eights {
    //     let mut chunks = window.chunks(2);
    //     let chunks: Vec<Vec<&String>> = chunks.map(|x| {
    //         Vec::from(x)
    //     }).collect();
    //     let wrong = run_a_check(&real_input, chunks.clone());
    //     if wrong != 12345{
    //         error!("Window {:?} has {wrong} wrong", window);
    //     }
    //     if wrong == 0 {
    //         return;
    //     }
    // }



}
