/*
part 1:
we have a line of stones in a line with each having a number on it
sometimes the number on the stone changes, sometimes the stone spits in 2
the behaviors follow rules:
-   stones with number 0 will be replaced by number 1
-   if stone number has even number of digits, it's replaced by 2 stones and the digits are split in 2, but no leading 0, so 1000 -> 10 and 0
-   if other rules don't apply, the stone number is multiplied by 2024

we have to get the number of stones after a 25 blinks

part 2:

we now blink 75 times, stone numbers explode out of proportion
*/

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_stone_count = stone_count(&example_1, 25);
    dbg!(&example_stone_count);
    assert_eq!(example_stone_count, 55312);

    let my_stone_count = stone_count(_my_input, 25);
    dbg!(my_stone_count);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let start = Instant::now();
    let my_stone_count = stone_count(_my_input, 75);
    dbg!(start.elapsed());
    dbg!(my_stone_count);
}

fn stone_count(input: &[String], blink_times: u8) -> u64 {
    let all_stones = parse_input(input);

    let mut stone_count = 0;

    let mut cache: HashMap<(u64, u8), u64> = HashMap::new();

    for current_stone in all_stones {
        let new_stones = blink(current_stone, blink_times, &mut cache);
        println!("stones counted! {}", new_stones);

        stone_count += new_stones;
    }

    stone_count
}

fn blink(current_stone: u64, blinks_left: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if blinks_left == 0 {
        return 1;
    }

    if let Some(&cached_result) = cache.get(&(current_stone, blinks_left)) {
        return cached_result;
    }

    let result = if current_stone == 0 {
        blink(1, blinks_left - 1, cache)
    } else {
        let stone_num_str = current_stone.to_string();

        if stone_num_str.len() % 2 == 0 {
            let (left, right) = split_in_half(stone_num_str);

            blink(left, blinks_left - 1, cache) + blink(right, blinks_left - 1, cache)
        } else {
            blink(current_stone * 2024, blinks_left - 1, cache)
        }
    };

    cache.insert((current_stone, blinks_left), result);

    result
}

fn split_in_half(mut num_str: String) -> (u64, u64) {
    let str_len = num_str.len();
    let right = num_str.split_off(str_len / 2);

    let left_num = num_str.parse::<u64>().unwrap();
    let right_num = right.parse::<u64>().unwrap();

    (left_num, right_num)
}

fn parse_input(input: &[String]) -> Vec<u64> {
    input[0]
        .split_whitespace()
        .map(|num_str| num_str.parse::<u64>().unwrap())
        .collect()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
