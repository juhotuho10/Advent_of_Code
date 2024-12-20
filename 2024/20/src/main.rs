/*
part 1:

we have a data racetrack and we are racing in a 3d grid from the start S to the end E
we have empty spaces (.) and walls (#)
each move takes a picosecond and we have to get to the end as fast as possible
but in this race we can cheat once per race. we can walk thnough walls for 2 steps per race (so 1 wall in total)

part 2:

we can now skip max 20 steps

*/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
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
struct Node {
    min_cost_from_start: u32,
    min_cost_to_end: u32,
    cost_with_cheat: Vec<u32>,
}

impl Node {
    fn new() -> Self {
        Node {
            min_cost_from_start: u32::MAX,
            min_cost_to_end: u32::MAX,

            cost_with_cheat: vec![],
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
    cost_map: HashMap<Coord, Node>,
    start_coord: Coord,
    end_coord: Coord,
}

impl Grid {
    fn from_string(input: &[String]) -> Self {
        let mut cost_map: HashMap<Coord, Node> = HashMap::new();
        let mut start_coord = Coord { x: 0, y: 0 };
        let mut end_coord = Coord { x: 0, y: 0 };

        for (y, y_line) in input.iter().enumerate() {
            for (x, x_char) in y_line.char_indices() {
                let current_coord = Coord {
                    x: x as u32,
                    y: y as u32,
                };
                match x_char {
                    '.' => cost_map.insert(current_coord, Node::new()),
                    'S' => {
                        start_coord = current_coord;
                        cost_map.insert(current_coord, Node::new())
                    }
                    'E' => {
                        end_coord = current_coord;
                        cost_map.insert(current_coord, Node::new())
                    }
                    '#' => continue,
                    _ => unreachable!(),
                };
            }
        }

        Grid {
            cost_map,
            start_coord,
            end_coord,
        }
    }

    fn min_steps_from_start(&mut self) -> Option<u32> {
        let mut cost_heap: BinaryHeap<Reverse<NodeSearcher>> = BinaryHeap::new();

        let start_searcher = NodeSearcher {
            accumulated_cost: 0,
            current_node: self.start_coord,
        };

        let mut lowest_cost = None;

        cost_heap.push(Reverse(start_searcher));

        while let Some(rev_searcher) = cost_heap.pop() {
            let searcher = rev_searcher.0;

            if searcher.current_node == self.end_coord {
                lowest_cost = Some(searcher.accumulated_cost);
                continue;
            }

            for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
                let new_coord = searcher.current_node.go_dir(dir);
                if let Some(node) = self.cost_map.get_mut(&new_coord) {
                    let mut copy_searcher = searcher.clone();
                    copy_searcher.current_node = new_coord;
                    let new_cost = copy_searcher.accumulated_cost + 1;
                    if new_cost < node.min_cost_from_start {
                        node.min_cost_from_start = new_cost;
                        copy_searcher.accumulated_cost = new_cost;
                        cost_heap.push(Reverse(copy_searcher));
                    }
                }
            }
        }

        lowest_cost
    }

    fn min_steps_to_end(&mut self) -> Option<u32> {
        let mut cost_heap: BinaryHeap<Reverse<NodeSearcher>> = BinaryHeap::new();

        let start_searcher = NodeSearcher {
            accumulated_cost: 0,
            current_node: self.end_coord,
        };

        let mut lowest_cost = None;

        cost_heap.push(Reverse(start_searcher));

        while let Some(rev_searcher) = cost_heap.pop() {
            let searcher = rev_searcher.0;

            if searcher.current_node == self.start_coord {
                lowest_cost = Some(searcher.accumulated_cost);
                continue;
            }

            for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
                let new_coord = searcher.current_node.go_dir(dir);
                if let Some(node) = self.cost_map.get_mut(&new_coord) {
                    let mut copy_searcher = searcher.clone();
                    copy_searcher.current_node = new_coord;
                    let new_cost = copy_searcher.accumulated_cost + 1;
                    if new_cost < node.min_cost_to_end {
                        node.min_cost_to_end = new_cost;
                        copy_searcher.accumulated_cost = new_cost;
                        cost_heap.push(Reverse(copy_searcher));
                    }
                }
            }
        }

        lowest_cost
    }

    fn search_cheats(&mut self, picoseconds: u32) {
        let all_coords: Vec<Coord> = self.cost_map.keys().cloned().collect();
        let copy_map = self.cost_map.clone();
        for (curr_coord, _) in copy_map.clone().into_iter() {
            let current_node = self.cost_map.get_mut(&curr_coord).unwrap();

            let coords_in_walking_dist: Vec<(&Coord, u32)> = all_coords
                .iter()
                .map(|c1| (c1, Self::coord_dist(c1, &curr_coord)))
                .filter(|(_, c1)| *c1 <= picoseconds)
                .collect();

            for (cheat_coord, walk_distance) in coords_in_walking_dist {
                if let Some(copy_node) = copy_map.get(cheat_coord) {
                    let cheat_node_to_end = copy_node.min_cost_to_end;

                    let cost_with_cheat =
                        current_node.min_cost_from_start + walk_distance + cheat_node_to_end;

                    current_node.cost_with_cheat.push(cost_with_cheat);
                }
            }
        }
    }

    fn coord_dist(c1: &Coord, c2: &Coord) -> u32 {
        c1.x.abs_diff(c2.x) + c1.y.abs_diff(c2.y)
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

    let example_count = cheats_that_save_100_steps(&example_1, 2);
    dbg!(&example_count);
    assert_eq!(example_count, 0);

    let my_count = cheats_that_save_100_steps(_my_input, 2);
    dbg!(my_count);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_count = cheats_that_save_100_steps(&example_2, 20);
    dbg!(&example_count);
    assert_eq!(example_count, 0);

    let my_count = cheats_that_save_100_steps(_my_input, 20);
    dbg!(my_count);
}

fn cheats_that_save_100_steps(input: &[String], picoseconds: u32) -> u32 {
    let mut grid = Grid::from_string(input);

    let start = grid.cost_map.get_mut(&grid.start_coord).unwrap();
    start.min_cost_from_start = 0;

    let end = grid.cost_map.get_mut(&grid.end_coord).unwrap();
    end.min_cost_to_end = 0;

    grid.min_steps_from_start();
    grid.min_steps_to_end();

    grid.search_cheats(picoseconds);

    let max_cost = grid
        .cost_map
        .values()
        .map(|n| n.min_cost_from_start)
        .max()
        .unwrap();

    let saved_costs: Vec<u32> = grid
        .cost_map
        .values()
        .flat_map(|n| n.cost_with_cheat.clone())
        .filter(|&cost_with_cheat| cost_with_cheat < max_cost)
        .map(|cost| max_cost - cost)
        .collect();

    saved_costs.iter().filter(|&&cost| cost >= 100).count() as u32
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
