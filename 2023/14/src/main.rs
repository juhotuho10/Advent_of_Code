/*
we have a collection on round (O) and square (#) rocks in a area. Only the round rocks can be moved
The round rocks cannot go over the square rocks so if we move all the rocks to a certain direction,
some of the rocks will roll until the end of the area and some rocks will be stuck on the square rocks

the higher the rock, the more it deforms the platform, we have to count the total amount that the rocks
deform the platform

part 2:

we move bolders to north, then west, then south, then east 1000000000 times and then count the points

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

    let example_sum = get_total_strain_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 136);

    let my_sum = get_total_strain_1(_my_input);
    dbg!(&my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);
    let example_sum = get_total_strain_2(&example_2, 500);
    dbg!(&example_sum);
    assert_eq!(example_sum, 64);

    let my_sum = get_total_strain_2(_my_input, 500);
    dbg!(&my_sum);
}

fn get_total_strain_1(input: &[String]) -> u32 {
    let mut parsed = parse_input(input);

    let mut all_points = 0;

    rotate_vec(&mut parsed);

    move_right(&mut parsed);

    pretty_print_vec(&parsed);

    for _ in 0..3 {
        rotate_vec(&mut parsed);
    }

    for (i, row) in parsed.iter().rev().enumerate() {
        let bolder_count = row.iter().filter(|&&c| c == 'O').count();
        let points = (i + 1) * bolder_count;

        all_points += points;
    }

    all_points as u32
}

fn get_total_strain_2(input: &[String], cycles: usize) -> u32 {
    let mut parsed = parse_input(input);

    let mut point_vec = vec![];

    for _ in 1..=cycles {
        rotate_vec(&mut parsed); // up points to west
        move_right(&mut parsed); // move north

        rotate_vec(&mut parsed); // up points to south
        move_right(&mut parsed); // move west

        rotate_vec(&mut parsed); // up points to east
        move_right(&mut parsed); // move south

        rotate_vec(&mut parsed); // up points to north
        move_right(&mut parsed); // move east

        let mut temp_points = 0;

        for (i, row) in parsed.iter().rev().enumerate() {
            let bolder_count = row.iter().filter(|&&c| c == 'O').count();
            let points = (i + 1) * bolder_count;

            temp_points += points;
        }

        point_vec.push(temp_points as u32);
    }

    let start_nums = 300;

    let num_loop = find_loop_pattern(&point_vec, start_nums - 1).unwrap();
    dbg!(&num_loop);

    // loop position at number 1000000000
    let loop_i = (1000000000 - start_nums) % num_loop.len();

    let final_point = num_loop[loop_i];
    dbg!(&final_point);

    final_point
}

fn find_loop_pattern(v: &[u32], start_index: usize) -> Option<Vec<u32>> {
    let sequence = &v[start_index..];
    let length = sequence.len();

    for loop_len in 1..=length / 2 {
        let current_loop = &sequence[0..loop_len];
        let check_loop = &sequence[loop_len..loop_len * 2.min(length)];

        if current_loop == check_loop {
            return Some(current_loop.to_vec());
        }
    }

    None
}

fn move_right(area: &mut [Vec<char>]) {
    let mut moved;

    loop {
        moved = false;
        for row in area.iter_mut() {
            for i in (0..row.len() - 1).rev() {
                if row[i] == 'O' && !['O', '#'].contains(&row[i + 1]) {
                    row.swap(i, i + 1);
                    moved = true;
                }
            }
        }
        if !moved {
            break;
        }
    }
}

fn rotate_vec(input: &mut Vec<Vec<char>>) {
    let max_cols = input.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut result = vec![Vec::with_capacity(input.len()); max_cols];

    for row in input.iter().rev() {
        for (col_index, &ch) in row.iter().enumerate() {
            result[col_index].push(ch);
        }
    }

    // Replace the original input with the rotated result
    *input = result;
}

fn pretty_print_vec(input: &[Vec<char>]) {
    let pretty_vec: Vec<String> = input
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect();

    dbg!(pretty_vec);
}

fn parse_input(input: &[String]) -> Vec<Vec<char>> {
    input.iter().map(|s| s.chars().collect()).collect()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
