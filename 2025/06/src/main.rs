/*
part 1:
we are solving cephalapods math homework with a list of problems, we need to add or multiply a group of numbers together
the biggest problem is parsing the problem correctly

part 2:
we need to parse the problems differently, 1 column at a time

*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

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

    let start = Instant::now();
    let my_sum = solution_1(_my_input);
    let elapsed = start.elapsed().as_micros();
    println!("elapsed: {elapsed}");
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 3263827);

    let start = Instant::now();
    let my_sum = solution_2(_my_input);
    let elapsed = start.elapsed().as_micros();
    println!("elapsed: {elapsed}");
    dbg!(my_sum);
}

fn solution_1(input: &[String]) -> u64 {
    let math_nums = parse_input_1(input);

    math_nums.into_iter().sum()
}

fn solution_2(input: &[String]) -> u64 {
    let math_nums = parse_input_2(input);

    math_nums.into_iter().sum()
}

fn parse_input_1(input: &[String]) -> Vec<u64> {
    let mut rows = Vec::new();
    for row in input {
        let split_row: Vec<&str> = row.split(" ").filter(|r| !r.is_empty()).collect();
        rows.push(split_row);
    }

    let (operators, nums_rows) = rows.split_last().unwrap();

    let row_len = nums_rows[0].len();

    let mut parsed = Vec::new();

    for i in 0..row_len {
        let mut nums = Vec::new();
        for row in nums_rows {
            let chars = row[i];
            nums.push(chars.parse::<u64>().unwrap());
        }

        match operators[i] {
            "*" => {
                parsed.push(nums.iter().product());
            }
            "+" => {
                parsed.push(nums.iter().sum());
            }
            _ => unreachable!(),
        }
    }

    parsed
}

fn parse_input_2(input: &[String]) -> Vec<u64> {
    let (operator_string, nums_rows) = input.split_last().unwrap();
    let mut operator_iter = operator_string.chars().filter(|c| *c != ' ');

    let row_len = nums_rows[0].len();

    let mut problem_nums = Vec::new();
    let mut nums: Vec<u64> = Vec::new();
    let mut row_chars: Vec<char>;

    assert!(nums_rows.iter().all(|row| row.len() == row_len));

    let mut row_iterators: Vec<_> = nums_rows.iter().map(|row| row.chars()).collect();

    for _ in 0..row_len {
        row_chars = row_iterators
            .iter_mut()
            .filter_map(|iter| {
                let next = iter.next().unwrap();
                if next == ' ' { None } else { Some(next) }
            })
            .collect();

        if row_chars.is_empty() {
            match operator_iter.next().unwrap() {
                '*' => {
                    problem_nums.push(nums.iter().product());
                }
                '+' => {
                    problem_nums.push(nums.iter().sum());
                }
                _ => unreachable!(),
            }

            nums.clear();
        } else {
            let num_str: String = row_chars.iter().cloned().collect();
            nums.push(num_str.parse().unwrap());
        }
    }

    match operator_iter.next().unwrap() {
        '*' => {
            problem_nums.push(nums.iter().product());
        }
        '+' => {
            problem_nums.push(nums.iter().sum());
        }
        _ => unreachable!(),
    }

    problem_nums
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
