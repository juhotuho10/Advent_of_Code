/*
we have a bunch of strings that we need to hash
get the aschii representation of each char, add it to total, multiply the total by 17, get remainder from total / 256, add the new aschii number

do this to all strings and return the sum

part 2:

we have bunch of lense boxed
the lensed have a identifier string, a - or = and a lens strenght number
we add, remove and change the lenses in the boxes based on the - or the = marks
and in the end, we take the total power of all the boxes with the sum of
(box num +1) * lens number * lens strenght
for all the boxes and return the sum

*/

use indexmap::IndexMap;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Sign {
    Remove,
    Add(u32),
}

struct HashString {
    complete_string: String,
    id_string: String,
    sign: Sign,
}

impl HashString {
    fn from_string(input: String) -> Self {
        let re = Regex::new(r"([a-zA-Z]+)([=/-])(\d+)?").unwrap();
        let captures = re.captures(&input).unwrap();

        let id_string = captures[1].to_string();
        let captured_sign = captures[2].chars().next().unwrap();

        let sign = match captured_sign {
            '-' => Sign::Remove,
            '=' => {
                let add_number = captures[3].parse::<u32>().unwrap();

                Sign::Add(add_number)
            }
            _ => unreachable!(),
        };

        HashString {
            complete_string: input.clone(),
            id_string,
            sign,
        }
    }

    fn get_string_hash(&self, input: &str) -> u32 {
        let mut total: u32 = 0;
        for c in input.chars() {
            assert!(c.is_ascii());

            let ascii_code = c as u8;
            total += ascii_code as u32;
            total *= 17;
            total %= 256;
        }

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
    dbg!(&example_1);

    let example_sum = get_hash_sum_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 1320);

    let my_sum = get_hash_sum_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = get_focus_power_sum_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 145);

    let my_sum = get_focus_power_sum_2(_my_input);
    dbg!(my_sum);
}

fn get_hash_sum_1(input: &[String]) -> u32 {
    let hash_strings = parse_input(input);

    hash_strings
        .iter()
        .map(|s| s.get_string_hash(&s.complete_string))
        .sum()
}

fn get_focus_power_sum_2(input: &[String]) -> u32 {
    let hash_strings = parse_input(input);
    let mut lense_boxes: HashMap<u32, IndexMap<String, u32>> = HashMap::new();

    for item in &hash_strings {
        let hash = item.get_string_hash(&item.id_string);

        let current_box = lense_boxes.entry(hash).or_default();

        match item.sign {
            Sign::Add(lense_num) => {
                *current_box.entry(item.id_string.clone()).or_insert(0) = lense_num;
            }
            Sign::Remove => {
                current_box.shift_remove(&item.id_string);
            }
        }
    }

    let mut total = 0;
    for box_num in 0..=256 {
        if let Some(lens_box) = lense_boxes.get(&box_num) {
            for (lens_num, power) in lens_box.values().enumerate() {
                total += (box_num + 1) * (lens_num + 1) as u32 * power;
            }
        }
    }
    total
}

fn parse_input(input: &[String]) -> Vec<HashString> {
    let strings: Vec<String> = input[0].split(",").map(|s| s.to_owned()).collect();

    strings
        .iter()
        .map(|s| HashString::from_string(s.to_owned()))
        .collect()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
