/*
we have a valley of ash (.) and rocks (#) filled with mirrors
we have to figure out where there are mirrors based on the valley terrain
to find mirrors, we we check for perfect reflections across vertical line between 2 columns

for the answer, we add up the number of columns to the left of mirrors
and we add 100 x columns above the reflection lines

part 2:
every mirror has a smudge, in the reflection there is a single (.) or (#) that should be the opposite
we have to used the fixed reflection even if the old one keeps being valid with the smudge fixed

*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

struct Valley {
    rows: Vec<String>,
    columns: Vec<String>,
}

impl Valley {
    fn from_string(input: &[String]) -> Self {
        let max_length = input.iter().map(|row| row.len()).max().unwrap_or(0);

        let mut columns = vec![String::new(); max_length];

        for row in input {
            for (i, ch) in row.chars().enumerate() {
                columns[i].push(ch);
            }
        }

        Valley {
            rows: input.to_vec(),
            columns,
        }
    }

    fn get_mirror_points(&self) -> (Option<Vec<usize>>, Option<Vec<usize>>) {
        let mut row_mirros: Vec<usize> = vec![];
        let mut columns_mirrors: Vec<usize> = vec![];

        for (i, row_pair) in self.rows.windows(2).enumerate() {
            let diff_count = row_pair[0]
                .chars()
                .zip(row_pair[1].chars())
                .filter(|(c1, c2)| c1 != c2)
                .count();

            if diff_count < 2 {
                row_mirros.push(i + 1);
            }
        }

        for (i, col_pair) in self.columns.windows(2).enumerate() {
            let diff_count = col_pair[0]
                .chars()
                .zip(col_pair[1].chars())
                .filter(|(c1, c2)| c1 != c2)
                .count();

            if diff_count < 2 {
                columns_mirrors.push(i + 1);
            }
        }

        // None if empty
        let row_spits = (!row_mirros.is_empty()).then_some(row_mirros);
        let col_spits = (!columns_mirrors.is_empty()).then_some(columns_mirrors);

        (row_spits, col_spits)
    }

    fn get_points(&self, diff: u32) -> u32 {
        let mut points = 0;
        let (possible_rows, possible_columns) = self.get_mirror_points();

        if let Some(row_locations) = possible_rows {
            for split_point in row_locations {
                let slice = self.rows.as_slice();
                let (left, right) = slice.split_at(split_point);

                let min_size = left.len().min(right.len());
                let same_count: usize = left
                    .iter()
                    .rev()
                    .zip(right.iter())
                    .take(min_size)
                    .map(|(l, r)| l.chars().zip(r.chars()).filter(|(lc, rc)| lc != rc).count())
                    .sum();

                if same_count == diff as usize {
                    points += 100 * split_point;
                    break;
                }
            }
        }

        if let Some(col_locations) = possible_columns {
            for split_point in col_locations {
                let slice = self.columns.as_slice();
                let (left, right) = slice.split_at(split_point);

                let min_size = left.len().min(right.len());
                let same_count: usize = left
                    .iter()
                    .rev()
                    .zip(right.iter())
                    .take(min_size)
                    .map(|(l, r)| l.chars().zip(r.chars()).filter(|(lc, rc)| lc != rc).count())
                    .sum();

                if same_count == diff as usize {
                    points += split_point;
                    break;
                }
            }
        }

        points as u32
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

    let example_sum = get_mirror_sum_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 405);

    let my_sum = get_mirror_sum_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = get_mirror_sum_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 400);

    let my_sum = get_mirror_sum_2(_my_input);
    dbg!(my_sum);
}

fn parse_input(input: &[String]) -> Vec<Valley> {
    let mut valleys = Vec::new();
    let mut current_group = Vec::new();

    for line in input {
        if line.is_empty() {
            if !current_group.is_empty() {
                valleys.push(current_group);
                current_group = Vec::new();
            }
        } else {
            current_group.push(line.clone());
        }
    }

    if !current_group.is_empty() {
        valleys.push(current_group);
    }

    valleys
        .iter()
        .map(|valley| Valley::from_string(valley))
        .collect()
}

fn get_mirror_sum_1(input: &[String]) -> u32 {
    let all_valleys = parse_input(input);

    let points: Vec<u32> = all_valleys
        .iter()
        .map(|valley| valley.get_points(0))
        .collect();

    points.iter().sum()
}

fn get_mirror_sum_2(input: &[String]) -> u32 {
    let all_valleys = parse_input(input);

    let points: Vec<u32> = all_valleys
        .iter()
        .map(|valley| valley.get_points(1))
        .collect();

    points.iter().sum()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
