/*
part 1:
we have banks of batteries, we only activate 2 of them to get the highest joltage per bank and add them together
part 2:
we now instead also get the highest joltage of 12 batteries in the bank
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

struct BatteryBank(Vec<u8>);
impl BatteryBank {
    #[allow(clippy::ptr_arg)]
    fn new(input: &String) -> Self {
        let mut batteries = Vec::new();
        for c in input.chars() {
            batteries.push(c.to_digit(10).unwrap() as u8);
        }
        BatteryBank(batteries)
    }

    fn get_highest_joltage(self, mut num_count: u16) -> u64 {
        let mut total: u64 = 0;
        let mut start_idx = 0;
        let total_len = self.0.len();
        while num_count > 0 {
            let num_search_slice = &self.0[start_idx..(total_len - (num_count - 1) as usize)];
            let max_num = num_search_slice.iter().max().unwrap();
            let (idx, max_val) = num_search_slice
                .iter()
                .enumerate()
                .find(|(_, num)| *num == max_num)
                .unwrap();

            start_idx += idx + 1;
            total += (*max_val as u64) * 10u64.pow(num_count as u32 - 1);

            num_count -= 1;
        }
        dbg!(&total);
        total
    }
}
fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");

    let example_sum = solution_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 357);

    let my_sum = solution_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 3121910778619);

    let my_sum = solution_2(_my_input);
    dbg!(my_sum);
}

fn solution_1(input: &[String]) -> u64 {
    let banks = parse_input(input);

    banks.into_iter().map(|b| b.get_highest_joltage(2)).sum()
}

fn solution_2(input: &[String]) -> u64 {
    let banks = parse_input(input);

    banks.into_iter().map(|b| b.get_highest_joltage(12)).sum()
}

fn parse_input(input: &[String]) -> Vec<BatteryBank> {
    input.iter().map(BatteryBank::new).collect()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
