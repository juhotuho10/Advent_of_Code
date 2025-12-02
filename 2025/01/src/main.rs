/*
part 1:

we turn a dial left and right and see how many times it hits 0 after we have turned the dial

part 2:

we also have to count how many times it hits 0 during the dial turning

*/

use std::fs::File;
use std::io::{BufRead, BufReader};

struct Dial(i16);

impl Default for Dial {
    fn default() -> Self {
        Dial(50)
    }
}

impl Dial {
    fn turn_1(&mut self, amount: i16) -> u8 {
        let mut zeros: u8 = 0;
        let dir = amount.signum();

        for _ in 0..amount.abs() {
            self.0 += dir;
            if self.0 == -1 {
                self.0 = 99
            } else if self.0 == 100 {
                self.0 = 0
            }
        }

        zeros += (self.0 == 0) as u8;

        zeros
    }

    fn turn_2(&mut self, amount: i16) -> u8 {
        let mut zeros: u8 = 0;
        let dir = amount.signum();

        for _ in 0..amount.abs() {
            self.0 += dir;
            if self.0 == -1 {
                self.0 = 99
            } else if self.0 == 100 {
                self.0 = 0
            }
            zeros += (self.0 == 0) as u8;
        }

        zeros
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

    let example_sum = solution_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 3);

    let my_sum = solution_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 6);
    let my_sum = solution_2(_my_input);

    dbg!(my_sum);
}

fn solution_1(input: &[String]) -> u16 {
    let parsed = parse_input(input);
    let mut count = 0;
    let mut dial = Dial::default();
    for num in parsed {
        count += dial.turn_1(num) as u16;
    }
    count
}

fn solution_2(input: &[String]) -> u16 {
    let parsed = parse_input(input);
    let mut count = 0;
    let mut dial = Dial::default();
    for num in parsed {
        count += dial.turn_2(num) as u16;
    }
    count
}

fn parse_input(input: &[String]) -> Vec<i16> {
    let mut turns = Vec::new();
    for line in input {
        let (dir, count_str) = line.split_at(1);
        let count = {
            let temp_count = count_str.parse::<i16>().unwrap();
            match dir {
                "R" => temp_count,
                "L" => -temp_count,
                _ => unreachable!(),
            }
        };
        turns.push(count);
    }

    turns
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
