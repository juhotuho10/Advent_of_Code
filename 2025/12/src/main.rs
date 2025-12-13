/*
part 1:
we have 6 types 3x3 pieces and we have to pack a number of them into an area, we have to find if we can pack the required amount of them in to the area
*/

#![allow(clippy::ptr_arg)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use ndarray::Array2;

#[derive(Debug)]
struct Object {
    squares: Array2<bool>,
}

struct ObjectBox {
    area_size: (u8, u8),
    n_objects: [u8; 6],
}

impl ObjectBox {
    fn new(input: &String) -> Self {
        let (shape, counts_str) = input.split_once(": ").unwrap();
        let (x_str, y_str) = shape.split_once("x").unwrap();
        let x: u8 = x_str.parse().unwrap();
        let y: u8 = y_str.parse().unwrap();

        let counts_vec: Vec<u8> = counts_str
            .split_whitespace()
            .map(|count| count.parse().unwrap())
            .collect();
        assert_eq!(counts_vec.len(), 6);
        let object_counts: [u8; 6] = counts_vec.try_into().unwrap();

        ObjectBox {
            area_size: (x, y),
            n_objects: object_counts,
        }
    }

    fn trivial_check_fit(&self, objects: &[Object; 6]) -> bool {
        let total_size = self.area_size.0 as u16 * self.area_size.1 as u16;
        let trivial_packing_size: u16 = self.n_objects.iter().map(|obj| *obj as u16 * 9).sum();

        if trivial_packing_size < total_size {
            return true;
        }

        let max_packing: u16 = self
            .n_objects
            .iter()
            .zip(objects.iter())
            .map(|(count, obj)| {
                let n_squares: u16 = obj.squares.map(|b| if *b { 1 } else { 0 }).sum();
                n_squares * *count as u16
            })
            .sum();

        if max_packing > total_size {
            return false;
        }

        true
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
    assert_eq!(example_1, 3);

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
    assert_eq!(example_2, 0);

    let start = Instant::now();
    let solution_2 = solution_2(_my_input);
    let elapsed = start.elapsed().as_micros();
    println!("Time to solve problem 2: {elapsed}µs");
    dbg!(solution_2);
}

fn solution_1(input: &[String]) -> u64 {
    let (objects, boxes) = parse_input(input);

    boxes
        .into_iter()
        .filter(|b| b.trivial_check_fit(&objects))
        .count() as u64
}

fn solution_2(_input: &[String]) -> u64 {
    0
}

fn parse_input(input: &[String]) -> ([Object; 6], Vec<ObjectBox>) {
    let mut split_data = input.split(|line| line.is_empty());
    let all_boxes = split_data.next_back().unwrap();
    let objectbox_vec: Vec<ObjectBox> = all_boxes.iter().map(ObjectBox::new).collect();

    let object_array: [Object; 6] = {
        let objects_vecs: Vec<Vec<Vec<bool>>> = split_data
            .take(6)
            .map(|rows| {
                rows[1..]
                    .iter()
                    .map(|row| {
                        row.chars()
                            .filter_map(|c| match c {
                                '#' => Some(true),
                                '.' => Some(false),
                                _ => None,
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        let array_vecs: Vec<Array2<bool>> = objects_vecs
            .into_iter()
            .map(|vec_vec| {
                let flat_vec: Vec<bool> = vec_vec.into_iter().flatten().collect();
                Array2::from_shape_vec((3, 3), flat_vec).unwrap()
            })
            .collect();
        let object_vec: Vec<Object> = array_vecs
            .into_iter()
            .map(|arr| Object { squares: arr })
            .collect();
        object_vec.try_into().unwrap()
    };

    (object_array, objectbox_vec)
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
