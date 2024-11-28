/*
we have a beam of light starting from top left going right that travels in a area of mirrors, mirrrs can be '/', '\', '-', '|'
and there are empty spaces represented by '.'
we always reflect from the mirror 90 degrees, for example going right and hitting \ means that we will travel downward
but if we hit a head on collision like going right and hitting | then we split the beam going up and down
the answer is the count of the tiles that we have visited in the area

part 2:
we can enter from any side going to the dir opposite of that side and we have to get the max tiles visited for any entering direction
*/

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Hash, Clone)]
struct VisitedDirs {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: u32,
    y: u32,
}
#[derive(Debug, Clone)]
struct Beam {
    pos: Coord,
    dir: Dir,
}

#[derive(Debug, Hash, Clone)]
struct AreaPos {
    mirror: Option<char>,
    visited: bool,
    dirs_visited: VisitedDirs,
}
#[derive(Debug, Clone)]
struct Area {
    area: HashMap<Coord, AreaPos>,
}

impl Area {
    fn from_grid(grid: &[String]) -> Self {
        let mut new_area_map: HashMap<Coord, AreaPos> = HashMap::new();
        for (y, y_line) in grid.iter().enumerate() {
            for (x, x_char) in y_line.chars().enumerate() {
                let current_coord = Coord {
                    x: x as u32,
                    y: y as u32,
                };

                let mirror = match x_char {
                    '.' => None,
                    other => Some(other),
                };

                let current_pos = AreaPos {
                    mirror,
                    visited: false,
                    dirs_visited: VisitedDirs {
                        up: false,
                        right: false,
                        down: false,
                        left: false,
                    },
                };

                new_area_map.insert(current_coord, current_pos);
            }
        }
        Area { area: new_area_map }
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

    let example_visited = get_visited_tiles_1(&example_1);
    dbg!(&example_visited);
    assert_eq!(example_visited, 46);

    let my_visited = get_visited_tiles_1(_my_input);
    dbg!(my_visited);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_visited = get_max_visited_tiles_2(&example_2);
    dbg!(&example_visited);
    assert_eq!(example_visited, 51);

    let my_visited = get_max_visited_tiles_2(_my_input);
    dbg!(my_visited);
}

fn get_visited_tiles_1(input: &[String]) -> u32 {
    let arena: Area = Area::from_grid(input);

    let starting_beam = Beam {
        pos: Coord { x: 0, y: 0 },
        dir: Dir::Right,
    };

    mark_visited(arena, starting_beam)
}

fn get_max_visited_tiles_2(input: &[String]) -> u32 {
    let arena: Area = Area::from_grid(input);

    let max_coords = arena.area.keys().max().unwrap();

    let mut all_beams: Vec<Beam> = vec![];

    for x in 0..=max_coords.x {
        let new_beam = Beam {
            pos: Coord { x, y: 0 },
            dir: Dir::Down,
        };
        all_beams.push(new_beam);
    }

    for x in 0..=max_coords.x {
        let new_beam = Beam {
            pos: Coord { x, y: max_coords.y },
            dir: Dir::Up,
        };
        all_beams.push(new_beam);
    }

    for y in 0..=max_coords.y {
        let new_beam = Beam {
            pos: Coord { x: 0, y },
            dir: Dir::Right,
        };
        all_beams.push(new_beam);
    }

    for y in 0..=max_coords.y {
        let new_beam = Beam {
            pos: Coord { x: max_coords.x, y },
            dir: Dir::Left,
        };
        all_beams.push(new_beam);
    }

    all_beams
        .into_iter()
        .map(|beam| mark_visited(arena.clone(), beam))
        .max()
        .unwrap()
}

fn mark_visited(mut arena: Area, starting_beam: Beam) -> u32 {
    let mut beams: VecDeque<Beam> = VecDeque::new();
    beams.push_back(starting_beam);

    while let Some(mut current_beam) = beams.pop_front() {
        if let Some(pos) = arena.area.get_mut(&current_beam.pos) {
            pos.visited = true;

            if has_already_visited(pos, &current_beam.dir) {
                continue;
            }

            match current_beam.dir {
                Dir::Right => match pos.mirror {
                    None | Some('-') => current_beam.pos.x += 1,
                    Some('/') => {
                        current_beam.pos.y -= 1;
                        current_beam.dir = Dir::Up
                    }
                    Some('\\') => {
                        current_beam.pos.y += 1;
                        current_beam.dir = Dir::Down
                    }
                    Some('|') => {
                        let mut new_beam = current_beam.clone();
                        new_beam.pos.y -= 1;
                        new_beam.dir = Dir::Up;

                        beams.push_back(new_beam);

                        current_beam.pos.y += 1;
                        current_beam.dir = Dir::Down
                    }

                    _ => unreachable!(),
                },
                Dir::Down => match pos.mirror {
                    None | Some('|') => current_beam.pos.y += 1,
                    Some('/') => {
                        current_beam.pos.x -= 1;
                        current_beam.dir = Dir::Left
                    }
                    Some('\\') => {
                        current_beam.pos.x += 1;
                        current_beam.dir = Dir::Right
                    }
                    Some('-') => {
                        let mut new_beam = current_beam.clone();
                        new_beam.pos.x += 1;
                        new_beam.dir = Dir::Right;

                        beams.push_back(new_beam);

                        current_beam.pos.x -= 1;
                        current_beam.dir = Dir::Left
                    }

                    _ => unreachable!(),
                },
                Dir::Left => match pos.mirror {
                    None | Some('-') => current_beam.pos.x -= 1,
                    Some('/') => {
                        current_beam.pos.y += 1;
                        current_beam.dir = Dir::Down
                    }
                    Some('\\') => {
                        current_beam.pos.y -= 1;
                        current_beam.dir = Dir::Up
                    }
                    Some('|') => {
                        let mut new_beam = current_beam.clone();
                        new_beam.pos.y -= 1;
                        new_beam.dir = Dir::Up;

                        beams.push_back(new_beam);

                        current_beam.pos.y += 1;
                        current_beam.dir = Dir::Down
                    }

                    _ => unreachable!(),
                },

                Dir::Up => match pos.mirror {
                    None | Some('|') => current_beam.pos.y -= 1,
                    Some('/') => {
                        current_beam.pos.x += 1;
                        current_beam.dir = Dir::Right
                    }
                    Some('\\') => {
                        current_beam.pos.x -= 1;
                        current_beam.dir = Dir::Left
                    }
                    Some('-') => {
                        let mut new_beam = current_beam.clone();
                        new_beam.pos.x += 1;
                        new_beam.dir = Dir::Right;

                        beams.push_back(new_beam);

                        current_beam.pos.x -= 1;
                        current_beam.dir = Dir::Left
                    }

                    _ => unreachable!(),
                },
            }
            beams.push_back(current_beam);
        }
    }

    //let visited: Vec<&AreaPos> = arena.area.values().filter(|value| value.visited).collect();

    //dbg!(visited);

    arena.area.values().filter(|value| value.visited).count() as u32
}

fn has_already_visited(cuttent_tile: &mut AreaPos, dir: &Dir) -> bool {
    cuttent_tile.visited = true;
    let visited = &mut cuttent_tile.dirs_visited;
    match dir {
        Dir::Up => {
            if !visited.up {
                visited.up = true;
                return false;
            }
            true
        }
        Dir::Left => {
            if !visited.left {
                visited.left = true;
                return false;
            }
            true
        }
        Dir::Down => {
            if !visited.down {
                visited.down = true;
                return false;
            }
            true
        }
        Dir::Right => {
            if !visited.right {
                visited.right = true;
                return false;
            }
            true
        }
    }
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
