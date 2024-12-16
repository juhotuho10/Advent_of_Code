/*
part 1:
we have a maze with reindeer in it. The reindeer start from S corner and they go to the E corner and the point is to go from corner
to corner with the least steps possible

going forward increases the score by 1 but turning increases the score by 1000

we have to find the smallest score possible and return that

part 2:

we have to find all the paths that have the lowest score to get to the end coord
and then we have to find the count of unique coords visited in these paths
and return the count of the coords

*/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    facing_dir: Dir,
    turned: bool,
    visited_coords: Vec<Coord>,
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
    fn get_forward_coord(&self) -> Coord {
        self.current_node.go_dir(self.facing_dir)
    }

    fn walk_forward(&mut self) {
        let next_coord = self.current_node.go_dir(self.facing_dir);
        self.accumulated_cost += 1;
        self.current_node = next_coord;
        self.turned = false;
        self.visited_coords.push(next_coord);
    }

    fn turn_left(&self) -> Self {
        let next_dir = match self.facing_dir {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        };

        NodeSearcher {
            accumulated_cost: self.accumulated_cost + 1000,
            current_node: self.current_node,
            facing_dir: next_dir,
            turned: true,
            visited_coords: self.visited_coords.clone(),
        }
    }

    fn turn_right(&self) -> Self {
        let next_dir = match self.facing_dir {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        };

        NodeSearcher {
            accumulated_cost: self.accumulated_cost + 1000,
            current_node: self.current_node,
            facing_dir: next_dir,
            turned: true,
            visited_coords: self.visited_coords.clone(),
        }
    }
}

struct NodeGraph {
    start_coord: Coord,
    end_coord: Coord,
    cost_map: HashMap<Coord, u32>,
}

impl NodeGraph {
    fn from_string(input: &[String]) -> Self {
        let mut start_coord = Coord { x: 0, y: 0 };
        let mut end_coord = Coord { x: 0, y: 0 };
        let mut cost_map: HashMap<Coord, u32> = HashMap::new();
        for (y, y_line) in input.iter().enumerate() {
            for (x, x_char) in y_line.char_indices() {
                let current_coord = Coord {
                    x: x as u32,
                    y: y as u32,
                };
                match x_char {
                    'S' => {
                        start_coord = current_coord;
                        cost_map.insert(current_coord, u32::MAX / 2);
                    }
                    'E' => {
                        end_coord = current_coord;
                        cost_map.insert(current_coord, u32::MAX / 2);
                    }
                    '.' => {
                        cost_map.insert(current_coord, u32::MAX / 2);
                    }
                    '#' => {}
                    _ => unreachable!(),
                }
            }
        }

        NodeGraph {
            start_coord,
            end_coord,
            cost_map,
        }
    }

    fn get_lowers_cost_paths(&mut self) -> Vec<NodeSearcher> {
        // constructing a min heap using Reverse ord trait
        let mut cost_heap: BinaryHeap<Reverse<NodeSearcher>> = BinaryHeap::new();

        let start_searcher = NodeSearcher {
            accumulated_cost: 0,
            current_node: self.start_coord,
            facing_dir: Dir::Right,
            turned: true,
            visited_coords: vec![self.start_coord],
        };
        cost_heap.push(Reverse(start_searcher));

        let mut end_node_searchers = vec![];

        while let Some(searcher) = cost_heap.pop() {
            //dbg!(searcher.current_node);
            //dbg!(searcher.accumulated_cost);
            let mut searcher = searcher.0;

            if searcher.current_node == self.end_coord {
                end_node_searchers.push(searcher);
                continue;
            }

            let current_node_min = self
                .cost_map
                .entry(searcher.current_node)
                .or_insert(u32::MAX / 2);
            if searcher.accumulated_cost > (*current_node_min + 1000) {
                continue;
            } else {
                *current_node_min = *current_node_min.min(&mut searcher.accumulated_cost);
            }

            let mut new_left_searcher = searcher.turn_left();
            let new_left_coord = new_left_searcher.get_forward_coord();
            if self.cost_map.contains_key(&new_left_coord) {
                new_left_searcher.walk_forward();
                cost_heap.push(Reverse(new_left_searcher));
            }

            let mut new_right_searcher = searcher.turn_right();
            let new_right_coord = new_right_searcher.get_forward_coord();
            if self.cost_map.contains_key(&new_right_coord) {
                new_right_searcher.walk_forward();
                cost_heap.push(Reverse(new_right_searcher));
            }

            let next_forward_coord = searcher.get_forward_coord();
            if self.cost_map.contains_key(&next_forward_coord) {
                searcher.walk_forward();
                cost_heap.push(Reverse(searcher));
            }
        }

        end_node_searchers
    }

    fn pretty_print(&self) {
        let max_x = self.cost_map.keys().map(|coord| coord.x).max().unwrap_or(0);
        let max_y = self.cost_map.keys().map(|coord| coord.y).max().unwrap_or(0);

        for y in 0..=max_y {
            for x in 0..=max_x {
                let coord = Coord { x, y };

                if let Some(&cost) = self.cost_map.get(&coord) {
                    if cost == u32::MAX / 2 {
                        print!(".");
                    } else {
                        print!("#");
                    }
                } else {
                    print!(" ");
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

    let example_path_len = find_lowest_path(&example_1);
    dbg!(&example_path_len);
    assert_eq!(example_path_len, 7036);

    let my_path_len = find_lowest_path(_my_input);
    dbg!(my_path_len);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = find_lowest_path_node_counts(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 64);

    let my_sum = find_lowest_path_node_counts(_my_input);
    dbg!(my_sum);
}

fn find_lowest_path(input: &[String]) -> u32 {
    let mut node_graph = NodeGraph::from_string(input);

    let end_node_searchers = node_graph.get_lowers_cost_paths();
    node_graph.pretty_print();
    end_node_searchers
        .iter()
        .map(|searcher| searcher.accumulated_cost)
        .min()
        .unwrap()
}

fn find_lowest_path_node_counts(input: &[String]) -> u32 {
    let mut node_graph = NodeGraph::from_string(input);

    let mut end_node_searchers = node_graph.get_lowers_cost_paths();
    node_graph.pretty_print();
    let lowest_path_len = end_node_searchers
        .iter()
        .map(|searcher| searcher.accumulated_cost)
        .min()
        .unwrap();

    end_node_searchers.retain(|searcher| searcher.accumulated_cost == lowest_path_len);

    let unique_coords_visited: HashSet<Coord> = end_node_searchers
        .into_iter()
        .flat_map(|searcher| searcher.visited_coords)
        .collect();

    unique_coords_visited.len() as u32
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
