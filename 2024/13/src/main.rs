/*
part 1:
we have a claw machine, we control the button with 2 buttons.
it costs 3 tokes to push the A button and 1 token to push the B button (typical EA game)
the buttons move the claw to the right and forward in the y axis

we have to figure out what is the smallest amount of tokens to get to a certain X and Y position
example:
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

part 2:

we had a unit conversion error and all the prices are actually 10000000000000 units higher than they are claimed to be

*/

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct ClawMachine {
    a: [f64; 2],
    b: [f64; 2],
    target: [f64; 2],
}

impl ClawMachine {
    fn from_string_1(input: Vec<&String>) -> Self {
        assert!(input.len() == 3);
        let button_regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
        let target_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

        let a_button_capture = button_regex.captures(input[0]).unwrap();
        let b_button_capture = button_regex.captures(input[1]).unwrap();
        let target_capture = target_regex.captures(input[2]).unwrap();

        ClawMachine {
            a: [
                a_button_capture[1].parse().unwrap(),
                a_button_capture[2].parse().unwrap(),
            ],
            b: [
                b_button_capture[1].parse().unwrap(),
                b_button_capture[2].parse().unwrap(),
            ],
            target: [
                target_capture[1].parse().unwrap(),
                target_capture[2].parse().unwrap(),
            ],
        }
    }

    fn from_string_2(input: Vec<&String>) -> Self {
        assert!(input.len() == 3);
        let button_regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
        let target_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

        let a_button_capture = button_regex.captures(input[0]).unwrap();
        let b_button_capture = button_regex.captures(input[1]).unwrap();
        let target_capture = target_regex.captures(input[2]).unwrap();

        ClawMachine {
            a: [
                a_button_capture[1].parse().unwrap(),
                a_button_capture[2].parse().unwrap(),
            ],
            b: [
                b_button_capture[1].parse().unwrap(),
                b_button_capture[2].parse().unwrap(),
            ],
            target: [
                target_capture[1].parse::<f64>().unwrap() + 10000000000000.0,
                target_capture[2].parse::<f64>().unwrap() + 10000000000000.0,
            ],
        }
    }

    fn solve_machine(&self) -> Option<u64> {
        // solved using cramers rule

        let a = self.a;
        let b = self.b;
        let c = self.target;

        let det = a[0] * b[1] - b[0] * a[1];

        if det.abs() < 1e-6 {
            return None;
        }

        let a_button_presses = (c[0] * b[1] - b[0] * c[1]) / det;
        let b_button_presses = (a[0] * c[1] - c[0] * a[1]) / det;

        // not even button presses
        if a_button_presses.fract().abs() > 0.001 || b_button_presses.fract().abs() > 0.001 {
            return None;
        }

        Some((a_button_presses * 3.0 + b_button_presses) as u64)
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

    let example_sum = button_press_cost_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 480);

    let my_sum = button_press_cost_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let my_sum = button_press_cost_2(_my_input);
    dbg!(my_sum);
}

fn button_press_cost_1(input: &[String]) -> u64 {
    let claw_machines = parse_input_1(input);

    claw_machines
        .iter()
        .filter_map(ClawMachine::solve_machine)
        .sum()
}

fn button_press_cost_2(input: &[String]) -> u64 {
    let claw_machines = parse_input_2(input);

    claw_machines
        .iter()
        .filter_map(ClawMachine::solve_machine)
        .sum()
}

fn parse_input_1(input: &[String]) -> Vec<ClawMachine> {
    let mut claw_machines = vec![];
    let mut claw_machine_strings: Vec<Vec<&String>> = vec![];

    let mut single_claw_machine = vec![];
    for line in input {
        if line.is_empty() {
            claw_machine_strings.push(single_claw_machine.clone());
            single_claw_machine.clear();
        } else {
            single_claw_machine.push(line);
        }
    }
    claw_machine_strings.push(single_claw_machine);

    for claw_machine in claw_machine_strings {
        claw_machines.push(ClawMachine::from_string_1(claw_machine));
    }

    claw_machines
}

fn parse_input_2(input: &[String]) -> Vec<ClawMachine> {
    let mut claw_machines = vec![];
    let mut claw_machine_strings: Vec<Vec<&String>> = vec![];

    let mut single_claw_machine = vec![];
    for line in input {
        if line.is_empty() {
            claw_machine_strings.push(single_claw_machine.clone());
            single_claw_machine.clear();
        } else {
            single_claw_machine.push(line);
        }
    }
    claw_machine_strings.push(single_claw_machine);

    for claw_machine in claw_machine_strings {
        claw_machines.push(ClawMachine::from_string_2(claw_machine));
    }

    claw_machines
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
