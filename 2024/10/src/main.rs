/*
part 1:
we have a topographical map that includes values from 0 to 9,
we have to travel through it finding the longest path going from 0 to 9 where each step goes up by exactly 1
a trailhead is the start of the paths, 0, and we have to check all trailheads and count the number of unique tops that we can reach

part 2:

we have to get the total number of possible paths for each trail head that it can take to get to the destination

*/

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    x: u8,
    y: u8,
}

impl Coord {
    fn up(&self) -> Self {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn right(&self) -> Self {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn down(&self) -> Self {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn left(&self) -> Self {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }
}

struct HikingMap {
    map: HashMap<Coord, u8>,
}

impl HikingMap {
    fn from_string(input: &[String]) -> Self {
        let mut hiking_map: HashMap<Coord, u8> = HashMap::new();
        for (y, y_line) in input.iter().enumerate() {
            for (x, x_char) in y_line.chars().enumerate() {
                let current_coord = Coord {
                    x: x as u8,
                    y: y as u8,
                };

                let height = x_char.to_digit(10).unwrap();

                hiking_map.insert(current_coord, height as u8);
            }
        }

        HikingMap { map: hiking_map }
    }

    fn get_trailheads(&self) -> Vec<Coord> {
        self.map
            .clone()
            .into_iter()
            .filter(|(_, value)| value == &0)
            .map(|(key, _)| key)
            .collect()
    }

    fn has_reached_unique_top(&self, current_coord: Coord, prev_value: &u8) -> HashSet<Coord> {
        match self.map.get(&current_coord) {
            Some(current_value) => {
                if *current_value != (prev_value + 1) {
                    return HashSet::new();
                }

                let mut return_set = HashSet::new();

                if *current_value == 9 {
                    return_set.insert(current_coord);
                    return return_set;
                }

                let up_count = self.has_reached_unique_top(current_coord.up(), current_value);
                let right_count = self.has_reached_unique_top(current_coord.right(), current_value);
                let down_count = self.has_reached_unique_top(current_coord.down(), current_value);
                let left_count = self.has_reached_unique_top(current_coord.left(), current_value);

                return_set.extend(up_count);
                return_set.extend(right_count);
                return_set.extend(down_count);
                return_set.extend(left_count);

                return_set
            }
            None => HashSet::new(),
        }
    }

    fn has_reached_top(
        &self,
        current_coord: Coord,
        prev_value: &u8,
        memo: &mut Vec<Vec<Option<u16>>>,
    ) -> u16 {
        match self.map.get(&current_coord) {
            Some(current_value) => {
                if *current_value != (prev_value + 1) {
                    return 0;
                }

                if *current_value == 9 {
                    return 1;
                }

                if let Some(cached_result) =
                    memo[current_coord.y as usize][current_coord.x as usize]
                {
                    return cached_result;
                }
                let up_count = self.has_reached_top(current_coord.up(), current_value, memo);
                let right_count = self.has_reached_top(current_coord.right(), current_value, memo);
                let down_count = self.has_reached_top(current_coord.down(), current_value, memo);
                let left_count = self.has_reached_top(current_coord.left(), current_value, memo);

                let total_count = up_count + right_count + down_count + left_count;

                memo[current_coord.y as usize][current_coord.x as usize] = Some(total_count);

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

    let mut trail_count = 0;

    for start in starting_coords {
        let mut total_trails = HashSet::new();
        let up_count = hiking_map.has_reached_unique_top(start.up(), &0);
        let right_count = hiking_map.has_reached_unique_top(start.right(), &0);
        let down_count = hiking_map.has_reached_unique_top(start.down(), &0);
        let left_count = hiking_map.has_reached_unique_top(start.left(), &0);

        total_trails.extend(up_count);
        total_trails.extend(right_count);
        total_trails.extend(down_count);
        total_trails.extend(left_count);

        trail_count += total_trails.len();
    }

    trail_count as u32
}

fn path_count_2(input: &[String]) -> u32 {
    let hiking_map = HikingMap::from_string(input);

    let starting_coords = hiking_map.get_trailheads();

    let mut trail_count = 0;

    let height = input.len();
    let width = input[0].len();

    let mut memo = vec![vec![None; width]; height];

    for start in starting_coords {
        let up_count = hiking_map.has_reached_top(start.up(), &0, &mut memo);
        let right_count = hiking_map.has_reached_top(start.right(), &0, &mut memo);
        let down_count = hiking_map.has_reached_top(start.down(), &0, &mut memo);
        let left_count = hiking_map.has_reached_top(start.left(), &0, &mut memo);

        trail_count += up_count + right_count + down_count + left_count;
    }

    trail_count as u32
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
