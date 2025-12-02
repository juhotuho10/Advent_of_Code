/*
part 1:
we have to check a range to see if there are any numbers that are made up of 2 repeating number sequences, for example 1212 is invalid

part 2:

number that consists of repeating sub numbers is also invalid, so 123123123123 is invalid

*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

struct IDRange(RangeInclusive<u64>);

impl IDRange {
    fn new(lower: u64, upper: u64) -> Self {
        IDRange(RangeInclusive::new(lower, upper))
    }
    fn get_invalid_count_1(self) -> u64 {
        let mut count = 0;
        for num in self.0 {
            let num_str = num.to_string();
            if num_str.len() % 2 != 0 {
                continue;
            }

            let half_len = num_str.len() / 2;

            if num_str[..half_len] == num_str[half_len..] {
                count += num;
            }
        }
        count
    }

    fn get_invalid_count_2(self) -> u64 {
        let mut count = 0;
        for num in self.0 {
            let num_str = num.to_string();
            let num_bytes = num_str.as_bytes();
            let half_len = num_bytes.len() / 2;

            for sub_range in 1..=half_len {
                if num_bytes.len() % sub_range != 0 {
                    continue;
                }

                let repeating: &[u8] = &num_bytes[..sub_range];

                let matches = num_bytes
                    .chunks(sub_range)
                    .skip(1)
                    .all(|window| window == repeating);

                if matches {
                    count += num;
                    break;
                }
            }
        }
        count
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
    assert_eq!(example_sum, 1227775554);

    let my_sum = solution_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 4174379265);

    let my_sum = solution_2(_my_input);
    dbg!(my_sum);
}

fn solution_1(input: &[String]) -> u64 {
    let parsed = parse_input(input);
    let mut count = 0;
    for range in parsed {
        count += range.get_invalid_count_1();
    }
    count
}

fn solution_2(input: &[String]) -> u64 {
    let parsed = parse_input(input);
    let mut count = 0;
    for range in parsed {
        count += range.get_invalid_count_2();
    }
    count
}

fn parse_input(input: &[String]) -> Vec<IDRange> {
    let ranges: Vec<&str> = input[0].split(",").collect();
    let mut id_ranges = Vec::new();
    for range in ranges {
        let (lower_str, upper_str) = range.split_once("-").unwrap();
        let lower: u64 = lower_str.parse().unwrap();
        let upper: u64 = upper_str.parse().unwrap();

        id_ranges.push(IDRange::new(lower, upper));
    }

    id_ranges
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
