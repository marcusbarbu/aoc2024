use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};
use aoc2024::{map_vec_extend::append_to_hash_map, AocHelper, AocHelperError, AocResult, RequestedAocInputType};
use itertools::all;
use regex::Regex;
use tracing::{debug, error, info};

const COMBINATION_REGEX_STR: &str = r"(\w+)\s*(AND|OR|XOR)\s*(\w+)\s*->\s*(\w+)";

#[derive(Debug, Clone, PartialEq)]
enum GateType {
    Input,
    Xor,
    And,
    Or
}
#[derive(Debug, Clone, PartialEq)]
struct Wire {
    original_name: String,
    gate_type: GateType,
    input_names: Vec<String>,
}

#[derive(Debug)]
struct Day24Part2 {
    raw: String,
    all_gates: HashMap<String, Wire>,
    inputs: Vec<(String, Wire)>,
    outputs: Vec<(String, Wire)>,
    gates_from_input: HashMap<String, Vec<Wire>>,
    swaps: Vec<(String, String)>,
    touched_gate_names: HashSet<String>,
}

impl Day24Part2 {
    pub fn new(s: &String) -> Self {
        Self {
            raw:  s.clone(),
            all_gates: HashMap::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            gates_from_input: HashMap::new(),
            swaps: Vec::new(),
            touched_gate_names: HashSet::new(),
        }
    }

    pub fn swap(&mut self, a: &str, b: &str) {
        self.swaps.push((a.to_string(), b.to_string()));
    }

    pub fn get_name(&self, name: &str) -> String {
        if let Some(new_name) = self.swaps.iter().find_map(|(a, b)| {
            if a == name {
                return Some(b.to_string());
            }
            else if b == name {
                return Some(a.to_string())
            }
            None
        }) {
            return new_name;
        }

        return name.to_string();
    }


    // will trim input to ignore reg start states for part b
    pub fn parse(&mut self) {
        let gates = self.raw.clone();
        let gate_regex = Regex::new(COMBINATION_REGEX_STR).unwrap();
        for line in gates.lines() {
            let Some(caps) = gate_regex.captures(line) else {
                continue;
            };
            let (full, [a, op, b, out]) = caps.extract();
            debug!("Full: {full} a: {a} op: {op} b: {b} out: {out}");
            let out = self.get_name(out);
            let gt: GateType = match op {
                "AND" => {
                    GateType::And
                }
                "OR" => {
                    GateType::Or
                }
                "XOR" => {
                    GateType::Xor
                }
                _ => {
                    panic!("Bad line: {line}");
                }
            };
            let wire = Wire{
                original_name: out.to_string(),
                gate_type: gt,
                input_names: vec![a.to_string(), b.to_string()]
            };
            append_to_hash_map(&mut self.gates_from_input, a.to_string(), wire.clone());
            append_to_hash_map(&mut self.gates_from_input, b.to_string(), wire.clone());

            self.all_gates.insert(out.to_string(), wire.clone());
            if a.starts_with("x") || b.starts_with("x") || a.starts_with("y") || b.starts_with("y") {
                self.inputs.push((out.to_string(), wire.clone()));
            }
            if out.starts_with("z") {
                self.outputs.push((out.to_string(), wire));
            }
            self.outputs.sort_by(|a, b|  {
                a.0.cmp(&b.0)
            });
        }
        
    }

    fn get_input_gates(&self, w: &Wire) -> Vec<&Wire> {
        let mut out: Vec<&Wire> = Vec::new();
        for name in w.input_names.iter() {
            out.push(self.all_gates.get(name).unwrap());
        }
        out
    }

    pub fn find_gate_by_input_and_type(&self, name: &String, gt: GateType) -> Option<Wire> {
        debug!("Looking for a {:?} gate with an input named {name}", gt);
        let all = self.gates_from_input.get(name).unwrap();
        for a in all {
            debug!("\tWire: {:?}", a);
            if a.gate_type == gt {
                return Some(a.clone());
            }
        }

        None
    }

    pub fn check_direct_input_matches(&self, x: &String, y: &String, gt: GateType) -> Option<Wire> {
        let x_gate= self.find_gate_by_input_and_type(x, gt.clone()).unwrap();
        let y_gate= self.find_gate_by_input_and_type(y, gt.clone()).unwrap();

        if x_gate != y_gate {
            error!("x and y do not match! {:?} != {:?}", x_gate, y_gate);
            return None;
        }
        Some(x_gate.clone())
    }



    pub fn compare_gates_to_ideal(&self, idx: usize, cin_prev_name: &String) -> AocResult<Option<String>> {
        info!("Working on idx {idx}, cin should be {cin_prev_name}");
        let correct = self.make_an_adder(idx);
        let mut ideal_name_real_name_map: HashMap<String, String> = HashMap::new();
        let mut real_name_ideal_name_map: HashMap<String, String> = HashMap::new();

        let x_input_name =format!("x{:0>2}", idx);
        let y_input_name =format!("y{:0>2}", idx);
        let xor_name = format!("x_y_xor_{:0>2}", idx);
        let and_name = format!("x_y_and_{:0>2}", idx);
        let out_name = format!("z{:0>2}", idx);
        let int_and_name = format!("int_and_{:0>2}", idx);
        let cin_name = format!("cin_{:0>2}", idx);
        let cout_name = format!("cout_{:0>2}", idx);

        let rname_rc: Rc<RefCell<HashMap<String, String>>> = Rc::new(RefCell::new(real_name_ideal_name_map));
        let id_name_rc: Rc<RefCell<HashMap<String, String>>> = Rc::new(RefCell::new(ideal_name_real_name_map));
        let get_gate_ideal_input_name = |name: &String, gt| {
            let map_ref = id_name_rc.borrow();
            let real_name = map_ref.get(name).unwrap();
            self.find_gate_by_input_and_type(real_name, gt)
        };

        let insert_name = |ideal_name: String, orig_name: String | {
            let mut ideal_map_ref = id_name_rc.borrow_mut();
            let mut rev_map_ref = rname_rc.borrow_mut();
            ideal_map_ref.insert(ideal_name.clone(), orig_name.clone());
            rev_map_ref.insert(orig_name.clone(), ideal_name.clone());
        };


        // check that the two inputs map into the same XOR
        if let Some(gate) = self.check_direct_input_matches(&x_input_name, &y_input_name, GateType::Xor) {
            // ideal_name_real_name_map.insert(xor_name.clone(), gate.original_name.clone());
            // real_name_ideal_name_map.insert(gate.original_name.clone(),xor_name.clone());
            insert_name(xor_name.clone(), gate.original_name.clone());
        }
        else {
            error!("XOR mismatch");
            return Ok(None);
        }
        
        // check that the two inputs map into the same AND
        if let Some(gate) = self.check_direct_input_matches(&x_input_name, &y_input_name, GateType::And) {
            // ideal_name_real_name_map.insert(and_name.clone(), gate.original_name.clone());
            // real_name_ideal_name_map.insert(gate.original_name.clone(),and_name.clone());
            insert_name(and_name.clone(), gate.original_name.clone());
        }
        else {
            error!("AND mismatch");
            return Ok(None);
        }


        // find one xor gate and one and gate that use the first XOR as an input
        // self.find_gate_by_input_and_type(name, gt)
        let output_gate = get_gate_ideal_input_name(&xor_name, GateType::Xor);
        let int_and = get_gate_ideal_input_name(&xor_name, GateType::And);

        if output_gate.is_some() {
            if output_gate.clone().unwrap().original_name != out_name {
                error!("Output {:?} should have name {out_name}", output_gate.unwrap());
                return Ok(None);
            }
        }
        else {
            error!("Could not find an output gate!");
            return Ok(None);
        }

        let out_in_names = output_gate.unwrap().input_names;
        if out_in_names.iter().find(|name| {*name == cin_prev_name}).is_some() {
            info!("CIN {idx} Looks good!")
        }
        else {
            error!("CIN {idx} is wrong");
            return Ok(None);
        }

        if int_and.is_some() {
            let int_and = int_and.unwrap();
            // ideal_name_real_name_map.insert(int_and_name.clone(), int_and.original_name.clone());
            // real_name_ideal_name_map.insert(int_and.original_name.clone(),int_and_name.clone());
            insert_name(int_and_name.clone(), int_and.original_name.clone());
        }
        else {
            error!("INT_AND doesn't exist for {idx}");
            return Ok(None);
        }

        // build COUT here
        let cout = get_gate_ideal_input_name(&and_name, GateType::Or);
        if cout.clone().is_some(){
            let cc = cout.clone().unwrap();
            // ideal_name_real_name_map.insert(cout_name.clone(), cc.original_name.clone());
            // real_name_ideal_name_map.insert(cc.original_name.clone(),cout_name.clone());
            insert_name(cout_name.clone(), cc.original_name.clone());
        }
        else {
            error!("couldn't build Cout");
            return Ok(None);
        };

        Ok(Some(cout.unwrap().original_name))

    }

    pub fn make_an_adder(&self, idx: usize) -> HashMap<String, Wire> {
        let mut names_to_gates_local: HashMap<String, Wire> = HashMap::new();
        let x_input_name =format!("x{:0>2}", idx);
        let y_input_name =format!("y{:0>2}", idx);
        let xor_name = format!("x_y_xor_{:0>2}", idx);
        let and_name = format!("x_y_and_{:0>2}", idx);
        let out_name = format!("z{:0>2}", idx);
        let int_and_name = format!("int_and_{:0>2}", idx);
        let cin_name = format!("cin_{:0>2}", idx);
        let cout_name = format!("cout_{:0>2}", idx);

        let first_xor = Wire{
            original_name: xor_name.clone(),
            gate_type: GateType::Xor,
            input_names: vec![x_input_name.clone(), y_input_name.clone()]
        };
        names_to_gates_local.insert(xor_name.clone(), first_xor);

        let first_and = Wire{
            original_name: and_name.clone(),
            gate_type: GateType::And,
            input_names: vec![x_input_name.clone(), y_input_name.clone()]
        };
        names_to_gates_local.insert(and_name.clone(), first_and);

        let single_bit_output = Wire{
            original_name: out_name.clone(),
            gate_type: GateType::Xor,
            input_names: vec![xor_name.clone(), cin_name.clone()]
        };
        names_to_gates_local.insert(out_name.clone(), single_bit_output);

        let internal_and_for_cout = Wire {
            original_name: int_and_name.clone(),
            gate_type: GateType::And,
            input_names: vec![xor_name.clone(), cin_name.clone()]
        };
        names_to_gates_local.insert(int_and_name.clone(), internal_and_for_cout);
        
        let cout = Wire {
            original_name: cout_name.clone(),
            gate_type: GateType::Or,
            input_names: vec![and_name.clone(), int_and_name.clone()]
        };
        names_to_gates_local.insert(cout_name.clone(), cout);
        debug!("Built adder at idx: {idx}\n{:?}", names_to_gates_local);

        names_to_gates_local
    }

    pub fn unusual_xors(&self) -> Vec<String> {
        let mut out = Vec::new();
        out = self.all_gates.iter().filter_map(|(name, gate)| {
            if gate.gate_type == GateType::Xor && !name.starts_with("z"){
                if !gate.input_names.iter().find(|x| x.starts_with("x") || x.starts_with("y")).is_some(){
                    return Some(name.to_string());
                }
            }
            None
        }).collect();

        out
    }

    pub fn render_swap_list(&self) -> String {
        let mut out = "".to_string();
        let mut flat_vec: Vec<String> = Vec::new();
        for (a, b) in self.swaps.iter() {
            flat_vec.push(a.clone());
            flat_vec.push(b.clone());
        }

        flat_vec.sort();
        out = flat_vec.join(",");

        out
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

    let mut d24b = Day24Part2::new(&real_input);
    d24b.swap("ffj", "z08");
    d24b.swap("gjh", "z22");
    d24b.swap("jdr", "z31");
    d24b.swap("dwp", "kfm");
    d24b.parse();
    debug!("{:#?}", d24b.outputs);
    let uxor = d24b.unusual_xors();
    info!("Out of place xors: {:?}", uxor);
    d24b.make_an_adder(1);
    // return;

    let mut next_cin = "rvh".to_string();
    for i in 1..44 {
        let res= d24b.compare_gates_to_ideal(i, &next_cin);
        match res {
            Ok(ans) => {
                match ans {
                    Some(out) => {
                        next_cin = out;
                    }
                    None => {}
                }
            }
            Err(_) => {}
        }
    }

    let ans = d24b.render_swap_list();
    info!("Finally!! {ans}");
    
}
