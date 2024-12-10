/*
part 1:
we have a topographical map that includes values from 0 to 9,
we have to travel through it finding the longest path going from 0 to 9 where each step goes up by exactly 1
a trailhead is the start of the paths, 0, and we have to check all trailheads and count the number of unique tops that we can reach

part 2:

we have to get the total number of possible paths for each trail head that it can take to get to the destination

*/

use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Clone, Copy)]
struct Location {
    height: u8,
    cached: Option<u16>,
}

struct HikingMap {
    map: Vec<Vec<Location>>,
}

impl HikingMap {
    pub fn get(&self, location: (u8, u8)) -> Option<&Location> {
        let y = location.0 as usize;
        let x = location.1 as usize;
        self.map.get(y)?.get(x)
    }

    fn from_string(input: &[String]) -> Self {
        let default_loc = Location {
            height: 0,
            cached: None,
        };

        let mut hiking_map: Vec<Vec<Location>> =
            vec![vec![default_loc; input[0].len()]; input.len()];

        for (y, y_line) in input.iter().enumerate() {
            for (x, x_char) in y_line.chars().enumerate() {
                let height = x_char.to_digit(10).unwrap();

                let loc = Location {
                    height: height as u8,
                    cached: None,
                };

                hiking_map[y][x] = loc;
            }
        }

        HikingMap { map: hiking_map }
    }

    fn get_trailheads(&self) -> Vec<(u8, u8)> {
        let mut starting_location = vec![];
        for (y, y_line) in self.map.iter().enumerate() {
            for (x, loc) in y_line.iter().enumerate() {
                if loc.height == 0 {
                    starting_location.push((y as u8, x as u8));
                }
            }
        }

        starting_location
    }

    fn has_reached_unique_top(
        &self,
        current_coord: (u8, u8),
        prev_value: &u8,
    ) -> HashSet<(u8, u8)> {
        match self.get(current_coord) {
            Some(current_value) => {
                let curr_height = current_value.height;
                if curr_height != (prev_value + 1) {
                    return HashSet::new();
                }

                let mut return_set = HashSet::new();

                if curr_height == 9 {
                    return_set.insert(current_coord);
                    return return_set;
                }

                let up_count = self
                    .has_reached_unique_top((current_coord.0 - 1, current_coord.1), &curr_height);
                let right_count = self
                    .has_reached_unique_top((current_coord.0, current_coord.1 + 1), &curr_height);
                let down_count = self
                    .has_reached_unique_top((current_coord.0 + 1, current_coord.1), &curr_height);
                let left_count = self
                    .has_reached_unique_top((current_coord.0, current_coord.1 - 1), &curr_height);

                return_set.extend(up_count);
                return_set.extend(right_count);
                return_set.extend(down_count);
                return_set.extend(left_count);

                return_set
            }
            None => HashSet::new(),
        }
    }

    fn has_reached_top(&mut self, current_coord: (u8, u8), prev_value: &u8) -> u16 {
        match self.get(current_coord) {
            Some(current_location) => {
                let curr_height = current_location.height;
                if curr_height != (prev_value + 1) {
                    return 0;
                }

                if curr_height == 9 {
                    return 1;
                }

                if let Some(cached_result) = current_location.cached {
                    return cached_result;
                }
                let up_count =
                    self.has_reached_top((current_coord.0 - 1, current_coord.1), &curr_height);
                let right_count =
                    self.has_reached_top((current_coord.0, current_coord.1 + 1), &curr_height);
                let down_count =
                    self.has_reached_top((current_coord.0 + 1, current_coord.1), &curr_height);
                let left_count =
                    self.has_reached_top((current_coord.0, current_coord.1 - 1), &curr_height);

                let total_count = up_count + right_count + down_count + left_count;

                self.map[current_coord.0 as usize][current_coord.1 as usize].cached =
                    Some(total_count);

                total_count
            }
            None => 0,
        }
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

    let example_sum = path_count_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 36);

    let my_sum = path_count_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = path_count_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 81);

    let start = Instant::now();
    let my_sum = path_count_2(_my_input);
    dbg!(start.elapsed());
    dbg!(my_sum);
}

fn path_count_1(input: &[String]) -> u32 {
    let hiking_map = HikingMap::from_string(input);

    let starting_coords = hiking_map.get_trailheads();

    let trail_count: usize = starting_coords
        .par_iter()
        .map(|start| {
            let mut total_trails = HashSet::new();
            let up_count = hiking_map.has_reached_unique_top((start.0 - 1, start.1), &0);
            let right_count = hiking_map.has_reached_unique_top((start.0, start.1 + 1), &0);
            let down_count = hiking_map.has_reached_unique_top((start.0 + 1, start.1), &0);
            let left_count = hiking_map.has_reached_unique_top((start.0, start.1 - 1), &0);

            total_trails.extend(up_count);
            total_trails.extend(right_count);
            total_trails.extend(down_count);
            total_trails.extend(left_count);

            total_trails.len()
        })
        .sum();

    trail_count as u32
}

fn path_count_2(input: &[String]) -> u32 {
    let mut hiking_map = HikingMap::from_string(input);

    let starting_coords = hiking_map.get_trailheads();

    let mut trail_count = 0;

    for start in starting_coords {
        let up_count = hiking_map.has_reached_top((start.0 - 1, start.1), &0);
        let right_count = hiking_map.has_reached_top((start.0, start.1 + 1), &0);
        let down_count = hiking_map.has_reached_top((start.0 + 1, start.1), &0);
        let left_count = hiking_map.has_reached_top((start.0, start.1 - 1), &0);

        trail_count += up_count + right_count + down_count + left_count;
    }

    trail_count as u32
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
