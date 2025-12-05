/*
part 1:
we have to search for fresh and spoiled ingredients with ids from fresh id ranges. we need to return the count of fresh ingredients
part 2:
we have to find the total number of fresh ids, the problem is that the ranges can be overlapping

*/
#![feature(new_range_api)]
#![feature(slice_split_once)]
use std::fs::File;
use std::io::{BufRead, BufReader};
struct FreshIDs(Vec<(u64, u64)>);
impl FreshIDs {
    fn new(input: &[String]) -> Self {
        let mut ranges: Vec<(u64, u64)> = Vec::new();
        for str in input {
            let (start_str, end_str) = str.split_once("-").unwrap();
            ranges.push((start_str.parse().unwrap(), end_str.parse().unwrap()));
        }

        FreshIDs(ranges)
    }

    fn find_fresh_produce(self, ids: &[u64]) -> Vec<u64> {
        ids.iter()
            .filter(|id| self.0.iter().any(|(start, end)| (start..=end).contains(id)))
            .copied()
            .collect()
    }

    fn remove_self_contained(&mut self) -> bool {
        let range_copies = self.0.clone();

        self.0.retain(|(start, end)| {
            range_copies.iter().all(|(copy_start, copy_end)| {
                (start == copy_start && end == copy_end)
                    || !((copy_start..=copy_end).contains(&start)
                        && (copy_start..=copy_end).contains(&end))
            })
        });

        range_copies != self.0
    }

    fn extend_start(&mut self) -> bool {
        let range_copies = self.0.clone();
        let mut change = false;

        for (start, end) in self.0.iter_mut() {
            for (copy_start, copy_end) in &range_copies {
                if (*start..=*end).contains(copy_end) && copy_start < start {
                    *start = *copy_start;
                    change = true;
                }
            }
        }

        change
    }

    fn extend_end(&mut self) -> bool {
        let range_copies = self.0.clone();
        let mut change = false;

        for (start, end) in self.0.iter_mut() {
            for (copy_start, copy_end) in &range_copies {
                if (*start..=*end).contains(copy_start) && copy_end > end {
                    *end = *copy_end;
                    change = true;
                }
            }
        }

        change
    }

    fn find_num_fresh_ids(mut self) -> u64 {
        // extend self
        // remove selfcontained
        // repeat if changed

        let mut changed = true;
        changed |= self.remove_self_contained();
        while changed {
            changed = false;
            changed |= self.extend_start();
            changed |= self.remove_self_contained();
            changed |= self.extend_end();
            changed |= self.remove_self_contained();
        }

        self.0.sort();
        self.0.dedup();

        self.0.iter().map(|(start, end)| end + 1 - start).sum()
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

    let example_sum = solution_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 3);

    let my_sum = solution_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 14);

    let my_sum = solution_2(_my_input);
    dbg!(my_sum);
}

fn solution_1(input: &[String]) -> u32 {
    let (fresh_ids, ids) = parse_input(input);

    fresh_ids.find_fresh_produce(&ids).len() as u32
}

fn solution_2(input: &[String]) -> u64 {
    let (fresh_ids, _) = parse_input(input);

    fresh_ids.find_num_fresh_ids()
}

fn parse_input(input: &[String]) -> (FreshIDs, Vec<u64>) {
    let mut split_input = input.split(|line| line.trim().is_empty());
    let fresh_ids = split_input.next().unwrap();
    let id_strings = split_input.next().unwrap();

    let ids: Vec<u64> = id_strings
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let fresh_ranges = FreshIDs::new(fresh_ids);
    (fresh_ranges, ids)
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
