/*
part 1:
we are solving cephalapods math homework with a list of problems, we need to add or multiply a group of numbers together
the biggest problem is parsing the problem correctly

part 2:
we need to parse the problems differently, 1 column at a time

*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum MathProblem {
    Add(Vec<u16>),
    Multiply(Vec<u16>),
}

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_sum = solution_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 4277556);

    let my_sum = solution_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 3263827);

    let my_sum = solution_2(_my_input);
    dbg!(my_sum);
}

fn solution_1(input: &[String]) -> u64 {
    let math_problems = parse_input_1(input);

    math_problems
        .into_iter()
        .map(|problem| match problem {
            MathProblem::Add(items) => items.into_iter().map(|i| i as u64).sum::<u64>(),
            MathProblem::Multiply(items) => items.into_iter().map(|i| i as u64).product(),
        })
        .sum()
}

fn solution_2(input: &[String]) -> u64 {
    let math_problems = parse_input_2(input);

    math_problems
        .into_iter()
        .map(|problem| match problem {
            MathProblem::Add(items) => items.into_iter().map(|i| i as u64).sum::<u64>(),
            MathProblem::Multiply(items) => items.into_iter().map(|i| i as u64).product(),
        })
        .sum()
}

fn parse_input_1(input: &[String]) -> Vec<MathProblem> {
    let mut rows = Vec::new();
    for row in input {
        let split_row: Vec<&str> = row.split(" ").filter(|r| !r.is_empty()).collect();
        rows.push(split_row);
    }

    let row_len = rows[0].len();

    let mut parsed = Vec::new();

    for i in 0..row_len {
        let mut nums = Vec::new();
        for row in &rows {
            let chars = row[i];
            match chars.parse::<u16>() {
                Ok(num) => nums.push(num),
                Err(_) => match chars {
                    "*" => {
                        parsed.push(MathProblem::Multiply(nums));
                        break;
                    }
                    "+" => {
                        parsed.push(MathProblem::Add(nums));
                        break;
                    }
                    _ => unreachable!(),
                },
            }
        }
    }

    parsed
}

fn parse_input_2(input: &[String]) -> Vec<MathProblem> {
    let row_len = input[0].len();

    let mut math_problems = Vec::new();
    let mut nums: Vec<u16> = Vec::new();
    let mut operator: Option<char> = None;

    for i in 0..row_len {
        let row_chars: Vec<char> = input
            .iter()
            .map(|row| row.chars().nth(i).unwrap())
            .filter(|c| *c != ' ')
            .collect();

        if row_chars.iter().all(|c| *c == ' ') {
            match operator.unwrap() {
                '*' => {
                    math_problems.push(MathProblem::Multiply(nums.clone()));
                }
                '+' => {
                    math_problems.push(MathProblem::Add(nums.clone()));
                }
                _ => unreachable!(),
            }

            operator = None;
            nums.clear();
        } else {
            let contains_op = row_chars.iter().any(|c| *c == '+' || *c == '*');
            if contains_op {
                let (op, num_chars) = row_chars.split_last().unwrap();
                operator = Some(*op);
                let num_str: String = num_chars.iter().cloned().collect();
                nums.push(num_str.parse().unwrap());
            } else {
                let num_str: String = row_chars.iter().cloned().collect();
                nums.push(num_str.parse().unwrap());
            }
        }
    }

    match operator.unwrap() {
        '*' => {
            math_problems.push(MathProblem::Multiply(nums.clone()));
        }
        '+' => {
            math_problems.push(MathProblem::Add(nums.clone()));
        }
        _ => unreachable!(),
    }

    math_problems
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
