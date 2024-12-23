/*
part 1:
we are at a lan party with a lot of connected computers
we know all the connections from computer to computer
we have to find the sets where 3 computers are all connected to eachother and at least one computers
name starts with t and then return the count of those sets
part 2:

we have to find the largest group where all the computers are connected to eachother
and return those IDs as a string in alphabetical order

*/

use fxhash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug)]
struct Connections {
    connected: FxHashMap<String, Vec<String>>,
}

impl Connections {
    fn from_string(input: &[String]) -> Self {
        let mut all_connections: FxHashMap<String, Vec<String>> = FxHashMap::default();

        for connect in input {
            let (c1, c2) = connect.split_once("-").unwrap();

            let pc_connection = all_connections.entry(c1.to_owned()).or_default();
            pc_connection.push(c2.to_owned());

            let pc_connection = all_connections.entry(c2.to_owned()).or_default();
            pc_connection.push(c1.to_owned());
        }

        Connections {
            connected: all_connections,
        }
    }

    fn recursive_loop_search(
        &self,
        started_from: &str,
        prev: &str,
        prev_nodes: Vec<String>,
        depth: u8,
    ) -> Vec<Vec<String>> {
        let current = prev_nodes.last().unwrap();
        let mut current_connected = self.connected[current].to_owned();
        current_connected.retain(|c| c != prev);

        if depth == 0 {
            if current_connected.contains(&started_from.to_owned()) {
                return vec![prev_nodes];
            } else {
                return vec![];
            }
        }

        let mut circle_connections = vec![];

        for connnection in &current_connected {
            let mut new_nodes = prev_nodes.clone();
            new_nodes.push(connnection.to_owned());
            let new_return =
                self.recursive_loop_search(started_from, current, new_nodes, depth - 1);
            circle_connections.extend(new_return);
        }

        circle_connections
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

    let example_sum = find_connected_nodes_with_t_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 7);

    let my_sum = find_connected_nodes_with_t_1(_my_input);
    assert!(my_sum < 2142);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = largest_connected_group_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, "co,de,ka,ta");

    let start = Instant::now();
    let my_sum = largest_connected_group_2(_my_input);
    dbg!(start.elapsed());
    dbg!(my_sum);
}

fn find_connected_nodes_with_t_1(input: &[String]) -> u32 {
    let connections = Connections::from_string(input);
    let mut t_connections: FxHashSet<Vec<String>> = FxHashSet::default();

    for start in connections.connected.keys() {
        let mut circle_connections =
            connections.recursive_loop_search(start, start, vec![start.to_owned()], 2);

        circle_connections.iter_mut().for_each(|vec| vec.sort());

        t_connections.extend(circle_connections);
    }

    t_connections.retain(|vec| vec.iter().any(|node| node.starts_with('t')));

    t_connections.len() as u32
}

fn largest_connected_group_2(input: &[String]) -> String {
    let original_connections = Connections::from_string(input);

    let mut largest_connection = vec![];
    let mut largest_connection_count = 0;

    let mut all_ids: VecDeque<&String> = original_connections.connected.keys().collect();

    while let Some(pc) = all_ids.pop_front() {
        let all_connected = original_connections.connected[pc].clone();

        let mut connection_connections: Vec<String> = all_connected
            .iter()
            .flat_map(|s| original_connections.connected[s].clone())
            .collect();

        connection_connections.extend(all_connected);

        let mut count_map: FxHashMap<String, usize> = FxHashMap::default();

        for conenction in connection_connections {
            *count_map.entry(conenction).or_insert(0) += 1;
        }

        let mut unique: Vec<&usize> = count_map
            .values()
            .collect::<FxHashSet<&usize>>()
            .into_iter()
            .collect();
        unique.sort();

        let mut current_connections: Vec<String> = vec![];
        let mut most_current_connections = 0;
        for &i in unique {
            let connections: Vec<(&String, &usize)> =
                count_map.iter().filter(|(_, count)| **count >= i).collect();

            if connections.len() > i {
                if i > most_current_connections {
                    current_connections =
                        connections.into_iter().map(|(s, _)| s.to_owned()).collect();
                    most_current_connections = i;
                }
            } else {
                all_ids.retain(|id| !current_connections.contains(id));

                break;
            }
        }

        if most_current_connections > largest_connection_count {
            largest_connection = current_connections;
            largest_connection_count = most_current_connections;
        }
    }

    largest_connection.sort();

    largest_connection.join(",")
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
