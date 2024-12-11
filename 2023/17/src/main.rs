/*
we have to push a hot cart of lava and we have a grid of numbers that indicate how much the lava is cooled on that tile
we have to push the cart from the top left to the bottom right while losing as little heat as possible
also the path must not have more than 3 tiles in a straight line

*/

use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

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
    global_min_cost_from_turn: Option<u16>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct AStarNode {
    current_node: Coord,
    accumulated_cost: u16,
    min_future_cost: u16,
    total_estimate: u16,
}

impl AStarNode {
    fn new(accumulated_cost: u16, min_future_cost: u16, current_node: Coord) -> Self {
        let total_estimate = accumulated_cost + min_future_cost;

        AStarNode {
            accumulated_cost,
            min_future_cost,
            total_estimate,
            current_node,
        }
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .total_estimate
            .cmp(&self.total_estimate)
            .then_with(|| other.min_future_cost.cmp(&self.min_future_cost))
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct NodeSearcher {
    accumulated_cost: u16,
    min_future_cost: u16,
    total_estimate: u16,
    current_node: Coord,
    visited_nodes_count: u16,
    subsequent_unoptimal_count: u16,
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
            subsequent_unoptimal_count: 0,
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
                        global_min_cost_from_turn: None,
                    },
                );
            }
        }

        NumberGraph { cost_map }
    }

    fn normal_a_star_cost(&self, start: &Coord, end: &Coord) -> Option<u16> {
        let mut heap: BinaryHeap<AStarNode> = BinaryHeap::new();
        let mut cost_so_far: HashMap<Coord, u16> = HashMap::new();

        heap.push(AStarNode::new(
            0,
            self.get_min_dist_to_end(start, end),
            *start,
        ));
        cost_so_far.insert(*start, 0);

        while let Some(current_node) = heap.pop() {
            let current_coord = current_node.current_node;

            if current_coord == *end {
                return Some(current_node.accumulated_cost);
            }

            let current_cost = cost_so_far[&current_coord];

            macro_rules! process_direction {
                ($new_coord:expr) => {
                    if let Some(next_node) = self.cost_map.get(&$new_coord) {
                        let new_cost = current_cost + next_node.cost as u16;

                        if new_cost < *cost_so_far.get(&$new_coord).unwrap_or(&u16::MAX) {
                            cost_so_far.insert($new_coord, new_cost);

                            let min_future_cost = self.get_min_dist_to_end(&$new_coord, end);
                            heap.push(AStarNode::new(new_cost, min_future_cost, $new_coord));
                        }
                    }
                };
            }

            process_direction!(current_coord.up());
            process_direction!(current_coord.right());
            process_direction!(current_coord.down());
            process_direction!(current_coord.left());
        }

        None // No path found
    }

    fn get_min_dist_to_end(&self, start: &Coord, end: &Coord) -> u16 {
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
            // 993
            //(a.accumulated_cost / a.visited_nodes_count + a.min_future_cost)
            //    .cmp(&(b.accumulated_cost / b.visited_nodes_count + b.min_future_cost))

            // 995
            //(a.accumulated_cost as f32 / a.visited_nodes_count as f32 + (a.min_future_cost as f32))
            //    .partial_cmp(
            //        &(b.accumulated_cost as f32 / b.visited_nodes_count as f32
            //            + (b.min_future_cost as f32)),
            //    )
            //    .unwrap()

            // 1000 <
            //a.accumulated_cost as f32 / a.visited_nodes_count as f32
            //   + (a.min_future_cost as f32 * 0.2))
            //   .partial_cmp(
            //       &(b.accumulated_cost as f32 / b.visited_nodes_count as f32
            //           + (b.min_future_cost as f32 * 0.2)),
            //   )
            //   .unwrap()

            (a.total_estimate).cmp(&(b.total_estimate))
        });
        let culled: Vec<NodeSearcher> = vec.into_iter().take(size).collect();

        BinaryHeap::from(culled)
    }

    fn can_go_direction(previous_dirs: &VecDeque<Direction>, current_dir: Direction) -> bool {
        if previous_dirs.len() < 3 {
            return true;
        }

        let prev_dir = previous_dirs.back().unwrap();
        match (prev_dir, current_dir) {
            (Direction::Up, Direction::Down) => return false,
            (Direction::Right, Direction::Left) => return false,
            (Direction::Down, Direction::Up) => return false,
            (Direction::Left, Direction::Right) => return false,
            (_, _) => {} // pass
        }

        !previous_dirs.iter().all(|&d| d == current_dir)
    }

    fn max_3_in_row_a_star(&mut self, start: Coord, end: Coord) -> Option<u16> {
        let mut heap: BinaryHeap<NodeSearcher> = BinaryHeap::new();
        let mut cost_cache: HashMap<Coord, u16> = HashMap::new();

        heap.push(NodeSearcher::new(
            0,
            self.normal_a_star_cost(&start, &end).unwrap(),
            start,
            1,
            VecDeque::new(),
        ));

        let mut counter = 0;

        while let Some(searcher) = heap.pop() {
            if searcher.current_node == end {
                //for c in searcher.visited_nodes {
                //    println!("{}, {}", c.x, c.y);
                //}
                return Some(searcher.accumulated_cost);
            }
            counter += 1;
            if counter % 10_000_000 == 0 {
                dbg!(&heap.len());
            }
            //let heap_len = heap.len();
            //const PRINT_COUNT: usize = 2_000;
            //if heap_len < PRINT_COUNT {
            //    dbg!(&heap_len);
            //}

            // cull the heap to make sure it doesnt get too large
            if heap.len() > 20_000_000 {
                dbg!("culling");
                heap = self.reduce_heap_size(heap, 16_000_000);
            }

            if searcher.subsequent_unoptimal_count > 5 {
                continue;
            }

            let current_coord = searcher.current_node;

            macro_rules! process_direction {
                ($direction:expr, $new_coord:expr) => {
                    if let Some(next_node) = self.cost_map.get_mut(&$new_coord) {
                        if NumberGraph::can_go_direction(&searcher.prev_dirs, $direction) {
                            let new_cost = searcher.accumulated_cost + next_node.cost as u16;
                            let mut should_continue = true;

                            if let Some(&prev_dir) = searcher.prev_dirs.back() {
                                let has_turned = prev_dir != $direction;

                                if has_turned {
                                    if let Some(prev_turn_cost) =
                                        next_node.global_min_cost_from_turn
                                    {
                                        if prev_turn_cost < new_cost {
                                            should_continue = false;
                                        }
                                    } else {
                                        next_node.global_min_cost_from_turn = Some(new_cost);
                                    }
                                }
                            }

                            if new_cost < (next_node.global_min_cost + 10) && should_continue {
                                let mut new_searcher = searcher.clone();

                                if new_cost <= next_node.global_min_cost {
                                    next_node.global_min_cost = new_cost;
                                    new_searcher.subsequent_unoptimal_count = 0;
                                } else {
                                    new_searcher.subsequent_unoptimal_count += 1;
                                }

                                new_searcher.current_node = $new_coord;
                                new_searcher.accumulated_cost = new_cost;

                                let mut new_cost = false;
                                new_searcher.min_future_cost = match cost_cache.get(&$new_coord) {
                                    Some(cost) => *cost,
                                    None => {
                                        new_cost = true;
                                        self.normal_a_star_cost(&$new_coord, &end).unwrap()
                                    }
                                };

                                if new_cost {
                                    cost_cache.insert($new_coord, new_searcher.min_future_cost);
                                }

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

    let start = Instant::now();
    let example_heatloss = get_min_heatloss_1(&example_1);
    dbg!(start.elapsed());
    dbg!(&example_heatloss);

    let my_heatloss = get_min_heatloss_1(_my_input);
    assert_eq!(my_heatloss, 1008);
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
