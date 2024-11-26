/*
-lines of text
    -value found by combining the first and last digit from line (In that order)
    -both values are a single digit (1 char)
    -value is a 2 digit number
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let test_lines = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

    for line in test_lines {
        println!("{}", get_value_1(line));
    }

    let value_sum: i32 = test_lines.iter().map(|&line| get_value_1(line)).sum();

    dbg!(value_sum);
    assert_eq!(value_sum, 142);

    let my_lines = read_file("my_input.txt");

    let value_sum: i32 = my_lines.iter().map(|line| get_value_1(line)).sum();
    dbg!(value_sum);
}

fn part_2() {
    let test_lines = [
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "r7pqstsixteen",
    ];

    for line in test_lines {
        println!("{}", get_value_2(line));
    }

    let value_sum: i32 = test_lines.iter().map(|&line| get_value_2(line)).sum();

    dbg!(value_sum);
    assert_eq!(value_sum, 281);

    let my_lines = read_file("my_input.txt");

    let value_sum: i32 = my_lines.iter().map(|line| get_value_2(line)).sum();
    dbg!(value_sum);
}

fn get_value_2(input: &str) -> i32 {
    let conversion_table = [
        ("1", "one", "1"),
        ("2", "two", "2"),
        ("3", "three", "3"),
        ("4", "four", "4"),
        ("5", "five", "5"),
        ("6", "six", "6"),
        ("7", "seven", "7"),
        ("8", "eight", "8"),
        ("9", "nine", "9"),
    ];

    let mut first: Option<String> = None;
    let mut last: Option<String> = None;

    for i in 0..input.len() {
        let current_string: String = input.chars().skip(i).collect();

        for (match1, match2, conversion_num) in conversion_table {
            if current_string.starts_with(match1) || current_string.starts_with(match2) {
                first = Some(conversion_num.to_owned());
                break;
            }
        }
        if first.is_some() {
            break;
        }
    }

    for i in (1..=input.len()).rev() {
        let current_string: String = input.chars().take(i).collect();

        for (match1, match2, conversion_num) in conversion_table {
            if current_string.ends_with(match1) || current_string.ends_with(match2) {
                last = Some(conversion_num.to_owned());
                break;
            }
        }

        if last.is_some() {
            break;
        }
    }

    let combined = match (first, last) {
        (Some(s1), Some(s2)) => s1 + &s2,
        (Some(s), None) | (None, Some(s)) => s,
        (None, None) => "".to_string(),
    };

    combined.parse::<i32>().unwrap()
}

fn get_value_1(input: &str) -> i32 {
    let mut first: Option<String> = None;
    let mut last: Option<String> = None;

    for c in input.chars() {
        if c.to_string().parse::<i32>().is_ok() {
            if first.is_none() {
                first = Some(c.to_string())
            }
            last = Some(c.to_string())
        }
    }

    let combined = match (first, last) {
        (Some(s1), Some(s2)) => s1 + &s2,
        (Some(s), None) | (None, Some(s)) => s,
        (None, None) => "".to_string(),
    };

    combined.parse::<i32>().ok().unwrap()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("wont fail");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
