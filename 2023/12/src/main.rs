/*
we have hotsprings that work or are broken
borken hotsprint = #, working hotspring = .
hotsprings are in a a line and we know how many consecutive hotsprings work or are broken
all consecutive broken hotsprings are separated by a working one
example:
.#...#....###. 1,1,3

we also have hotsprings we dont know the condition of, marked ?
we have to figure out how many different orders the broken hotsprings can be in
like:
.??..??...?##. 1,1,3

can be:

.#...#....###
..#..#....###
.#....#...###
..#...#...###
so 4 diffrent combinations
and we have to figure out the sum of all possible combinations for all hotsprings


part 2:

all springs become 5x longer, separated by ? symbol

so
.??..??...?##. 1,1,3
becomes:

.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##. 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3


*/

use rayon::prelude::*;
use regex::Regex;
use std::clone;
use std::fs::File;

use std::{
    io::{BufRead, BufReader},
    iter::repeat_n,
    mem::discriminant,
    time::Instant,
};

#[derive(Debug, Clone, Copy)]
enum Condition {
    Good,
    Bad,
    Idk,
}

impl PartialEq for Condition {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Condition::Good, Condition::Good)
                | (Condition::Bad, Condition::Bad)
                | (Condition::Idk, _)
                | (_, Condition::Idk)
        )
    }
}
#[derive(Debug, Clone)]
struct Spring {
    arrangement_str: String,
    conditions: Vec<Condition>,
    broken: Vec<u32>,
}

impl Spring {
    fn from_string_1(input: String) -> Self {
        let (arrangement_str, broken_str) = input.split_once(" ").unwrap();
        let broken: Vec<u32> = broken_str
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let conditions: Vec<Condition> = arrangement_str
            .chars()
            .map(|c| match c {
                '.' => Condition::Good,
                '#' => Condition::Bad,
                '?' => Condition::Idk,
                _ => unreachable!(),
            })
            .collect();
        Spring {
            arrangement_str: arrangement_str.to_owned(),
            conditions,
            broken,
        }
    }

    fn from_string_2(input: String) -> Self {
        let (arrangement_str, broken_str) = input.split_once(" ").unwrap();

        let count_multiplier = 5;
        let arrangement_str: String = repeat_n(arrangement_str, count_multiplier)
            .collect::<Vec<_>>()
            .join("?");

        let broken_str: String = repeat_n(broken_str, count_multiplier)
            .collect::<Vec<_>>()
            .join(",");

        dbg!(&arrangement_str);
        dbg!(&broken_str);

        let broken: Vec<u32> = broken_str
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let conditions: Vec<Condition> = arrangement_str
            .chars()
            .map(|c| match c {
                '.' => Condition::Good,
                '#' => Condition::Bad,
                '?' => Condition::Idk,
                _ => unreachable!(),
            })
            .collect();
        Spring {
            arrangement_str: arrangement_str.to_owned(),
            conditions,
            broken,
        }
    }

    fn translate_combination(&self, combination: &[u32]) -> Vec<Condition> {
        let total_size: usize = combination
            .iter()
            .map(|&count| count as usize)
            .sum::<usize>()
            + self
                .broken
                .iter()
                .map(|&count| count as usize)
                .sum::<usize>();

        let mut sprint_conditions = Vec::with_capacity(total_size);

        // Add initial good conditions, this is done because there is one more good than there is bad
        sprint_conditions.resize(combination[0] as usize, Condition::Good);

        for (bad_count, good_count) in self.broken.iter().zip(&combination[1..]) {
            sprint_conditions.resize(
                sprint_conditions.len() + *bad_count as usize,
                Condition::Bad,
            );
            sprint_conditions.resize(
                sprint_conditions.len() + *good_count as usize,
                Condition::Good,
            );
        }

        sprint_conditions
    }

    fn is_valid_comb(&self, combination: &[u32]) -> bool {
        let translate_comb = self.translate_combination(combination);
        self.conditions.starts_with(&translate_comb)
    }
    fn max_possible_good(&self) -> u32 {
        // gets the max good tiles in a row
        let mut max_count = 0;
        let mut current_count = 0;

        for condition in &self.conditions {
            match condition {
                Condition::Good | Condition::Idk => {
                    current_count += 1;
                    if current_count > max_count {
                        max_count = current_count;
                    }
                }
                Condition::Bad => {
                    current_count = 0;
                }
            }
        }

        max_count
    }

    fn valid_combination_count(&self) -> u64 {
        let spring_len = self.conditions.len();
        let broken_len: usize = self.broken.len();
        let total_broken: u32 = self.broken.iter().sum();
        let total_good = spring_len as u32 - total_broken;

        let max_value = self.max_possible_good().min(total_good);
        let mut min_values = vec![0];
        min_values.resize(broken_len, 1);
        min_values.push(0);

        let mut current = min_values.clone();

        self.new_combinations(&min_values, &mut current, 0, total_good, max_value)
    }

    fn new_combinations(
        &self,
        min_values: &Vec<u32>,
        current: &mut Vec<u32>,
        index: usize,
        target_sum: u32,
        max_value: u32,
    ) -> u64 {
        if index == current.len() {
            return u64::from(
                // bool as u64
                current.iter().sum::<u32>() == target_sum,
            );
        }

        let mut sum = 0;

        let mut found_valid = false;

        for value in min_values[index]..=max_value {
            current[index] = value;
            if !self.is_valid_comb(&current[..=index]) {
                if found_valid {
                    return sum;
                } else {
                    continue;
                }
            }
            found_valid = true;
            sum += self.new_combinations(min_values, current, index + 1, target_sum, max_value);
        }

        sum
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

    let example_arrangements = get_num_arrangements_1(&example_1);
    dbg!(&example_arrangements);
    assert_eq!(example_arrangements, 21);

    let my_arrangements = get_num_arrangements_1(_my_input);
    dbg!(my_arrangements);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let start = Instant::now();
    let example_arrangements = get_num_arrangements_2(&example_2);
    dbg!(start.elapsed().as_millis());
    dbg!(&example_arrangements);
    assert_eq!(example_arrangements, 525152);

    //let my_arrangements = get_num_arrangements_2(_my_input);
    //dbg!(my_arrangements);
}

fn get_num_arrangements_1(input: &[String]) -> u64 {
    let springs = parse_input_1(input);

    springs
        .par_iter()
        .map(|spring| spring.valid_combination_count())
        .sum()
}

fn get_num_arrangements_2(input: &[String]) -> u64 {
    let springs = parse_input_2(input);

    springs
        .par_iter()
        .map(|spring| spring.valid_combination_count())
        .sum()
}

fn parse_input_1(input: &[String]) -> Vec<Spring> {
    input
        .iter()
        .map(|str| Spring::from_string_1(str.to_owned()))
        .collect()
}

fn parse_input_2(input: &[String]) -> Vec<Spring> {
    input
        .iter()
        .map(|str| Spring::from_string_2(str.to_owned()))
        .collect()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
