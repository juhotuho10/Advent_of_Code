/*
part 1:
we have a garden with bocks,
we have to determine all the tiles that we could be on after taking 64 steps

part 2:

*/

use std::ascii::escape_default;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy)]
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

    fn go_dir(&self, dir: Dir) -> Self {
        match dir {
            Dir::Up => self.up(),
            Dir::Left => self.left(),
            Dir::Down => self.down(),
            Dir::Right => self.right(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct NodeSearcher {
    accumulated_cost: u32,
    current_node: Coord,
}

impl Ord for NodeSearcher {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.accumulated_cost.cmp(&other.accumulated_cost)
    }
}

impl PartialOrd for NodeSearcher {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    start_pos: Coord,
    max_pos: Coord,
    cost_map: HashMap<Coord, u32>,
}

impl Grid {
    fn from_string(input: &[String]) -> Self {
        let mut cost_map: HashMap<Coord, u32> = HashMap::new();

        let mut start_pos = Coord { x: 0, y: 0 };
        let max_pos = Coord {
            x: input[0].len() as u32,
            y: input.len() as u32,
        };

        for (y, y_line) in input.iter().enumerate() {
            for (x, x_char) in y_line.char_indices() {
                if x_char == '#' {
                    continue;
                }

                let current_coord = Coord {
                    x: x as u32,
                    y: y as u32,
                };

                if x_char == 'S' {
                    start_pos = current_coord;
                }

                cost_map.insert(current_coord, 0);
            }
        }

        Grid {
            start_pos,
            max_pos,
            cost_map,
        }
    }

    fn pretty_print(&self) {
        for y in 0..self.max_pos.y {
            for x in 0..self.max_pos.x {
                let current_coord = Coord { x, y };
                if let Some(&cost) = self.cost_map.get(&current_coord) {
                    if cost == 0 {
                        print!(".");
                    } else {
                        print!("O");
                    }
                } else {
                    print!("#")
                }
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

    let example_sum = reachable_garden_spots_1(&example_1, 6);
    dbg!(&example_sum);
    assert_eq!(example_sum, 16);

    let my_sum = reachable_garden_spots_1(_my_input, 64);
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

fn reachable_garden_spots_1(input: &[String], steps: u32) -> u32 {
    let mut grid = Grid::from_string(input);

    let mut cost_heap: BinaryHeap<Reverse<NodeSearcher>> = BinaryHeap::new();

    let mut final_coords: HashSet<Coord> = HashSet::new();

    let start_searcher = NodeSearcher {
        accumulated_cost: 0,
        current_node: grid.start_pos,
    };

    grid.cost_map.insert(Coord { x: 0, y: 0 }, 0);

    cost_heap.push(Reverse(start_searcher));

    while let Some(rev_searcher) = cost_heap.pop() {
        let searcher = rev_searcher.0;

        if searcher.accumulated_cost == steps {
            final_coords.insert(searcher.current_node);
            continue;
        }

        for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
            let new_coord = searcher.current_node.go_dir(dir);
            if grid.cost_map.get_mut(&new_coord).is_some() {
                let mut copy_searcher = searcher.clone();
                copy_searcher.current_node = new_coord;
                let new_cost = copy_searcher.accumulated_cost + 1;

                let current_node_costs: Vec<&u32> = cost_heap
                    .iter()
                    .filter(|s| s.0.current_node == new_coord)
                    .map(|s| &s.0.accumulated_cost)
                    .collect();

                // current node already has a node searcher that will get a better position than the current searcher
                if current_node_costs.iter().any(|&&cost| {
                    let diff = cost as i32 - new_cost as i32;
                    diff >= 0 && diff % 2 == 0
                }) {
                    continue;
                }
                copy_searcher.accumulated_cost = new_cost;
                cost_heap.push(Reverse(copy_searcher));
            }
        }
    }

    for coord in &final_coords {
        grid.cost_map.insert(*coord, 1);
    }

    grid.pretty_print();

    final_coords.len() as u32
}

fn solution_2(input: &[String]) -> u32 {
    let grid = Grid::from_string(input);
    0
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
