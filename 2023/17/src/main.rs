/*
we have to push a hot cart of lava and we have a grid of numbers that indicate how much the lava is cooled on that tile
we have to push the cart from the top left to the bottom right while losing as little heat as possible
also the path must not have more than 3 tiles in a straight line

*/

extern crate fxhash;
use fxhash::FxHashMap;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn up(&self) -> Self {
        let new_y = self.y - 1;
        Coord {
            x: self.x,
            y: new_y,
        }
    }

    fn right(&self) -> Self {
        let new_x = self.x + 1;
        Coord {
            x: new_x,
            y: self.y,
        }
    }

    fn down(&self) -> Self {
        let new_y = self.y + 1;
        Coord {
            x: self.x,
            y: new_y,
        }
    }

    fn left(&self) -> Self {
        let new_x = self.x - 1;
        Coord {
            x: new_x,
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

struct Node {
    cost: u16,
}

impl Node {
    fn new(cost: u16) -> Self {
        Node { cost }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct NodeSearcher {
    accumulated_cost: u16,
    current_node: Coord,
    straigh_steps: u8,
    facing_dir: Dir,
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

impl NodeSearcher {
    fn turn_left(&self) -> Self {
        let new_dir = match self.facing_dir {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        };

        NodeSearcher {
            accumulated_cost: self.accumulated_cost,
            current_node: self.current_node,
            straigh_steps: 0,
            facing_dir: new_dir,
        }
    }

    fn turn_right(&self) -> Self {
        let new_dir = match self.facing_dir {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        };

        NodeSearcher {
            accumulated_cost: self.accumulated_cost,
            current_node: self.current_node,
            straigh_steps: 0,
            facing_dir: new_dir,
        }
    }
    fn get_forward(&self) -> Coord {
        self.current_node.go_dir(self.facing_dir)
    }

    fn step_into_node(&mut self, coords: Coord, node: &Node) {
        self.accumulated_cost += node.cost;
        self.current_node = coords;
        self.straigh_steps += 1;
    }
}

struct NumberGraph {
    valid_coords: FxHashMap<Coord, Node>,
    cost_map: FxHashMap<(Coord, Dir, u8), u16>,
}

impl NumberGraph {
    fn from_string(input: &[String]) -> Self {
        let mut valid_coords: FxHashMap<Coord, Node> = FxHashMap::default();

        for (y, y_line) in input.iter().enumerate() {
            for (x, x_char) in y_line.char_indices() {
                let num = x_char.to_digit(10).unwrap() as u16;

                let current_coord = Coord {
                    x: x as i32,
                    y: y as i32,
                };
                valid_coords.insert(current_coord, Node::new(num));
            }
        }

        let cost_map: FxHashMap<(Coord, Dir, u8), u16> = FxHashMap::default();
        NumberGraph {
            valid_coords,
            cost_map,
        }
    }

    fn max_3_in_row_djiksta(&mut self, start: Coord, end: Coord) -> Option<u16> {
        // cost min heap
        let mut searcher_heap: BinaryHeap<Reverse<NodeSearcher>> = BinaryHeap::new();

        searcher_heap.push(Reverse(NodeSearcher {
            accumulated_cost: 0,
            current_node: start,
            straigh_steps: 0,
            facing_dir: Dir::Right,
        }));

        while let Some(min_dist_searcher) = searcher_heap.pop() {
            let mut searcher = min_dist_searcher.0;
            if searcher.current_node == end {
                return Some(searcher.accumulated_cost);
            }

            let mut right_searcher = searcher.turn_right();
            let next_right_coords = right_searcher.get_forward();
            if let Some(right_node) = self.valid_coords.get(&next_right_coords) {
                right_searcher.step_into_node(next_right_coords, right_node);

                if self.can_continue_1(next_right_coords, &right_searcher) {
                    searcher_heap.push(Reverse(right_searcher));
                }
            }

            let mut left_searcher = searcher.turn_left();

            let next_left_coords = left_searcher.get_forward();
            if let Some(left_node) = self.valid_coords.get(&next_left_coords) {
                left_searcher.step_into_node(next_left_coords, left_node);

                if self.can_continue_1(next_left_coords, &left_searcher) {
                    searcher_heap.push(Reverse(left_searcher));
                }
            }

            let next_straight_coords = searcher.get_forward();
            if let Some(straight_node) = self.valid_coords.get(&next_straight_coords) {
                searcher.step_into_node(next_straight_coords, straight_node);

                if self.can_continue_1(next_straight_coords, &searcher) {
                    searcher_heap.push(Reverse(searcher));
                }
            }
        }

        None
    }

    fn can_continue_1(&mut self, next_node_coord: Coord, searcher: &NodeSearcher) -> bool {
        match searcher.straigh_steps {
            (1..=3) => {
                let key = (next_node_coord, searcher.facing_dir, searcher.straigh_steps);
                if let Some(prev_min_cost) = self.cost_map.get_mut(&key) {
                    if *prev_min_cost <= searcher.accumulated_cost {
                        return false;
                    }
                    *prev_min_cost = searcher.accumulated_cost;
                } else {
                    self.cost_map.insert(key, searcher.accumulated_cost);
                };
            }
            4 => {
                return false;
            }
            _ => unreachable!(),
        }

        true
    }

    fn min_4_max_10_in_row_djiksta(&mut self, start: Coord, end: Coord) -> Option<u16> {
        // cost min heap
        let mut searcher_heap: BinaryHeap<Reverse<NodeSearcher>> = BinaryHeap::new();

        searcher_heap.push(Reverse(NodeSearcher {
            accumulated_cost: 0,
            current_node: start,
            straigh_steps: 0,
            facing_dir: Dir::Right,
        }));

        while let Some(min_dist_searcher) = searcher_heap.pop() {
            let searcher = min_dist_searcher.0;
            if searcher.current_node == end {
                return Some(searcher.accumulated_cost);
            }

            let mut straight_searcher = searcher.clone();
            let next_straight_coords = straight_searcher.get_forward();
            if let Some(straight_node) = self.valid_coords.get(&next_straight_coords) {
                straight_searcher.step_into_node(next_straight_coords, straight_node);

                if self.can_continue_2(next_straight_coords, &straight_searcher) {
                    searcher_heap.push(Reverse(straight_searcher));
                }
            }

            if searcher.straigh_steps >= 4 {
                let mut right_searcher = searcher.turn_right();
                let next_right_coords = right_searcher.get_forward();
                if let Some(right_node) = self.valid_coords.get(&next_right_coords) {
                    right_searcher.step_into_node(next_right_coords, right_node);

                    if self.can_continue_2(next_right_coords, &right_searcher) {
                        searcher_heap.push(Reverse(right_searcher));
                    }
                }

                let mut left_searcher = searcher.turn_left();

                let next_left_coords = left_searcher.get_forward();
                if let Some(left_node) = self.valid_coords.get(&next_left_coords) {
                    left_searcher.step_into_node(next_left_coords, left_node);

                    if self.can_continue_2(next_left_coords, &left_searcher) {
                        searcher_heap.push(Reverse(left_searcher));
                    }
                }
            }
        }

        None
    }

    fn can_continue_2(&mut self, next_node_coord: Coord, searcher: &NodeSearcher) -> bool {
        match searcher.straigh_steps {
            (1..=10) => {
                let key = (next_node_coord, searcher.facing_dir, searcher.straigh_steps);
                if let Some(prev_min_cost) = self.cost_map.get_mut(&key) {
                    if *prev_min_cost <= searcher.accumulated_cost {
                        return false;
                    }
                    *prev_min_cost = searcher.accumulated_cost;
                } else {
                    self.cost_map.insert(key, searcher.accumulated_cost);
                };
            }
            11 => {
                return false;
            }
            _ => unreachable!(),
        }

        true
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

    let start = Instant::now();
    let example_heatloss = get_min_heatloss_1(&example_1);
    assert_eq!(example_heatloss, 102);
    dbg!(start.elapsed());
    dbg!(&example_heatloss);

    let start = Instant::now();
    let my_heatloss = get_min_heatloss_1(_my_input);
    dbg!(start.elapsed());
    dbg!(my_heatloss);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let start = Instant::now();
    let example_heatloss = get_min_heatloss_2(&example_2);
    assert_eq!(example_heatloss, 94);
    dbg!(start.elapsed());
    dbg!(&example_heatloss);

    let start = Instant::now();
    let my_heatloss = get_min_heatloss_2(_my_input);
    dbg!(start.elapsed());
    dbg!(my_heatloss);
}

fn get_min_heatloss_1(input: &[String]) -> u16 {
    let mut graph = NumberGraph::from_string(input);

    let start_node = Coord { x: 0, y: 0 };

    let end_node = *graph.valid_coords.keys().max().unwrap();

    //dbg!(end_node);

    graph.max_3_in_row_djiksta(start_node, end_node).unwrap()
}

fn get_min_heatloss_2(input: &[String]) -> u16 {
    let mut graph = NumberGraph::from_string(input);

    let start_node = Coord { x: 0, y: 0 };

    let end_node = *graph.valid_coords.keys().max().unwrap();

    //dbg!(end_node);

    graph
        .min_4_max_10_in_row_djiksta(start_node, end_node)
        .unwrap()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
