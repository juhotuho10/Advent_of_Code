/*
part 1:
we have a grid of letters we have to find how many times the word XMAS appears on it
part 2:

We now instead have to find the word X-MAS, where we have 2 MAS shaped in a X shape overlapping eachother

*/

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct WordGrid {
    grid: Vec<String>,
    vertical_grid: Vec<String>,
}

impl WordGrid {
    fn from_strings(input: &[String]) -> Self {
        let rotated = WordGrid::rotate_strings(input.to_owned());

        WordGrid {
            grid: input.to_owned(),
            vertical_grid: rotated,
        }
    }

    fn get_xmas_count(&self) -> u32 {
        let re = Regex::new(r"XMAS").unwrap();

        let mut total_xmas = 0;

        for line in &self.grid {
            let horisontal_xmas_count = re.find_iter(line).count();
            total_xmas += horisontal_xmas_count;
        }

        for line in &self.vertical_grid {
            let vertical_xmas_count = re.find_iter(line).count();
            total_xmas += vertical_xmas_count;
        }

        let re = Regex::new(r"SAMX").unwrap();

        for line in &self.grid {
            let horisontal_xmas_count = re.find_iter(line).count();
            total_xmas += horisontal_xmas_count;
        }

        for line in &self.vertical_grid {
            let vertical_xmas_count = re.find_iter(line).count();
            total_xmas += vertical_xmas_count;
        }

        let diag_xmas_count = self.search_grid_for_xmas() as usize;
        total_xmas += diag_xmas_count;

        total_xmas as u32
    }

    fn search_grid_for_2mas(&self) -> u32 {
        let mut total_2mas_count = 0;
        for start_y in 0..=(self.grid.len() - 3) {
            let end_y = start_y + 3;

            for start_x in 0..=(self.grid[0].len() - 3) {
                let end_x = start_x + 3;
                let y_grid: Vec<String> = self.grid[start_y..end_y]
                    .iter()
                    .map(|s| s[start_x..end_x].to_owned())
                    .collect();

                let (diag_1, diag_2) = WordGrid::get_diag_x(y_grid);

                if (diag_1 == "SAM" || diag_1 == "MAS") && (diag_2 == "SAM" || diag_2 == "MAS") {
                    total_2mas_count += 1;
                }
            }
        }

        total_2mas_count
    }

    fn search_grid_for_xmas(&self) -> u32 {
        let mut total_xmas_count = 0;
        for start_y in 0..=(self.grid.len() - 4) {
            let end_y = start_y + 4;

            for start_x in 0..=(self.grid[0].len() - 4) {
                let end_x = start_x + 4;
                let y_grid: Vec<String> = self.grid[start_y..end_y]
                    .iter()
                    .map(|s| s[start_x..end_x].to_owned())
                    .collect();

                let new_xmas_count = WordGrid::search_grid_square_for_xmas(y_grid);
                total_xmas_count += new_xmas_count;
            }
        }

        total_xmas_count
    }
    fn search_grid_square_for_xmas(grid_input: Vec<String>) -> u32 {
        assert_eq!(grid_input.len(), 4);
        assert_eq!(grid_input[0].len(), 4);

        let mut xmas_count = 0;

        let diag = WordGrid::get_diag_line(&grid_input);

        if diag == "XMAS" || diag == "SAMX" {
            xmas_count += 1;
        }

        let rotated = WordGrid::rotate_strings(grid_input);

        let rotated_diag = WordGrid::get_diag_line(&rotated);

        if rotated_diag == "XMAS" || rotated_diag == "SAMX" {
            xmas_count += 1;
        }

        xmas_count
    }

    fn get_diag_x(input: Vec<String>) -> (String, String) {
        let diag_1 = WordGrid::get_diag_line(&input);

        let reversed: Vec<String> = input
            .into_iter()
            .map(|s| s.chars().rev().collect())
            .collect();

        let diag_2 = WordGrid::get_diag_line(&reversed);

        (diag_1, diag_2)
    }

    fn get_diag_line(input: &[String]) -> String {
        let mut diag = "".to_owned();

        for (i, str) in input.iter().enumerate() {
            diag += &str.chars().nth(i).unwrap().to_string();
        }

        diag
    }

    fn rotate_strings(input: Vec<String>) -> Vec<String> {
        let rows = input.len();
        let cols = input[0].len();
        let mut rotated: Vec<String> = vec![String::with_capacity(rows); cols];
        for s in input {
            for (col, char) in s.chars().rev().enumerate() {
                rotated[col].push(char);
            }
        }
        rotated
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

    let example_sum = xmas_count_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 18);

    let my_sum = xmas_count_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 9);

    let my_sum = solution_2(_my_input);
    dbg!(my_sum);
}

fn xmas_count_1(input: &[String]) -> u32 {
    let word_grid = WordGrid::from_strings(input);

    word_grid.get_xmas_count()
}

fn solution_2(input: &[String]) -> u32 {
    let word_grid = WordGrid::from_strings(input);

    word_grid.search_grid_for_2mas()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
