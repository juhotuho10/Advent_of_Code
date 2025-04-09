/*
part 1:
we have a collection of pillars in a falling state
we get the base coords for both sides of the pillars
the pillars have to fall down until they hit a ground or are stopped by another pillar
then from the pillars that have fallen, we have to figure out which ones are safe to delete without disturbing any other pillars

and the answer is the count of pillars that are safe to delete

part 2:

*/

use ndarray::{s, Array3, ArrayViewMut3, Dim};
use std::fs::File;
use std::io::{BufRead, BufReader};

type ID = i16;

#[derive(Debug, Clone)]
struct Coord {
    x: u16,
    y: u16,
    z: u16,
}

impl Coord {
    fn new(x: u16, y: u16, z: u16) -> Self {
        Coord { x, y, z }
    }
    fn from_string(input: &str) -> Self {
        let coords: Vec<&str> = input.splitn(3, ",").collect();

        Coord {
            x: coords[0].parse().unwrap(),
            y: coords[2].parse().unwrap(),
            z: coords[1].parse().unwrap(),
        }
    }

    fn get_range(&self, other: &Coord) -> Vec<Coord> {
        let mut min_y_blocks = vec![];
        for x in self.x..=other.x {
            for y in self.y..=other.y {
                for z in self.z..=other.z {
                    min_y_blocks.push(Coord::new(x, y, z));
                }
            }
        }

        min_y_blocks
    }
}

struct Arena {
    blocks: Array3<ID>,
}

impl Arena {
    fn get_blocks(
        &self,
        start: &Coord,
        end: &Coord,
    ) -> ndarray::ArrayBase<ndarray::ViewRepr<&ID>, Dim<[usize; 3]>> {
        self.blocks.slice(s![
            (start.x as usize)..=(end.x as usize),
            (start.y as usize)..=(end.y as usize),
            (start.z as usize)..=(end.z as usize)
        ])
    }

    fn get_blocks_mut(&mut self, start: &Coord, end: &Coord) -> ArrayViewMut3<ID> {
        self.blocks.slice_mut(s![
            (start.x as usize)..=(end.x as usize),
            (start.y as usize)..=(end.y as usize),
            (start.z as usize)..=(end.z as usize)
        ])
    }

    fn get_pillars_ids(&self, start: &Coord, end: &Coord) -> Vec<ID> {
        let blocks = self.get_blocks(start, end);
        let mut id_list: Vec<&ID> = blocks.iter().collect();
        id_list.sort();
        id_list.dedup();
        id_list.retain(|&&id| id != -1);
        id_list.iter().map(|id| **id).collect()
    }
}

struct Pillar {
    id: ID,
    start: Coord,
    end: Coord,
}

impl Pillar {
    fn from_string(input: &str, id: ID) -> Self {
        let (start_str, end_str) = input.split_once("~").unwrap();

        let start = Coord::from_string(start_str);
        let end = Coord::from_string(end_str);

        let _blocks = start.get_range(&end);

        Pillar { id, start, end }
    }

    fn get_under(&self) -> (Coord, Coord) {
        let mut copy_start = self.start.clone();
        let mut copy_end = self.end.clone();

        copy_start.y -= 1;
        copy_end.y -= 1;

        (copy_start, copy_end)
    }

    fn get_upper(&self) -> (Coord, Coord) {
        let mut copy_start = self.start.clone();
        let mut copy_end = self.end.clone();

        copy_start.y += 1;
        copy_end.y += 1;

        (copy_start, copy_end)
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
    assert_eq!(example_sum, 5);

    let my_sum = solution_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 0);

    let my_sum = solution_2(_my_input);
    dbg!(my_sum);
}

fn solution_1(input: &[String]) -> u32 {
    let (mut pillars, mut arena) = parse_input(input);

    dbg!(arena.blocks.dim());

    for pillar in pillars.iter_mut() {
        loop {
            let blocks = arena.get_blocks(&pillar.start, &pillar.end);

            if blocks.iter().all(|block_id| *block_id == -1) {
                if pillar.start.y == 0 || pillar.end.y == 0 {
                    // hit the ground
                    let mutable_blocks = arena.get_blocks_mut(&pillar.start, &pillar.end);

                    for block in mutable_blocks {
                        *block = pillar.id;
                    }
                    break;
                } else {
                    // in the air
                    pillar.start.y -= 1;
                    pillar.end.y -= 1;
                }
            } else {
                // collided with another pillar
                pillar.start.y += 1;
                pillar.end.y += 1;

                let mutable_blocks = arena.get_blocks_mut(&pillar.start, &pillar.end);

                for block in mutable_blocks {
                    *block = pillar.id;
                }
                break;
            }
        }
    }

    // there are 2 ways the pillar is safe to disintegrage:
    //  1. the pillar isnt supporting any other pillars
    //  2. all the pillars that are ontop are supported by multiple pillars

    pillars.sort_by_key(|pillar| (pillar.start.y).min(pillar.end.y));

    pillars
        .iter()
        .filter(|pillar| {
            // filtering safe to remove pillars
            let (top_start, top_end) = pillar.get_upper();
            let mut above_ids = arena.get_pillars_ids(&top_start, &top_end);
            above_ids.retain(|&id| id != pillar.id);

            // nothing abouve the pillar, so safe to remove
            if above_ids.is_empty() {
                true
            } else {
                // something above the pillar, we have to check that every pillar on top is supported by at least 2 pillars for
                // the pillar below to be safe to remove
                above_ids.iter().all(|above_id| {
                    let above_pillar = pillars
                        .iter()
                        .find(|p| p.id == *above_id)
                        .expect("pillars never removed, so shouldnt fail");

                    let (below_start, below_end) = above_pillar.get_under();

                    let mut supporting_ids = arena.get_pillars_ids(&below_start, &below_end);
                    supporting_ids.retain(|&id| id != above_pillar.id);
                    supporting_ids.len() >= 2 // if there are 2 or more unique pillars under, then it's also supported, else, it is not
                })
            }
        })
        .count() as u32 // count of the pillars that are safe to remove
}

fn solution_2(input: &[String]) -> u32 {
    let parsed = parse_input(input);
    0
}

fn parse_input(input: &[String]) -> (Vec<Pillar>, Arena) {
    let mut pillars: Vec<Pillar> = input
        .iter()
        .enumerate()
        .map(|(id, line)| Pillar::from_string(line, id as ID))
        .collect();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for pillar in &pillars {
        if pillar.start.x > max_x {
            max_x = pillar.start.x
        }
        if pillar.start.y > max_y {
            max_y = pillar.start.y
        }
        if pillar.start.z > max_z {
            max_z = pillar.start.z
        }

        if pillar.end.x > max_x {
            max_x = pillar.end.x
        }
        if pillar.end.y > max_y {
            max_y = pillar.end.y
        }
        if pillar.end.z > max_z {
            max_z = pillar.end.z
        }
    }

    max_x += 1;
    max_y += 1;
    max_z += 1;

    // sort to min y
    pillars.sort_by_key(|pillar| (pillar.start.y).min(pillar.end.y));

    let blocks = Array3::from_elem((max_x as usize, max_y as usize, max_z as usize), -1);

    (pillars, Arena { blocks })
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
