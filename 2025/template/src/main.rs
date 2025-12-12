/*
part 1:

part 2:

*/

#![allow(clippy::ptr_arg)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_1 = solution_1(&example_1);
    dbg!(&example_1);
    assert_eq!(example_1, 0);

    let start = Instant::now();
    let solution_1 = solution_1(_my_input);
    let elapsed = start.elapsed().as_micros();
    println!("Time to solve problem 1: {elapsed}µs");
    dbg!(solution_1);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_2 = solution_2(&example_2);
    dbg!(&example_2);
    assert_eq!(example_2, 0);

    let start = Instant::now();
    let solution_2 = solution_2(_my_input);
    let elapsed = start.elapsed().as_micros();
    println!("Time to solve problem 2: {elapsed}µs");
    dbg!(solution_2);
}

fn solution_1(input: &[String]) -> u64 {
    let parsed = parse_input(input);
    0
}

fn solution_2(input: &[String]) -> u64 {
    let parsed = parse_input(input);
    0
}

fn parse_input(input: &[String]) {}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
