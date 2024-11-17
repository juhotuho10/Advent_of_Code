/*
we have a collection of stars in a grid
if a grid column or row doesnt have any stars, it is 2 wide, otherwise it is one wide
we calculate the smallest path between all combinations of stars and sum the total path len for all of them
for example 9 stars has 36 combinations (8 + 7 + .... 1)

part 2:
if row doesnt have stars, it is 1000000 wide instead of 2 wide
*/

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Coord {
    x: u32,
    y: u32,
}

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_sum = calculate_path_sum(&example_1, 2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 374);

    let my_sum = calculate_path_sum(_my_input, 2);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = calculate_path_sum(&example_2, 100);
    dbg!(&example_sum);
    assert_eq!(example_sum, 8410);

    let my_sum = calculate_path_sum(_my_input, 1000000);
    dbg!(my_sum);
}

fn calculate_path_sum(input: &[String], void_len: usize) -> u128 {
    let expanded_input = expand_input(input);

    let mut star_map: HashMap<u32, Coord> = HashMap::new();
    let mut star_num = 1;
    for (y, line) in expanded_input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let coords = Coord {
                    x: x as u32,
                    y: y as u32,
                };

                star_map.insert(star_num, coords);

                star_num += 1;
            }
        }
    }

    let star_count = star_map.keys().max().unwrap();

    let mut path_sum: u128 = 0;

    for start_star in 1..*star_count {
        for end_star in (start_star + 1)..=*star_count {
            let start_coords = star_map.get(&start_star).unwrap();
            let end_coords = star_map.get(&end_star).unwrap();

            let x_diff = start_coords.x.abs_diff(end_coords.x);
            let y_diff = start_coords.y.abs_diff(end_coords.y);

            let lower_x = start_coords.x.min(end_coords.x);

            let lower_y = start_coords.y.min(end_coords.y);

            let x_chars: Vec<char> = expanded_input[lower_y as usize]
                [(lower_x as usize)..=(lower_x + x_diff) as usize]
                .chars()
                .collect(); // always includes the staring position char

            let y_chars: Vec<char> = expanded_input
                [(1 + lower_y as usize)..=(lower_y + y_diff) as usize]
                .iter()
                .map(|row| row.chars().nth(end_coords.x as usize).unwrap())
                .collect();

            let mut all_chars: Vec<char> = y_chars;
            all_chars.extend(x_chars);

            // we include the start start even though we shouldnt so - 1
            let star_count = all_chars.iter().filter(|&&c| c == '.' || c == '#').count() - 1;
            let void_count = all_chars.iter().filter(|&&c| c == '%').count();

            let total_path_len = star_count + void_count * void_len;

            // panic on overflow
            path_sum = path_sum
                .checked_add(total_path_len as u128)
                .expect("overflow");
        }
    }

    path_sum
}

fn expand_input(input: &[String]) -> Vec<String> {
    let mut new_input = vec![];

    let galaxy_line = "%".repeat(input[0].len());

    for input_line in input.iter() {
        if input_line.chars().all(|x| x == '.') {
            new_input.push(galaxy_line.clone());
        } else {
            new_input.push(input_line.clone());
        }
    }

    new_input = rotate_vector(new_input);

    let mut new_rotated_input = vec![];
    let galaxy_line = "%".repeat(new_input[0].len());
    for input_line in new_input.iter() {
        if input_line.chars().all(|x| x == '.' || x == '%') {
            new_rotated_input.push(galaxy_line.clone());
        } else {
            new_rotated_input.push(input_line.clone());
        }
    }

    rotate_vector(new_rotated_input)
}

fn rotate_vector(input: Vec<String>) -> Vec<String> {
    let mut rotated: Vec<String> = vec![String::new(); input[0].len()];

    for (col_i, col) in rotated.iter_mut().enumerate() {
        for row in &input {
            col.push(row.chars().nth(col_i).unwrap());
        }
    }

    rotated
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
