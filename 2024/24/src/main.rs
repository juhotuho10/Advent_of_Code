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

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
enum Gate {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
    sus_instruction: FxHashSet<Instruction>,
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
            sus_instruction: FxHashSet::default(),
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
        let bool_values = self.values_to_bool(start_char);

        bool_values
            .iter()
            .fold(0, |acc, bit| (acc << 1) | *bit as u64)
    }

    fn values_to_bool(&self, start_char: char) -> Vec<bool> {
        let mut copy_values = self.values.clone();

        copy_values.retain(|s, _| s.starts_with(start_char));

        let mut key_value_pairs: Vec<(&String, &bool)> = copy_values.iter().collect();

        key_value_pairs.sort_by(|a, b| a.0.cmp(b.0));

        key_value_pairs
            .into_iter()
            .rev()
            .map(|(_, val)| *val)
            .collect()
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

    fn get_sus_instructions(&mut self) {
        let max_num = self.values.len() / 2;
        let mut working_instructions = vec![];

        for current_num in 0..max_num {
            for iter_num in 0..=2 {
                let mut copy_self = self.clone();

                let mut bools = vec![false; max_num + 1];

                copy_self.values.values_mut().for_each(|v| *v = false);

                let key_str_y = format!("y{:02}", current_num);
                let key_str_x = format!("x{:02}", current_num);

                match iter_num {
                    0 => {
                        copy_self.values.insert(key_str_x, true);

                        bools[current_num] = true;
                    }
                    1 => {
                        copy_self.values.insert(key_str_y, true);

                        bools[current_num] = true;
                    }
                    2 => {
                        copy_self.values.insert(key_str_x, true);

                        copy_self.values.insert(key_str_y, true);

                        bools[current_num + 1] = true;
                    }
                    _ => unreachable!(),
                }

                let rev_bools: Vec<bool> = bools.into_iter().rev().collect();

                let mut used_instructions = vec![];

                let mut available_instructions: Vec<Instruction> = vec![];
                loop {
                    copy_self.instructions.retain(|ins| {
                        let all_values: Vec<&String> = copy_self.values.keys().collect();

                        if all_values.contains(&&ins.num1) && all_values.contains(&&ins.num2) {
                            available_instructions.push(ins.clone());
                            false
                        } else {
                            true
                        }
                    });

                    if available_instructions.is_empty() {
                        break;
                    }

                    for curr_ins in &available_instructions {
                        let val_1 = copy_self.values[&curr_ins.num1];
                        let val_2 = copy_self.values[&curr_ins.num2];

                        if val_1 || val_2 {
                            used_instructions.push(curr_ins.clone());
                        }

                        let result = match curr_ins.gate {
                            Gate::AND => val_1 & val_2,
                            Gate::OR => val_1 | val_2,
                            Gate::XOR => val_1 ^ val_2,
                        };

                        copy_self.values.insert(curr_ins.target.clone(), result);
                    }

                    available_instructions.clear();

                    if copy_self.instructions.is_empty() {
                        break;
                    }
                }

                if copy_self.values_to_bool('z') == rev_bools {
                    working_instructions.extend(used_instructions.clone());
                } else {
                    self.sus_instruction.extend(used_instructions.clone());
                }
            }
        }

        self.sus_instruction
            .retain(|ins| !working_instructions.contains(ins));

        dbg!(&self.sus_instruction);

        dbg!(&self.sus_instruction.len());
    }

    fn find_wrong_instruction(&self) -> Vec<Instruction> {
        let mut self_copy = self.clone();

        self_copy
            .instructions
            .retain(|ins| !self_copy.sus_instruction.contains(ins));

        let original_swap_wires = 8;
        let manual_confirmed: Vec<Instruction> = self_copy
            .sus_instruction
            .clone()
            .into_iter()
            .filter(|ins| ["z10", "z32", "z39", "grm", "twr", "ggn"].contains(&ins.target.as_str()))
            .collect();

        self_copy
            .sus_instruction
            .retain(|ins| !manual_confirmed.contains(ins));

        let unsure_swaps = original_swap_wires - manual_confirmed.len();

        let mut counter = 0;
        for mut combination in self
            .sus_instruction
            .clone()
            .into_iter()
            .combinations(unsure_swaps)
        {
            combination.extend(manual_confirmed.clone());

            for permutation in combination.into_iter().permutations(original_swap_wires) {
                dbg!(permutation);
                counter += 1;
            }
        }

        dbg!(counter);

        vec![]
    }
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
    let my_sum = solution_2(_my_input);
    assert_eq!(my_sum, "ggn,grm,jcb,ndw,twr,z10,z32,z39");
    dbg!(my_sum);
}

fn run_circuit_1(input: &[String]) -> u64 {
    let mut all_instructions = AllInstructions::from_string(input);

    all_instructions.process_instructions();

    dbg!(all_instructions.xyz_nums());
    all_instructions.values_to_num('z')
}

fn solution_2(input: &[String]) -> String {
    let mut all_instructions = AllInstructions::from_string(input);

    all_instructions.get_sus_instructions();
    //all_instructions.find_wrong_instruction();
    "".to_owned()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
