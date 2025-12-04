/*
part 1:
we have a grid of papers and we need to check if they are reachable, they are only reachable if there is less than 4 other
papers in the 8 adjacent tiles, then we jus return the count of reachable papers
part 2:
we now remove the rolls that we can reach and we need to recheck if we can reach more papers

*/

use std::fs::File;
use std::io::{BufRead, BufReader};

struct PaperGrid(Vec<Vec<bool>>);

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
        let mut grid = Vec::new();
        for y in input {
            {
                let y_vec = y.chars().map(|c| c == '@').collect();
                grid.push(y_vec);
            }
        }
        PaperGrid(grid)
    }

    fn find_reachable(&self) -> Vec<(usize, usize)> {
        let mut reachable = Vec::new();

        for (y, y_vec) in self.0.iter().enumerate() {
            for (x, x_paper) in y_vec.iter().enumerate() {
                if !x_paper {
                    continue;
                }
                let mut total_adjacent = 0;
                for (y_diff, x_diff) in ADJACENT {
                    let has_paper = match self
                        .0
                        .get((y as i32 + y_diff) as usize)
                        .and_then(|row| row.get((x as i32 + x_diff) as usize))
                    {
                        Some(true) => 1,
                        None | Some(false) => 0, // out of bounds or in bounds and doesnt have paper
                    };

                    total_adjacent += has_paper;
                }

                if total_adjacent < 4 {
                    reachable.push((y, x));
                }
            }
        }

        reachable
    }

    fn find_and_remove(mut self) -> u32 {
        let mut reachable = 0;

        loop {
            let reached_papers = self.find_reachable();

            let reached = reached_papers.len();
            if reached == 0 {
                break;
            }

            reachable += reached;

            for (y, x) in reached_papers {
                self.0[y][x] = false;
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

    let my_sum = solution_2(_my_input);
    dbg!(my_sum);
}

fn solution_1(input: &[String]) -> u32 {
    let paper_grid = parse_input(input);

    paper_grid.find_reachable().len() as u32
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
