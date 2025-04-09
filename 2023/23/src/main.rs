/*
part 1:
we have a path with slopes, and we need to determine the longest path possible
there are only single lane paths and no ways to have a roundabout route

part 2:

*/

use fxhash::FxHashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Dir {
    Up,
    Left,
    Down,
    Right,
}

enum Path {
    Normal,
    Dir(Dir),
}

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    x: u8,
    y: u8,
}

impl Coord {
    fn new(x: u8, y: u8) -> Self {
        Coord { x, y }
    }

    fn up(&self) -> Self {
        let (x, y) = (self.x, self.y);
        Coord::new(x, y - 1)
    }
    fn left(&self) -> Self {
        let (x, y) = (self.x, self.y);
        Coord::new(x - 1, y)
    }
    fn down(&self) -> Self {
        let (x, y) = (self.x, self.y);
        Coord::new(x, y + 1)
    }
    fn right(&self) -> Self {
        let (x, y) = (self.x, self.y);
        Coord::new(x + 1, y)
    }

    fn get_surrounding(&self) -> [Coord; 4] {
        let (x, y) = (self.x, self.y);
        [
            Coord::new(x, y - 1), // up
            Coord::new(x + 1, y), // left
            Coord::new(x, y + 1), // down
            Coord::new(x - 1, y), // right
        ]
    }

    fn get_dir(&self, dir: &Dir) -> Coord {
        match dir {
            Dir::Up => self.up(),
            Dir::Left => self.left(),
            Dir::Down => self.down(),
            Dir::Right => self.right(),
        }
    }
}

struct Trails {
    trail_map: FxHashMap<Coord, Path>,
}

impl Trails {
    fn from_string(input: &[String]) -> Trails {
        let mut trail: FxHashMap<Coord, Path> = FxHashMap::default();

        for (y, y_line) in input.iter().enumerate() {
            for (x, x_char) in y_line.char_indices() {
                let path_type = match x_char {
                    '#' => continue,
                    '.' => Path::Normal,
                    '^' => Path::Dir(Dir::Up),
                    '>' => Path::Dir(Dir::Right),
                    'v' => Path::Dir(Dir::Down),
                    '<' => Path::Dir(Dir::Left),
                    _ => unreachable!("parsing err"),
                };

                trail.insert(Coord::new(x as u8, y as u8), path_type);
            }
        }

        Trails { trail_map: trail }
    }

    fn get_start_and_finish(&self) -> (Coord, Coord) {
        let start = *self.trail_map.keys().min_by_key(|c| c.y).unwrap();
        let end = *self.trail_map.keys().max_by_key(|c| c.y).unwrap();

        (start, end)
    }

    fn longest_path_search(&self, start: Coord, end: Coord) -> u32 {
        assert!(start < end);
        self.recursive_search(1, start, start.down(), end).unwrap()
    }

    fn recursive_search(
        &self,
        total: u32,
        came_from: Coord,
        current: Coord,
        end_coord: Coord,
    ) -> Option<u32> {
        if current == end_coord {
            return Some(total);
        }

        if let Some(char) = self.trail_map.get(&current) {
            let mut all_next_paths: Vec<Coord> = match char {
                Path::Normal => current.get_surrounding().into_iter().collect(),
                Path::Dir(dir) => vec![current.get_dir(dir)],
            };
            all_next_paths.retain(|&coord| coord != came_from);

            all_next_paths
                .iter()
                .filter_map(|&next_coord| {
                    self.recursive_search(total + 1, current, next_coord, end_coord)
                })
                .max()
        } else {
            None
        }
    }

    fn debug_print(&self) {
        let min_x = self.trail_map.keys().map(|c| c.x).min().unwrap_or(0);
        let max_x = self.trail_map.keys().map(|c| c.x).max().unwrap_or(0);
        let min_y = self.trail_map.keys().map(|c| c.y).min().unwrap_or(0);
        let max_y = self.trail_map.keys().map(|c| c.y).max().unwrap_or(0);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let coord = Coord::new(x, y);
                let ch = if let Some(path) = self.trail_map.get(&coord) {
                    match path {
                        Path::Normal => '.',
                        Path::Dir(Dir::Up) => '^',
                        Path::Dir(Dir::Down) => 'v',
                        Path::Dir(Dir::Left) => '<',
                        Path::Dir(Dir::Right) => '>',
                    }
                } else {
                    ' '
                };
                print!("{}", ch);
            }
            println!();
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

    let example_sum = solution_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 94);

    let my_sum = solution_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 0);

    let my_sum = solution_2(_my_input);
    dbg!(my_sum);
}

fn solution_1(input: &[String]) -> u32 {
    let trails = parse_input(input);
    trails.debug_print();
    let (start, end) = trails.get_start_and_finish();
    trails.longest_path_search(start, end)
}

fn solution_2(input: &[String]) -> u32 {
    let parsed = parse_input(input);
    0
}

fn parse_input(input: &[String]) -> Trails {
    Trails::from_string(input)
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
