/*
part 1:
We have a grid with antennas that indicate their frequence with letters (A, O, I, etc. )

The signals that the antennas emmit create antinodes, but only when 2 antennas of the same
frequency are on the same line and one of the antenna is twice as far away as the other

Example of "a" antennas and antinode placements with "#":
..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........

we have to check how many unique locations do antinodes occur in

part 2:

antennas now create harmonic frequencies that extend indefinitely

exmple of the antinode places for T antennas:
T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........


*/

use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x.saturating_add(other.x),
            y: self.y.saturating_add(other.y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Antenna {
    freq: char,
    pos: Coord,
}

#[derive(Debug, Clone)]
struct AllAntenna {
    antennas: Vec<Antenna>,
    unique_freqs: HashSet<char>,
    min_coord: Coord,
    max_coord: Coord,
}

impl AllAntenna {
    fn get_antinodes_1(&self) -> HashSet<Coord> {
        let mut antinode_coords: HashSet<Coord> = HashSet::new();

        for current_freq in &self.unique_freqs {
            let freq_antennas: Vec<&Antenna> = self
                .antennas
                .iter()
                .filter(|a| a.freq == *current_freq)
                .collect();

            for combination in freq_antennas.into_iter().combinations(2) {
                let antenna_1 = combination[0];
                let antenna_2 = combination[1];

                let coord_diff = antenna_1.pos - antenna_2.pos;

                let antinode_pos_1 = antenna_1.pos + coord_diff;
                let antinode_pos_2 = antenna_2.pos - coord_diff;

                if (self.min_coord.x..=self.max_coord.x).contains(&antinode_pos_1.x)
                    && (self.min_coord.y..=self.max_coord.y).contains(&antinode_pos_1.y)
                {
                    antinode_coords.insert(antinode_pos_1);
                }

                if (self.min_coord.x..=self.max_coord.x).contains(&antinode_pos_2.x)
                    && (self.min_coord.y..=self.max_coord.y).contains(&antinode_pos_2.y)
                {
                    antinode_coords.insert(antinode_pos_2);
                }
            }
        }
        antinode_coords
    }

    fn get_antinodes_2(&self) -> HashSet<Coord> {
        let mut antinode_coords: HashSet<Coord> = HashSet::new();

        for current_freq in &self.unique_freqs {
            let freq_antennas: Vec<&Antenna> = self
                .antennas
                .iter()
                .filter(|a| a.freq == *current_freq)
                .collect();

            if freq_antennas.len() > 1 {
                for antenna in &freq_antennas {
                    antinode_coords.insert(antenna.pos);
                }
            }

            for combination in freq_antennas.into_iter().combinations(2) {
                let antenna_1 = combination[0];
                let antenna_2 = combination[1];

                let coord_diff = antenna_1.pos - antenna_2.pos;

                let mut antinode_pos_1 = antenna_1.pos + coord_diff;

                // while the position is in bounds, we keep adding to it
                while (self.min_coord.x..=self.max_coord.x).contains(&antinode_pos_1.x)
                    && (self.min_coord.y..=self.max_coord.y).contains(&antinode_pos_1.y)
                {
                    antinode_coords.insert(antinode_pos_1);
                    antinode_pos_1 = antinode_pos_1 + coord_diff;
                }

                let mut antinode_pos_2 = antenna_2.pos - coord_diff;

                while (self.min_coord.x..=self.max_coord.x).contains(&antinode_pos_2.x)
                    && (self.min_coord.y..=self.max_coord.y).contains(&antinode_pos_2.y)
                {
                    antinode_coords.insert(antinode_pos_2);
                    antinode_pos_2 = antinode_pos_2 - coord_diff;
                }
            }
        }
        antinode_coords
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

    let example_count = get_antinode_count_1(&example_1);
    dbg!(&example_count);
    assert_eq!(example_count, 14);

    let my_count = get_antinode_count_1(_my_input);
    dbg!(my_count);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_count = get_antinode_count_2(&example_2);
    dbg!(&example_count);
    assert_eq!(example_count, 34);

    let my_count = get_antinode_count_2(_my_input);
    dbg!(my_count);
}

fn get_antinode_count_1(input: &[String]) -> u32 {
    let all_antenna = parse_input(input);

    let antinode_pos = all_antenna.get_antinodes_1();

    antinode_pos.len() as u32
}

fn get_antinode_count_2(input: &[String]) -> u32 {
    let all_antenna = parse_input(input);

    let antinode_pos = all_antenna.get_antinodes_2();

    antinode_pos.len() as u32
}
fn parse_input(input: &[String]) -> AllAntenna {
    let mut antennas = vec![];
    let mut frequencies = HashSet::new();

    let min_coord = Coord { x: 0, y: 0 };

    let max_coord = Coord {
        x: (input[0].len() - 1) as i32,
        y: (input.len() - 1) as i32,
    };

    for (y, y_line) in input.iter().enumerate() {
        for (x, x_char) in y_line.chars().enumerate() {
            if x_char != '.' {
                let current_coords = Coord {
                    x: x as i32,
                    y: y as i32,
                };

                let antenna = Antenna {
                    freq: x_char,
                    pos: current_coords,
                };

                frequencies.insert(x_char);

                antennas.push(antenna);
            }
        }
    }

    AllAntenna {
        antennas,
        unique_freqs: frequencies,
        min_coord,
        max_coord,
    }
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
