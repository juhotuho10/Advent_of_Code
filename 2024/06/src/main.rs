/*
part 1:
we have a grid with a guard in it, the guard moves until it hits an obstacle (#) and then turns right
the guard keeps going forward and hitting obstacles until it will go out of the grid when no more obstacles are hit
we have to count the tiles that are covered by the guard

part 2:

we have to find all the positions in the guards path where we could place an obstacle and have the guard looping forever
and return the count of the positions that would cause this to happen

*/

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn_right(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn get_up(&self) -> Self {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn get_right(&self) -> Self {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn get_down(&self) -> Self {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn get_left(&self) -> Self {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    pos: Coord,
    dir: Dir,
}

impl Guard {
    fn get_next(&self) -> Coord {
        match self.dir {
            Dir::Up => self.pos.get_up(),
            Dir::Right => self.pos.get_right(),
            Dir::Down => self.pos.get_down(),
            Dir::Left => self.pos.get_left(),
        }
    }
}

#[derive(Debug, Clone)]
struct VisitDir {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
}

impl VisitDir {
    fn mark_dir(&mut self, dir: Dir) {
        match dir {
            Dir::Up => self.up = true,
            Dir::Right => self.right = true,
            Dir::Down => self.down = true,
            Dir::Left => self.left = true,
        }
    }
    fn is_dir_visited(&self, dir: Dir) -> bool {
        match dir {
            Dir::Up => self.up,
            Dir::Right => self.right,
            Dir::Down => self.down,
            Dir::Left => self.left,
        }
    }
}

#[derive(Debug, Clone)]
struct GridPos {
    obstacle: bool,
    visited: bool,
    visit_dirs: VisitDir,
}

#[derive(Debug, Clone)]
struct Grid {
    guard: Guard,
    obstacles: HashMap<Coord, GridPos>,
}

impl Grid {
    fn from_string(input: &[String]) -> Self {
        let mut grid_map: HashMap<Coord, GridPos> = HashMap::new();
        let mut guard = Guard {
            pos: Coord { x: 0, y: 0 },
            dir: Dir::Up,
        };
        for (y, y_line) in input.iter().enumerate() {
            for (x, x_char) in y_line.chars().enumerate() {
                let current_coord = Coord {
                    x: x as u32,
                    y: y as u32,
                };

                let (is_obstacle, is_guard) = match x_char {
                    '.' => (false, false),
                    '#' => (true, false),
                    '^' => {
                        guard = Guard {
                            pos: current_coord,
                            dir: Dir::Up,
                        };
                        (false, true)
                    }
                    _ => unreachable!(),
                };

                let current_pos = GridPos {
                    obstacle: is_obstacle,
                    visited: is_guard,
                    visit_dirs: VisitDir {
                        up: false,
                        right: false,
                        down: false,
                        left: false,
                    },
                };

                grid_map.insert(current_coord, current_pos);
            }
        }

        Grid {
            guard,
            obstacles: grid_map,
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

    let example_visited = get_visited_tiles_sum(&example_1);
    dbg!(&example_visited);
    assert_eq!(example_visited, 41);

    let my_visited = get_visited_tiles_sum(_my_input);
    dbg!(my_visited);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_loop_count = get_possible_loop_count(&example_2);
    dbg!(&example_loop_count);
    assert_eq!(example_loop_count, 6);

    let my_loop_count = get_possible_loop_count(_my_input);
    dbg!(my_loop_count);
}

fn get_visited_tiles_sum(input: &[String]) -> u32 {
    let mut grid = Grid::from_string(input);

    loop {
        let next_coord = grid.guard.get_next();
        match grid.obstacles.get_mut(&next_coord) {
            Some(pos) => {
                if pos.obstacle {
                    grid.guard.dir.turn_right();
                } else {
                    grid.guard.pos = next_coord;
                    pos.visited = true;
                    pos.visit_dirs.mark_dir(grid.guard.dir);
                }
            }
            None => break,
        }
    }

    grid.obstacles
        .values()
        .filter(|grid_pos| grid_pos.visited)
        .count() as u32
}

fn get_possible_loop_count(input: &[String]) -> u32 {
    let mut grid = Grid::from_string(input);
    let starting_guard = grid.guard;
    let guard_starting_coords = starting_guard.pos;

    let mut looping_coords: HashSet<Coord> = HashSet::new();

    loop {
        let next_coord = grid.guard.get_next();
        match grid.obstacles.get_mut(&next_coord) {
            Some(forward_pos) => {
                if forward_pos.obstacle {
                    grid.guard.dir.turn_right();
                } else {
                    if next_coord != guard_starting_coords {
                        let grid_copy = {
                            let mut grid_copy = grid.clone();
                            let grid_copy_next = grid_copy.obstacles.get_mut(&next_coord).unwrap();
                            grid_copy_next.obstacle = true;
                            grid_copy.guard = starting_guard;
                            grid_copy
                        };

                        if grid_loop_found(grid_copy) {
                            looping_coords.insert(next_coord);
                        }
                    }

                    grid.guard.pos = next_coord;
                }
            }
            None => break,
        }
    }

    looping_coords.len() as u32
}

fn grid_loop_found(mut copy_grid: Grid) -> bool {
    loop {
        let next_coord = copy_grid.guard.get_next();
        match copy_grid.obstacles.get_mut(&next_coord) {
            Some(forward_pos) => {
                if forward_pos.obstacle {
                    copy_grid.guard.dir.turn_right();
                } else {
                    copy_grid.guard.pos = next_coord;
                    forward_pos.visited = true;
                    if forward_pos.visit_dirs.is_dir_visited(copy_grid.guard.dir) {
                        return true;
                    } else {
                        forward_pos.visit_dirs.mark_dir(copy_grid.guard.dir);
                    }
                }
            }
            // we have walked off the map
            None => return false,
        }
    }
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
