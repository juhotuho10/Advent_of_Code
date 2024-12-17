/*
part 1:
we have a 3 bit computer with registers from a to c.
teh computer has 8 instructions, each identified by one of the 3bit numbers opcode.

we have a set of numbers which are instruction pointers.
operands 0 - 3 represent the value itself
3-6 represent registers a - c
7 is reserved and wont appear in a valid program

the
opcode number opcode number opcode number...

opcodes:
0 - Division operator with register A, the denominator is 2 to the power of the number, written in A register
1 - bitwise xorwith register B and stores result in register B
2 - number modulo 8 stored to the B register
3 - does nothing if A is 0, but if A is non-zero, we take the number in A to be the instruction and executes the instruction
4 - bitwise cor with register B and register C and store result in register B, number ignored
5 - calculates number mod 8 then outputs the number
6 - works like opcode 0 but result is read from A and stored in B
7 - works like opcode 0 but result is read from A and stored in C

we have a program out, joined by commas, like:
4,6,3,5,6,3,5,2,1,0

we have to join the number into a single num and use that result as the answer

part 2:

we have to find a initial value for register A so that the program output will be a copy the starting program

*/
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    op_pointer: usize,
    porgram: Vec<u8>,
    program_output: Vec<u8>,
}

impl Computer {
    fn from_string(input: &[String]) -> Self {
        let reg_a_string = input[0].splitn(3, " ").collect::<Vec<&str>>()[2];
        let reg_b_string = input[1].splitn(3, " ").collect::<Vec<&str>>()[2];
        let reg_c_string = input[2].splitn(3, " ").collect::<Vec<&str>>()[2];

        let reg_a = reg_a_string.parse::<i64>().unwrap();
        let reg_b = reg_b_string.parse::<i64>().unwrap();
        let reg_c = reg_c_string.parse::<i64>().unwrap();

        let (_, program_string) = input[4].split_once(" ").unwrap();

        let program_nums: Vec<&str> = program_string.split(",").collect();

        let operations: Vec<u8> = program_nums
            .iter()
            .map(|num| num.parse().unwrap())
            .collect();

        Computer {
            reg_a,
            reg_b,
            reg_c,
            op_pointer: 0,
            porgram: operations,
            program_output: vec![],
        }
    }

    fn run_all_operations(&mut self) -> Vec<u8> {
        loop {
            let result = self.run_operation();

            match result {
                Some(bool) => {
                    if bool {
                        self.op_pointer += 2;
                    }
                }
                None => return self.program_output.clone(),
            }
        }
    }

    fn get_combo_num_value(&self, num: &u8) -> Option<i64> {
        match num {
            0..=3 => Some(*num as i64),
            4 => Some(self.reg_a),
            5 => Some(self.reg_b),
            6 => Some(self.reg_c),
            7 => None,
            _ => unreachable!(),
        }
    }

    fn run_operation(&mut self) -> Option<bool> {
        let op_num = self.porgram.get(self.op_pointer);
        let num = self.porgram.get(self.op_pointer + 1);

        match (op_num, num) {
            (Some(op_num), Some(num)) => match op_num {
                0 => {
                    let combo_num = self.get_combo_num_value(num)?;
                    self.reg_a /= i64::pow(2, combo_num as u32)
                }
                1 => self.reg_b ^= *num as i64,

                2 => {
                    let combo_num = self.get_combo_num_value(num)?;
                    self.reg_b = combo_num % 8
                }
                3 => {
                    if self.reg_a == 0 {
                        return Some(true);
                    }
                    self.op_pointer = *num as usize;
                    return Some(false);
                }
                4 => self.reg_b ^= self.reg_c,
                5 => {
                    let combo_num = self.get_combo_num_value(num)?;
                    self.program_output.push((combo_num % 8) as u8)
                }
                6 => {
                    let combo_num = self.get_combo_num_value(num)?;
                    self.reg_b = self.reg_a / i64::pow(2, combo_num as u32)
                }
                7 => {
                    let combo_num = self.get_combo_num_value(num)?;
                    self.reg_c = self.reg_a / i64::pow(2, combo_num as u32)
                }
                _ => unimplemented!(),
            },
            (_, _) => return None,
        }

        Some(true)
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

    let example_output = program_output_1(&example_1);
    dbg!(&example_output);
    assert_eq!(example_output, "4,6,3,5,6,3,5,2,1,0");

    let my_output = program_output_1(_my_input);
    dbg!(my_output);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_output = program_output_2(&example_2);
    dbg!(&example_output);
    assert_eq!(example_output, 117440);

    let my_output = program_output_2(_my_input);
    dbg!(my_output);
}

fn program_output_1(input: &[String]) -> String {
    let mut computer = Computer::from_string(input);

    let output = computer.run_all_operations();

    output
        .into_iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn program_output_2(input: &[String]) -> i64 {
    let computer = Computer::from_string(input);

    let mut valid_nums = vec![];

    recursive_num_search(
        computer.clone(),
        0,
        &mut valid_nums,
        (computer.porgram.len() - 1) as u32,
    );

    dbg!(&valid_nums);

    valid_nums.into_iter().min().unwrap()
}

fn recursive_num_search(
    computer: Computer,
    total: i64,
    valid_nums: &mut Vec<i64>,
    current_pow: u32,
) {
    for i in 0..=7 {
        let start = total + pow_8(current_pow) * i;

        let mut clone_computer = computer.clone();
        clone_computer.reg_a = start;
        let current_output: Vec<u8> = clone_computer.run_all_operations();

        if current_output.len() != computer.porgram.len() {
            continue;
        }
        if current_output == clone_computer.porgram {
            valid_nums.push(start);
        }
        if current_pow == 0 {
            continue;
        }

        if current_output[(current_pow) as usize] == clone_computer.porgram[(current_pow) as usize]
        {
            recursive_num_search(computer.clone(), start, valid_nums, current_pow - 1);
        }
    }
}

fn pow_8(pow: u32) -> i64 {
    i64::pow(8, pow)
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
