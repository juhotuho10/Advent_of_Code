/*
part 1:
we have a 2 dimensional that is 71 x 71 blocks grid with falling bytes
we have to find the exit when 1024 byts have fallen onto the grid

part 2:

we have to see how many blocks can fall until all paths are blocked and we have to return the coords for the final block
that blocks the path

*/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
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
    cost_map: HashMap<Coord, u32>,
}

impl Grid {
    fn from_bytes(bytes: &[String], end_coord: Coord) -> Self {
        let mut cost_map: HashMap<Coord, u32> = HashMap::new();

        let mut byte_pos: Vec<Coord> = vec![];

        for b in bytes {
            let (x_str, y_str) = b.split_once(",").unwrap();
            let byte_coord = Coord {
                x: x_str.parse().unwrap(),
                y: y_str.parse().unwrap(),
            };

            byte_pos.push(byte_coord);
        }

        for y in 0..=end_coord.y {
            for x in 0..end_coord.x + 1 {
                let coord = Coord { x, y };

                if byte_pos.contains(&coord) {
                    continue;
                }

                cost_map.insert(coord, u32::MAX);
            }
        }

        Grid { cost_map }
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

    let example_sum = min_steps_till_exit(&example_1, 12, Coord { x: 6, y: 6 }).unwrap();
    dbg!(&example_sum);
    assert_eq!(example_sum, 22);

    let my_sum = min_steps_till_exit(_my_input, 1024, Coord { x: 70, y: 70 });
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_coords = first_byte_to_block_exit(&example_2, Coord { x: 6, y: 6 });
    dbg!(&example_coords);
    assert_eq!(example_coords, "6,1");

    let start = Instant::now();
    let my_coords = first_byte_to_block_exit(_my_input, Coord { x: 70, y: 70 });
    dbg!(start.elapsed());
    dbg!(my_coords);
}

fn min_steps_till_exit(input: &[String], byte_count: usize, end_coord: Coord) -> Option<u32> {
    let mut grid = Grid::from_bytes(&input[0..byte_count], end_coord);

    let mut cost_heap: BinaryHeap<Reverse<NodeSearcher>> = BinaryHeap::new();

    let start_searcher = NodeSearcher {
        accumulated_cost: 0,
        current_node: Coord { x: 0, y: 0 },
    };

    grid.cost_map.insert(Coord { x: 0, y: 0 }, 0);

    cost_heap.push(Reverse(start_searcher));

    while let Some(rev_searcher) = cost_heap.pop() {
        let searcher = rev_searcher.0;

        if searcher.current_node == end_coord {
            return Some(searcher.accumulated_cost);
        }

        for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
            let new_coord = searcher.current_node.go_dir(dir);
            if let Some(cost) = grid.cost_map.get_mut(&new_coord) {
                let mut copy_searcher = searcher.clone();
                copy_searcher.current_node = new_coord;
                let new_cost = copy_searcher.accumulated_cost + 1;
                if new_cost < *cost {
                    *cost = new_cost;
                    copy_searcher.accumulated_cost = new_cost;
                    cost_heap.push(Reverse(copy_searcher));
                }
            }
        }
    }

    None
}

fn first_byte_to_block_exit(input: &[String], end_coord: Coord) -> String {
    let mut min = 1;
    let mut max = input.len();
    while min != max {
        let middle = (min + max) / 2;
        // count
        if min_steps_till_exit(input, middle, end_coord).is_none() {
            max = middle;
        } else {
            min = middle + 1;
        }
    }

    input[min - 1].clone()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
