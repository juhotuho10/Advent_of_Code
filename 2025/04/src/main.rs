/*
part 1:
we have a grid of papers and we need to check if they are reachable, they are only reachable if there is less than 4 other
papers in the 8 adjacent tiles, then we jus return the count of reachable papers
part 2:
we now remove the rolls that we can reach and we need to recheck if we can reach more papers

*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

struct PaperGrid(Vec<bool>, usize);

const ADJACENT: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl PaperGrid {
    fn new(input: &[String]) -> Self {
        let len = input[0].len();

        let grid = input
            .iter()
            .flat_map(|s| s.chars().map(|c| c == '@').collect::<Vec<bool>>())
            .collect();

        PaperGrid(grid, len)
    }

    fn find_reachable(&self, search_space: &Vec<usize>) -> Vec<usize> {
        let mut reachable = Vec::new();

        let input_range = 0..self.1;
        for index in search_space {
            let y = index / self.1;
            let x = index % self.1;
            let total_adjacent = ADJACENT
                .iter()
                .filter(|(y_diff, x_diff)| {
                    let new_y = (y as i32 + y_diff) as usize;
                    let new_x = (x as i32 + x_diff) as usize;
                    let new_index = new_y * self.1 + new_x;
                    // the location is in the map and has paper
                    input_range.contains(&new_y)
                        && input_range.contains(&new_x)
                        && self.0[new_index]
                })
                .count();

            if total_adjacent < 4 {
                reachable.push(*index);
            }
        }

        reachable.sort();
        reachable.dedup();

        reachable
    }

    fn find_and_remove(mut self) -> u32 {
        let mut reachable = 0;
        let input_range = 0..self.1;

        let mut seach_papers = Vec::new();

        for (i, paper) in self.0.iter().enumerate() {
            if *paper {
                seach_papers.push(i);
            }
        }

        loop {
            let reached_papers = self.find_reachable(&seach_papers);

            if reached_papers.is_empty() {
                break;
            }

            reachable += reached_papers.len();

            for index in &reached_papers {
                self.0[*index] = false;
            }

            seach_papers.clear();

            for index in reached_papers {
                let y = index / self.1;
                let x = index % self.1;

                for (y_diff, x_diff) in ADJACENT {
                    let new_y = (y as i32 + y_diff) as usize;
                    let new_x = (x as i32 + x_diff) as usize;
                    let new_index = new_y * self.1 + new_x;
                    if input_range.contains(&new_y)
                        && input_range.contains(&new_x)
                        && self.0[new_index]
                    {
                        seach_papers.push(new_index);
                    };
                }
            }
        }

        reachable as u32
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
    assert_eq!(example_sum, 13);

    let my_sum = solution_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 43);

    let start = Instant::now();
    let my_sum = solution_2(_my_input);
    let dur = start.elapsed().as_micros();
    println!("elapsed: {dur} micros");
    dbg!(my_sum);
}

fn solution_1(input: &[String]) -> u32 {
    let paper_grid = parse_input(input);
    let seach_papers = paper_grid
        .0
        .iter()
        .enumerate()
        .filter(|(_, paper)| **paper)
        .map(|(index, _)| index)
        .collect();

    paper_grid.find_reachable(&seach_papers).len() as u32
}

fn solution_2(input: &[String]) -> u32 {
    let paper_grid = parse_input(input);

    paper_grid.find_and_remove()
}

fn parse_input(input: &[String]) -> PaperGrid {
    PaperGrid::new(input)
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
