/*
part 1:
we have to parse an input to find all the mul(x,y)
then do the multiplications and add the total

part 2:
we have additional instructions,
do() enables future multiplications
and don't() disables future multiplications

we have multiplications enabled, until we hit a don't() then we ignore them until we hit a do()

*/

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_sum = find_multiply_sum_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 161);

    let my_sum = find_multiply_sum_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = find_multiply_sum_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 48);

    let my_sum = find_multiply_sum_2(_my_input);
    dbg!(my_sum);
}

fn find_multiply_sum_1(input: &[String]) -> u32 {
    let parsed = parse_input_1(input);

    parsed.iter().map(|nums| nums.0 * nums.1).sum()
}

fn find_multiply_sum_2(input: &[String]) -> u32 {
    let parsed = parse_input_2(input);

    parsed.iter().map(|nums| nums.0 * nums.1).sum()
}

fn parse_input_1(input: &[String]) -> Vec<(u32, u32)> {
    let text = input.join("");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut nums = vec![];

    for captures in re.captures_iter(&text) {
        let num_1 = captures[1].parse::<u32>().unwrap();
        let num_2 = captures[2].parse::<u32>().unwrap();

        nums.push((num_1, num_2));
    }

    nums
}

fn parse_input_2(input: &[String]) -> Vec<(u32, u32)> {
    let text = input.join("");
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don\'t\(\)").unwrap();

    let mut nums = vec![];
    let mut multiply = true;

    for captures in re.captures_iter(&text) {
        dbg!(&captures[0]);
        match &captures[0] {
            "do()" => multiply = true,
            "don't()" => multiply = false,
            _ => {
                if multiply {
                    let num_1 = captures[1].parse::<u32>().unwrap();
                    let num_2 = captures[2].parse::<u32>().unwrap();
                    nums.push((num_1, num_2));
                }
            }
        }
    }

    nums
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
