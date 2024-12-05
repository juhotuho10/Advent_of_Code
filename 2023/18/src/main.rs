/*
part 1:
we have a collection of instruction, we dig a trench according to the instructions, then we hollow it out and get the total volume of the trench

part 2:
the instructions were messed up and we need to parse the hex code given as the wall color to be the the direction and color
*/

use core::panic;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    U,
    R,
    D,
    L,
}
#[derive(Debug, Clone)]
struct Instruction {
    dir: Dir,
    count: u64,
}

impl Instruction {
    fn from_string_1(input: &str) -> Self {
        let re: Regex = Regex::new(r"([RDLU]) (\d+) \((\#[a-f\d]+)\)").unwrap();

        let captures = re.captures(input).unwrap();

        let dir = match &captures[1] {
            "U" => Dir::U,
            "R" => Dir::R,
            "D" => Dir::D,
            "L" => Dir::L,
            _ => unreachable!(),
        };

        Instruction {
            dir,
            count: captures[2].parse().unwrap(),
        }
    }

    fn from_string_2(input: &str) -> Self {
        let re: Regex = Regex::new(r"([RDLU]) (\d+) \((\#[a-f\d]+)\)").unwrap();

        let captures = re.captures(input).unwrap();
        let mut hex_string = captures[3].to_owned();
        hex_string.remove(0);
        let last_index = hex_string.len() - 1;

        let dir = match &hex_string.remove(last_index) {
            '0' => Dir::R,
            '1' => Dir::D,
            '2' => Dir::L,
            '3' => Dir::U,
            _ => unreachable!(),
        };

        let count: u64 = u32::from_str_radix(&hex_string, 16).unwrap() as u64;

        Instruction { dir, count }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn go_dir(&self, dir: Dir) -> Self {
        match dir {
            Dir::U => self.up(),
            Dir::R => self.right(),
            Dir::D => self.down(),
            Dir::L => self.left(),
        }
    }
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

#[derive(Debug, Clone)]
struct TrenchPart {
    dug: bool,
    wall_part: bool,
    outside: Option<bool>,
}

#[derive(Debug, Clone)]
struct Trench {
    instructions: Vec<Instruction>,
    trench_map: HashMap<Coord, TrenchPart>,
}

impl Trench {
    fn from_instructions(instructions: Vec<Instruction>) -> Self {
        let mut trench_map: HashMap<Coord, TrenchPart> = HashMap::new();
        let mut current_coord = Coord { x: 0, y: 0 };
        let starting_hole = TrenchPart {
            dug: true,
            wall_part: true,
            outside: Some(false),
        };

        trench_map.insert(current_coord.clone(), starting_hole);

        for inst in &instructions {
            for _ in 0..inst.count {
                current_coord = current_coord.go_dir(inst.dir);

                let starting_hole = TrenchPart {
                    dug: true,
                    wall_part: true,
                    outside: Some(false),
                };

                trench_map.insert(current_coord.clone(), starting_hole);
            }
        }

        let min_x = trench_map.keys().map(|coord| coord.x).min().unwrap();
        let min_y = trench_map.keys().map(|coord| coord.y).min().unwrap();

        let max_x = trench_map.keys().map(|coord| coord.x).max().unwrap();
        let max_y = trench_map.keys().map(|coord| coord.y).max().unwrap();

        // add outside ring for easier way to distinguish outside and inside parts
        for y in (min_y - 1)..=(max_y + 1) {
            for x in (min_x - 1)..=(max_x + 1) {
                let coord = Coord { x, y };

                trench_map.entry(coord).or_insert_with(|| TrenchPart {
                    dug: false,
                    wall_part: false,
                    outside: None,
                });
            }
        }

        Trench {
            instructions,
            trench_map,
        }
    }

    fn mark_outside(&mut self) {
        let start_coord = self.trench_map.keys().min().unwrap().to_owned();
        {
            let start_hole = self.trench_map.get_mut(&start_coord).unwrap();
            start_hole.outside = Some(true);
        }

        let mut checking_coords: VecDeque<Coord> = VecDeque::new();
        checking_coords.push_back(start_coord);

        let possible_dirs = [Dir::U, Dir::D, Dir::L, Dir::R];
        while let Some(coord) = checking_coords.pop_front() {
            for dir in possible_dirs {
                let dir_coord = coord.go_dir(dir);
                if let Some(dir_location) = self.trench_map.get_mut(&dir_coord) {
                    if dir_location.outside.is_none() {
                        dir_location.outside = Some(true);
                        checking_coords.push_back(dir_coord);
                    }
                }
            }
        }
    }

    fn dig_hole(&mut self) {
        for part in self.trench_map.values_mut() {
            if part.outside.is_none() {
                part.outside = Some(false);
                part.dug = true;
            }
        }
    }

    fn get_inside_count(&self) -> u64 {
        self.trench_map
            .values()
            .filter(|part| part.outside == Some(false))
            .count() as u64
    }

    fn print_trench(&self) {
        let min_x = self.trench_map.keys().map(|coord| coord.x).min().unwrap();
        let min_y = self.trench_map.keys().map(|coord| coord.y).min().unwrap();

        let max_x = self.trench_map.keys().map(|coord| coord.x).max().unwrap();
        let max_y = self.trench_map.keys().map(|coord| coord.y).max().unwrap();

        for y in min_y..=max_y {
            let mut line_string = "".to_owned();
            for x in min_x..=max_x {
                let coord = Coord { x, y };
                let current = self.trench_map.get(&coord).unwrap();
                match current.dug {
                    true => line_string += "#",
                    false => line_string += ".",
                }
            }

            println!("{}", &line_string);
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

    let example_volume = get_trench_volume_1(&example_1);
    dbg!(&example_volume);
    assert_eq!(example_volume, 62);

    let my_volume = get_trench_volume_1(_my_input);
    dbg!(my_volume);

    panic!();
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = get_trench_volume_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 952408144115);

    //let my_sum = get_trench_volume_2(_my_input);
    //dbg!(my_sum);
}

fn get_trench_volume_1(input: &[String]) -> u128 {
    //let mut trench = parse_input_1(input);
    //
    ////trench.print_trench();
    //trench.mark_outside();
    //trench.dig_hole();
    ////trench.print_trench();
    //trench.get_inside_count()

    let test_corners = [
        (0, 0),
        (0, 4),
        (2, 4),
        (2, 2),
        (3, 2),
        (3, 4),
        (4, 4),
        (4, 0),
        (0, 0),
    ];

    dbg!(area_from_corners(&test_corners));
    panic!();

    let corners = parse_input_to_corners_1(input);
    dbg!(&corners);
    calculate_buffered_area(&corners, 0.45)
}

fn get_trench_volume_2(input: &[String]) -> u64 {
    let mut trench = parse_input_2(input);

    trench.mark_outside();
    trench.dig_hole();
    trench.get_inside_count()
}

fn parse_input_1(input: &[String]) -> Trench {
    let instructions: Vec<Instruction> = input
        .iter()
        .map(|s| Instruction::from_string_1(s))
        .collect();

    Trench::from_instructions(instructions)
}

fn parse_input_to_corners_1(input: &[String]) -> Vec<(i64, i64)> {
    let instructions: Vec<Instruction> = input
        .iter()
        .map(|s| Instruction::from_string_1(s))
        .collect();

    let mut current_coords = (0, 0);
    let mut coords: Vec<(i64, i64)> = vec![current_coords];

    for inst in &instructions {
        match inst.dir {
            Dir::U => current_coords.1 -= inst.count as i64,

            Dir::R => current_coords.0 += inst.count as i64,

            Dir::D => current_coords.1 += inst.count as i64,

            Dir::L => current_coords.0 -= inst.count as i64,
        }

        coords.push(current_coords);
    }

    coords
}

fn area_from_corners(corners: &[(i64, i64)]) -> u128 {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for coord_pair in corners.windows(2) {
        let coords_1 = coord_pair[0];
        let coords_2 = coord_pair[1];

        sum1 += coords_1.0 * coords_2.1;
        sum2 += coords_1.1 * coords_2.0;
    }

    (sum1.abs_diff(sum2) as f64 / 2.0) as u128
}

fn calculate_buffered_area(corners: &[(i64, i64)], thickness: f64) -> u128 {
    let interior_area = area_from_corners(corners) as f64; // Shoelace Formula for interior area
    let mut edge_buffer_area = 0.0;
    let mut corner_buffer_area = 0.0;

    let n = corners.len();

    for i in 0..n - 1 {
        let (x1, y1) = corners[i];
        let (x2, y2) = corners[i + 1];

        // Length of the edge
        let edge_length = (((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f64).sqrt();

        // Buffer area for the edge
        edge_buffer_area += edge_length * thickness;
    }

    // Buffer area for the corners (approximate as squares)
    corner_buffer_area = n as f64 * (thickness * thickness);

    // Total area
    (interior_area + edge_buffer_area + corner_buffer_area) as u128
}

fn parse_input_2(input: &[String]) -> Trench {
    let instructions: Vec<Instruction> = input
        .iter()
        .map(|s| Instruction::from_string_2(s))
        .collect();

    Trench::from_instructions(instructions)
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
