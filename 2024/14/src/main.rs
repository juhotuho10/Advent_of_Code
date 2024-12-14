/*
part 1:
we have a lobby of robots with position and velocity, the robots are constantly running around in the lobby
we have to predict where the robots will be in 100 steps, count their positions in each quadrant of the lobby
and then we multiply the count of the robots in each quadrant

part 2:

*/

use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<u32> for Coord {
    type Output = Self;

    fn mul(self, scalar: u32) -> Self::Output {
        Self {
            x: self.x * (scalar as i32),
            y: self.y * (scalar as i32),
        }
    }
}

#[derive(Debug)]
struct RobotPath {
    pos: Coord,
    velocity: Coord,
}

impl RobotPath {
    fn from_string(input: &str) -> Self {
        let parse_regex = Regex::new(r"p=(\d+),(\d+) v=(\-?\d+),(\-?\d+)").unwrap();

        let caps = parse_regex.captures(input).unwrap();

        RobotPath {
            pos: Coord {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
            },
            velocity: Coord {
                x: caps[3].parse().unwrap(),
                y: caps[4].parse().unwrap(),
            },
        }
    }
    fn step_forward(&mut self, times: u32, max_coords: Coord) {
        let step_coords = self.pos + self.velocity * times;

        let mut ending_x = step_coords.x % max_coords.x;
        let mut ending_y = step_coords.y % max_coords.y;

        if ending_x < 0 {
            ending_x += max_coords.x;
        }

        if ending_y < 0 {
            ending_y += max_coords.y;
        }

        self.pos = Coord {
            x: ending_x,
            y: ending_y,
        };
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

    let example_sum = robot_safety_factor_1(&example_1, Coord { x: 11, y: 7 });
    dbg!(&example_sum);
    assert_eq!(example_sum, 12);

    let my_sum = robot_safety_factor_1(_my_input, Coord { x: 101, y: 103 });
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    //let example_sum = time_till_easter_egg(&example_2, Coord { x: 11, y: 7 });

    let my_time = time_till_easter_egg(_my_input, Coord { x: 101, y: 103 });
    dbg!(my_time);
}

fn robot_safety_factor_1(input: &[String], max_coords: Coord) -> u32 {
    let mut paths = parse_input(input);

    paths
        .iter_mut()
        .for_each(|r| r.step_forward(100, max_coords));

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let middle_x = max_coords.x / 2;
    let middle_y = max_coords.y / 2;

    for robot in paths {
        let pos = robot.pos;

        if pos.x == middle_x || pos.y == middle_y {
            continue;
        }

        if pos.y < middle_y {
            if pos.x < middle_x {
                q1 += 1;
            } else {
                q2 += 1;
            }
        } else if pos.x < middle_x {
            q3 += 1;
        } else {
            q4 += 1;
        }
    }

    q1 * q2 * q3 * q4
}

fn time_till_easter_egg(input: &[String], max_coords: Coord) -> u32 {
    let mut paths = parse_input(input);

    let display = vec![vec!["  "; max_coords.x as usize + 2]; max_coords.y as usize + 2];

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("day_14_printouts.txt")
        .expect("Opening missing songs files");

    for step in 1..10_000 {
        let mut current_display = display.clone();
        paths.iter_mut().for_each(|r| r.step_forward(1, max_coords));

        for robot in &paths {
            let pos = robot.pos;
            current_display[pos.y as usize][pos.x as usize] = "##";
        }

        writeln!(file, "\n\n\n\n\n\n").expect("Writing in missing songs file");

        for line in current_display {
            let line_string: String = line.into_iter().collect();

            writeln!(file, "{line_string}").expect("Writing in missing songs file");
        }

        writeln!(file, "iteration - {}", step).expect("Writing in missing songs file");
    }

    0
}

fn parse_input(input: &[String]) -> Vec<RobotPath> {
    input.iter().map(|s| RobotPath::from_string(s)).collect()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
