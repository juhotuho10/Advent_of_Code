/*
part 1:
we have a collection of instruction, we dig a trench according to the instructions, then we hollow it out and get the total volume of the trench

part 2:
the instructions were messed up and we need to parse the hex code given as the wall color to be the the direction and color
*/

use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    U,
    R,
    D,
    L,
}

#[derive(Debug, Clone)]
struct Corner {
    dir: Dir,
    corner: Coord,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Copy)]
struct Coord {
    x: i64,
    y: i64,
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

    fn go_dir(&self, dir: Dir) -> Self {
        match dir {
            Dir::U => self.up(),
            Dir::R => self.right(),
            Dir::D => self.down(),
            Dir::L => self.left(),
        }
    }
}

struct CornerTracer {
    prev_dir: Option<Dir>,
    current_pos: Coord,
    outside_pos: Coord,
    pos_visited: Vec<Coord>,
}

impl CornerTracer {
    fn new() -> Self {
        let default_coord = Coord { x: 0, y: 0 };
        CornerTracer {
            prev_dir: None,
            current_pos: default_coord,
            outside_pos: default_coord,
            pos_visited: vec![default_coord],
        }
    }

    fn go_to_corner(&mut self, inst: Instruction) {
        // 0,0 - 0,0 |   -> R | None
        // 0,7 - 1,0 | R -> D | Right
        // 6,7 - 1,1 | D -> L | Right
        // 6,5 - 1,1 | L -> D | Left
        // 7,5 - 1,0 | D -> R | Left
        // 7,7 - 1,0 | R -> D | Right
        // 10,7- 1,1 | D -> L | Right

        let turn: Option<Turn> = match (self.prev_dir, inst.dir) {
            (None, _) => None,
            (Some(Dir::U), Dir::L)
            | (Some(Dir::L), Dir::D)
            | (Some(Dir::D), Dir::R)
            | (Some(Dir::R), Dir::U) => Some(Turn::Left),
            (Some(Dir::U), Dir::R)
            | (Some(Dir::R), Dir::D)
            | (Some(Dir::D), Dir::L)
            | (Some(Dir::L), Dir::U) => Some(Turn::Right),
            (_, _) => unreachable!(),
        };

        self.prev_dir = Some(inst.dir);

        match inst.dir {
            Dir::U => self.current_pos.y -= inst.count as i64,

            Dir::R => self.current_pos.x += inst.count as i64,

            Dir::D => self.current_pos.y += inst.count as i64,

            Dir::L => self.current_pos.x -= inst.count as i64,
        }

        self.outside_pos = match turn {
            None => self.outside_pos,
            Some(Turn::Left) => match self.outside_pos {
                Coord { x: 0, y: 0 } => Coord { x: 0, y: 1 },
                Coord { x: 0, y: 1 } => Coord { x: 1, y: 1 },
                Coord { x: 1, y: 1 } => Coord { x: 1, y: 0 },
                Coord { x: 1, y: 0 } => Coord { x: 1, y: 0 }, // exception
                _ => unreachable!(),
            },
            Some(Turn::Right) => match self.outside_pos {
                Coord { x: 0, y: 0 } => Coord { x: 1, y: 0 },
                Coord { x: 1, y: 0 } => Coord { x: 1, y: 1 },
                Coord { x: 1, y: 1 } => Coord { x: 0, y: 1 },
                Coord { x: 0, y: 1 } => Coord { x: 0, y: 1 }, // exception

                _ => unreachable!(),
            },
        };

        let last_visited = self.pos_visited.last_mut().unwrap();
        last_visited.x += self.outside_pos.x;
        last_visited.y += self.outside_pos.y;

        //dbg!(self.outside_pos);
        dbg!(turn);

        self.pos_visited.push(self.current_pos);
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

fn get_trench_volume_1(input: &[String]) -> u64 {
    let instructions = parse_input_1(input);

    dbg!(&instructions);

    //dbg!(parse_input_to_corners(instructions));

    let mut corner_tracer = CornerTracer::new();

    for ins in instructions {
        corner_tracer.go_to_corner(ins);
    }

    dbg!(corner_tracer.pos_visited);

    0
}

fn get_trench_volume_2(input: &[String]) -> u64 {
    let instructions = parse_input_2(input);

    dbg!(parse_input_to_corners(instructions));

    0
}

fn parse_input_1(input: &[String]) -> Vec<Instruction> {
    input
        .iter()
        .map(|s| Instruction::from_string_1(s))
        .collect()
}

fn parse_input_2(input: &[String]) -> Vec<Instruction> {
    input
        .iter()
        .map(|s| Instruction::from_string_2(s))
        .collect()
}

fn parse_input_to_corners(instructions: Vec<Instruction>) -> Vec<Corner> {
    let mut current_coords = (0, 0);
    let mut coords: Vec<Corner> = vec![];

    for inst in &instructions {
        match inst.dir {
            Dir::U => current_coords.1 -= inst.count as i64,

            Dir::R => current_coords.0 += inst.count as i64,

            Dir::D => current_coords.1 += inst.count as i64,

            Dir::L => current_coords.0 -= inst.count as i64,
        }

        let new_corner = Corner {
            dir: inst.dir,
            corner: Coord {
                x: current_coords.0,
                y: current_coords.1,
            },
        };

        coords.push(new_corner);
    }

    coords
}

fn area_from_corners(corners: &[(i64, i64)]) -> u64 {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for coord_pair in corners.windows(2) {
        let coords_1 = coord_pair[0];
        let coords_2 = coord_pair[1];

        sum1 += coords_1.0 * coords_2.1;
        sum2 += coords_1.1 * coords_2.0;
    }

    (sum1.abs_diff(sum2) as f64 / 2.0) as u64
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
