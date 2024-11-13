/*

We have a map of locations in a key - value map sort of way
each key place has the possibility of going left of right
the left and right options are another key values
we start at AAA and we have to go through this map in a predefinied pattern of left and rights until we get to ZZZ
the answer is the amount of steps that we had to take to get there

part 2:
    I have to start on all nodes that end with "A" and I have to move them all simultaniously until every single on of the nodes
    is a place that ends with "Z" all the same time
    and then i return the steps that I took
*/

use regex::Regex;
use std::collections::HashMap;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_path = get_shortest_path_1(&example_1);
    dbg!(&example_path);
    assert_eq!(example_path, 6);

    let my_path = get_shortest_path_1(_my_input);
    dbg!(my_path);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_path = get_shortest_path_2(&example_2);
    dbg!(&example_path);
    assert_eq!(example_path, 6);

    let my_path = get_shortest_path_2(_my_input);
    dbg!(my_path);
}

fn get_shortest_path_1(input: &[String]) -> u32 {
    let mut current_location = "AAA";
    let mut steps_taken = 0;
    let (path_vec, location_map) = parse_input(input);

    for turn in path_vec.iter().cycle() {
        let next_locations = location_map.get(current_location).unwrap();
        current_location = &next_locations[*turn];
        steps_taken += 1;
        if current_location == "ZZZ" {
            break;
        }
    }
    steps_taken
}

fn least_common_multiple(a: u64, b: u64) -> u64 {
    let greatest_divisor = {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }

        a
    };

    a * b / greatest_divisor
}

fn get_shortest_path_2(input: &[String]) -> u64 {
    let (path_vec, location_map) = parse_input(input);
    let mut cycle_lens: Vec<u64> = vec![];

    let mut current_locations: Vec<&String> = location_map
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect();

    for location in current_locations.iter_mut() {
        let mut steps = 0;

        for turn in path_vec.iter().cycle() {
            let next_locations = location_map.get(*location).unwrap();
            *location = &next_locations[*turn];
            steps += 1;

            if location.ends_with("Z") {
                cycle_lens.push(steps);
                break;
            }
        }
    }

    // least common multiple to find when the cycles intersect
    cycle_lens
        .iter()
        .copied()
        .reduce(least_common_multiple)
        .unwrap()
}

fn parse_input(input: &[String]) -> (Vec<usize>, HashMap<String, [String; 2]>) {
    let mut location_map: HashMap<String, [String; 2]> = HashMap::new();

    let path_str = input[0].to_owned();

    let mut path_vec: Vec<usize> = vec![];
    for c in path_str.chars() {
        if c == 'L' {
            path_vec.push(0);
        } else {
            path_vec.push(1);
        }
    }

    // AAA = (BBB, BBB)
    let re = Regex::new(r"([A-Z\d]{3}) = \(([A-Z\d]{3})\, ([A-Z\d]{3})\)").unwrap();

    let locations: &[String] = &input[2..];
    for loc in locations {
        let caputers = re.captures(loc).unwrap();
        let from = caputers[1].to_owned();
        let to_l = caputers[2].to_owned();
        let to_r = caputers[3].to_owned();

        location_map.insert(from, [to_l, to_r]);
    }

    (path_vec, location_map)
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("wont fail");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
