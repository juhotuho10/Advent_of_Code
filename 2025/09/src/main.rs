/*
part 1:
we have a collection of possible tiles
we want the biggest rectangle between the tiles possible
part 2:

all the tiles are connected by green tiles and we must have the biggest rectangle of red tile from corner to corner
that does not go outside the marked area of red and green tiles

*/

#![feature(array_windows)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

struct Tiles(Vec<(u32, u32)>);

impl Tiles {
    fn new(input: &[String]) -> Self {
        let mut tiles_vec = Vec::new();
        for line in input {
            let (x_str, y_str) = line.split_once(",").unwrap();
            tiles_vec.push((x_str.parse().unwrap(), y_str.parse().unwrap()));
        }

        Tiles(tiles_vec)
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
    assert_eq!(example_1, 50);

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
    assert_eq!(example_2, 24);

    let start = Instant::now();
    let solution_2 = solution_2(_my_input);
    let elapsed = start.elapsed().as_micros();
    println!("Time to solve problem 2: {elapsed}µs");
    dbg!(solution_2);
}

fn solution_1(input: &[String]) -> u64 {
    let tiles = parse_input(input).0;
    let tile_len = tiles.len();
    (0..tile_len - 1)
        .map(|tile1_i| {
            ((tile1_i + 1)..tile_len)
                .map(|tile2_i| {
                    let (x1, y1) = tiles[tile1_i];
                    let (x2, y2) = tiles[tile2_i];
                    (x1.abs_diff(x2) + 1) as u64 * (y1.abs_diff(y2) + 1) as u64
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn solution_2(input: &[String]) -> u64 {
    let tiles = parse_input(input).0;
    let pairs_len = (tiles.len() * (tiles.len() - 2)) / 2;
    let mut valid_tile_pairs = Vec::with_capacity(pairs_len);

    let mut border_tiles: Vec<(u32, u32)> = Vec::with_capacity(4096);

    {
        let mut copy_tiles = tiles.clone();
        copy_tiles.push(copy_tiles[0]);

        for [tile_1, tile_2] in copy_tiles.array_windows::<2>() {
            let (x1, y1) = tile_1;
            let (x2, y2) = tile_2;

            if x1 == x2 {
                let (small_y, big_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

                border_tiles.extend(((small_y + 1)..*big_y).map(|new_y| (*x2, new_y)));
            } else if y1 == y2 {
                // y1 == y2
                let (small_x, big_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                border_tiles.extend(((small_x + 1)..*big_x).map(|new_x| (new_x, *y2)));
            } else {
                unreachable!("the next tile isnt aligned, we should never arrive here")
            }
        }
    }

    for tile_1_i in 0..(tiles.len() - 1) {
        for tile_2_i in (tile_1_i + 1)..tiles.len() {
            let (x_1, y_1) = tiles[tile_1_i];
            let (x_2, y_2) = tiles[tile_2_i];

            let (small_x, bix_x) = if x_1 < x_2 { (x_1, x_2) } else { (x_2, x_1) };
            let (small_y, bix_y) = if y_1 < y_2 { (y_1, y_2) } else { (y_2, y_1) };

            // cehck if contains a corner tile inside the area that isnt a border
            let invalid_corner_inside = tiles.iter().any(|(other_x, other_y)| {
                ((small_x + 1)..bix_x).contains(other_x) && ((small_y + 1)..bix_y).contains(other_y)
            });

            if invalid_corner_inside {
                continue;
            }

            // cehck if contains a border tile inside the area that isnt a border
            let invalid_border_inside = border_tiles.iter().any(|(border_x, border_y)| {
                ((small_x + 1)..bix_x).contains(border_x)
                    && ((small_y + 1)..bix_y).contains(border_y)
            });

            if invalid_border_inside {
                continue;
            }

            valid_tile_pairs.push((tile_1_i, tile_2_i));
        }
    }

    valid_tile_pairs
        .into_iter()
        .map(|(valid_i_1, valid_i_2)| {
            let (x_1, y_1) = tiles[valid_i_1];
            let (x_2, y_2) = tiles[valid_i_2];
            (x_1.abs_diff(x_2) + 1) as u64 * (y_1.abs_diff(y_2) + 1) as u64
        })
        .max()
        .unwrap()
}

fn parse_input(input: &[String]) -> Tiles {
    Tiles::new(input)
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
