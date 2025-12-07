/*
part 1:
we have a beam with a lot of splitters, we need to continue the beam and split it as many times as needed and count the amount of splits
part 2:
we actually have a quantum beam and we need to count all the possible ways the beam can split if it chooses either left or right at every splitter
*/

use ahash::{AHashMap, AHashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const SPLITTER: u8 = b'^';
const SPACE: u8 = b'.';
const START: u8 = b'S';

struct TachyonBeam<'a> {
    beam: AHashSet<usize>,
    map: &'a [String],
}

impl<'a> TachyonBeam<'a> {
    fn new(input: &'a [String]) -> Self {
        let (start_str, rest) = input.split_first().unwrap();
        let beam: AHashSet<usize> = start_str
            .char_indices()
            .filter(|(_, c)| *c as u8 == START)
            .map(|(i, _)| i)
            .collect();
        TachyonBeam { beam, map: rest }
    }

    fn split_beam(mut self) -> u32 {
        let mut split_count = 0;

        let max_len = self.map[0].len();
        for row in self.map {
            for (i, char) in row.char_indices() {
                if char as u8 == SPLITTER && self.beam.remove(&i) {
                    split_count += 1;
                    let prev = i - 1;
                    let next = i + 1;
                    if i > 0 {
                        self.beam.insert(prev);
                    }

                    if next <= max_len {
                        self.beam.insert(next);
                    }
                }
            }
        }

        split_count
    }

    fn quantum_beam(memo: &mut AHashMap<(u8, u8), u64>, beam: usize, map: &[String]) -> u64 {
        if map.is_empty() {
            return 1;
        }

        let memo_key = (beam as u8, map.len() as u8);

        if let Some(memo_result) = memo.get(&memo_key) {
            return *memo_result;
        }

        let result = match map[0].as_bytes().get(beam) {
            Some(&SPLITTER) => {
                TachyonBeam::quantum_beam(memo, beam - 1, &map[1..])
                    + TachyonBeam::quantum_beam(memo, beam + 1, &map[1..])
            }
            Some(&SPACE) => TachyonBeam::quantum_beam(memo, beam, &map[1..]),
            None => 0,
            _ => unreachable!(),
        };

        memo.insert(memo_key, result);
        result
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

    let example_sum = solution_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 21);

    let my_sum = solution_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 40);

    let my_sum = solution_2(_my_input);
    dbg!(my_sum);
}

fn solution_1(input: &[String]) -> u32 {
    let beam = parse_input(input);
    beam.split_beam()
}

fn solution_2(input: &[String]) -> u64 {
    let beam = parse_input(input);
    let mut memo: AHashMap<(u8, u8), u64> = AHashMap::new();
    let start = beam.beam.iter().next().unwrap();
    TachyonBeam::quantum_beam(&mut memo, *start, beam.map)
}

fn parse_input(input: &[String]) -> TachyonBeam<'_> {
    TachyonBeam::new(input)
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
