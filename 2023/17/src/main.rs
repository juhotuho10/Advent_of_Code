/*
we have to push a hot cart of lava and we have a grid of numbers that indicate how much the lava is cooled on that tile
we have to push the cart from the top left to the bottom right while losing as little heat as possible
also the path must not have more than 3 tiles in a straight line

*/

use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
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
}

struct Node {
    cost: u16,
    global_min_cost: u16,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct NodeSearcher {
    accumulated_cost: u16,
    min_future_cost: u16,
    total_estimate: u16,
    current_node: Coord,
    visited_nodes_count: u16,
    prev_dirs: VecDeque<Direction>,
}

impl NodeSearcher {
    fn new(
        accumulated_cost: u16,
        min_future_cost: u16,
        current_node: Coord,
        visited_nodes_count: u16,
        prev_dirs: VecDeque<Direction>,
    ) -> Self {
        let total_estimate = accumulated_cost + min_future_cost;

        NodeSearcher {
            accumulated_cost,
            min_future_cost,
            total_estimate,
            current_node,
            visited_nodes_count,
            prev_dirs,
        }
    }

    fn new_total_estimate(&mut self) {
        self.total_estimate = self.accumulated_cost + self.min_future_cost;
    }

    fn add_direction(&mut self, new_dir: Direction) {
        self.prev_dirs.push_back(new_dir);
        if self.prev_dirs.len() > 3 {
            self.prev_dirs.pop_front();
        }
    }
}
impl Ord for NodeSearcher {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .total_estimate
            .cmp(&self.total_estimate)
            .then_with(|| other.min_future_cost.cmp(&self.min_future_cost))
    }
}

impl PartialOrd for NodeSearcher {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct NumberGraph {
    cost_map: HashMap<Coord, Node>,
}

impl NumberGraph {
    fn from_string(input: &[String]) -> Self {
        let mut cost_map: HashMap<Coord, Node> = HashMap::new();

        for (y, y_line) in input.iter().enumerate() {
            for (x, x_char) in y_line.chars().enumerate() {
                let num = x_char.to_digit(10).unwrap() as u16;

                let current_coord = Coord {
                    x: x as i32,
                    y: y as i32,
                };
                cost_map.insert(
                    current_coord,
                    Node {
                        cost: num,
                        global_min_cost: u16::MAX / 2,
                    },
                );
            }
        }

        NumberGraph { cost_map }
    }

    fn get_cost_estimate(&self, start: &Coord, end: &Coord) -> u16 {
        let diff_x = end.x.abs_diff(start.x);
        let diff_y = end.y.abs_diff(start.y);

        (diff_x + diff_y) as u16
    }

    fn reduce_heap_size(
        &self,
        heap: BinaryHeap<NodeSearcher>,
        size: usize,
    ) -> BinaryHeap<NodeSearcher> {
        let mut vec: Vec<NodeSearcher> = heap.into_vec();

        //vec.sort();

        //let culled: Vec<NodeSearcher> = vec.into_iter().rev().take(size).collect();

        vec.sort_by(|a, b| {
            (a.accumulated_cost / a.visited_nodes_count + a.min_future_cost)
                .cmp(&(b.accumulated_cost / b.visited_nodes_count + b.min_future_cost))
        });
        let culled: Vec<NodeSearcher> = vec.into_iter().take(size).collect();

        BinaryHeap::from(culled)
    }

    fn can_go_direction(previous_dirs: &VecDeque<Direction>, current_dir: Direction) -> bool {
        if previous_dirs.len() < 3 {
            return true;
        }

        !previous_dirs.iter().all(|&d| d == current_dir)
    }

    fn max_3_in_row_a_star(&mut self, start: Coord, end: Coord) -> Option<u16> {
        let mut heap: BinaryHeap<NodeSearcher> = BinaryHeap::new();

        heap.push(NodeSearcher::new(
            0,
            self.get_cost_estimate(&start, &end),
            start,
            1,
            VecDeque::new(),
        ));

        while let Some(searcher) = heap.pop() {
            if searcher.current_node == end {
                //for c in searcher.visited_nodes {
                //    println!("{}, {}", c.x, c.y);
                //}
                return Some(searcher.accumulated_cost);
            }
            //let heap_len = heap.len();
            //const PRINT_COUNT: usize = 2_000;
            //if heap_len < PRINT_COUNT {
            //    dbg!(&heap_len);
            //}

            // cull the heap to make sure it doesnt get too large
            if heap.len() > 5_000_000 {
                dbg!("culling");
                heap = self.reduce_heap_size(heap, 4_000_000);
            }

            //if heap.len() < 20 {
            //    let last_3_dirs: Vec<&Direction> =
            //        searcher.prev_dirs.iter().rev().take(3).collect();
            //    dbg!(last_3_dirs);
            //    dbg!(&searcher.visited_nodes);
            //}

            let current_coord = searcher.current_node;

            macro_rules! process_direction {
                ($direction:expr, $new_coord:expr) => {
                    if let Some(next_node) = self.cost_map.get_mut(&$new_coord) {
                        if NumberGraph::can_go_direction(&searcher.prev_dirs, $direction)
                        //&& !searcher.visited_nodes.contains(&$new_coord)
                        {
                            let new_cost = searcher.accumulated_cost + next_node.cost as u16;
                            if new_cost < (next_node.global_min_cost + 10) {
                                next_node.global_min_cost = next_node.global_min_cost.min(new_cost);

                                let mut new_searcher = searcher.clone();
                                new_searcher.current_node = $new_coord;
                                new_searcher.accumulated_cost = new_cost;
                                new_searcher.min_future_cost =
                                    self.get_cost_estimate(&$new_coord, &end);
                                new_searcher.new_total_estimate();

                                new_searcher.add_direction($direction);
                                //new_searcher.visited_nodes.push_back($new_coord);
                                new_searcher.visited_nodes_count += 1;

                                heap.push(new_searcher);
                            }
                        }
                    }
                };
            }

            process_direction!(Direction::Up, current_coord.up());

            process_direction!(Direction::Right, current_coord.right());

            process_direction!(Direction::Down, current_coord.down());

            process_direction!(Direction::Left, current_coord.left());
        }

        None
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

    //let example_heatloss = get_min_heatloss_1(&example_1);
    //dbg!(&example_heatloss);
    //assert_eq!(example_heatloss, 102);

    let my_heatloss = get_min_heatloss_1(_my_input);
    dbg!(my_heatloss);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);
}

fn get_min_heatloss_1(input: &[String]) -> u16 {
    let mut graph = NumberGraph::from_string(input);

    let start_node = Coord { x: 0, y: 0 };

    let end_node = *graph.cost_map.keys().max().unwrap();

    //dbg!(end_node);

    graph.max_3_in_row_a_star(start_node, end_node).unwrap()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
