/*
part 1:
we have a list of devices and output devices that those devices are connected to
we need to start with device labeled "you" find every path to the "out" device
and return the count of paths
part 2:
we now know that we need to find paths from "svr" -> "out" that also visits "dac" and "fft" at some point

*/
#![allow(clippy::ptr_arg)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use ahash::AHashMap;

type Name = [u8; 3];
const START: [u8; 3] = *"you".as_bytes().as_array().unwrap();
const OUT: [u8; 3] = *"out".as_bytes().as_array().unwrap();

const SVR: [u8; 3] = *"svr".as_bytes().as_array().unwrap();
const FFT: [u8; 3] = *"fft".as_bytes().as_array().unwrap();
const DAC: [u8; 3] = *"dac".as_bytes().as_array().unwrap();

struct Device {
    name: [u8; 3],
    out: Vec<[u8; 3]>,
}

impl Device {
    fn new(input: &String) -> Self {
        let (name_str, outputs_str) = input.split_once(": ").unwrap();
        let name: [u8; 3] = *name_str.as_bytes().as_array().unwrap();
        let outputs_vec: Vec<[u8; 3]> = outputs_str
            .split_whitespace()
            .map(|str_name| *str_name.as_bytes().as_array().unwrap())
            .collect();

        Device {
            name,
            out: outputs_vec,
        }
    }
}

struct AllDevices(AHashMap<Name, Vec<Name>>);

impl AllDevices {
    fn new(devices: Vec<Device>) -> Self {
        let mut hashmap = AHashMap::new();
        for device in devices {
            hashmap.insert(device.name, device.out);
        }
        AllDevices(hashmap)
    }
    fn find_out_paths(self) -> u64 {
        let mut memo_map = AHashMap::new();

        self.recursive_path_search(&mut memo_map, &START, &OUT)
    }

    fn find_svr_out_paths(self) -> u64 {
        let svr_fft = self.recursive_path_search(&mut AHashMap::new(), &SVR, &FFT);
        let fft_dac = self.recursive_path_search(&mut AHashMap::new(), &FFT, &DAC);
        let dac_out = self.recursive_path_search(&mut AHashMap::new(), &DAC, &OUT);
        svr_fft * fft_dac * dac_out
    }

    fn recursive_path_search(
        &self,
        memo: &mut AHashMap<Name, u64>,
        current: &Name,
        out: &Name,
    ) -> u64 {
        if current == out {
            return 1;
        } else if let Some(cached_result) = memo.get(current) {
            return *cached_result;
        }

        let Some(out_devices) = self.0.get(current) else {
            return 0; // the path is output only, but not the path that we wanted
        };

        let out_sum: u64 = out_devices
            .iter()
            .map(|name| self.recursive_path_search(memo, name, out))
            .sum();

        memo.insert(*current, out_sum);

        out_sum
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

    let example_1 = solution_1(&example_1);
    dbg!(&example_1);
    assert_eq!(example_1, 5);

    let start = Instant::now();
    let solution_1 = solution_1(_my_input);
    let elapsed = start.elapsed().as_micros();
    println!("Time to solve problem 1: {elapsed}µs");
    dbg!(solution_1);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_2 = solution_2(&example_2);
    dbg!(&example_2);
    assert_eq!(example_2, 2);

    let start = Instant::now();
    let solution_2 = solution_2(_my_input);
    let elapsed = start.elapsed().as_micros();
    println!("Time to solve problem 2: {elapsed}µs");
    dbg!(solution_2);
}

fn solution_1(input: &[String]) -> u64 {
    let devices = parse_input(input);
    devices.find_out_paths()
}

fn solution_2(input: &[String]) -> u64 {
    let devices = parse_input(input);
    devices.find_svr_out_paths()
}

fn parse_input(input: &[String]) -> AllDevices {
    AllDevices::new(input.iter().map(Device::new).collect())
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
