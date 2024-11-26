/*
we have a boat race that we need to win
the boat races are based on how much distance you can cover in a certain time
the distance covered is determined by how long you charge the boat, but the longer you charge, the less time you have to go forward
get the possible count of winning combinations of charing for every race
multiply the winning combinations for all races together

part 2:
    it's all just a single race and you have to combine the time and distance letters
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

struct TimeDistance {
    time: u64,
    distance: u64,
}

impl TimeDistance {
    fn calculate_num_winning(&self) -> u64 {
        // x = total time
        // m = time taken to charge

        // (m) M/s * (x - m) S = M)
        // (m) * (x - m) = Distance)
        // -m^2 * xm = Distance
        // -m^2 * xm - Distance = 0

        let (min_charge_time, max_charge_time) =
            TimeDistance::solve_quadratic(-1, self.time as i64, -(self.distance as i64)).unwrap();

        max_charge_time - min_charge_time - 1
    }

    fn solve_quadratic(a: i64, b: i64, c: i64) -> Option<(u64, u64)> {
        let discriminant: f64 = (b * b - 4 * a * c) as f64;

        if discriminant < 0.0 {
            return None;
        }

        let min_time = (-b as f64 + discriminant.sqrt()) / (2.0 * a as f64);
        let max_time = (-b as f64 - discriminant.sqrt()) / (2.0 * a as f64);

        Some((min_time as u64, max_time.ceil() as u64))
    }

    fn solve_part_2(&self) -> u64 {
        let mut min_time: u64 = 0;
        let mut max_time: u64 = 0;

        for t in 0..self.time {
            if t * (self.time - t) > self.distance {
                min_time = t;
                break;
            }
        }

        for t in (0..self.time).rev() {
            if t * (self.time - t) > self.distance {
                max_time = t;
                break;
            }
        }

        max_time - min_time + 1
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

    let example_times = get_winning_times_1(&example_1);
    dbg!(example_times);
    assert_eq!(example_times, 288);

    let my_times = get_winning_times_1(_my_input);
    dbg!(my_times);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_times = get_winning_times_2(&example_2);
    dbg!(example_times);
    assert_eq!(example_times, 71503);

    let my_times = get_winning_times_2(_my_input);
    dbg!(my_times);
}

fn get_winning_times_1(input: &[String]) -> u64 {
    let mut winning_times_count: Vec<u64> = vec![];
    let all_time_distances = parse_input_1(input);

    for td in all_time_distances {
        winning_times_count.push(td.calculate_num_winning());
    }

    winning_times_count.iter().product::<u64>()
}

fn get_winning_times_2(input: &[String]) -> u64 {
    let mut winning_times_count: Vec<u64> = vec![];
    let all_time_distances = parse_input_2(input);

    for td in all_time_distances {
        winning_times_count.push(td.solve_part_2());
    }

    winning_times_count[0]
}

fn parse_input_1(input: &[String]) -> Vec<TimeDistance> {
    let (_, times_str) = input[0].split_once(":").unwrap();
    let (_, distance_str) = input[1].split_once(":").unwrap();

    let times: Vec<u64> = times_str
        .trim()
        .split(" ")
        .filter(|str| !str.is_empty())
        .map(|time_str| time_str.trim().parse::<u64>().unwrap())
        .collect();

    let distances: Vec<u64> = distance_str
        .trim()
        .split(" ")
        .filter(|str| !str.is_empty())
        .map(|distance_str| distance_str.parse::<u64>().unwrap())
        .collect();

    let mut all_times = vec![];
    for (time, distance) in times.iter().zip(distances) {
        let new_td = TimeDistance {
            time: *time,
            distance,
        };

        all_times.push(new_td);
    }

    all_times
}

fn parse_input_2(input: &[String]) -> Vec<TimeDistance> {
    let (_, times_str) = input[0].split_once(":").unwrap();
    let (_, distance_str) = input[1].split_once(":").unwrap();

    let time = times_str.replace(" ", "").parse::<u64>().unwrap();
    let distance = distance_str.replace(" ", "").parse::<u64>().unwrap();

    vec![TimeDistance { time, distance }]
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("wont fail");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
