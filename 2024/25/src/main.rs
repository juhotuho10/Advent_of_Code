/*
part 1:

we have to pick a lock. We have a bunch of keys to try on the lock and we have to rule out some ouf them.

we have locks and keys that are 2d grids, with empty space being (.) and filled space being (#) and based on the filled space
we have to check if the key can possible fit the lock without overlapping with the filled in space.

we have to check all the key lock combinations that work and then return the count of fitting keys

part 2:

finish every other day in the AoC year

*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Lock {
    pins: [u8; 5],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Key {
    pins: [u8; 5],
}

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_sum = fitting_lock_key_combinations(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 0);

    let my_sum = fitting_lock_key_combinations(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 0);

    let my_sum = solution_2(_my_input);
    dbg!(my_sum);
}

fn fitting_lock_key_combinations(input: &[String]) -> u32 {
    let (locks, keys) = parse_input(input);

    let mut fit_pairs = 0;

    for curr_lock in &locks {
        for curr_key in &keys {
            if curr_lock
                .pins
                .iter()
                .zip(curr_key.pins.iter())
                .all(|(lock_pin, key_pin)| key_pin <= lock_pin)
            {
                fit_pairs += 1;
            }
        }
    }

    dbg!(fit_pairs);

    0
}

fn solution_2(input: &[String]) -> u32 {
    let parsed = parse_input(input);
    0
}

fn parse_input(input: &[String]) -> (Vec<Lock>, Vec<Key>) {
    let mut key_lock_strings: Vec<Vec<String>> = vec![];

    let mut curr_lock = vec![];
    for line in input {
        if line.is_empty() {
            key_lock_strings.push(curr_lock.clone());
            curr_lock.clear();
        } else {
            curr_lock.push(line.to_string());
        }
    }
    key_lock_strings.push(curr_lock);

    let mut all_locks = vec![];
    let mut all_keys = vec![];

    for key_lock in key_lock_strings {
        let is_lock = key_lock[0].chars().all(|c| c == '#');
        //dbg!(&key_lock);
        let rotated = rotate(key_lock);
        //dbg!(&rotated);

        let mut lock_nums: Vec<u8> = vec![];
        if is_lock {
            for line in &rotated {
                lock_nums.push(line.chars().filter(|c| *c == '.').count() as u8);
            }
            let pins: [u8; 5] = lock_nums.try_into().unwrap();
            all_locks.push(Lock { pins });
        } else {
            for line in &rotated {
                lock_nums.push(line.chars().filter(|c| *c == '#').count() as u8);
            }
            let pins: [u8; 5] = lock_nums.try_into().unwrap();

            all_keys.push(Key { pins });
        }
    }

    (all_locks, all_keys)
}

fn rotate(grid: Vec<String>) -> Vec<String> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut rotated = vec![String::new(); cols];

    for col in 0..cols {
        let mut new_row = String::new();
        for row in (0..rows).rev() {
            new_row.push(grid[row].chars().nth(col).unwrap());
        }
        rotated[col] = new_row;
    }

    rotated
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
