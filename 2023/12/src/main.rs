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

use regex::Regex;
use std::fs::File;

use std::io::{BufRead, BufReader};
use std::mem::discriminant;

#[derive(Debug, Clone, Copy)]
enum Condition {
    Good,
    Bad,
    Idk,
}

impl PartialEq for Condition {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Condition::Good, Condition::Idk) | (Condition::Idk, Condition::Good) => true,
            (Condition::Bad, Condition::Idk) | (Condition::Idk, Condition::Bad) => true,

            _ => discriminant(self) == discriminant(other),
        }
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

        let count_multiplier = 3;
        let arrangement_str: String = std::iter::repeat(arrangement_str)
            .take(count_multiplier)
            .collect::<Vec<_>>()
            .join("?");

        let broken_str: String = std::iter::repeat(broken_str)
            .take(count_multiplier)
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

    fn valid_combination_count(&self) -> u32 {
        let all_combinations = Self::get_all_combinations(self);

        dbg!(all_combinations.len());

        all_combinations
            .iter()
            .map(|comb| Self::translate_combination(self, comb))
            .filter(|combination| {
                combination
                    .iter()
                    .zip(self.conditions.iter())
                    .all(|(other_cond, self_cond)| other_cond == self_cond)
            })
            .count() as u32
    }

    fn translate_combination(&self, combination: &[u32]) -> Vec<Condition> {
        let mut sprint_conditions: Vec<Condition> = vec![Condition::Good; combination[0] as usize];

        for (bad_count, good_count) in self.broken.iter().zip(&combination[1..]) {
            let bad = vec![Condition::Bad; *bad_count as usize];
            sprint_conditions.extend_from_slice(&bad);

            let good = vec![Condition::Good; *good_count as usize];
            sprint_conditions.extend_from_slice(&good);
        }

        sprint_conditions
    }

    fn max_possible_good(&self) -> u32 {
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
                _ => {
                    current_count = 0;
                }
            }
        }

        max_count
    }

    fn get_all_combinations(&self) -> Vec<Vec<u32>> {
        let spring_len = self.conditions.len();
        let broken_len: usize = self.broken.len();
        let total_broken: u32 = self.broken.iter().sum();
        let total_good = spring_len as u32 - total_broken;

        let max_value = self.max_possible_good().min(total_good);
        let mut min_values = vec![0];
        min_values.resize(broken_len, 1);
        min_values.push(0);

        let mut current = min_values.clone();
        let mut results = Vec::new();
        Self::new_combinations(
            &min_values,
            &mut current,
            0,
            total_good,
            max_value,
            &mut results,
        );

        results
    }

    fn new_combinations(
        min_values: &Vec<u32>,
        current: &mut Vec<u32>,
        index: usize,
        target_sum: u32,
        max_value: u32,
        results: &mut Vec<Vec<u32>>,
    ) {
        if index == current.len() {
            if current.iter().sum::<u32>() == target_sum {
                results.push(current.clone());
            }
            return;
        }

        for value in min_values[index]..=max_value {
            current[index] = value;
            Self::new_combinations(
                min_values,
                current,
                index + 1,
                target_sum,
                max_value,
                results,
            );
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

    let example_arrangements = get_num_arrangements_1(&example_1);
    dbg!(&example_arrangements);
    assert_eq!(example_arrangements, 21);

    let my_arrangements = get_num_arrangements_1(_my_input);
    dbg!(my_arrangements);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_arrangements = get_num_arrangements_2(&example_2);
    dbg!(&example_arrangements);
    assert_eq!(example_arrangements, 525152);

    //let my_arrangements = get_num_arrangements_2(_my_input);
    //dbg!(my_arrangements);
}

fn get_num_arrangements_1(input: &[String]) -> u32 {
    let springs = parse_input_1(input);

    springs
        .iter()
        .map(|spring| spring.valid_combination_count())
        .sum()
}

fn get_num_arrangements_2(input: &[String]) -> u32 {
    let springs = parse_input_2(input);

    springs
        .iter()
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
