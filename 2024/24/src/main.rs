/*
part 1:
we have devices that produce numbers with boolean logic
each gate will wait for both of it's input numbers to be filled and will produce a single output in the circuit

part 2:

instead of adding individual numbers together, we are trying to add numbers togther
bits that start with x are all one number, bits starting with y are all one number
and the output is put on wires that start with z
00 is the least significan bit, then 01 and then 02...

but one problem is that we have 4 pairs of gates where the output wire has been swapped
and we have to find the gates that have been swapped and give them as output alphabetically joined by commas

*/

use fxhash::FxHashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
enum Gate {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone)]
struct Instruction {
    num1: String,
    num2: String,
    target: String,
    gate: Gate,
}

#[derive(Debug, Clone)]
struct AllInstructions {
    values: FxHashMap<String, bool>,
    instructions: Vec<Instruction>,
    sus_instruction: Vec<Instruction>,
    ins_num_1: Vec<String>,
    ins_num_2: Vec<String>,
    ins_target: Vec<String>,
    ins: Vec<Gate>,
}

impl AllInstructions {
    fn from_string(input: &[String]) -> Self {
        let mut num_strings: Vec<&str> = vec![];
        let mut instruction_strings: Vec<&str> = vec![];
        let mut nums_section = true;

        for line in input {
            if line.is_empty() {
                nums_section = false;
                continue;
            }

            if nums_section {
                num_strings.push(line);
            } else {
                instruction_strings.push(line);
            }
        }

        let mut start_values: FxHashMap<String, bool> = FxHashMap::default();
        let mut all_instructions: Vec<Instruction> = vec![];

        for line in num_strings {
            let (id, val) = line.split_once(": ").unwrap();
            let bool_val = val == "1";

            start_values.insert(id.to_string(), bool_val);
        }

        let mut ins_num_1: Vec<String> = vec![];
        let mut ins_num_2: Vec<String> = vec![];
        let mut ins_target: Vec<String> = vec![];
        let mut insstructions: Vec<Gate> = vec![];

        for line in instruction_strings {
            let sections: Vec<&str> = line.split(" ").collect();
            let inst = match sections[1] {
                "AND" => Gate::AND,
                "OR" => Gate::OR,
                "XOR" => Gate::XOR,
                _ => unreachable!(),
            };

            ins_num_1.push(sections[0].to_owned());
            ins_num_2.push(sections[2].to_owned());
            ins_target.push(sections[4].to_owned());
            insstructions.push(inst);

            let new_ins = Instruction {
                num1: sections[0].to_owned(),
                num2: sections[2].to_owned(),
                target: sections[4].to_owned(),
                gate: inst,
            };

            all_instructions.push(new_ins);
        }

        AllInstructions {
            values: start_values,
            instructions: all_instructions,
            sus_instruction: vec![],
            ins_num_1,
            ins_num_2,
            ins_target,
            ins: insstructions,
        }
    }

    fn process_instructions(&mut self) {
        let mut available_instructions: Vec<Instruction> = vec![];
        loop {
            self.instructions.retain(|ins| {
                let all_values: Vec<&String> = self.values.keys().collect();

                if all_values.contains(&&ins.num1) && all_values.contains(&&ins.num2) {
                    available_instructions.push(ins.clone());
                    false
                } else {
                    true
                }
            });

            for ins in &available_instructions {
                let val_1 = self.values[&ins.num1];
                let val_2 = self.values[&ins.num2];

                let result = match ins.gate {
                    Gate::AND => val_1 & val_2,
                    Gate::OR => val_1 | val_2,
                    Gate::XOR => val_1 ^ val_2,
                };

                self.values.insert(ins.target.clone(), result);
            }

            if self.instructions.is_empty() {
                break;
            }
        }
    }

    fn values_to_num(&self, start_char: char) -> u64 {
        let mut copy_values = self.values.clone();

        copy_values.retain(|s, _| s.starts_with(start_char));

        let mut key_value_pairs: Vec<(&String, &bool)> = copy_values.iter().collect();

        key_value_pairs.sort_by(|a, b| a.0.cmp(b.0));

        key_value_pairs
            .iter()
            .rev()
            .map(|(_, val)| **val)
            .fold(0, |acc, bit| (acc << 1) | bit as u64)
    }

    fn xyz_nums(&self) -> (u64, u64, u64) {
        let mut key_value_pairs: Vec<(&String, &bool)> = self.values.iter().collect();

        key_value_pairs.sort_by(|a, b| a.0.cmp(b.0));

        let mut xyz_nums = vec![];

        for start_char in ['x', 'y', 'z'] {
            let mut clone_values = key_value_pairs.clone();
            clone_values.retain(|(s, _)| s.starts_with(start_char));

            let num = clone_values
                .iter()
                .rev()
                .map(|(_, val)| **val)
                .fold(0, |acc, bit| (acc << 1) | bit as u64);

            xyz_nums.push(num);
        }

        (xyz_nums[0], xyz_nums[1], xyz_nums[2])
    }

    fn get_sus_instructions(&mut self) {}
}

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_sum = run_circuit_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 2024);

    let start = Instant::now();
    let my_sum = run_circuit_1(_my_input);
    dbg!(start.elapsed());

    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 0);

    let my_sum = solution_2(_my_input);
    dbg!(my_sum);
}

fn run_circuit_1(input: &[String]) -> u64 {
    let mut all_instructions = AllInstructions::from_string(input);

    all_instructions.process_instructions();

    dbg!(all_instructions.xyz_nums());
    all_instructions.values_to_num('z')
}

fn solution_2(input: &[String]) -> u32 {
    let all_instructions = AllInstructions::from_string(input);
    0
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
