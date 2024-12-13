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

use ndarray::{arr1, stack, Array1, Array2, Axis};
use ndarray_linalg::Inverse;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug)]
struct ClawMachine {
    a: Array1<f64>,
    b: Array1<f64>,
    target: Array1<f64>,
}

impl ClawMachine {
    fn from_string(input: Vec<&String>, add_on_num: f64, regex_string: &[&Regex]) -> Self {
        assert!(input.len() == 3);

        let a_button_capture = regex_string[0].captures(input[0]).unwrap();
        let b_button_capture = regex_string[0].captures(input[1]).unwrap();
        let target_capture = regex_string[1].captures(input[2]).unwrap();

        ClawMachine {
            a: arr1(&[
                a_button_capture[1].parse::<f64>().unwrap(),
                a_button_capture[2].parse::<f64>().unwrap(),
            ]),
            b: arr1(&[
                b_button_capture[1].parse().unwrap(),
                b_button_capture[2].parse().unwrap(),
            ]),
            target: arr1(&[
                target_capture[1].parse::<f64>().unwrap() + add_on_num,
                target_capture[2].parse::<f64>().unwrap() + add_on_num,
            ]),
        }
    }

    fn solve_machine(&self) -> Option<u64> {
        // [A B] * [X Y] = [Xn Yn]
        // [A B] = [Xn Yn] * [X Y].inv()

        let multiply: Array1<f64> = arr1(&[3.0, 1.0]);
        let matrix: Array2<f64> = stack![Axis(0), self.a, self.b];

        match matrix.inv() {
            Ok(inverse) => {
                let product = self.target.dot(&inverse);

                if !product.iter().all(|&val| (val - val.round()).abs() < 0.001) {
                    return None;
                }

                let result = product.dot(&multiply);

                Some(result.round() as u64)
            }
            Err(_) => None,
        }
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

    let start = Instant::now();
    let my_sum = button_press_cost_2(_my_input);
    dbg!(start.elapsed());
    dbg!(my_sum);
}

fn button_press_cost_1(input: &[String]) -> u64 {
    let claw_machines = parse_input(input, 0.0);

    claw_machines
        .iter()
        .filter_map(ClawMachine::solve_machine)
        .sum()
}

fn button_press_cost_2(input: &[String]) -> u64 {
    let claw_machines = parse_input(input, 10000000000000.0);

    claw_machines
        .iter()
        .filter_map(ClawMachine::solve_machine)
        .sum()
}

fn parse_input(input: &[String], add_on_num: f64) -> Vec<ClawMachine> {
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

    let button_regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let target_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    for claw_machine in claw_machine_strings {
        claw_machines.push(ClawMachine::from_string(
            claw_machine,
            add_on_num,
            &[&button_regex, &target_regex],
        ));
    }

    claw_machines
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
