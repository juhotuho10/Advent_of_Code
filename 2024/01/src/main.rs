/*
we have 2 lists, we need to take the difference between the smallest on the first list to the smallest on the second and add up the difference

part 2:
we have similarty scores that are calculated:
left num * how many times the num appears in right list
and add those together
*/

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

    let example_dist = get_list_distance_1(&example_1);
    dbg!(&example_dist);
    assert_eq!(example_dist, 11);

    let my_dist = get_list_distance_1(_my_input);
    dbg!(my_dist);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_dist = get_list_similarity_2(&example_2);
    dbg!(&example_dist);
    assert_eq!(example_dist, 31);

    let my_dist = get_list_similarity_2(_my_input);
    dbg!(my_dist);
}

fn get_list_distance_1(input: &[String]) -> u64 {
    let (mut left_list, mut right_list) = parse_input(input);

    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

fn get_list_similarity_2(input: &[String]) -> u64 {
    let (left_list, right_list) = parse_input(input);

    left_list
        .iter()
        .map(|left_num| {
            let num_count = right_list.iter().filter(|right| *right == left_num).count();
            num_count as u64 * left_num
        })
        .sum()
}

fn parse_input(input: &[String]) -> (Vec<u64>, Vec<u64>) {
    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in input {
        let (left, right) = line.split_once(" ").unwrap();
        left_list.push(left.trim().parse::<u64>().unwrap());
        right_list.push(right.trim().parse::<u64>().unwrap());
    }

    (left_list, right_list)
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
