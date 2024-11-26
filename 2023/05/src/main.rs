/*
- the input is a list of seeds
- we have maps that map the seed num to other values
- the map are [destination range start] [source range start] [range length]
- the seeds are mapped from seed -> soil -> fertilizer -> water -> light -> temperature -> huminidty -> location
- if the input doesnt have a map value, the ending value is the same as the input value
- the result is the min location value from the seeds
*/

use std::fs::File;

use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct RangeMapping {
    dest_start: u32,
    source_start: u32,
    range_len: u32,
}

impl RangeMapping {
    fn from_string(input_str: &str) -> Self {
        let strings: Vec<&str> = input_str.trim().splitn(3, ' ').collect();

        RangeMapping {
            dest_start: strings[0].to_owned().parse::<u32>().unwrap(),
            source_start: strings[1].to_owned().parse::<u32>().unwrap(),
            range_len: strings[2].to_owned().parse::<u32>().unwrap(),
        }
    }

    fn get_next(&self, value: &u32) -> Option<u32> {
        let diff: i32 = (*value as i32) - (self.source_start as i32);
        if (diff >= 0) && (diff < self.range_len as i32) {
            let new_value = self.dest_start + diff as u32;
            return Some(new_value);
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct AllRanges {
    ranges: Vec<RangeMapping>,
}

impl AllRanges {
    fn get_next_all(&self, value: &u32) -> Option<u32> {
        self.ranges
            .iter()
            .map(|range_mapping| range_mapping.get_next(value))
            .find(|value| value.is_some())
            .flatten()
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

    let example_location = seed_to_location_mapping_1(&example_1);
    dbg!(&example_location);
    assert_eq!(example_location, 35);

    let my_location = seed_to_location_mapping_1(_my_input);
    dbg!(&my_location);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_location = seed_to_location_mapping_2(&example_2);
    dbg!(&example_location);
    assert_eq!(example_location, 46);

    let my_location = seed_to_location_mapping_2(_my_input);
    dbg!(&my_location);
}

fn seed_to_location_mapping_1(input: &[String]) -> u32 {
    let mut map_values = parse_input_to_vecs(input);
    let seeds_str_vec = map_values.remove(0)[0].clone();
    let seeds: Vec<u32> = seeds_str_vec
        .split(" ")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mapper = get_location_mapper(map_values);

    let mut final_locations = vec![];
    for seed in seeds {
        let seed_location = get_final_seed_location(&mapper, seed);
        final_locations.push(seed_location);
    }

    *final_locations.iter().min().to_owned().unwrap()
}

fn seed_to_location_mapping_2(input: &[String]) -> u32 {
    let mut map_values = parse_input_to_vecs(input);
    let seeds_str_vec = map_values.remove(0)[0].clone();
    let seed_ranges: Vec<u32> = seeds_str_vec
        .split(" ")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mapper = get_location_mapper(map_values);

    let mut min_value = u32::MAX;
    for vec_slice in seed_ranges.chunks(2) {
        let iter_start = vec_slice[0];
        let iter_len = vec_slice[1];
        for seed in iter_start..(iter_start + iter_len) {
            let new_location = get_final_seed_location(&mapper, seed);
            min_value = min_value.min(new_location)
        }
    }

    min_value
}

fn get_location_mapper(map_values: Vec<Vec<String>>) -> Vec<AllRanges> {
    let mut all_hashmaps: Vec<AllRanges> = vec![];
    for ranges_vec in &map_values {
        let mut range_collection = vec![];

        for range_string in ranges_vec {
            let new_range_map = RangeMapping::from_string(range_string);

            range_collection.push(new_range_map);
        }
        let full_map = AllRanges {
            ranges: range_collection,
        };

        all_hashmaps.push(full_map);
    }

    all_hashmaps
}

fn get_final_seed_location(mapper: &Vec<AllRanges>, seed: u32) -> u32 {
    let mut current_value = seed;
    for map in mapper {
        let next_value = map.get_next_all(&current_value);
        if let Some(value) = next_value {
            current_value = value;
        }
    }
    current_value
}

fn parse_input_to_vecs(input: &[String]) -> Vec<Vec<String>> {
    let mut mapping_vectors: Vec<Vec<String>> = vec![];

    let mut temp_vec = vec![];
    for line in input {
        if line.is_empty() {
            mapping_vectors.push(temp_vec.clone());
            temp_vec.clear();
        } else {
            temp_vec.push(line.clone());
        }
    }
    mapping_vectors.push(temp_vec.clone());

    let seeds_str: String = mapping_vectors[0][0].clone();
    let (_, seed_values) = seeds_str.split_once(": ").unwrap();
    let seed_value_vec: Vec<String> = vec![seed_values.to_owned()];

    let mut mapping_vectors: Vec<Vec<String>> = mapping_vectors
        .iter()
        .skip(1)
        .map(|vec| vec.iter().skip(1).cloned().collect())
        .collect();

    mapping_vectors.insert(0, seed_value_vec);
    mapping_vectors
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("wont fail");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
