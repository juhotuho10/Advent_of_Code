/*
we have a grid of pipes with an animal in them
the pipes a quaranteed to form a circle
there are also some pipes that arent connected to the current pipe system but we can ignore them
the pipes look like:
| north south
- west east
L north east
J north west
7 west south
F east south

we also have:
. empty ground tile
S the tile animal is on


We have to navigate through the loop and see how far we can get away from the animal in the loop and return that number
*/

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Coord {
    x: u32,
    y: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Dir {
    North(Coord),
    East(Coord),
    South(Coord),
    West(Coord),
}

#[derive(Debug, Clone)]
struct Tile {
    symbol: char,
    valid_moves: Vec<Dir>,
    original: bool,
    flooded: bool,
    done_flooding: bool,
    part_of_loop: bool,
}

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_distance = get_furthest_location_1(&example_1);
    dbg!(&example_distance);
    assert_eq!(example_distance, 8);

    let my_distance = get_furthest_location_1(_my_input);
    dbg!(my_distance);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_enclosed_tiles = get_enclosed_tiles_count_2(&example_2);
    dbg!(&example_enclosed_tiles);
    assert_eq!(example_enclosed_tiles, 10);

    let my_enclosed_tiles = get_enclosed_tiles_count_2(_my_input);
    dbg!(my_enclosed_tiles);
}

fn get_furthest_location_1(input: &[String]) -> u32 {
    let grid = parse_input(input);

    let (_, animal_tile) = grid
        .iter()
        .find(|(_, location)| location.symbol == 'S')
        .unwrap();

    let mut next_move: Dir = animal_tile.valid_moves[0];
    let mut next_coord = get_direction_coord(&next_move);
    let mut current_tile = grid.get(&next_coord).unwrap();
    let mut steps_taken = 1;

    while current_tile.symbol != 'S' {
        let valid_moves = remove_previous_dir(&current_tile.valid_moves, &next_move);
        next_move = valid_moves[0];
        next_coord = get_direction_coord(&next_move);
        steps_taken += 1;
        current_tile = grid.get(&next_coord).unwrap();
    }

    assert!(steps_taken % 2 == 0);
    steps_taken / 2
}

fn get_enclosed_tiles_count_2(input: &[String]) -> u32 {
    let mut grid: HashMap<Coord, Tile> = parse_input(input);
    grid = mark_the_main_loop(grid);
    let mut expanded_grid = expand_input(grid);

    let current = Coord { x: 0, y: 0 };
    let first_flooded = expanded_grid.get_mut(&current).unwrap();
    first_flooded.flooded = true;

    let mut changed;
    let surrounding = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    loop {
        changed = false;
        let available_flooding: Vec<Coord> = expanded_grid
            .iter()
            .filter(|(_, tile)| tile.flooded && !tile.done_flooding)
            .map(|(coord, _)| coord)
            .cloned()
            .collect();

        for coord in available_flooding {
            for (surr_x, surr_y) in surrounding {
                let new_coord = Coord {
                    x: (coord.x as i32 + surr_x) as u32,
                    y: (coord.y as i32 + surr_y) as u32,
                };

                if let Some(flood_tile) = expanded_grid.get_mut(&new_coord) {
                    if !flood_tile.part_of_loop {
                        flood_tile.flooded = true;
                    }
                };
            }

            let current_flooding = expanded_grid.get_mut(&coord).unwrap();
            current_flooding.done_flooding = true;
            changed = true;
        }

        if !changed {
            break;
        }
    }

    let flood_count = expanded_grid
        .values()
        .filter(|tile| !tile.part_of_loop && tile.original && !tile.flooded)
        .count();

    flood_count as u32
}

fn get_direction_coord(dir: &Dir) -> Coord {
    match dir {
        Dir::North(coord) | Dir::East(coord) | Dir::South(coord) | Dir::West(coord) => *coord,
    }
}

fn mark_the_main_loop(mut grid: HashMap<Coord, Tile>) -> HashMap<Coord, Tile> {
    let (_, animal_tile) = grid
        .iter_mut()
        .find(|(_, location)| location.symbol == 'S')
        .unwrap();

    animal_tile.part_of_loop = true;

    let mut next_move: Dir = animal_tile.valid_moves[0];
    let mut next_coord = get_direction_coord(&next_move);
    let mut current_tile = grid.get_mut(&next_coord).unwrap();
    current_tile.part_of_loop = true;

    while current_tile.symbol != 'S' {
        let valid_moves = remove_previous_dir(&current_tile.valid_moves, &next_move);

        next_move = valid_moves[0];
        next_coord = get_direction_coord(&next_move);
        current_tile = grid.get_mut(&next_coord).unwrap();
        current_tile.part_of_loop = true;
    }

    grid
}

fn expand_input(input: HashMap<Coord, Tile>) -> HashMap<Coord, Tile> {
    let mut new_hashmap: HashMap<Coord, Tile> = HashMap::new();
    for (key, value) in input {
        let new_key = Coord {
            x: key.x * 2 + 1,
            y: key.y * 2 + 1,
        };

        new_hashmap.insert(new_key, value);
    }

    let max_coord = *new_hashmap.keys().max().unwrap();

    for x in (2..max_coord.x).step_by(2) {
        for y in (1..=max_coord.y).step_by(2) {
            let current = Coord { x, y };
            let left = Coord { x: x - 1, y };
            let right = Coord { x: x + 1, y };

            let left_tile = new_hashmap.get(&left).unwrap();

            let new_tile = match left_tile.symbol {
                'F' | 'L' | '-' => Tile {
                    symbol: '-',
                    valid_moves: vec![Dir::West(left), Dir::East(right)],
                    original: false,
                    flooded: false,
                    done_flooding: false,
                    part_of_loop: left_tile.part_of_loop,
                },
                _ => Tile {
                    symbol: '.',
                    valid_moves: vec![],
                    original: false,
                    flooded: false,
                    done_flooding: false,
                    part_of_loop: false,
                },
            };

            new_hashmap.insert(current, new_tile);
        }
    }

    for x in (1..=max_coord.x).step_by(2) {
        for y in (2..=max_coord.y).step_by(2) {
            let current = Coord { x, y };
            let up = Coord { x, y: y - 1 };
            let down = Coord { x, y: y + 1 };

            let up_tile = new_hashmap.get(&up).unwrap();

            let new_tile = match up_tile.symbol {
                '|' | 'F' | '7' => Tile {
                    symbol: '|',
                    valid_moves: vec![Dir::North(up), Dir::South(down)],
                    original: false,
                    flooded: false,
                    done_flooding: false,
                    part_of_loop: up_tile.part_of_loop,
                },
                _ => Tile {
                    symbol: '.',
                    valid_moves: vec![],
                    original: false,
                    flooded: false,
                    done_flooding: false,
                    part_of_loop: false,
                },
            };

            new_hashmap.insert(current, new_tile);
        }
    }

    for x in 0..=max_coord.x + 1 {
        for y in 0..=max_coord.y + 1 {
            let current = Coord { x, y };
            new_hashmap.entry(current).or_insert_with(|| Tile {
                symbol: '.',
                valid_moves: vec![],
                original: false,
                flooded: false,
                done_flooding: false,
                part_of_loop: false,
            });
        }
    }

    let (s_coord, s_tile) = new_hashmap
        .iter()
        .find(|(_, tile)| tile.symbol == 'S')
        .map(|(coord, tile)| (*coord, tile.clone()))
        .unwrap();

    if s_tile
        .valid_moves
        .iter()
        .any(|dir| matches!(dir, Dir::East(_)))
    {
        let left_coord = Coord {
            x: s_coord.x + 1,
            y: s_coord.y,
        };

        let left_two_coord = Coord {
            x: s_coord.x,
            y: s_coord.y + 2,
        };

        let left_tile = new_hashmap.get_mut(&left_coord).unwrap();
        left_tile.symbol = '-';
        left_tile.part_of_loop = true;
        left_tile.valid_moves = vec![Dir::West(s_coord), Dir::East(left_two_coord)]
    }

    if s_tile
        .valid_moves
        .iter()
        .any(|dir| matches!(dir, Dir::South(_)))
    {
        let down_coord = Coord {
            x: s_coord.x,
            y: s_coord.y + 1,
        };

        let down_two_coord = Coord {
            x: s_coord.x,
            y: s_coord.y + 2,
        };

        let down_tile = new_hashmap.get_mut(&down_coord).unwrap();
        down_tile.symbol = '|';
        down_tile.part_of_loop = true;
        down_tile.valid_moves = vec![Dir::North(s_coord), Dir::South(down_two_coord)]
    }

    let mut all_rows = vec![];
    for y in 0..=max_coord.y + 1 {
        let mut current_string = "".to_owned();
        for x in 0..=max_coord.x + 1 {
            let current = Coord { x, y };
            let new_symbo = new_hashmap.get(&current).unwrap().symbol;
            current_string += &new_symbo.to_string();
        }

        all_rows.push(current_string);
    }

    for row in all_rows {
        dbg!(row);
    }

    new_hashmap
}

fn remove_previous_dir(directions: &[Dir], prev: &Dir) -> Vec<Dir> {
    let mut valid_directions = directions.to_vec();
    match prev {
        Dir::North(_) => valid_directions.retain(|&x| !matches!(x, Dir::South(_))),
        Dir::East(_) => valid_directions.retain(|&x| !matches!(x, Dir::West(_))),
        Dir::South(_) => valid_directions.retain(|&x| !matches!(x, Dir::North(_))),
        Dir::West(_) => valid_directions.retain(|&x| !matches!(x, Dir::East(_))),
    }
    valid_directions
}

fn parse_input(input: &[String]) -> HashMap<Coord, Tile> {
    let mut input_hashmap: HashMap<Coord, Tile> = HashMap::new();
    for (y, y_string) in input.iter().enumerate() {
        for (x, x_char) in y_string.chars().enumerate() {
            let current_coord = Coord {
                x: x as u32,
                y: y as u32,
            };

            let north_coord = Coord {
                x: x as u32,
                y: (y - 1) as u32,
            };
            let east_coord = Coord {
                x: (x + 1) as u32,
                y: y as u32,
            };
            let south_coord = Coord {
                x: x as u32,
                y: (y + 1) as u32,
            };
            let west_coord = Coord {
                x: (x - 1) as u32,
                y: y as u32,
            };

            let valid_moves: Vec<Dir> = match x_char {
                '|' => vec![Dir::North(north_coord), Dir::South(south_coord)],
                '-' => vec![Dir::East(east_coord), Dir::West(west_coord)],
                'L' => vec![Dir::North(north_coord), Dir::East(east_coord)],
                'J' => vec![Dir::North(north_coord), Dir::West(west_coord)],
                '7' => vec![Dir::South(south_coord), Dir::West(west_coord)],
                'F' => vec![Dir::South(south_coord), Dir::East(east_coord)],
                '.' | 'S' => vec![],
                _ => unreachable!(),
            };

            let current_location = Tile {
                symbol: x_char,
                valid_moves,
                original: true,
                flooded: false,
                done_flooding: false,
                part_of_loop: false,
            };

            input_hashmap.insert(current_coord, current_location);
        }
    }

    // find the Animal 'S' starting location and determine the valid moves from 'S'
    let mut valid_animal_moves = vec![];
    let animal_coords;
    {
        let (coord, _) = input_hashmap
            .iter()
            .find(|(_, location)| location.symbol == 'S')
            .unwrap();

        animal_coords = *coord;
    }

    {
        let mut left_coord = animal_coords;
        left_coord.x -= 1;
        if let Some(tile) = input_hashmap.get(&left_coord) {
            if tile
                .valid_moves
                .iter()
                .any(|dir| matches!(dir, Dir::East(_)))
            {
                valid_animal_moves.push(Dir::West(left_coord));
            }
        }
    }

    {
        let mut up_coord = animal_coords;
        up_coord.y -= 1;
        if let Some(tile) = input_hashmap.get(&up_coord) {
            if tile
                .valid_moves
                .iter()
                .any(|dir| matches!(dir, Dir::South(_)))
            {
                valid_animal_moves.push(Dir::North(up_coord));
            }
        }
    }

    {
        let mut right_coord = animal_coords;
        right_coord.x += 1;
        if let Some(tile) = input_hashmap.get(&right_coord) {
            if tile
                .valid_moves
                .iter()
                .any(|dir| matches!(dir, Dir::West(_)))
            {
                valid_animal_moves.push(Dir::East(right_coord));
            }
        }
    }

    {
        let mut down_coord = animal_coords;
        down_coord.y += 1;
        if let Some(tile) = input_hashmap.get(&down_coord) {
            if tile
                .valid_moves
                .iter()
                .any(|dir| matches!(dir, Dir::North(_)))
            {
                valid_animal_moves.push(Dir::South(down_coord));
            }
        }
    }

    let animal_time = input_hashmap.get_mut(&animal_coords).unwrap();
    dbg!(&valid_animal_moves);
    animal_time.valid_moves = valid_animal_moves;

    input_hashmap
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
