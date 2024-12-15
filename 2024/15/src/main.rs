/*
part 1:
we have a warehouse full of robots that move.
we know the robot movements but the robot movements fail sometimes if the box or the robot is blocked by the wall

we have a long set of moves that the robot will move according to and see what state the warehouse will end up in

the answer is the box GPS coordinates, 100 points per the coords from the top of the board and point per the coords from the left of the warehouse

sum up all the box coordinate points and return the answer

part 2:

all the boxes are expanded to be 2 parts and the coordinates are stretched horisontally


*/

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    BoxL,
    BoxR,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: u32,
    y: u32,
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

    fn get_dir(&self, dir: &char) -> Self {
        match dir {
            '^' => self.up(),
            '>' => self.right(),
            'v' => self.down(),
            '<' => self.left(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Warehouse {
    robot: Coord,
    boxes: HashMap<Coord, Option<Tile>>,
    moves: VecDeque<char>,
}

impl Warehouse {
    fn from_string(input: &[String], expand: bool) -> Self {
        let input = input.to_vec();
        let mut warehouse_lines = vec![];
        let mut moves = VecDeque::new();

        let mut warehouse_part = true;

        for mut line in input {
            if line.is_empty() {
                warehouse_part = false;
                continue;
            }

            if warehouse_part {
                if expand {
                    line = line
                        .replace("#", "##")
                        .replace("O", "[]")
                        .replace(".", "..")
                        .replace("@", "@.")
                }
                warehouse_lines.push(line);
            } else {
                moves.extend(line.chars());
            }
        }

        let mut warehouse_tiles = HashMap::new();

        let mut robot_coords = Coord { x: 0, y: 0 };

        for (y, y_line) in warehouse_lines.iter().enumerate() {
            for (x, x_char) in y_line.char_indices() {
                let current_coord = Coord {
                    x: x as u32,
                    y: y as u32,
                };
                let tile: Option<Tile> = match x_char {
                    '.' => None,
                    '@' => {
                        robot_coords = current_coord.clone();
                        None
                    }
                    '#' => Some(Tile::Wall),
                    '[' => Some(Tile::BoxL),
                    ']' => Some(Tile::BoxR),
                    'O' => Some(Tile::Box),
                    _ => unreachable!(),
                };

                warehouse_tiles.insert(current_coord, tile);
            }
        }

        Warehouse {
            robot: robot_coords,
            boxes: warehouse_tiles,
            moves,
        }
    }

    fn arrange_warehouse(&mut self) {
        while let Some(dir) = &self.moves.pop_front() {
            let next_coord = self.robot.get_dir(dir);

            let mut movable_tiles = HashSet::new();

            let moved = self.search_boxes(next_coord.clone(), dir, &mut movable_tiles);

            if moved {
                // take all the coordinates for the boxes that are meant to be moved
                // and turn in into (coord, box tile)
                let mut coord_boxes: Vec<(Coord, Tile)> = movable_tiles
                    .into_iter()
                    .map(|coord| {
                        (
                            coord.clone(),
                            self.boxes.get(&coord).unwrap().clone().unwrap(),
                        )
                    })
                    .collect();

                // turn all the tiles in the hashmap that are mean to be moved to not have tiles in them
                coord_boxes.iter().for_each(|(coord, _)| {
                    self.boxes.insert(coord.clone(), None);
                });

                // increment all the tile coords to the direction that we are moving to for the boxes that we took out of the hashmap
                coord_boxes
                    .iter_mut()
                    .for_each(|(coord, _)| *coord = coord.get_dir(dir));

                // insert all the boxes with the moved coords back in to the hashmap
                coord_boxes.into_iter().for_each(|(coord, tile)| {
                    self.boxes.insert(coord.clone(), Some(tile));
                });

                // move robot
                self.boxes.insert(next_coord.clone(), None);
                self.robot = next_coord;
            }
        }
    }

    fn search_boxes(&self, curr_coord: Coord, dir: &char, box_coords: &mut HashSet<Coord>) -> bool {
        let curr_tile = self.boxes.get(&curr_coord);

        match curr_tile {
            Some(pos) => {
                match pos {
                    None => {
                        true // in hashmap, empty coord, can move here
                    }

                    Some(Tile::Wall) => {
                        false // wall, cannot move here
                    }

                    Some(box_tile) => {
                        if box_coords.insert(curr_coord.clone()) {
                            // new tile, not previously visited
                            match box_tile {
                                Tile::Box => {
                                    let next_coord = curr_coord.get_dir(dir);

                                    self.search_boxes(next_coord, dir, box_coords)
                                }
                                Tile::BoxL => {
                                    let next_coord = curr_coord.get_dir(dir);
                                    let other_side_coord = curr_coord.right();

                                    self.search_boxes(next_coord, dir, box_coords)
                                        && self.search_boxes(other_side_coord, dir, box_coords)
                                }
                                Tile::BoxR => {
                                    let next_coord = curr_coord.get_dir(dir);
                                    let other_side_coord = curr_coord.left();

                                    self.search_boxes(next_coord, dir, box_coords)
                                        && self.search_boxes(other_side_coord, dir, box_coords)
                                }

                                Tile::Wall => unreachable!(),
                            }
                        } else {
                            // tile already checked
                            true
                        }
                    }
                }
            }
            None => false, // not in hashmap, cannot move to this spot
        }
    }

    fn get_box_sum(&self) -> u32 {
        self.boxes
            .iter()
            .filter(|(_, val)| **val == Some(Tile::Box) || **val == Some(Tile::BoxL))
            .map(|(coord, _)| coord.y * 100 + coord.x)
            .sum()
    }

    fn pretty_print(&self) {
        let max_x = self.boxes.keys().map(|coord| coord.x).max().unwrap_or(0);
        let max_y = self.boxes.keys().map(|coord| coord.y).max().unwrap_or(0);

        for y in 0..=max_y {
            for x in 0..=max_x {
                let coord = Coord { x, y };
                if self.robot == coord {
                    print!("@");
                } else if let Some(Some(tile)) = self.boxes.get(&coord) {
                    match tile {
                        Tile::Wall => print!("#"),
                        Tile::Box => print!("O"),
                        Tile::BoxL => print!("["),
                        Tile::BoxR => print!("]"),
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
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

    let example_sum = warehouse_box_sum_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 10092);

    let my_sum = warehouse_box_sum_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 9021);

    let my_sum = solution_2(_my_input);
    dbg!(my_sum);
}

fn warehouse_box_sum_1(input: &[String]) -> u32 {
    let mut warehouse = Warehouse::from_string(input, false);

    warehouse.pretty_print();
    warehouse.arrange_warehouse();
    warehouse.pretty_print();

    warehouse.get_box_sum()
}

fn solution_2(input: &[String]) -> u32 {
    let mut warehouse = Warehouse::from_string(input, true);
    warehouse.pretty_print();
    warehouse.arrange_warehouse();
    warehouse.pretty_print();

    warehouse.get_box_sum()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
