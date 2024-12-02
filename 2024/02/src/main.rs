/*
part 1:
we have reports with levels of numbers separated by spaces
example:
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9

this would be six reports containing 5 levels
levels are safe if they are ascending or decending in order and the numbers near each other differ by at least 1 and at most 3

we return the num of safe reports

part 2:

reactors tolerate a single bad level for report to be safe

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

    let example_safe = check_level_safety_1(&example_1);
    dbg!(&example_safe);
    assert_eq!(example_safe, 2);

    let my_safe = check_level_safety_1(_my_input);
    dbg!(my_safe);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_safe = check_level_safety_2(&example_2);
    dbg!(&example_safe);
    assert_eq!(example_safe, 4);

    let my_safe = check_level_safety_2(_my_input);
    dbg!(my_safe);
}

fn check_level_safety_1(input: &[String]) -> u32 {
    let reports = parse_input(input);

    let mut safe_count = 0;

    for report in reports {
        if check_report_safety(report) {
            safe_count += 1;
        }
    }

    safe_count
}

fn check_level_safety_2(input: &[String]) -> u32 {
    let reports = parse_input(input);

    let mut safe_count = 0;

    for report in reports {
        let mut is_safe = false;
        for removed_level in 0..report.len() {
            let mut modified_report = report.clone();
            modified_report.remove(removed_level);

            is_safe |= check_report_safety(modified_report);
        }

        if is_safe {
            safe_count += 1;
        }
    }

    safe_count
}

fn check_report_safety(report: Vec<u32>) -> bool {
    let desc = report.windows(2).all(|r| r[0] > r[1]);
    let asc = report.windows(2).all(|r| r[0] < r[1]);

    let differences: Vec<u32> = report.windows(2).map(|r| r[0].abs_diff(r[1])).collect();
    let max_diff = differences.iter().max().unwrap();
    let min_diff = differences.iter().min().unwrap();

    if (desc || asc) && *max_diff <= 3 && *min_diff >= 1 {
        return true;
    }

    false
}

fn parse_input(input: &[String]) -> Vec<Vec<u32>> {
    let mut levels = vec![];
    for line in input {
        let new_report: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

        levels.push(new_report);
    }

    levels
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
